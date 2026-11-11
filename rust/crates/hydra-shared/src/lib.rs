//! Rust port of `src/shared` (Pure helpers used by both the Electron main
//! process and the renderer).

pub mod archive_org;
pub mod bytes;
pub mod char_map;
pub mod download_directories;
pub mod format_bit_rate;
pub mod playtime;
pub mod routing;
pub mod screenshot_size;
pub mod tracker_list;
pub mod trim_trailing_slashes;

pub use archive_org::{
    is_archive_org_file_uri, parse_archive_org_file_uri, resolve_archive_org_file,
    ArchiveOrgFile, ResolvedArchiveOrgFile,
};
pub use bytes::{
    format_bytes, format_bytes_to_mbps, format_name, parse_bytes, remove_diacritics,
    remove_duplicate_spaces, remove_release_year_from_name, remove_symbols_from_name,
    replace_dots_with_space, replace_nbsp_with_space, replace_underscore_with_space,
};
pub use download_directories::{
    add_optional_download_directory, get_download_directory_preferences,
    get_download_directory_title, prepare_default_download_path_sync, remove_download_directory,
    replace_saved_download_directory_and_set_default, resolve_download_directories,
    set_default_download_directory, DownloadDirectoryError, DownloadDirectoryPreferences,
    PreparedDefaultDownloadPathSync, ResolvedDownloadDirectories, ReplaceResult,
    MAX_DOWNLOAD_DIRECTORIES, MAX_OPTIONAL_DOWNLOAD_DIRECTORIES,
};
pub use format_bit_rate::format_bit_rate;
pub use playtime::{
    get_displayed_play_time_in_milliseconds, get_play_time_hours_and_minutes,
    get_remote_play_time_parts, merge_local_and_remote_play_time, MergedPlayTime, PlayTimeParts,
    RemotePlayTimeInput, RemotePlayTimeParts, RuntimeByPlatform,
};
pub use routing::{
    apply_hoster_availability, collect_hoster_availability_uris, get_downloaders_for_uri,
    get_downloaders_for_uris, supports_hoster_availability_check, HosterAvailabilityResult,
    HOSTER_AVAILABILITY_MAX_URLS,
};
pub use screenshot_size::{fit_screenshot_to_1080p, ScreenshotSize, MAX_SCREENSHOT_HEIGHT};
pub use tracker_list::{is_valid_tracker_list_url, is_valid_tracker_url, parse_tracker_list};
pub use trim_trailing_slashes::trim_trailing_slashes;

/// `src/shared/index.ts` `UserNotLoggedInError`.
#[derive(Debug, thiserror::Error)]
#[error("user not logged in")]
pub struct UserNotLoggedInError;

/// `src/shared/index.ts` `SubscriptionRequiredError`.
#[derive(Debug, thiserror::Error)]
#[error("user does not have hydra cloud subscription")]
pub struct SubscriptionRequiredError;
