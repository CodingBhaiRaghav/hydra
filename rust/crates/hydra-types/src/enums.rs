use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;

/// `src/shared/constants.ts` `Downloader`.
///
/// The numeric values are part of persisted data, so they must never change.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum Downloader {
    RealDebrid = 0,
    Torrent = 1,
    Gofile = 2,
    PixelDrain = 3,
    Datanodes = 4,
    Mediafire = 5,
    TorBox = 6,
    Hydra = 7,
    FuckingFast = 9,
    VikingFile = 10,
    Rootz = 11,
    Premiumize = 12,
    AllDebrid = 13,
    ArchiveOrg = 14,
    Epic = 15,
    FileKeeper = 16,
}

impl Downloader {
    pub const ALL: [Downloader; 16] = [
        Downloader::RealDebrid,
        Downloader::Torrent,
        Downloader::Gofile,
        Downloader::PixelDrain,
        Downloader::Datanodes,
        Downloader::Mediafire,
        Downloader::TorBox,
        Downloader::Hydra,
        Downloader::FuckingFast,
        Downloader::VikingFile,
        Downloader::Rootz,
        Downloader::Premiumize,
        Downloader::AllDebrid,
        Downloader::ArchiveOrg,
        Downloader::Epic,
        Downloader::FileKeeper,
    ];

    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Downloader::RealDebrid),
            1 => Some(Downloader::Torrent),
            2 => Some(Downloader::Gofile),
            3 => Some(Downloader::PixelDrain),
            4 => Some(Downloader::Datanodes),
            5 => Some(Downloader::Mediafire),
            6 => Some(Downloader::TorBox),
            7 => Some(Downloader::Hydra),
            9 => Some(Downloader::FuckingFast),
            10 => Some(Downloader::VikingFile),
            11 => Some(Downloader::Rootz),
            12 => Some(Downloader::Premiumize),
            13 => Some(Downloader::AllDebrid),
            14 => Some(Downloader::ArchiveOrg),
            15 => Some(Downloader::Epic),
            16 => Some(Downloader::FileKeeper),
            _ => None,
        }
    }
}

impl Serialize for Downloader {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(self.as_i32())
    }
}

impl<'de> Deserialize<'de> for Downloader {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = i32::deserialize(deserializer)?;
        Downloader::from_i32(value).ok_or_else(|| {
            serde::de::Error::custom(format!("unknown Downloader value: {value}"))
        })
    }
}

/// `src/shared/constants.ts` `DownloadSourceStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DownloadSourceStatus {
    PendingMatching,
    Matched,
    Matching,
    Failed,
    Dead,
}

/// `src/types/game.types.ts` `GameShop`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameShop {
    Steam,
    Epic,
    Custom,
    Launchbox,
}

impl GameShop {
    pub const fn as_str(self) -> &'static str {
        match self {
            GameShop::Steam => "steam",
            GameShop::Epic => "epic",
            GameShop::Custom => "custom",
            GameShop::Launchbox => "launchbox",
        }
    }
}

impl fmt::Display for GameShop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The shop a game record was imported from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameSource {
    Hydra,
    Steam,
    Epic,
}

/// `src/types/download.types.ts` `DownloadStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadStatus {
    Active,
    /// Legacy/raw compatibility only; not part of the official contract.
    Waiting,
    Paused,
    Error,
    Complete,
    Seeding,
    Removed,
    /// Legacy/raw compatibility only; extraction state lives on `Download`.
    Extracting,
}

/// `src/types/game.types.ts` `ShortcutLocation`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShortcutLocation {
    Desktop,
    StartMenu,
}

/// `src/shared/constants.ts` `CatalogueCategory`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CatalogueCategory {
    Hot,
    Weekly,
    Achievements,
}

/// `src/shared/constants.ts` `SteamContentDescriptor`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SteamContentDescriptor {
    SomeNudityOrSexualContent = 1,
    FrequenceViolenceOrGore = 2,
    AdultOnlySexualContent = 3,
    FrequentNudityOrSexualContent = 4,
    GeneralMatureContent = 5,
}

/// `src/shared/constants.ts` `Cracker`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cracker {
    #[serde(rename = "CODEX")]
    Codex,
    #[serde(rename = "RUNE")]
    Rune,
    #[serde(rename = "OnlineFix")]
    OnlineFix,
    #[serde(rename = "Goldberg")]
    Goldberg,
    #[serde(rename = "user_stats")]
    UserStats,
    #[serde(rename = "Steam")]
    Steam,
    #[serde(rename = "RLD!")]
    Rld,
    #[serde(rename = "EMPRESS")]
    Empress,
    #[serde(rename = "SKIDROW")]
    Skidrow,
    #[serde(rename = "CreamAPI")]
    CreamApi,
    #[serde(rename = "SmartSteamEmu")]
    SmartSteamEmu,
    #[serde(rename = "3dm")]
    ThreeDm,
    #[serde(rename = "FLT")]
    Flt,
    #[serde(rename = "RLE")]
    Rle,
    #[serde(rename = "RAZOR1911")]
    Razor1911,
    #[serde(rename = "ALI213")]
    Ali213,
}

