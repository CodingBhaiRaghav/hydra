//! Port of `src/main/services/hosters/filekeeper.ts`.
//!
//! FileKeeper runs XFileSharing: the file page shows a short countdown, then a
//! form post redirects to the file on its download host.

use std::collections::BTreeMap;
use std::sync::OnceLock;
use std::time::Duration;

use regex::Regex;
use reqwest::header::{COOKIE, LOCATION, REFERER, SET_COOKIE, USER_AGENT};
use reqwest::{redirect::Policy, Client, StatusCode};
use url::Url;

use crate::{HosterError, HosterResult};

pub const FILEKEEPER_ORIGIN: &str = "https://filekeeper.net";
const USER_AGENT_VALUE: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:144.0) Gecko/20100101 Firefox/144.0";
const MAX_REDIRECTS: usize = 5;
const MAX_COUNTDOWN_SECONDS: u64 = 30;

struct PageOutcome {
    status: StatusCode,
    url: String,
    /// True when FileKeeper redirected to another host, which serves the file.
    left_site: bool,
    body: Option<String>,
}

fn data_attribute_pattern(name: &str) -> Regex {
    Regex::new(&format!(r#"data-{name}="([^"]*)""#)).expect("valid data attribute pattern")
}

pub(crate) fn read_data_attribute(html: &str, name: &str) -> Option<String> {
    static CACHE: OnceLock<std::sync::Mutex<BTreeMap<String, Regex>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| std::sync::Mutex::new(BTreeMap::new()));

    let mut cache = cache.lock().expect("data attribute cache lock");
    let pattern = cache
        .entry(name.to_string())
        .or_insert_with(|| data_attribute_pattern(name));

    pattern
        .captures(html)
        .and_then(|captures| captures.get(1))
        .map(|matched| matched.as_str().to_string())
}

fn origin_of(url: &str) -> Option<String> {
    Url::parse(url).ok().map(|parsed| parsed.origin().ascii_serialization())
}

/// Manual redirect handling: FileKeeper's file code moves into a cookie and
/// the final redirect leaves the site entirely.
async fn request(
    client: &Client,
    url: &str,
    cookies: &mut BTreeMap<String, String>,
    form: Option<Vec<(&'static str, String)>>,
    referer: Option<&str>,
) -> HosterResult<PageOutcome> {
    let mut current_url = url.to_string();
    let mut pending_form = form;

    for _ in 0..=MAX_REDIRECTS {
        let cookie_header = cookies
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; ");

        let request = match &pending_form {
            Some(fields) => client.post(&current_url).form(fields),
            None => client.get(&current_url),
        };

        let mut request = request.header(USER_AGENT, USER_AGENT_VALUE);

        if !cookie_header.is_empty() {
            request = request.header(COOKIE, &cookie_header);
        }
        if let Some(referer) = referer {
            request = request.header(REFERER, referer);
        }

        let response = request.send().await?;

        for cookie in response.headers().get_all(SET_COOKIE) {
            let Ok(cookie) = cookie.to_str() else {
                continue;
            };
            let pair = cookie.split(';').next().unwrap_or_default();

            if let Some(separator) = pair.find('=') {
                cookies.insert(pair[..separator].to_string(), pair[separator + 1..].to_string());
            }
        }

        let location = response
            .headers()
            .get(LOCATION)
            .and_then(|value| value.to_str().ok())
            .map(str::to_string);

        let Some(location) = location else {
            let status = response.status();
            let body = response.text().await.ok();
            return Ok(PageOutcome {
                status,
                url: current_url,
                left_site: false,
                body,
            });
        };

        let next_url = Url::parse(&current_url)
            .ok()
            .and_then(|base| base.join(&location).ok())
            .map(|url| url.to_string())
            .ok_or_else(|| HosterError::message("FileKeeper returned an invalid redirect"))?;

        let status = response.status();
        drop(response);

        current_url = next_url;

        if origin_of(&current_url).as_deref() != Some(FILEKEEPER_ORIGIN) {
            return Ok(PageOutcome {
                status,
                url: current_url,
                left_site: true,
                body: None,
            });
        }

        // Redirects inside the site are plain GETs.
        pending_form = None;
    }

    Err(HosterError::message("FileKeeper redirected too many times"))
}

pub struct FileKeeperApi;

impl FileKeeperApi {
    /// A client that does not follow redirects, as the resolver inspects them.
    pub fn client() -> HosterResult<Client> {
        Client::builder()
            .redirect(Policy::none())
            .user_agent(USER_AGENT_VALUE)
            .build()
            .map_err(HosterError::from)
    }

    pub async fn get_download_url(uri: &str) -> HosterResult<String> {
        Self::get_download_url_with_client(&Self::client()?, uri).await
    }

    pub async fn get_download_url_with_client(client: &Client, uri: &str) -> HosterResult<String> {
        if origin_of(uri).as_deref() != Some(FILEKEEPER_ORIGIN) {
            return Err(HosterError::message("Invalid FileKeeper URL"));
        }

        let mut cookies: BTreeMap<String, String> = BTreeMap::new();
        let page = request(client, uri, &mut cookies, None, None).await?;

        if page.status == StatusCode::NOT_FOUND {
            return Err(HosterError::message("File not found or deleted"));
        }
        if !page.status.is_success() || page.left_site {
            return Err(HosterError::message(format!(
                "FileKeeper returned HTTP {}",
                page.status.as_u16()
            )));
        }

        let html = page.body.unwrap_or_default();
        let code = read_data_attribute(&html, "code")
            .ok_or_else(|| HosterError::message("Could not find the FileKeeper download form"))?;

        if read_data_attribute(&html, "has-password").as_deref() == Some("true") {
            return Err(HosterError::message("FileKeeper file is password protected"));
        }
        if read_data_attribute(&html, "has-captcha").as_deref() == Some("true") {
            return Err(HosterError::message(
                "FileKeeper requires a captcha for this file",
            ));
        }

        // Honour the page's countdown in case the server starts enforcing it.
        let countdown = read_data_attribute(&html, "countdown")
            .and_then(|value| value.parse::<f64>().ok())
            .unwrap_or(0.0)
            .max(0.0) as u64;
        let countdown = countdown.min(MAX_COUNTDOWN_SECONDS);

        if countdown > 0 {
            tokio::time::sleep(Duration::from_secs(countdown)).await;
        }

        let method = read_data_attribute(&html, "method")
            .filter(|method| !method.is_empty())
            .unwrap_or_else(|| "Free download".to_string());

        let form = vec![
            ("op", "download2".to_string()),
            ("id", code),
            (
                "rand",
                read_data_attribute(&html, "rand").unwrap_or_default(),
            ),
            (
                "referer",
                read_data_attribute(&html, "referer").unwrap_or_default(),
            ),
            ("method_free", method),
            ("down_direct", "1".to_string()),
        ];

        let page_url = page.url.clone();
        let result = request(client, &page_url, &mut cookies, Some(form), Some(&page_url)).await?;

        if !result.left_site {
            return Err(HosterError::message(
                "FileKeeper did not return a download link",
            ));
        }

        Ok(result.url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_data_attributes_from_the_file_page() {
        let html = r#"
            <form method="post">
              <input type="hidden" data-code="abc123" data-rand="r4nd" data-referer="" />
              <span data-countdown="7" data-has-password="false" data-has-captcha="false"
                    data-method="Free download"></span>
            </form>
        "#;

        assert_eq!(read_data_attribute(html, "code").as_deref(), Some("abc123"));
        assert_eq!(read_data_attribute(html, "rand").as_deref(), Some("r4nd"));
        assert_eq!(read_data_attribute(html, "referer").as_deref(), Some(""));
        assert_eq!(read_data_attribute(html, "countdown").as_deref(), Some("7"));
        assert_eq!(
            read_data_attribute(html, "has-password").as_deref(),
            Some("false")
        );
        assert_eq!(read_data_attribute(html, "missing"), None);
    }

    #[test]
    fn detects_the_site_origin() {
        assert_eq!(origin_of("https://filekeeper.net/abc").as_deref(), Some(FILEKEEPER_ORIGIN));
        assert_ne!(origin_of("https://example.com/abc").as_deref(), Some(FILEKEEPER_ORIGIN));
        assert_eq!(origin_of("not-a-url"), None);
    }
}
