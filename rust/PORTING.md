# Rust port status

Hydra's TypeScript source is ~419k lines (`src/`, tests included): ~126k in the
Electron main process, ~134k in the React renderer, ~95k in the Big Picture
renderer, plus ~57k lines of locale JSON and ~10k of shared code.

Converting all of it is not a mechanical translation: the renderer and Big
Picture UIs are React/SCSS component trees, and the main process is built on
Electron APIs. Those need replacement frameworks (a Rust UI toolkit and a Rust
app shell), not a line-by-line port. The work that *can* be ported faithfully
is the domain model and the non-UI backend logic, and that is what this
workspace does.

No paid services or API keys are involved: every dependency here is an open
source crate, the hoster resolvers talk to the same public endpoints Hydra
already uses, and none of the ported code needs a key.

## Layout

```
rust/
  crates/hydra-types    # src/types/*.ts + src/shared/constants.ts
  crates/hydra-shared   # src/shared/*.ts pure logic (with ported tests)
  crates/hydra-store    # src/main/level/* on redb (pure-Rust embedded KV)
  crates/hydra-hosters  # src/main/services/hosters/*
  crates/hydra-demo     # runnable tour: cargo run -p hydra-demo
  scripts/gen-char-map.mjs
```

Build and test:

```bash
cd rust
cargo test --workspace
cargo run -p hydra-demo
```

## Ported 1:1

| TypeScript | Rust | Notes |
| --- | --- | --- |
| `src/shared/playtime.ts` | `hydra-shared::playtime` | All 8 cases from `playtime.test.ts` ported |
| `src/shared/format-bit-rate.ts` | `hydra-shared::format_bit_rate` | Tests ported |
| `src/shared/tracker-list.ts` | `hydra-shared::tracker_list` | Tests ported |
| `src/shared/trim-trailing-slashes.ts` | `hydra-shared::trim_trailing_slashes` | Tests ported |
| `src/shared/archive-org.ts` | `hydra-shared::archive_org` | Percent encode/decode reimplemented |
| `src/shared/char-map.ts` | `hydra-shared::char_map` | Generated (459 entries) from the TS file |
| `src/shared/download-directories.ts` | `hydra-shared::download_directories` | Full 5-directory/ordering semantics |
| `src/main/services/screenshot-size.ts` | `hydra-shared::screenshot_size` | Tests ported |
| `src/shared/index.ts` (pure parts) | `hydra-shared::bytes`, `::routing` | `formatBytes`, `parseBytes`, `formatBytesToMbps`, name normalisation, `formatName`, `getDownloadersForUri(s)`, availability helpers, error types |
| `src/shared/constants.ts` | `hydra-types::enums` | `Downloader` keeps its persisted discriminants (`8` is unused), `Cracker`, `DownloadError`, `DownloadSourceStatus`, ... |
| `src/types/game.types.ts` | `hydra-types::game` | |
| `src/types/level.types.ts` | `hydra-types::level` | `Game`, `Download`, `UserPreferences`, layouts, screen state |
| `src/types/download.types.ts` | `hydra-types::download` | Progress, torrent files, TorBox/Real-Debrid/Premiumize/AllDebrid payloads |
| `src/types/emulator.types.ts` | `hydra-types::emulator` | Systems, configs, discs, memory-card records, install options |
| `src/types/theme.types.ts` | `hydra-types::theme` | |
| `src/types/index.ts` (models) | `hydra-types::catalogue` | `GameRepack`, `DownloadSource`, `ShopAssets`, `StartGameDownloadPayload`, ... |
| `src/main/level/level.ts`, `sublevels/keys.ts`, `sublevels/games.ts` | `hydra-store::{level, keys, repos}` | Same sublevel names and `shop:objectId` keys; values stay JSON |
| `src/main/level/sublevels/{downloads,user-preferences,themes,download-sources}` | `hydra-store::repos` | Typed repositories |
| `src/main/services/hosters/pixeldrain.ts` | `hydra-hosters::pixeldrain` | |
| `src/main/services/hosters/mediafire.ts` | `hydra-hosters::mediafire` | Lookbehind regexes rewritten for the `regex` crate |
| `src/main/services/hosters/filekeeper.ts` | `hydra-hosters::filekeeper` | Manual redirects, cookie jar, countdown, form post |

Because values are still JSON with the same field names, an existing Hydra
level database can be read by the Rust store without a migration.

## Deliberately deferred, with reasons

- **Renderer and Big Picture (`src/renderer`, `src/big-picture`, ~230k lines).**
  React/SCSS UI. A Rust port means rewriting the UI in a Rust toolkit, which is
  a product decision (component set, styling, accessibility) rather than a
  translation. No UI code was touched.
- **Electron main-process runtime** (`window-manager`, `events/*/index.ts` IPC
  wiring, `update-manager`, tray, `power-save-blocker`, ...). These are
  Electron API calls; they need a different shell (Tauri commands, or an
  axum/tao app) before the ported services have something to plug into.
- **Torrent engine** (`services/torrent-client.ts`, `native/torrent-bridge`).
  libtorrent is C++ bound through N-API today; under a Rust shell that layer is
  replaced by a Rust libtorrent binding, which must be chosen and pinned.
- **`Module` fields left as raw JSON.** A few nested models live in type files
  that were not ported yet: `Game.epicInstallation`
  (`epic-managed.types.ts`), `Download.epic`, `UserPreferences.screenshotHotkey`
  / `screenshotGamepadCombo` (`screenshot.types.ts`), and `ShopAssets.
  selectedArtworkTypes` (`artwork.types.ts`). They round-trip unchanged; only
  compile-time typing is missing.
- **Services not yet ported** (all substantial, none started): cloud saves
  (`main/services/cloud-save` + `native/hydra-native/src/cloud_save`),
  achievements, RetroAchievements, Steam/Epic integrations, emulation
  (PS1/PS2 memory cards, shadPS4 trophies, RetroArch), screenshots capture and
  upload, SSE/WebSocket client, UMU/Wine/Proton launching, ludusavi, themes,
  recompilation, notifications, `hydra-api` client.
- **Locales (`src/locales`, 36 languages).** Data, not code; reusable as-is by
  any Rust app (e.g. `fluent`/`rust-i18n`).
- **`date-fns`-based helpers** (`getDateLocale`, `formatDate`) and
  `generateAchievementCustomNotificationTest` — depend on renderer libraries
  and i18n bindings rather than domain logic.

## Verification

`cargo test --workspace` runs 96 tests across the four library crates (62 shared, 13 store, 11 types, 10 hosters): the TypeScript tests for playtime,
bit-rate, trackers, trailing slashes and screenshot sizing (ported case for
case), plus new tests for archive.org parsing, download-directory rules, the
level store (isolation, ordering, reopen, typed repositories), and the hoster
URL/HTML extraction logic. Network-dependent hoster calls are not exercised by
tests.