/// `src/shared/constants.ts` `AuthPage`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthPage {
    #[serde(rename = "/")]
    SignIn,
    #[serde(rename = "/update-email")]
    UpdateEmail,
    #[serde(rename = "/update-password")]
    UpdatePassword,
}

impl AuthPage {
    pub const fn as_str(self) -> &'static str {
        match self {
            AuthPage::SignIn => "/",
            AuthPage::UpdateEmail => "/update-email",
            AuthPage::UpdatePassword => "/update-password",
        }
    }
}

/// `src/types/index.ts` `ProfileVisibility`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProfileVisibility {
    Public,
    Private,
    Friends,
}

/// `src/shared/constants.ts` `DownloadError`.
///
/// Serialized as the i18n key the TypeScript code throws.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DownloadError {
    NotCachedOnRealDebrid,
    RealDebridTorrentNotReady,
    RealDebridLinksNotReady,
    NotCachedOnTorBox,
    GofileQuotaExceeded,
    RealDebridAccountNotAuthorized,
    NotCachedOnHydra,
    NotCachedOnPremiumize,
    PremiumizeTransferStarted,
    NotCachedOnAllDebrid,
    PremiumizeAccountNotAuthorized,
    AllDebridAccountNotAuthorized,
    PremiumizeRateLimitExceeded,
    AllDebridRateLimitExceeded,
    PremiumizeUnavailable,
    AllDebridUnavailable,
    VikingFileQuotaExceeded,
    VikingFileSubscriptionRequired,
    HosterUnlockLoginRequired,
    HosterUnlockFileNotFound,
    HosterUnlockUnavailable,
    DownloadLinkReturnedWebPage,
    InvalidMagnet,
    TorrentMetadataTimeout,
    TorrentMetadataIncomplete,
    TorrentNoFilesSelected,
    TorrentInvalidFileSelection,
    TorrentTooManyFiles,
    TorrentFilesUnavailable,
    TorrentInvalidTrackers,
    ArchiveOrgInvalidFileUrl,
    TorBoxAccountNotAuthorized,
    TorBoxRateLimitExceeded,
    TorBoxUnavailable,
    TorBoxTorrentNotReady,
    TorBoxLinkUnavailable,
    NetworkUnreachable,
    NetworkCertificateRejected,
}

impl DownloadError {
    pub const fn as_str(self) -> &'static str {
        match self {
            DownloadError::NotCachedOnRealDebrid => "download_error_not_cached_on_real_debrid",
            DownloadError::RealDebridTorrentNotReady => {
                "download_error_real_debrid_torrent_not_ready"
            }
            DownloadError::RealDebridLinksNotReady => "download_error_real_debrid_links_not_ready",
            DownloadError::NotCachedOnTorBox => "download_error_not_cached_on_torbox",
            DownloadError::GofileQuotaExceeded => "download_error_gofile_quota_exceeded",
            DownloadError::RealDebridAccountNotAuthorized => {
                "download_error_real_debrid_account_not_authorized"
            }
            DownloadError::NotCachedOnHydra => "download_error_not_cached_on_hydra",
            DownloadError::NotCachedOnPremiumize => "download_error_not_cached_on_premiumize",
            DownloadError::PremiumizeTransferStarted => {
                "download_error_premiumize_transfer_started"
            }
            DownloadError::NotCachedOnAllDebrid => "download_error_not_cached_on_alldebrid",
            DownloadError::PremiumizeAccountNotAuthorized => {
                "download_error_premiumize_account_not_authorized"
            }
            DownloadError::AllDebridAccountNotAuthorized => {
                "download_error_alldebrid_account_not_authorized"
            }
            DownloadError::PremiumizeRateLimitExceeded => {
                "download_error_premiumize_rate_limit_exceeded"
            }
            DownloadError::AllDebridRateLimitExceeded => {
                "download_error_alldebrid_rate_limit_exceeded"
            }
            DownloadError::PremiumizeUnavailable => "download_error_premiumize_unavailable",
            DownloadError::AllDebridUnavailable => "download_error_alldebrid_unavailable",
            DownloadError::VikingFileQuotaExceeded => "download_error_vikingfile_quota_exceeded",
            DownloadError::VikingFileSubscriptionRequired => {
                "download_error_vikingfile_subscription_required"
            }
            DownloadError::HosterUnlockLoginRequired => {
                "download_error_hoster_unlock_login_required"
            }
            DownloadError::HosterUnlockFileNotFound => {
                "download_error_hoster_unlock_file_not_found"
            }
            DownloadError::HosterUnlockUnavailable => "download_error_hoster_unlock_unavailable",
            DownloadError::DownloadLinkReturnedWebPage => {
                "download_error_link_returned_web_page"
            }
            DownloadError::InvalidMagnet => "download_error_invalid_magnet",
            DownloadError::TorrentMetadataTimeout => "download_error_torrent_metadata_timeout",
            DownloadError::TorrentMetadataIncomplete => {
                "download_error_torrent_metadata_incomplete"
            }
            DownloadError::TorrentNoFilesSelected => "download_error_torrent_no_files_selected",
            DownloadError::TorrentInvalidFileSelection => {
                "download_error_torrent_invalid_file_selection"
            }
            DownloadError::TorrentTooManyFiles => "download_error_torrent_too_many_files",
            DownloadError::TorrentFilesUnavailable => "download_error_torrent_files_unavailable",
            DownloadError::TorrentInvalidTrackers => "download_error_torrent_invalid_trackers",
            DownloadError::ArchiveOrgInvalidFileUrl => "download_error_archive_org_invalid_file_url",
            DownloadError::TorBoxAccountNotAuthorized => {
                "download_error_torbox_account_not_authorized"
            }
            DownloadError::TorBoxRateLimitExceeded => "download_error_torbox_rate_limit_exceeded",
            DownloadError::TorBoxUnavailable => "download_error_torbox_unavailable",
            DownloadError::TorBoxTorrentNotReady => "download_error_torbox_torrent_not_ready",
            DownloadError::TorBoxLinkUnavailable => "download_error_torbox_link_unavailable",
            DownloadError::NetworkUnreachable => "download_error_network_unreachable",
            DownloadError::NetworkCertificateRejected => {
                "download_error_network_certificate_rejected"
            }
        }
    }
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::error::Error for DownloadError {}

