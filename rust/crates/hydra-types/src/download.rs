use serde::{Deserialize, Serialize};

use crate::{Download, IsoDateTime};

/// `src/types/download.types.ts` `DownloadProgress`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub download_speed: f64,
    pub time_remaining: f64,
    pub num_peers: i64,
    pub num_seeds: i64,
    pub is_downloading_metadata: bool,
    pub is_checking_files: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_reconnecting: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_recovering: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_progress: Option<f64>,
    pub progress: f64,
    pub game_id: String,
    pub download: Download,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_files_total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_files_downloaded: Option<i64>,
}

/// `src/types/index.ts` `TorrentFile`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TorrentFile {
    pub index: i64,
    pub path: String,
    pub length: i64,
}

/// `src/types/index.ts` `TorrentFilesResponse`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TorrentFilesResponse {
    pub info_hash: String,
    pub name: String,
    pub total_size: i64,
    pub files: Vec<TorrentFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_only: Option<bool>,
}

/// `src/types/download.types.ts` `SeedingStatus`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SeedingStatus {
    pub game_id: String,
    pub status: Option<crate::DownloadStatus>,
    pub upload_speed: f64,
}

/* ── TorBox ─────────────────────────────────────────────────────────────── */

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TorBoxUser {
    pub id: i64,
    pub email: String,
    pub plan: String,
    pub expiration: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TorBoxUserRequest {
    pub success: bool,
    pub detail: String,
    pub error: String,
    pub data: Option<TorBoxUser>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TorBoxFile {
    pub id: i64,
    pub md5: String,
    pub s3_path: String,
    pub name: String,
    pub size: i64,
    pub mimetype: String,
    pub short_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zipped: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TorBoxTorrentInfo {
    pub id: i64,
    pub hash: String,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
    pub magnet: String,
    pub size: i64,
    pub active: bool,
    pub cached: bool,
    pub auth_id: String,
    pub download_state: String,
    pub download_finished: bool,
    pub download_present: bool,
    pub seeds: i64,
    pub ratio: f64,
    pub progress: f64,
    pub download_speed: f64,
    pub upload_speed: f64,
    pub name: String,
    pub eta: f64,
    pub files: Vec<TorBoxFile>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TorBoxTorrentInfoRequest {
    pub success: bool,
    pub detail: String,
    pub error: String,
    pub data: Vec<TorBoxTorrentInfo>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TorBoxAddTorrentData {
    pub torrent_id: i64,
    pub name: String,
    pub hash: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TorBoxAddTorrentRequest {
    pub success: bool,
    pub detail: String,
    pub error: String,
    pub data: Option<TorBoxAddTorrentData>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TorBoxRequestLinkRequest {
    pub success: bool,
    pub detail: String,
    pub error: String,
    pub data: Option<String>,
}

/* ── Real-Debrid ────────────────────────────────────────────────────────── */

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RealDebridUnrestrictLink {
    pub id: String,
    pub filename: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub filesize: i64,
    pub link: String,
    pub host: String,
    pub host_icon: String,
    pub chunks: i64,
    pub crc: i64,
    pub download: String,
    pub streamable: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RealDebridAddMagnet {
    pub id: String,
    pub uri: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RealDebridTorrentFile {
    pub id: i64,
    pub path: String,
    pub bytes: i64,
    pub selected: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RealDebridTorrentInfo {
    pub id: String,
    pub filename: String,
    pub original_filename: String,
    pub hash: String,
    pub bytes: i64,
    pub original_bytes: i64,
    pub host: String,
    pub split: i64,
    pub progress: f64,
    pub status: String,
    pub added: IsoDateTime,
    pub files: Vec<RealDebridTorrentFile>,
    pub links: Vec<String>,
    pub ended: Option<IsoDateTime>,
    pub speed: Option<f64>,
    pub seeders: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RealDebridUser {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub points: i64,
    pub locale: String,
    pub avatar: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub premium: i64,
    pub expiration: IsoDateTime,
}

/* ── Premiumize / AllDebrid ─────────────────────────────────────────────── */

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PremiumizeUser {
    pub customer_id: String,
    pub premium_until: f64,
    pub space_used: f64,
    pub limit_used: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AllDebridUser {
    pub username: String,
    pub email: String,
    #[serde(rename = "isPremium")]
    pub is_premium: bool,
    #[serde(rename = "premiumUntil")]
    pub premium_until: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_debrid_torrent_accepts_null_optional_fields() {
        let json = serde_json::json!({
            "id": "abc",
            "filename": "game.rar",
            "original_filename": "game.rar",
            "hash": "hash",
            "bytes": 100,
            "original_bytes": 100,
            "host": "real-debrid.com",
            "split": 100,
            "progress": 100,
            "status": "downloaded",
            "added": "2026-01-01T00:00:00.000Z",
            "files": [{ "id": 1, "path": "/game.rar", "bytes": 100, "selected": 1 }],
            "links": ["https://example.test/link"]
        });

        let info: RealDebridTorrentInfo = serde_json::from_value(json).unwrap();
        assert_eq!(info.files.len(), 1);
        assert_eq!(info.ended, None);
    }
}
