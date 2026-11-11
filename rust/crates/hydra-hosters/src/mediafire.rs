//! Port of `src/main/services/hosters/mediafire.ts`.

use std::sync::OnceLock;

use regex::Regex;
use reqwest::Client;

use crate::{HosterError, HosterResult};

pub struct MediafireApi;

fn identifier_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"(?m)^[a-zA-Z0-9]+$").expect("valid identifier pattern"))
}

fn pre_download_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        // The original uses lookbehind/lookahead for the surrounding quotes;
        // Rust's regex crate has neither, so the quotes are matched and the
        // URL is captured instead (behaviourally identical).
        Regex::new(
            r#"['"]((?:https?:)?(?://)?(?:www\.)?mediafire\.com/(?:file|view|download)/[^'"?]+\?dkey=[^'"]+)['"]"#,
        )
        .expect("valid mediafire pattern")
    })
}

fn dynamic_download_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(r#"['"](https?://download\d+\.mediafire\.com/[^'"]+)['"]"#)
            .expect("valid dynamic download pattern")
    })
}

fn http_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"(?m)^https?://").expect("valid http pattern"))
}

impl MediafireApi {
    pub fn process_url(url: &str) -> String {
        let mut processed = url.replace("http://", "https://");

        if identifier_pattern().is_match(&processed) {
            processed = format!("https://mediafire.com/?{processed}");
        }

        if !http_pattern().is_match(&processed) {
            processed = if processed.starts_with("//") {
                format!("https:{processed}")
            } else {
                format!("https://{processed}")
            };
        }

        processed
    }

    pub fn extract_direct_url(html: &str) -> HosterResult<String> {
        if let Some(captures) = pre_download_pattern().captures(html) {
            let matched = captures.get(1).map(|m| m.as_str()).unwrap_or_default();
            return Ok(if matched.starts_with("//") {
                format!("https:{matched}")
            } else {
                matched.to_string()
            });
        }

        if let Some(captures) = dynamic_download_pattern().captures(html) {
            if let Some(matched) = captures.get(1) {
                return Ok(matched.as_str().to_string());
            }
        }

        Err(HosterError::message("No valid download links found"))
    }

    pub async fn get_download_url(client: &Client, mediafire_url: &str) -> HosterResult<String> {
        let processed_url = Self::process_url(mediafire_url);

        let response = client
            .get(&processed_url)
            .send()
            .await
            .map_err(|_| HosterError::message("Failed to get download URL"))?;

        if !response.status().is_success() {
            return Err(HosterError::message("Failed to get download URL"));
        }

        let html = response
            .text()
            .await
            .map_err(|_| HosterError::message("Failed to get download URL"))?;

        Self::extract_direct_url(&html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_mediafire_inputs() {
        assert_eq!(
            MediafireApi::process_url("mediafire.com/file/abc"),
            "https://mediafire.com/file/abc"
        );
        assert_eq!(
            MediafireApi::process_url("http://mediafire.com/file/abc"),
            "https://mediafire.com/file/abc"
        );
        assert_eq!(
            MediafireApi::process_url("//mediafire.com/file/abc"),
            "https://mediafire.com/file/abc"
        );
        assert_eq!(
            MediafireApi::process_url("abc123"),
            "https://mediafire.com/?abc123"
        );
        assert_eq!(
            MediafireApi::process_url("https://mediafire.com/file/abc"),
            "https://mediafire.com/file/abc"
        );
    }

    #[test]
    fn extracts_the_page_download_link() {
        let html = r#"<a id="downloadButton" href="//www.mediafire.com/file/abc/game.rar?dkey=xyz" rel="nofollow"><span>Download</span></a>"#;
        assert_eq!(
            MediafireApi::extract_direct_url(html).unwrap(),
            "https://www.mediafire.com/file/abc/game.rar?dkey=xyz"
        );

        let html = r#"<a href="https://www.mediafire.com/file/abc/game.rar?dkey=xyz">Download</a>"#;
        assert_eq!(
            MediafireApi::extract_direct_url(html).unwrap(),
            "https://www.mediafire.com/file/abc/game.rar?dkey=xyz"
        );
    }

    #[test]
    fn falls_back_to_the_dynamic_download_host() {
        let html = r#"<script>window.location.href = "https://download2392.mediafire.com/abc/game.rar";</script>"#;
        assert_eq!(
            MediafireApi::extract_direct_url(html).unwrap(),
            "https://download2392.mediafire.com/abc/game.rar"
        );
    }

    #[test]
    fn rejects_pages_without_a_download_link() {
        let error = MediafireApi::extract_direct_url("<html><body>Not found</body></html>").unwrap_err();
        assert_eq!(error.to_string(), "No valid download links found");
    }
}
