use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

/// `src/types/emulator.types.ts` `EmulatorSystem`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmulatorSystem {
    Ps1,
    Ps2,
    Ps3,
    Ps4,
    Psp,
    Dolphin,
}

impl EmulatorSystem {
    pub const ALL: [EmulatorSystem; 6] = [
        EmulatorSystem::Ps1,
        EmulatorSystem::Ps2,
        EmulatorSystem::Ps3,
        EmulatorSystem::Ps4,
        EmulatorSystem::Psp,
        EmulatorSystem::Dolphin,
    ];
}

/// `src/types/emulator.types.ts` `EmulatorBinary`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmulatorBinary {
    Duckstation,
    Pcsx2,
    Rpcs3,
    Shadps4,
    Ppsspp,
    Dolphin,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SkuRegion {
    Us,
    Eu,
    Jp,
    Kr,
    Asia,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RomFolder {
    pub id: String,
    pub path: String,
    pub scan_subfolders: bool,
    pub file_count: i64,
    pub size_bytes: i64,
    pub last_scan_at: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmulatorConfig {
    pub system: EmulatorSystem,
    pub binary: EmulatorBinary,
    pub executable_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rpcs3_config_root: Option<String>,
    pub detected_version: Option<String>,
    pub detected_at: Option<i64>,
    pub bios_path: Option<String>,
    #[serde(default)]
    pub rom_folders: Vec<RomFolder>,
    pub last_scan_at: Option<i64>,
    pub total_files: i64,
    pub total_size_bytes: i64,
}

pub type EmulatorConfigMap = BTreeMap<EmulatorSystem, EmulatorConfig>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EmulatorSaveLocationStatusKind {
    Ready,
    NotConfigured,
    Missing,
    Ambiguous,
    InvalidSelection,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct EmulatorSaveLocationStatus {
    pub status: Option<EmulatorSaveLocationStatusKind>,
    pub selected_root: Option<String>,
    pub resolved_root: Option<String>,
    pub candidates: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EmulatorDiscIdentityStatusKind {
    Ready,
    Missing,
    Unverified,
    Mismatch,
    StaleSku,
    CatalogueUnavailable,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct EmulatorDiscIdentityStatus {
    pub status: Option<EmulatorDiscIdentityStatusKind>,
    pub path: Option<String>,
    pub title_id: Option<String>,
}

/// `src/types/emulator.types.ts` `DetectedRom`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DetectedRom {
    pub object_id: String,
    pub title: String,
    pub cover_image_url: Option<String>,
    pub library_image_url: Option<String>,
    pub icon_url: Option<String>,
    pub custom_cover_image_url: Option<String>,
    pub custom_icon_url: Option<String>,
    pub size_bytes: Option<i64>,
    pub skus: Vec<String>,
}

/// `src/types/emulator.types.ts` `ClassicsDisc`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ClassicsDisc {
    pub path: String,
    pub label: String,
    pub file_name: String,
    pub sku: Option<String>,
    pub region: Option<SkuRegion>,
    pub is_executable: Option<bool>,
    pub recompilation_slug: Option<String>,
    pub icon_url: Option<String>,
}

/// `src/types/emulator.types.ts` `Ps2MemoryCardSaveRecord`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Ps2MemoryCardSaveRecord {
    pub card_file_path: String,
    pub card_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    pub folder_name: String,
    pub sku: Option<String>,
    pub object_id: Option<String>,
    pub shop: Option<crate::GameShop>,
    pub title: Option<String>,
    pub icon_url: Option<String>,
    pub library_image_url: Option<String>,
    pub library_hero_image_url: Option<String>,
    pub logo_image_url: Option<String>,
    pub file_count: i64,
    pub size_bytes: i64,
    pub created_at: i64,
    pub modified_at: i64,
    pub detected_at: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EmulatorInstallKind {
    WindowsInstaller,
    LinuxAppimage,
    WindowsArchive,
    PortableArchive,
    MacosDmg,
    Link,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmulatorInstallChannel {
    Release,
    Prerelease,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmulatorInstallLinkKind {
    Aur,
    Flatpak,
    ReleasePage,
}

/// `src/types/emulator.types.ts` `ResolvedInstallOption`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ResolvedInstallOption {
    pub id: String,
    pub binary: Option<EmulatorBinary>,
    pub kind: Option<EmulatorInstallKind>,
    pub channel: Option<EmulatorInstallChannel>,
    pub download_url: Option<String>,
    pub file_name: Option<String>,
    pub version: Option<String>,
    pub html_url: Option<String>,
    pub link_url: Option<String>,
    pub link_kind: Option<EmulatorInstallLinkKind>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmulatorInstallPhase {
    Downloading,
    Extracting,
    Running,
    Done,
    Error,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct EmulatorInstallProgress {
    pub binary: Option<EmulatorBinary>,
    pub option_id: String,
    pub phase: Option<EmulatorInstallPhase>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loaded: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct EmulatorInstallResult {
    pub ok: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emulator_config_map_keys_are_system_strings() {
        let mut map = EmulatorConfigMap::new();
        map.insert(
            EmulatorSystem::Ps2,
            EmulatorConfig {
                system: EmulatorSystem::Ps2,
                binary: EmulatorBinary::Pcsx2,
                executable_path: None,
                rpcs3_config_root: None,
                detected_version: None,
                detected_at: None,
                bios_path: None,
                rom_folders: Vec::new(),
                last_scan_at: None,
                total_files: 0,
                total_size_bytes: 0,
            },
        );

        let json = serde_json::to_value(&map).unwrap();
        assert!(json.get("ps2").is_some());
    }

    #[test]
    fn sku_region_serializes_uppercase() {
        assert_eq!(serde_json::to_string(&SkuRegion::Eu).unwrap(), "\"EU\"");
    }
}
