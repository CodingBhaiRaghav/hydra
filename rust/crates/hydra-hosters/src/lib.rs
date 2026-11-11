//! Rust port of `src/main/services/hosters`.
//!
//! Each hoster resolves a share URL into a direct download URL. The TypeScript
//! implementations use `axios`/`node-fetch`; this port uses one `reqwest`
//! client so callers can share connections and proxy settings.

pub mod filekeeper;
pub mod mediafire;
pub mod pixeldrain;

pub use filekeeper::{FileKeeperApi, FILEKEEPER_ORIGIN};
pub use mediafire::MediafireApi;
pub use pixeldrain::PixelDrainApi;

use hydra_types::DownloadError;

#[derive(Debug, thiserror::Error)]
pub enum HosterError {
    /// A user-facing i18n key, thrown the way the TypeScript code throws it.
    #[error("{0}")]
    Download(#[from] DownloadError),
    #[error("invalid url: {0}")]
    InvalidUrl(String),
    #[error("{0}")]
    Message(String),
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
}

impl HosterError {
    pub fn message(message: impl Into<String>) -> Self {
        HosterError::Message(message.into())
    }
}

pub type HosterResult<T> = std::result::Result<T, HosterError>;

/// Default client: rustls TLS, HTTP/2, gzip. Redirects are followed unless a
/// resolver (FileKeeper) needs to inspect them itself.
pub fn http_client() -> HosterResult<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent("Hydra")
        .build()
        .map_err(HosterError::from)
}
