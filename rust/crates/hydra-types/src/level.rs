use serde::{Deserialize, Serialize};

use crate::{
    ClassicsDisc, DownloadStatus, Downloader, GameShop, GameSource, IsoDateTime,
};

/// `src/types/level.types.ts` `SubscriptionStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Active,
    Pending,
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethod {
    Pix,
    Paypal,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SubscriptionPlan {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: String,
    pub status: SubscriptionStatus,
    pub plan: SubscriptionPlan,
    pub expires_at: Option<IsoDateTime>,
    pub payment_method: PaymentMethod,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Auth {
    pub access_token: String,
    pub refresh_token: String,
    pub token_expiration_timestamp: i64,
    pub workwonders_jwt: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub profile_image_url: Option<String>,
    pub background_image_url: Option<String>,
    pub subscription: Option<Subscription>,
}

/// `src/types/level.types.ts` `CatalogueGameIdentity`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogueGameIdentity {
    pub shop: GameShop,
    pub object_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LibraryCatalogueMetadata {
    pub canonical_id: Option<String>,
    pub catalogue_game: Option<CatalogueGameIdentity>,
    pub available_shops: Option<Vec<GameShop>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpicImportScope {
    pub environment: String,
    pub user_id: String,
    pub connection_id: String,
}

/// `src/types/level.types.ts` `Game`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Game {
    #[serde(flatten)]
    pub catalogue: LibraryCatalogueMetadata,
    pub title: String,
    pub icon_url: Option<String>,
    pub library_hero_image_url: Option<String>,
    pub logo_image_url: Option<String>,
    pub custom_icon_url: Option<String>,
    pub custom_logo_image_url: Option<String>,
    pub custom_hero_image_url: Option<String>,
    pub custom_cover_image_url: Option<String>,
    pub original_icon_path: Option<String>,
    pub original_logo_path: Option<String>,
    pub original_hero_path: Option<String>,
    pub custom_original_icon_path: Option<String>,
    pub custom_original_logo_path: Option<String>,
    pub custom_original_hero_path: Option<String>,
    pub custom_original_cover_path: Option<String>,
    pub play_time_in_milliseconds: i64,
    pub steam_play_time_in_milliseconds: Option<i64>,
    pub epic_play_time_in_milliseconds: Option<i64>,
    /// `EpicInstallation` currently lives in `epic-managed.types.ts`; kept as
    /// raw JSON until that module is ported.
    pub epic_installation: Option<serde_json::Value>,
    pub has_active_epic_import: Option<bool>,
    pub epic_import_scope: Option<EpicImportScope>,
    pub has_active_steam_import: Option<bool>,
    pub enable_hydra_playtime_tracking: Option<bool>,
    pub unsynced_delta_play_time_in_milliseconds: Option<i64>,
    pub last_time_played: Option<IsoDateTime>,
    pub added_to_library_at: Option<IsoDateTime>,
    pub object_id: String,
    pub shop: GameShop,
    pub remote_id: Option<String>,
    pub source: Option<GameSource>,
    pub collection_ids: Option<Vec<String>>,
    pub is_deleted: bool,
    pub wine_prefix_path: Option<String>,
    pub proton_path: Option<String>,
    pub executable_path: Option<String>,
    pub executable_path_updated_at: Option<IsoDateTime>,
    pub tracking_executable_paths: Option<Vec<String>>,
    pub tracking_executable_paths_updated_at: Option<IsoDateTime>,
    pub launch_options: Option<String>,
    pub auto_run_mangohud: Option<bool>,
    pub auto_run_gamemode: Option<bool>,
    pub favorite: Option<bool>,
    pub is_hidden_from_others: Option<bool>,
    pub is_concealed: Option<bool>,
    pub is_pinned: Option<bool>,
    pub achievement_count: Option<i64>,
    pub unlocked_achievement_count: Option<i64>,
    pub reported_unlocked_achievement_count: Option<i64>,
    pub pinned_date: Option<IsoDateTime>,
    pub automatic_cloud_sync: Option<bool>,
    pub has_manually_updated_playtime: Option<bool>,
    pub new_download_options_count: Option<i64>,
    pub installed_size_in_bytes: Option<i64>,
    pub installer_size_in_bytes: Option<i64>,
    pub steam_shortcut_app_id: Option<i64>,
    pub platform: Option<String>,
    pub discs: Option<Vec<ClassicsDisc>>,
    pub selected_disc_path: Option<String>,
    pub dont_ask_disc_selection: Option<bool>,
    pub rom_size_bytes: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DownloadFile {
    pub index: i64,
    pub path: String,
    pub size: i64,
    pub bytes_downloaded: i64,
    pub completed: bool,
}

/// `src/types/level.types.ts` `Download`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Download {
    /// `EpicDownloadMetadata` is ported with the Epic manifest module.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epic: Option<serde_json::Value>,
    pub shop: GameShop,
    pub object_id: String,
    pub uri: String,
    /// Ordered part URIs of a multi-part hoster download; `uri` is the first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<String>>,
    pub folder_name: Option<String>,
    pub download_path: String,
    pub progress: f64,
    pub downloader: Downloader,
    pub bytes_downloaded: i64,
    pub file_size: Option<i64>,
    pub should_seed: bool,
    pub status: Option<DownloadStatus>,
    pub queued: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_to_hero: Option<bool>,
    pub timestamp: i64,
    pub extracting: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extraction_progress: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installer_launched: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installer_running: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installer_started_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installer_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_installer: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_archive_name: Option<String>,
    /// Archives Hydra started extracting in place; only these are resumed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_place_archives: Option<Vec<String>>,
    pub automatically_extract: bool,
    pub automatically_delete_archive_files: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_indices: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<DownloadFile>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_files_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub real_debrid_torrent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awaiting_debrid: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debrid_auto_resume: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debrid_queue_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debrid_queue_position: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debrid_preparation_deadline: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_trackers: Option<Vec<String>>,
}

/// `src/types/level.types.ts` `DownloadLayoutState`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DownloadLayoutState {
    pub version: u8,
    pub queue_order: Vec<String>,
    pub paused_order: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NotificationPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadDirectorySource {
    Manual,
    Auto,
}

impl Default for DownloadDirectorySource {
    fn default() -> Self {
        DownloadDirectorySource::Manual
    }
}

/// `src/types/level.types.ts` `DownloadDirectoryPreference`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadDirectoryPreference {
    pub path: String,
    pub created_at: IsoDateTime,
    pub source: DownloadDirectorySource,
}

/// `src/types/level.types.ts` `UserPreferences`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct UserPreferences {
    pub downloads_path: Option<String>,
    pub default_wine_prefix_path: Option<String>,
    pub download_directories: Option<Vec<DownloadDirectoryPreference>>,
    pub optional_downloads_paths: Option<Vec<String>>,
    pub gg_deals_api_key: Option<String>,
    pub language: Option<String>,
    pub real_debrid_api_token: Option<String>,
    pub premiumize_api_token: Option<String>,
    pub all_debrid_api_token: Option<String>,
    pub tor_box_api_token: Option<String>,
    pub retro_achievements_web_api_key: Option<String>,
    pub retro_achievements_username: Option<String>,
    pub prefer_quit_instead_of_hiding: Option<bool>,
    pub open_friends_window_on_startup: Option<bool>,
    pub run_at_startup: Option<bool>,
    pub start_minimized: Option<bool>,
    pub launch_to_library_page: Option<bool>,
    pub big_picture_launch_to_library_page: Option<bool>,
    pub launch_in_big_picture: Option<bool>,
    pub disable_nsfw_alert: Option<bool>,
    pub enable_auto_install: Option<bool>,
    pub seed_after_download_complete: Option<bool>,
    pub show_hidden_achievements_description: Option<bool>,
    pub show_download_speed_in_megabits: Option<bool>,
    pub download_notifications_enabled: Option<bool>,
    pub repack_updates_notifications_enabled: Option<bool>,
    pub achievement_notifications_enabled: Option<bool>,
    pub achievement_custom_notifications_enabled: Option<bool>,
    pub achievement_custom_notification_position: Option<NotificationPosition>,
    pub achievement_sound_volume: Option<f64>,
    pub friend_request_notifications_enabled: Option<bool>,
    pub friend_start_game_notifications_enabled: Option<bool>,
    pub chat_message_notifications_enabled: Option<bool>,
    pub leaderboard_slam_enabled: Option<bool>,
    pub leaderboard_drop_notifications_enabled: Option<bool>,
    pub show_download_speed_in_megabytes: Option<bool>,
    pub extract_files_by_default: Option<bool>,
    pub delete_archive_files_after_extraction_by_default: Option<bool>,
    pub enable_steam_achievements: Option<bool>,
    pub enable_achievement_souvenirs: Option<bool>,
    pub achievement_screenshots_path: Option<String>,
    pub autoplay_game_trailers: Option<bool>,
    pub hide_to_tray_on_game_start: Option<bool>,
    pub use_native_window_decorations: Option<bool>,
    pub enable_new_download_options_badges: Option<bool>,
    pub create_start_menu_shortcut: Option<bool>,
    pub big_picture_sounds_enabled: Option<bool>,
    pub big_picture_virtual_keyboard_enabled: Option<bool>,
    pub big_picture_diagnostics_enabled: Option<bool>,
    pub big_picture_diagnostics_position: Option<NotificationPosition>,
    pub max_download_speed_bytes_per_second: Option<i64>,
    pub torrent_network_interface: Option<String>,
    pub global_trackers: Option<Vec<String>>,
    pub append_global_trackers: Option<bool>,
    pub global_trackers_url: Option<String>,
    pub append_global_trackers_url: Option<bool>,
    pub default_proton_path: Option<String>,
    pub auto_run_mangohud: Option<bool>,
    pub auto_run_gamemode: Option<bool>,
    pub hide_classics_bookmark: Option<bool>,
    pub classics_use_hero_layout: Option<bool>,
    pub hide_library_game_badges: Option<bool>,
    pub hide_library_ready_size_badges: Option<bool>,
    pub hide_library_classics_badges: Option<bool>,
    pub hide_steam_library_badges: Option<bool>,
    pub hide_epic_library_badges: Option<bool>,
    pub hide_library_achievement_progress: Option<bool>,
    pub autoplay_animated_artwork: Option<bool>,
    pub persist_filters_and_sorting: Option<bool>,
    pub screenshots_enabled: Option<bool>,
    pub screenshots_folder_path: Option<String>,
    /// `ScreenshotHotkeyBinding` is ported with the screenshot module.
    pub screenshot_hotkey: Option<serde_json::Value>,
    pub screenshot_gamepad_enabled: Option<bool>,
    pub screenshot_gamepad_combo: Option<Vec<serde_json::Value>>,
    pub screenshot_capture_sound_enabled: Option<bool>,
    pub screenshot_toast_enabled: Option<bool>,
    pub screenshot_default_visibility: Option<crate::ProfileVisibility>,
    pub show_sensitive_screenshots: Option<bool>,
    pub hide_community_screenshots: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NetworkInterface {
    pub name: String,
    pub addresses: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ScreenState {
    pub x: Option<i64>,
    pub y: Option<i64>,
    pub height: i64,
    pub width: i64,
    pub is_maximized: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_round_trips_camel_case_json() {
        let json = serde_json::json!({
            "title": "Celeste",
            "objectId": "504230",
            "shop": "steam",
            "playTimeInMilliseconds": 3_600_000,
            "isDeleted": false,
            "lastTimePlayed": "2026-01-01T10:00:00.000Z",
            "discs": [{ "path": "/roms/celeste.bin", "label": "Disc 1", "fileName": "celeste.bin" }]
        });

        let game: Game = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(game.title, "Celeste");
        assert_eq!(game.play_time_in_milliseconds, 3_600_000);
        assert_eq!(game.discs.as_ref().unwrap().len(), 1);

        let serialized = serde_json::to_value(&game).unwrap();
        assert_eq!(serialized["lastTimePlayed"], "2026-01-01T10:00:00.000Z");
    }

    #[test]
    fn download_preserves_downloader_discriminant() {
        let download = Download {
            shop: GameShop::Steam,
            object_id: "1".into(),
            uri: "magnet:?xt=urn:btih:abc".into(),
            download_path: "/downloads".into(),
            progress: 0.0,
            downloader: Downloader::Torrent,
            bytes_downloaded: 0,
            file_size: None,
            should_seed: false,
            status: Some(DownloadStatus::Active),
            queued: false,
            timestamp: 1,
            extracting: false,
            automatically_extract: true,
            automatically_delete_archive_files: false,
            epic: None,
            parts: None,
            folder_name: None,
            pinned_to_hero: None,
            extraction_progress: None,
            installer_launched: None,
            installer_running: None,
            installer_started_at: None,
            installer_path: None,
            has_installer: None,
            source_archive_name: None,
            in_place_archives: None,
            file_indices: None,
            files: None,
            selected_files_size: None,
            real_debrid_torrent_id: None,
            awaiting_debrid: None,
            debrid_auto_resume: None,
            debrid_queue_only: None,
            debrid_queue_position: None,
            debrid_preparation_deadline: None,
            custom_trackers: None,
        };

        let value = serde_json::to_value(&download).unwrap();
        assert_eq!(value["downloader"], 1);
        assert_eq!(value["status"], "active");
    }
}
