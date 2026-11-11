//! Port of `src/shared/download-directories.ts`.
//!
//! The logic is kept function-for-function so behaviour (including the
//! five-directory cap and the "most recent first" ordering) matches the
//! TypeScript implementation.

use serde::{Deserialize, Serialize};
use time::{macros::format_description, format_description::well_known::Rfc3339, Duration, OffsetDateTime, UtcOffset};

use hydra_types::{DownloadDirectoryPreference, DownloadDirectorySource, UserPreferences};

pub const MAX_DOWNLOAD_DIRECTORIES: usize = 5;
pub const MAX_OPTIONAL_DOWNLOAD_DIRECTORIES: usize = MAX_DOWNLOAD_DIRECTORIES - 1;

const SYNTHETIC_CREATED_AT_STEP_MS: i64 = 1000;
const EPOCH_ISO: &str = "1970-01-01T00:00:00.000Z";
const ISO_FORMAT: &[time::format_description::FormatItem<'static>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z");

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DownloadDirectoryError {
    #[error("A fallback downloads path is required.")]
    MissingFallback,
}

/// The `UserPreferences` subset this module owns.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DownloadDirectoryPreferences {
    pub downloads_path: Option<String>,
    pub download_directories: Option<Vec<DownloadDirectoryPreference>>,
    pub optional_downloads_paths: Option<Vec<String>>,
}

