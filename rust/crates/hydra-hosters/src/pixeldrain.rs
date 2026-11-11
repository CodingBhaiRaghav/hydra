//! Port of `src/main/services/hosters/pixeldrain.ts`.

use hydra_types::DownloadError;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

use crate::{HosterError, HosterResult};

pub const API_BASE_URL: &str = "https://pixeldrain.com/api/file";

#[derive(Debug, Default, Deserialize)]
struct PixelDrainFileInfo {
    availability: Option<String>,
    can_download: Option<bool>,
}

pub struct PixelDrainApi;

impl PixelDrainApi {
    pub fn can_handle(url: &str) -> bool {
        Url::parse(url)
            .ok()
            .and_then(|parsed| parsed.host_str().map(|host| host.contains("pixeldrain.com")))
            .unwrap_or(false)
    }

    pub fn extract_id(url: &str) -> HosterResult<String> {
        let parsed = Url::parse(url)
            .map_err(|_| HosterError::InvalidUrl(format!("Invalid pixeldrain URL: {url}")))?;

        let path_parts: Vec<&str> = parsed
            .path()
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect();

        match (path_parts.first(), path_parts.get(1)) {
            (Some(&"u"), Some(id)) if !id.is_empty() => Ok((*id).to_string()),
            _ => Err(HosterError::InvalidUrl(format!(
                "Invalid pixeldrain URL: {url}"
            ))),
        }
    }

    pub async fn check_availability(client: &Client, id: &str) -> HosterResult<()> {
        let response = client
            .get(format!("{API_BASE_URL}/{id}/info"))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        let info: PixelDrainFileInfo = serde_json::from_str(&body).unwrap_or_default();

        if status.as_u16() == 404 {
            return Err(HosterError::Download(DownloadError::HosterUnlockFileNotFound));
        }

        if status.as_u16() >= 400 || info.availability.is_some() || info.can_download == Some(false)
        {
            return Err(HosterError::Download(DownloadError::HosterUnlockUnavailable));
        }

        Ok(())
    }

    pub async fn unlock(client: &Client, url: &str) -> HosterResult<String> {
        let id = Self::extract_id(url)?;
        Self::check_availability(client, &id).await?;
        Ok(format!("{API_BASE_URL}/{id}?download"))
    }

    pub async fn get_download_url(client: &Client, url: &str) -> HosterResult<String> {
        Self::unlock(client, url).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_pixeldrain_hosts() {
        assert!(PixelDrainApi::can_handle("https://pixeldrain.com/u/abc123"));
        assert!(!PixelDrainApi::can_handle("https://example.com/u/abc123"));
        assert!(!PixelDrainApi::can_handle("not-a-url"));
    }

    #[test]
    fn extracts_ids_from_single_file_urls() {
        assert_eq!(
            PixelDrainApi::extract_id("https://pixeldrain.com/u/abc123").unwrap(),
            "abc123"
        );
        assert_eq!(
            PixelDrainApi::extract_id("https://pixeldrain.com/u/abc123/").unwrap(),
            "abc123"
        );
    }

    #[test]
    fn rejects_album_and_short_urls() {
        assert!(PixelDrainApi::extract_id("https://pixeldrain.com/l/abc123").is_err());
        assert!(PixelDrainApi::extract_id("https://pixeldrain.com/u/").is_err());
        assert!(PixelDrainApi::extract_id("nonsense").is_err());
    }

    #[test]
    fn errors_deserialize_into_i18n_keys() {
        let error = HosterError::from(DownloadError::HosterUnlockUnavailable);
        assert_eq!(error.to_string(), "download_error_hoster_unlock_unavailable");
    }
}
