//! Port of `src/shared/playtime.ts` with its test suite.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PlayTimeParts {
    pub play_time_in_milliseconds: Option<i64>,
    pub steam_play_time_in_milliseconds: Option<i64>,
    pub epic_play_time_in_milliseconds: Option<i64>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RuntimeByPlatform {
    pub hydra: Option<f64>,
    pub steam: Option<f64>,
    pub epic: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RemotePlayTimeInput {
    pub has_active_epic_import: Option<bool>,
    pub play_time_in_milliseconds: Option<f64>,
    pub play_time_in_seconds: Option<f64>,
    pub runtime: Option<f64>,
    pub runtime_by_platform: Option<RuntimeByPlatform>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RemotePlayTimeParts {
    pub hydra_ms: i64,
    pub steam_ms: i64,
    pub epic_ms: i64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MergedPlayTime {
    pub play_time_in_milliseconds: i64,
    pub steam_play_time_in_milliseconds: i64,
    pub epic_play_time_in_milliseconds: i64,
}

fn to_milliseconds(seconds: Option<f64>) -> i64 {
    match seconds {
        Some(seconds) if seconds.is_finite() && seconds >= 0.0 => (seconds * 1000.0) as i64,
        _ => 0,
    }
}

pub fn get_displayed_play_time_in_milliseconds(game: PlayTimeParts) -> i64 {
    game.play_time_in_milliseconds.unwrap_or(0).max(0)
        + game.steam_play_time_in_milliseconds.unwrap_or(0).max(0)
        + game.epic_play_time_in_milliseconds.unwrap_or(0).max(0)
}

pub fn get_play_time_hours_and_minutes(milliseconds: i64) -> (i64, i64) {
    let total_minutes = milliseconds.max(0) / 60_000;
    (total_minutes / 60, total_minutes % 60)
}

pub fn get_remote_play_time_parts(remote: RemotePlayTimeInput) -> RemotePlayTimeParts {
    let platform = remote.runtime_by_platform.unwrap_or_default();
    let hydra_seconds = platform.hydra;
    let steam_seconds = platform.steam;
    let epic_seconds = platform.epic;

    let combined_seconds = remote
        .runtime
        .or(remote.play_time_in_seconds)
        .or(remote.play_time_in_milliseconds.map(|ms| ms / 1000.0))
        .unwrap_or(0.0);

    if hydra_seconds.is_some() || steam_seconds.is_some() || epic_seconds.is_some() {
        let steam_ms = to_milliseconds(steam_seconds);
        let epic_ms = to_milliseconds(epic_seconds);
        let hydra_ms = match hydra_seconds {
            Some(seconds) => to_milliseconds(Some(seconds)),
            None => (to_milliseconds(Some(combined_seconds)) - steam_ms - epic_ms).max(0),
        };

        return RemotePlayTimeParts {
            hydra_ms,
            steam_ms,
            epic_ms,
        };
    }

    RemotePlayTimeParts {
        hydra_ms: to_milliseconds(Some(combined_seconds)),
        steam_ms: 0,
        epic_ms: 0,
    }
}

pub fn merge_local_and_remote_play_time(
    local: PlayTimeParts,
    remote: RemotePlayTimeInput,
) -> MergedPlayTime {
    // An older API lacks Epic fields entirely; absence is not a zero snapshot.
    let supports_epic = remote
        .runtime_by_platform
        .and_then(|platform| platform.epic)
        .is_some()
        || remote.has_active_epic_import.is_some();

    let compatible_remote = match (supports_epic, local.epic_play_time_in_milliseconds) {
        (false, Some(local_epic)) => {
            let mut platform = remote.runtime_by_platform.unwrap_or_default();
            platform.epic = Some(local_epic as f64 / 1000.0);
            RemotePlayTimeInput {
                runtime_by_platform: Some(platform),
                ..remote
            }
        }
        _ => remote,
    };

    let parts = get_remote_play_time_parts(compatible_remote);

    MergedPlayTime {
        play_time_in_milliseconds: local.play_time_in_milliseconds.unwrap_or(0).max(parts.hydra_ms),
        steam_play_time_in_milliseconds: parts.steam_ms,
        epic_play_time_in_milliseconds: parts.epic_ms,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parts(play: Option<i64>, steam: Option<i64>, epic: Option<i64>) -> PlayTimeParts {
        PlayTimeParts {
            play_time_in_milliseconds: play,
            steam_play_time_in_milliseconds: steam,
            epic_play_time_in_milliseconds: epic,
        }
    }

    fn runtime(hydra: Option<f64>, steam: Option<f64>, epic: Option<f64>) -> RemotePlayTimeInput {
        RemotePlayTimeInput {
            runtime_by_platform: Some(RuntimeByPlatform {
                hydra,
                steam,
                epic,
            }),
            ..RemotePlayTimeInput::default()
        }
    }

    #[test]
    fn sums_hydra_and_steam_playtime() {
        assert_eq!(
            get_displayed_play_time_in_milliseconds(parts(
                Some(3_600_000),
                Some(7_200_000),
                None
            )),
            10_800_000
        );
    }

    #[test]
    fn treats_missing_steam_playtime_as_zero() {
        assert_eq!(
            get_displayed_play_time_in_milliseconds(parts(Some(1_000), None, None)),
            1_000
        );
    }

    #[test]
    fn splits_milliseconds_into_hours_and_leftover_minutes() {
        assert_eq!(get_play_time_hours_and_minutes(7_800_000), (2, 10));
        assert_eq!(get_play_time_hours_and_minutes(-1), (0, 0));
    }

    #[test]
    fn keeps_local_hydra_hours_and_adds_steam_from_runtime_by_platform() {
        let merged = merge_local_and_remote_play_time(
            parts(Some(360_000_000), None, None),
            RemotePlayTimeInput {
                play_time_in_milliseconds: Some(187_200_000.0),
                ..runtime(Some(7_200.0), Some(180_000.0), None)
            },
        );

        assert_eq!(merged.play_time_in_milliseconds, 360_000_000);
        assert_eq!(merged.steam_play_time_in_milliseconds, 180_000_000);
        assert_eq!(merged.epic_play_time_in_milliseconds, 0);
    }

    #[test]
    fn uses_remote_hydra_when_it_is_higher_than_local() {
        let merged = merge_local_and_remote_play_time(
            parts(Some(1_000), None, None),
            runtime(Some(10.0), Some(50.0), None),
        );

        assert_eq!(merged.play_time_in_milliseconds, 10_000);
        assert_eq!(merged.steam_play_time_in_milliseconds, 50_000);
    }

    #[test]
    fn falls_back_to_the_combined_total_when_platforms_are_missing() {
        let merged = merge_local_and_remote_play_time(
            parts(Some(1_000), None, None),
            RemotePlayTimeInput {
                play_time_in_milliseconds: Some(5_000.0),
                ..RemotePlayTimeInput::default()
            },
        );

        assert_eq!(merged.play_time_in_milliseconds, 5_000);
        assert_eq!(merged.steam_play_time_in_milliseconds, 0);
    }

    #[test]
    fn replaces_epic_totals_without_reattributing_aggregate_time_to_hydra() {
        let local = parts(Some(10), Some(20), Some(30));
        let remote = RemotePlayTimeInput {
            play_time_in_milliseconds: Some(999_999.0),
            ..runtime(Some(0.0), Some(2.0), Some(3.0))
        };

        let merged = merge_local_and_remote_play_time(local, remote);
        assert_eq!(merged.play_time_in_milliseconds, 10);
        assert_eq!(merged.steam_play_time_in_milliseconds, 2_000);
        assert_eq!(merged.epic_play_time_in_milliseconds, 3_000);
        assert_eq!(merge_local_and_remote_play_time(local, remote), merged);
        assert_eq!(get_displayed_play_time_in_milliseconds(local), 60);

        let merged_parts = parts(
            Some(merged.play_time_in_milliseconds),
            Some(merged.steam_play_time_in_milliseconds),
            Some(merged.epic_play_time_in_milliseconds),
        );
        assert_eq!(get_displayed_play_time_in_milliseconds(merged_parts), 5_010);
        assert_eq!(
            merge_local_and_remote_play_time(merged_parts, runtime(Some(0.0), Some(2.0), Some(0.0)))
                .epic_play_time_in_milliseconds,
            0
        );
    }

    #[test]
    fn discounts_imports_only_when_the_hydra_attribution_is_absent() {
        let without_hydra = RemotePlayTimeInput {
            play_time_in_seconds: Some(50.0),
            ..runtime(None, Some(10.0), Some(20.0))
        };
        assert_eq!(
            merge_local_and_remote_play_time(PlayTimeParts::default(), without_hydra)
                .play_time_in_milliseconds,
            20_000
        );

        let with_hydra = RemotePlayTimeInput {
            play_time_in_seconds: Some(50.0),
            ..runtime(Some(0.0), Some(10.0), Some(20.0))
        };
        assert_eq!(
            merge_local_and_remote_play_time(PlayTimeParts::default(), with_hydra)
                .play_time_in_milliseconds,
            0
        );
    }

    #[test]
    fn preserves_epic_contribution_when_an_older_api_omits_its_fields() {
        let local = parts(Some(1_000), None, Some(3_000));

        let merged = merge_local_and_remote_play_time(local, runtime(Some(0.0), Some(2.0), None));
        assert_eq!(merged.play_time_in_milliseconds, 1_000);
        assert_eq!(merged.steam_play_time_in_milliseconds, 2_000);
        assert_eq!(merged.epic_play_time_in_milliseconds, 3_000);

        let combined = RemotePlayTimeInput {
            play_time_in_seconds: Some(8.0),
            ..runtime(None, Some(2.0), None)
        };
        assert_eq!(
            merge_local_and_remote_play_time(local, combined).play_time_in_milliseconds,
            3_000
        );

        let explicit = RemotePlayTimeInput {
            has_active_epic_import: Some(false),
            ..runtime(Some(0.0), Some(2.0), None)
        };
        assert_eq!(
            merge_local_and_remote_play_time(local, explicit).epic_play_time_in_milliseconds,
            0
        );
    }
}