impl Serialize for DownloadError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DownloadError {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        DownloadError::ALL
            .iter()
            .copied()
            .find(|error| error.as_str() == value)
            .ok_or_else(|| serde::de::Error::custom(format!("unknown DownloadError: {value}")))
    }
}

impl DownloadError {
    pub const ALL: [DownloadError; 38] = [
        DownloadError::NotCachedOnRealDebrid,
        DownloadError::RealDebridTorrentNotReady,
        DownloadError::RealDebridLinksNotReady,
        DownloadError::NotCachedOnTorBox,
        DownloadError::GofileQuotaExceeded,
        DownloadError::RealDebridAccountNotAuthorized,
        DownloadError::NotCachedOnHydra,
        DownloadError::NotCachedOnPremiumize,
        DownloadError::PremiumizeTransferStarted,
        DownloadError::NotCachedOnAllDebrid,
        DownloadError::PremiumizeAccountNotAuthorized,
        DownloadError::AllDebridAccountNotAuthorized,
        DownloadError::PremiumizeRateLimitExceeded,
        DownloadError::AllDebridRateLimitExceeded,
        DownloadError::PremiumizeUnavailable,
        DownloadError::AllDebridUnavailable,
        DownloadError::VikingFileQuotaExceeded,
        DownloadError::VikingFileSubscriptionRequired,
        DownloadError::HosterUnlockLoginRequired,
        DownloadError::HosterUnlockFileNotFound,
        DownloadError::HosterUnlockUnavailable,
        DownloadError::DownloadLinkReturnedWebPage,
        DownloadError::InvalidMagnet,
        DownloadError::TorrentMetadataTimeout,
        DownloadError::TorrentMetadataIncomplete,
        DownloadError::TorrentNoFilesSelected,
        DownloadError::TorrentInvalidFileSelection,
        DownloadError::TorrentTooManyFiles,
        DownloadError::TorrentFilesUnavailable,
        DownloadError::TorrentInvalidTrackers,
        DownloadError::ArchiveOrgInvalidFileUrl,
        DownloadError::TorBoxAccountNotAuthorized,
        DownloadError::TorBoxRateLimitExceeded,
        DownloadError::TorBoxUnavailable,
        DownloadError::TorBoxTorrentNotReady,
        DownloadError::TorBoxLinkUnavailable,
        DownloadError::NetworkUnreachable,
        DownloadError::NetworkCertificateRejected,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn downloader_numbers_round_trip() {
        for downloader in Downloader::ALL {
            assert_eq!(Downloader::from_i32(downloader.as_i32()), Some(downloader));
        }
        // 8 is intentionally absent in the TypeScript enum.
        assert_eq!(Downloader::from_i32(8), None);
    }

    #[test]
    fn downloader_serializes_as_number() {
        assert_eq!(serde_json::to_string(&Downloader::Hydra).unwrap(), "7");
        assert_eq!(
            serde_json::from_str::<Downloader>("14").unwrap(),
            Downloader::ArchiveOrg
        );
    }

    #[test]
    fn shop_serializes_lowercase() {
        assert_eq!(serde_json::to_string(&GameShop::Launchbox).unwrap(), "\"launchbox\"");
    }

    #[test]
    fn download_error_keys_match_i18n_keys() {
        assert_eq!(
            DownloadError::HosterUnlockFileNotFound.as_str(),
            "download_error_hoster_unlock_file_not_found"
        );
        assert_eq!(DownloadError::ALL.len(), 38);
    }
}

impl Default for GameShop {
    fn default() -> Self {
        GameShop::Steam
    }
}
