use serde::{Deserialize, Serialize};

use crate::{Downloader, GameShop, IsoDateTime};

/// `src/types/index.ts` `CatalogueImageVariants`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CatalogueImageVariants {
    pub base: String,
    pub widths: Vec<i64>,
    pub thumb_hash: Option<String>,
}

/// Resized CDN copies keyed by the original image URL.
pub type CatalogueImages = std::collections::BTreeMap<String, CatalogueImageVariants>;

/// `src/types/index.ts` `ShopAssets`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ShopAssets {
    pub object_id: String,
    pub shop: Option<GameShop>,
    pub title: String,
    pub icon_url: Option<String>,
    pub library_hero_image_url: Option<String>,
    pub library_image_url: Option<String>,
    pub logo_image_url: Option<String>,
    pub logo_position: Option<String>,
    pub cover_image_url: Option<String>,
    #[serde(default)]
    pub download_sources: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_artwork_types: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epic_artwork_updated_at_by_type: Option<std::collections::BTreeMap<String, i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub images: Option<CatalogueImages>,
}

/// `src/types/index.ts` `GameRepack`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GameRepack {
    pub id: String,
    pub title: String,
    pub file_size: Option<String>,
    pub uris: Vec<String>,
    /// Mirrors split into parts; one hoster's parts, in order, per group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri_groups: Option<Vec<Vec<String>>>,
    #[serde(default)]
    pub unavailable_uris: Vec<String>,
    pub upload_date: Option<IsoDateTime>,
    pub download_source_id: String,
    pub download_source_name: String,
    pub created_at: IsoDateTime,
}

/// `src/types/index.ts` `DownloadSource`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DownloadSource {
    pub id: String,
    pub name: String,
    pub url: String,
    pub status: Option<crate::DownloadSourceStatus>,
    pub download_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_remote: Option<bool>,
    pub created_at: IsoDateTime,
}

/// `src/types/index.ts` `ProtonVersion`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ProtonVersion {
    pub name: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<ProtonVersionSource>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtonVersionSource {
    Steam,
    CompatibilityTools,
    Unknown,
}

/// `src/types/index.ts` `StartGameDownloadPayload`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct StartGameDownloadPayload {
    pub object_id: String,
    pub title: String,
    pub shop: Option<GameShop>,
    pub uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<String>>,
    pub download_path: String,
    pub downloader: Option<Downloader>,
    pub automatically_extract: bool,
    pub automatically_delete_archive_files: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_indices: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_files_size: Option<i64>,
}

/// `src/types/index.ts` `DiskUsage`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DiskUsage {
    pub free: i64,
    pub total: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repack_keeps_uri_groups_optional() {
        let repack: GameRepack =
            serde_json::from_str(r#"{"id":"1","title":"A","uris":["magnet:?xt=1"]}"#).unwrap();
        assert_eq!(repack.uri_groups, None);
        assert_eq!(repack.unavailable_uris.len(), 0);
    }
}