impl DownloadDirectoryPreferences {
    pub fn from_user_preferences(preferences: &UserPreferences) -> Self {
        Self {
            downloads_path: preferences.downloads_path.clone(),
            download_directories: preferences.download_directories.clone(),
            optional_downloads_paths: preferences.optional_downloads_paths.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResolvedDownloadDirectories {
    pub persisted_default_path: Option<String>,
    pub default_path: String,
    pub directories: Vec<DownloadDirectoryPreference>,
    pub saved_paths: Vec<String>,
    pub optional_paths: Vec<String>,
    pub all_paths: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreparedDefaultDownloadPathSync {
    Noop,
    SetExisting {
        next_preferences: DownloadDirectoryPreferences,
        next_default_path: String,
    },
    AddAndSet {
        next_preferences: DownloadDirectoryPreferences,
        next_default_path: String,
    },
    ReplaceRequired {
        next_path: String,
        next_default_path: String,
        replaceable_directories: Vec<DownloadDirectoryPreference>,
        recommended_replacement_path: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplaceResult {
    pub next_preferences: DownloadDirectoryPreferences,
    pub next_default_path: String,
}

fn to_iso(datetime: OffsetDateTime) -> String {
    datetime
        .to_offset(UtcOffset::UTC)
        .format(ISO_FORMAT)
        .unwrap_or_else(|_| EPOCH_ISO.to_string())
}

fn parse_js_date(value: Option<&str>) -> Option<OffsetDateTime> {
    let value = value?.trim();
    if value.is_empty() {
        return None;
    }

    if let Ok(datetime) = OffsetDateTime::parse(value, &Rfc3339) {
        return Some(datetime);
    }

    let date_format = format_description!("[year]-[month]-[day]");
    time::Date::parse(value, date_format)
        .ok()
        .and_then(|date| date.with_hms(0, 0, 0).ok())
        .map(|datetime| datetime.assume_utc())
}

fn sanitize_path(value: Option<&str>) -> Option<String> {
    let value = value?;
    let trimmed = value.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn dedupe_paths(paths: impl IntoIterator<Item = Option<String>>) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    let mut result: Vec<String> = Vec::new();

    for path in paths {
        let Some(path) = sanitize_path(path.as_deref()) else {
            continue;
        };
        if seen.contains(&path) {
            continue;
        }
        seen.push(path.clone());
        result.push(path);
    }

    result
}

fn sanitize_created_at(value: Option<&str>) -> String {
    parse_js_date(value).map(to_iso).unwrap_or_else(|| EPOCH_ISO.to_string())
}

fn sanitize_directory_source(source: Option<DownloadDirectorySource>) -> DownloadDirectorySource {
    match source {
        Some(DownloadDirectorySource::Auto) => DownloadDirectorySource::Auto,
        _ => DownloadDirectorySource::Manual,
    }
}

fn create_record(
    path: String,
    created_at: String,
    source: DownloadDirectorySource,
) -> DownloadDirectoryPreference {
    DownloadDirectoryPreference {
        path,
        created_at,
        source,
    }
}

fn normalize_download_directory_records(
    records: &[DownloadDirectoryPreference],
    fixed_path: &str,
    preserved_path: Option<&str>,
) -> Vec<DownloadDirectoryPreference> {
    #[derive(Clone)]
    struct PendingRecord {
        path: String,
        created_at: String,
        source: DownloadDirectorySource,
        timestamp: i64,
        original_index: usize,
    }

    let mut normalized: Vec<PendingRecord> = Vec::new();

    for (original_index, record) in records.iter().enumerate() {
        let Some(path) = sanitize_path(Some(record.path.as_str())) else {
            continue;
        };
        if path == fixed_path {
            continue;
        }

        let created_at = sanitize_created_at(Some(record.created_at.as_str()));
        let source = sanitize_directory_source(Some(record.source));
        let timestamp = parse_js_date(Some(&created_at))
            .map(|datetime| datetime.unix_timestamp_nanos().div_euclid(1_000_000) as i64)
            .unwrap_or(0);

        normalized.push(PendingRecord {
            path,
            created_at,
            source,
            timestamp,
            original_index,
        });
    }

    normalized.sort_by(|first, second| {
        second
            .timestamp
            .cmp(&first.timestamp)
            .then(first.original_index.cmp(&second.original_index))
    });

    let mut unique: Vec<PendingRecord> = Vec::new();
    for record in normalized {
        if unique.iter().any(|existing| existing.path == record.path) {
            continue;
        }
        unique.push(record);
    }

    let normalized_preserved_path = sanitize_path(preserved_path).filter(|path| path != fixed_path);

    let preserved_index = normalized_preserved_path
        .and_then(|preserved| unique.iter().position(|record| record.path == preserved));

    let limited: Vec<PendingRecord> = match preserved_index {
        Some(index) if index >= MAX_OPTIONAL_DOWNLOAD_DIRECTORIES => {
            let mut result = unique[..MAX_OPTIONAL_DOWNLOAD_DIRECTORIES - 1].to_vec();
            result.push(PendingRecord {
                path: unique[index].path.clone(),
                created_at: unique[index].created_at.clone(),
                source: unique[index].source,
                timestamp: unique[index].timestamp,
                original_index: unique[index].original_index,
            });
            result
        }
        _ => unique[..unique.len().min(MAX_OPTIONAL_DOWNLOAD_DIRECTORIES)].to_vec(),
    };

    limited
        .into_iter()
        .map(|record| create_record(record.path, record.created_at, record.source))
        .collect()
}

fn build_legacy_download_directory_records(
    preferences: Option<&DownloadDirectoryPreferences>,
    fallback_default_path: &str,
) -> Vec<DownloadDirectoryPreference> {
    let persisted_default_path =
        sanitize_path(preferences.and_then(|preferences| preferences.downloads_path.as_deref()));
    let normalized_persisted_default_path =
        persisted_default_path.filter(|path| path != fallback_default_path);

    let mut ordered_paths = dedupe_paths(
        preferences
            .and_then(|preferences| preferences.optional_downloads_paths.clone())
            .unwrap_or_default()
            .into_iter()
            .map(Some),
    );

    if let Some(path) = normalized_persisted_default_path {
        if !ordered_paths.contains(&path) {
            ordered_paths.insert(0, path);
        }
    }

    let synthetic_base_timestamp = OffsetDateTime::now_utc();

    ordered_paths
        .into_iter()
        .filter(|path| path != fallback_default_path)
        .enumerate()
        .map(|(index, path)| {
            create_record(
                path,
                to_iso(synthetic_base_timestamp - Duration::milliseconds(index as i64 * SYNTHETIC_CREATED_AT_STEP_MS)),
                DownloadDirectorySource::Manual,
            )
        })
        .collect()
}

pub fn resolve_download_directories(
    preferences: Option<&DownloadDirectoryPreferences>,
    fallback_default_path: &str,
) -> Result<ResolvedDownloadDirectories, DownloadDirectoryError> {
    let sanitized_fallback_default_path = sanitize_path(Some(fallback_default_path))
        .ok_or(DownloadDirectoryError::MissingFallback)?;

    let raw_persisted_default_path =
        sanitize_path(preferences.and_then(|preferences| preferences.downloads_path.as_deref()));
    let preserved_default_path = raw_persisted_default_path
        .clone()
        .filter(|path| path != &sanitized_fallback_default_path);

    let directories = match preferences.and_then(|preferences| preferences.download_directories.as_ref()) {
        Some(records) => normalize_download_directory_records(
            records,
            &sanitized_fallback_default_path,
            preserved_default_path.as_deref(),
        ),
        None => {
            let legacy = build_legacy_download_directory_records(
                preferences,
                &sanitized_fallback_default_path,
            );
            normalize_download_directory_records(
                &legacy,
                &sanitized_fallback_default_path,
                preserved_default_path.as_deref(),
            )
        }
    };

    let saved_paths: Vec<String> = directories
        .iter()
        .map(|directory| directory.path.clone())
        .collect();

    let persisted_default_path = raw_persisted_default_path.filter(|path| {
        path != &sanitized_fallback_default_path && saved_paths.contains(path)
    });
    let default_path = persisted_default_path
        .clone()
        .unwrap_or_else(|| sanitized_fallback_default_path.clone());

    let mut all_paths = saved_paths.clone();
    all_paths.push(sanitized_fallback_default_path.clone());

    Ok(ResolvedDownloadDirectories {
        persisted_default_path,
        default_path,
        optional_paths: saved_paths.clone(),
        directories,
        saved_paths,
        all_paths,
    })
}

pub fn get_download_directory_preferences(
    preferences: Option<&DownloadDirectoryPreferences>,
    fallback_default_path: &str,
) -> Result<DownloadDirectoryPreferences, DownloadDirectoryError> {
    let resolved = resolve_download_directories(preferences, fallback_default_path)?;

    Ok(DownloadDirectoryPreferences {
        downloads_path: resolved.persisted_default_path,
        download_directories: Some(resolved.directories.clone()),
        optional_downloads_paths: Some(
            resolved
                .directories
                .iter()
                .map(|directory| directory.path.clone())
                .collect(),
        ),
    })
}

pub fn set_default_download_directory(
    preferences: Option<&DownloadDirectoryPreferences>,
    next_default_path: &str,
    fallback_default_path: &str,
) -> Result<DownloadDirectoryPreferences, DownloadDirectoryError> {
    let resolved = resolve_download_directories(preferences, fallback_default_path)?;
    let sanitized_next_default_path = sanitize_path(Some(next_default_path));

    let Some(next_path) = sanitized_next_default_path else {
        return get_download_directory_preferences(preferences, fallback_default_path);
    };

    if next_path == resolved.default_path {
        return get_download_directory_preferences(preferences, fallback_default_path);
    }

    if next_path == fallback_default_path {
        return get_download_directory_preferences(
            Some(&DownloadDirectoryPreferences {
                downloads_path: None,
                download_directories: Some(resolved.directories),
                optional_downloads_paths: None,
            }),
            fallback_default_path,
        );
    }

    if !resolved.saved_paths.contains(&next_path) {
        return get_download_directory_preferences(preferences, fallback_default_path);
    }

    get_download_directory_preferences(
        Some(&DownloadDirectoryPreferences {
            downloads_path: Some(next_path),
            download_directories: Some(resolved.directories),
            optional_downloads_paths: None,
        }),
        fallback_default_path,
    )
}

pub fn prepare_default_download_path_sync(
    preferences: Option<&DownloadDirectoryPreferences>,
    next_path: &str,
    fallback_default_path: &str,
) -> Result<PreparedDefaultDownloadPathSync, DownloadDirectoryError> {
    let resolved = resolve_download_directories(preferences, fallback_default_path)?;
    let sanitized_next_path = sanitize_path(Some(next_path));
    let sanitized_fallback_default_path = sanitize_path(Some(fallback_default_path));
    let raw_persisted_default_path =
        sanitize_path(preferences.and_then(|preferences| preferences.downloads_path.as_deref()));

    let current_default_path = raw_persisted_default_path
        .clone()
        .or_else(|| sanitized_fallback_default_path.clone())
        .unwrap_or_default();

    let Some(next_path) = sanitized_next_path else {
        return Ok(PreparedDefaultDownloadPathSync::Noop);
    };

    if next_path == current_default_path {
        return Ok(PreparedDefaultDownloadPathSync::Noop);
    }

    if Some(&next_path) == sanitized_fallback_default_path.as_ref() {
        let next_preferences = get_download_directory_preferences(
            Some(&DownloadDirectoryPreferences {
                downloads_path: None,
                download_directories: Some(resolved.directories),
                optional_downloads_paths: None,
            }),
            fallback_default_path,
        )?;

        return Ok(PreparedDefaultDownloadPathSync::SetExisting {
            next_preferences,
            next_default_path: sanitized_fallback_default_path.unwrap_or_default(),
        });
    }

    if resolved.saved_paths.contains(&next_path) {
        let next_preferences = get_download_directory_preferences(
            Some(&DownloadDirectoryPreferences {
                downloads_path: Some(next_path.clone()),
                download_directories: Some(resolved.directories),
                optional_downloads_paths: None,
            }),
            fallback_default_path,
        )?;

        return Ok(PreparedDefaultDownloadPathSync::SetExisting {
            next_preferences,
            next_default_path: next_path,
        });
    }

    let most_recent_saved_directory = resolved.directories.first().cloned();

    if most_recent_saved_directory
        .as_ref()
        .is_some_and(|directory| directory.source == DownloadDirectorySource::Auto)
    {
        let mut directories = vec![create_record(
            next_path.clone(),
            to_iso(OffsetDateTime::now_utc()),
            DownloadDirectorySource::Auto,
        )];
        let most_recent_path = most_recent_saved_directory.map(|directory| directory.path);

        directories.extend(resolved.directories.into_iter().filter(|directory| {
            Some(&directory.path) != most_recent_path.as_ref() && directory.path != next_path
        }));

        let next_preferences = get_download_directory_preferences(
            Some(&DownloadDirectoryPreferences {
                downloads_path: Some(next_path.clone()),
                download_directories: Some(directories),
                optional_downloads_paths: None,
            }),
            fallback_default_path,
        )?;

        return Ok(PreparedDefaultDownloadPathSync::AddAndSet {
            next_preferences,
            next_default_path: next_path,
        });
    }

    if resolved.saved_paths.len() < MAX_OPTIONAL_DOWNLOAD_DIRECTORIES {
        let mut directories = vec![create_record(
            next_path.clone(),
            to_iso(OffsetDateTime::now_utc()),
            DownloadDirectorySource::Auto,
        )];
        directories.extend(resolved.directories);

        let next_preferences = get_download_directory_preferences(
            Some(&DownloadDirectoryPreferences {
                downloads_path: Some(next_path.clone()),
                download_directories: Some(directories),
                optional_downloads_paths: None,
            }),
            fallback_default_path,
        )?;

        return Ok(PreparedDefaultDownloadPathSync::AddAndSet {
            next_preferences,
            next_default_path: next_path,
        });
    }

    let current_saved_default_path = raw_persisted_default_path
        .filter(|path| resolved.saved_paths.contains(path));
    let recommended_replacement_path = current_saved_default_path
        .or_else(|| resolved.directories.last().map(|directory| directory.path.clone()))
        .or_else(|| resolved.directories.first().map(|directory| directory.path.clone()))
        .unwrap_or_else(|| next_path.clone());

    Ok(PreparedDefaultDownloadPathSync::ReplaceRequired {
        next_path: next_path.clone(),
        next_default_path: next_path,
        replaceable_directories: resolved.directories,
        recommended_replacement_path,
    })
}

pub fn replace_saved_download_directory_and_set_default(
    preferences: Option<&DownloadDirectoryPreferences>,
    next_path: &str,
    path_to_replace: &str,
    fallback_default_path: &str,
) -> Result<ReplaceResult, DownloadDirectoryError> {
    let resolved = resolve_download_directories(preferences, fallback_default_path)?;
    let sanitized_next_path = sanitize_path(Some(next_path));
    let sanitized_path_to_replace = sanitize_path(Some(path_to_replace));

    let (Some(next_path), Some(path_to_replace)) = (sanitized_next_path, sanitized_path_to_replace)
    else {
        return Ok(ReplaceResult {
            next_preferences: get_download_directory_preferences(
                preferences,
                fallback_default_path,
            )?,
            next_default_path: resolved.default_path,
        });
    };

    if !resolved.saved_paths.contains(&path_to_replace) {
        return Ok(ReplaceResult {
            next_preferences: get_download_directory_preferences(
                preferences,
                fallback_default_path,
            )?,
            next_default_path: resolved.default_path,
        });
    }

    let mut directories = vec![create_record(
        next_path.clone(),
        to_iso(OffsetDateTime::now_utc()),
        DownloadDirectorySource::Auto,
    )];
    directories.extend(resolved.directories.into_iter().filter(|directory| {
        directory.path != path_to_replace && directory.path != next_path
    }));

    Ok(ReplaceResult {
        next_preferences: get_download_directory_preferences(
            Some(&DownloadDirectoryPreferences {
                downloads_path: Some(next_path.clone()),
                download_directories: Some(directories),
                optional_downloads_paths: None,
            }),
            fallback_default_path,
        )?,
        next_default_path: next_path,
    })
}

pub fn add_optional_download_directory(
    preferences: Option<&DownloadDirectoryPreferences>,
    next_path: &str,
    fallback_default_path: &str,
) -> Result<DownloadDirectoryPreferences, DownloadDirectoryError> {
    let resolved = resolve_download_directories(preferences, fallback_default_path)?;
    let sanitized_next_path = sanitize_path(Some(next_path));

    let Some(next_path) = sanitized_next_path else {
        return get_download_directory_preferences(preferences, fallback_default_path);
    };

    if next_path == fallback_default_path
        || resolved.all_paths.contains(&next_path)
        || resolved.saved_paths.len() >= MAX_OPTIONAL_DOWNLOAD_DIRECTORIES
    {
        return get_download_directory_preferences(preferences, fallback_default_path);
    }

    let mut directories = vec![create_record(
        next_path,
        to_iso(OffsetDateTime::now_utc()),
        DownloadDirectorySource::Manual,
    )];
    directories.extend(resolved.directories);

    get_download_directory_preferences(
        Some(&DownloadDirectoryPreferences {
            downloads_path: resolved.persisted_default_path,
            download_directories: Some(directories),
            optional_downloads_paths: None,
        }),
        fallback_default_path,
    )
}

pub fn remove_download_directory(
    preferences: Option<&DownloadDirectoryPreferences>,
    path_to_remove: &str,
    fallback_default_path: &str,
) -> Result<DownloadDirectoryPreferences, DownloadDirectoryError> {
    let resolved = resolve_download_directories(preferences, fallback_default_path)?;
    let sanitized_path_to_remove = sanitize_path(Some(path_to_remove));

    let Some(path_to_remove) = sanitized_path_to_remove else {
        return get_download_directory_preferences(preferences, fallback_default_path);
    };

    if path_to_remove == fallback_default_path
        || !resolved.saved_paths.contains(&path_to_remove)
    {
        return get_download_directory_preferences(preferences, fallback_default_path);
    }

    if path_to_remove != resolved.default_path {
        return get_download_directory_preferences(
            Some(&DownloadDirectoryPreferences {
                downloads_path: resolved.persisted_default_path,
                download_directories: Some(
                    resolved
                        .directories
                        .into_iter()
                        .filter(|directory| directory.path != path_to_remove)
                        .collect(),
                ),
                optional_downloads_paths: None,
            }),
            fallback_default_path,
        );
    }

    let remaining_directories: Vec<DownloadDirectoryPreference> = resolved
        .directories
        .into_iter()
        .filter(|directory| directory.path != path_to_remove)
        .collect();

    get_download_directory_preferences(
        Some(&DownloadDirectoryPreferences {
            downloads_path: None,
            download_directories: Some(remaining_directories),
            optional_downloads_paths: None,
        }),
        fallback_default_path,
    )
}

pub fn get_download_directory_title(path: &str) -> String {
    let segments: Vec<&str> = path
        .split(['\\', '/'])
        .filter(|segment| !segment.is_empty())
        .collect();

    segments
        .last()
        .map(|segment| (*segment).to_string())
        .unwrap_or_else(|| path.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FALLBACK: &str = "/fallback/downloads";

    fn record(path: &str, created_at: &str, source: DownloadDirectorySource) -> DownloadDirectoryPreference {
        DownloadDirectoryPreference {
            path: path.into(),
            created_at: created_at.into(),
            source,
        }
    }

    #[test]
    fn requires_a_fallback_path() {
        assert_eq!(
            resolve_download_directories(None, "   "),
            Err(DownloadDirectoryError::MissingFallback)
        );
    }

    #[test]
    fn resolves_an_empty_profile_to_the_fallback_only() {
        let resolved = resolve_download_directories(None, FALLBACK).unwrap();
        assert_eq!(resolved.default_path, FALLBACK);
        assert_eq!(resolved.persisted_default_path, None);
        assert!(resolved.directories.is_empty());
        assert_eq!(resolved.all_paths, vec![FALLBACK.to_string()]);
    }

    #[test]
    fn migrates_legacy_optional_paths_into_records_newest_first() {
        let preferences = DownloadDirectoryPreferences {
            downloads_path: None,
            download_directories: None,
            optional_downloads_paths: Some(vec![
                "/a".into(),
                "/a".into(),
                "  ".into(),
                "/b".into(),
                FALLBACK.into(),
            ]),
        };

        let resolved = resolve_download_directories(Some(&preferences), FALLBACK).unwrap();
        let paths: Vec<&str> = resolved
            .directories
            .iter()
            .map(|directory| directory.path.as_str())
            .collect();

        assert_eq!(paths, vec!["/a", "/b"]);
        assert_eq!(resolved.default_path, FALLBACK);
        assert_eq!(
            resolved.all_paths,
            vec!["/a".to_string(), "/b".to_string(), FALLBACK.to_string()]
        );
    }

    #[test]
    fn sorts_by_creation_time_then_original_index() {
        let preferences = DownloadDirectoryPreferences {
            downloads_path: None,
            download_directories: Some(vec![
                record("/old", "2026-01-01T00:00:00.000Z", DownloadDirectorySource::Manual),
                record("/new", "2026-06-01T00:00:00.000Z", DownloadDirectorySource::Manual),
                record("/invalid", "not-a-date", DownloadDirectorySource::Auto),
            ]),
            optional_downloads_paths: None,
        };

        let resolved = resolve_download_directories(Some(&preferences), FALLBACK).unwrap();
        let paths: Vec<&str> = resolved
            .directories
            .iter()
            .map(|directory| directory.path.as_str())
            .collect();

        assert_eq!(paths, vec!["/new", "/old", "/invalid"]);
        assert_eq!(resolved.directories[2].created_at, EPOCH_ISO);
        assert_eq!(
            resolved.directories[2].source,
            DownloadDirectorySource::Auto
        );
    }

    #[test]
    fn keeps_a_persisted_default_that_is_still_saved() {
        let preferences = DownloadDirectoryPreferences {
            downloads_path: Some("/b".into()),
            download_directories: Some(vec![
                record("/a", "2026-01-01T00:00:00.000Z", DownloadDirectorySource::Manual),
                record("/b", "2026-02-01T00:00:00.000Z", DownloadDirectorySource::Manual),
            ]),
            optional_downloads_paths: None,
        };

        let resolved = resolve_download_directories(Some(&preferences), FALLBACK).unwrap();
        assert_eq!(resolved.default_path, "/b");
        assert_eq!(resolved.persisted_default_path, Some("/b".into()));
    }

    #[test]
    fn ignores_a_persisted_default_that_is_not_saved() {
        let preferences = DownloadDirectoryPreferences {
            downloads_path: Some("/missing".into()),
            download_directories: Some(vec![record(
                "/a",
                "2026-01-01T00:00:00.000Z",
                DownloadDirectorySource::Manual,
            )]),
            optional_downloads_paths: None,
        };

        let resolved = resolve_download_directories(Some(&preferences), FALLBACK).unwrap();
        assert_eq!(resolved.persisted_default_path, None);
        assert_eq!(resolved.default_path, FALLBACK);
    }

    #[test]
    fn caps_saved_directories_and_preserves_the_current_default() {
        let mut records: Vec<DownloadDirectoryPreference> = (0..6)
            .map(|index| {
                record(
                    &format!("/dir{index}"),
                    &format!("2026-01-0{}T00:00:00.000Z", index + 1),
                    DownloadDirectorySource::Manual,
                )
            })
            .collect();
        // /dir0 is the oldest, so it is the one that falls off the list.
        let preferences = DownloadDirectoryPreferences {
            downloads_path: Some("/dir0".into()),
            download_directories: Some(records.clone()),
            optional_downloads_paths: None,
        };

        let resolved = resolve_download_directories(Some(&preferences), FALLBACK).unwrap();
        let paths: Vec<&str> = resolved
            .directories
            .iter()
            .map(|directory| directory.path.as_str())
            .collect();

        assert_eq!(paths.len(), MAX_OPTIONAL_DOWNLOAD_DIRECTORIES);
        assert!(paths.contains(&"/dir0"));
        assert_eq!(resolved.persisted_default_path, Some("/dir0".into()));

        records.truncate(4);
        let preferences = DownloadDirectoryPreferences {
            downloads_path: None,
            download_directories: Some(records),
            optional_downloads_paths: None,
        };
        let resolved = resolve_download_directories(Some(&preferences), FALLBACK).unwrap();
        assert_eq!(resolved.directories.len(), 4);
    }

    #[test]
    fn sets_and_clears_the_default_directory() {
        let preferences = DownloadDirectoryPreferences {
            downloads_path: None,
            download_directories: Some(vec![record(
                "/a",
                "2026-01-01T00:00:00.000Z",
                DownloadDirectorySource::Manual,
            )]),
            optional_downloads_paths: None,
        };

        let next = set_default_download_directory(Some(&preferences), "/a", FALLBACK).unwrap();
        assert_eq!(next.downloads_path, Some("/a".into()));

        let cleared = set_default_download_directory(Some(&next), FALLBACK, FALLBACK).unwrap();
        assert_eq!(cleared.downloads_path, None);

        let unchanged = set_default_download_directory(Some(&cleared), "/nope", FALLBACK).unwrap();
        assert_eq!(unchanged.downloads_path, None);
    }

    #[test]
    fn prepare_reports_noop_for_the_current_default() {
        assert_eq!(
            prepare_default_download_path_sync(None, FALLBACK, FALLBACK).unwrap(),
            PreparedDefaultDownloadPathSync::Noop
        );
        assert_eq!(
            prepare_default_download_path_sync(None, "  ", FALLBACK).unwrap(),
            PreparedDefaultDownloadPathSync::Noop
        );
    }

    #[test]
    fn prepare_promotes_an_existing_saved_directory() {
        let preferences = DownloadDirectoryPreferences {
            downloads_path: None,
            download_directories: Some(vec![record(
                "/a",
                "2026-01-01T00:00:00.000Z",
                DownloadDirectorySource::Manual,
            )]),
            optional_downloads_paths: None,
        };

        match prepare_default_download_path_sync(Some(&preferences), "/a", FALLBACK).unwrap() {
            PreparedDefaultDownloadPathSync::SetExisting {
                next_preferences,
                next_default_path,
            } => {
                assert_eq!(next_default_path, "/a");
                assert_eq!(next_preferences.downloads_path, Some("/a".into()));
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn prepare_adds_a_new_directory_while_capacity_remains() {
        match prepare_default_download_path_sync(None, "/new", FALLBACK).unwrap() {
            PreparedDefaultDownloadPathSync::AddAndSet {
                next_preferences,
                next_default_path,
            } => {
                assert_eq!(next_default_path, "/new");
                let directories = next_preferences.download_directories.unwrap();
                assert_eq!(directories.len(), 1);
                assert_eq!(directories[0].path, "/new");
                assert_eq!(directories[0].source, DownloadDirectorySource::Auto);
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn prepare_requires_replacement_when_full() {
        let records: Vec<DownloadDirectoryPreference> = (0..4)
            .map(|index| {
                record(
                    &format!("/dir{index}"),
                    &format!("2026-01-0{}T00:00:00.000Z", index + 1),
                    DownloadDirectorySource::Manual,
                )
            })
            .collect();
        let preferences = DownloadDirectoryPreferences {
            downloads_path: Some("/dir1".into()),
            download_directories: Some(records),
            optional_downloads_paths: None,
        };

        match prepare_default_download_path_sync(Some(&preferences), "/new", FALLBACK).unwrap() {
            PreparedDefaultDownloadPathSync::ReplaceRequired {
                next_path,
                next_default_path,
                replaceable_directories,
                recommended_replacement_path,
            } => {
                assert_eq!(next_path, "/new");
                assert_eq!(next_default_path, "/new");
                assert_eq!(replaceable_directories.len(), 4);
                assert_eq!(recommended_replacement_path, "/dir1");
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn replaces_a_saved_directory_and_sets_the_default() {
        let preferences = DownloadDirectoryPreferences {
            downloads_path: None,
            download_directories: Some(vec![
                record("/a", "2026-01-01T00:00:00.000Z", DownloadDirectorySource::Manual),
                record("/b", "2026-01-02T00:00:00.000Z", DownloadDirectorySource::Manual),
            ]),
            optional_downloads_paths: None,
        };

        let replaced =
            replace_saved_download_directory_and_set_default(Some(&preferences), "/new", "/a", FALLBACK)
                .unwrap();

        assert_eq!(replaced.next_default_path, "/new");
        let paths: Vec<String> = replaced
            .next_preferences
            .download_directories
            .unwrap()
            .into_iter()
            .map(|directory| directory.path)
            .collect();
        assert_eq!(paths, vec!["/new", "/b"]);
    }

    #[test]
    fn adds_optional_directories_up_to_the_cap() {
        let mut preferences = DownloadDirectoryPreferences::default();

        for index in 0..6 {
            preferences = add_optional_download_directory(
                Some(&preferences),
                &format!("/dir{index}"),
                FALLBACK,
            )
            .unwrap();
        }

        let directories = preferences.download_directories.clone().unwrap();
        assert_eq!(directories.len(), MAX_OPTIONAL_DOWNLOAD_DIRECTORIES);

        let duplicate = add_optional_download_directory(Some(&preferences), "/dir1", FALLBACK).unwrap();
        assert_eq!(
            duplicate.download_directories.unwrap().len(),
            MAX_OPTIONAL_DOWNLOAD_DIRECTORIES
        );
    }

    #[test]
    fn removing_the_default_falls_back_to_the_fallback_path() {
        let preferences = DownloadDirectoryPreferences {
            downloads_path: Some("/a".into()),
            download_directories: Some(vec![
                record("/a", "2026-01-01T00:00:00.000Z", DownloadDirectorySource::Manual),
                record("/b", "2026-01-02T00:00:00.000Z", DownloadDirectorySource::Manual),
            ]),
            optional_downloads_paths: None,
        };

        let removed = remove_download_directory(Some(&preferences), "/a", FALLBACK).unwrap();
        assert_eq!(removed.downloads_path, None);
        let paths: Vec<String> = removed
            .download_directories
            .unwrap()
            .into_iter()
            .map(|directory| directory.path)
            .collect();
        assert_eq!(paths, vec!["/b"]);
    }

    #[test]
    fn removing_a_non_default_directory_keeps_the_default() {
        let preferences = DownloadDirectoryPreferences {
            downloads_path: Some("/b".into()),
            download_directories: Some(vec![
                record("/a", "2026-01-01T00:00:00.000Z", DownloadDirectorySource::Manual),
                record("/b", "2026-01-02T00:00:00.000Z", DownloadDirectorySource::Manual),
            ]),
            optional_downloads_paths: None,
        };

        let removed = remove_download_directory(Some(&preferences), "/a", FALLBACK).unwrap();
        assert_eq!(removed.downloads_path, Some("/b".into()));
    }

    #[test]
    fn titles_use_the_last_path_segment() {
        assert_eq!(get_download_directory_title("/home/user/Games"), "Games");
        assert_eq!(get_download_directory_title("C:\\Games\\"), "Games");
        assert_eq!(get_download_directory_title("Games"), "Games");
        assert_eq!(get_download_directory_title("/"), "/");
    }
}
