//! Rust port of Hydra's `src/main/level` storage layer.
//!
//! The TypeScript implementation uses `classic-level` (LevelDB) with
//! sublevels that store JSON values. This port keeps the same logical model
//! (named sublevels, JSON values, `shop:objectId` keys for games) on top of
//! `redb`, a pure-Rust embedded key/value store, so no native dependency is
//! needed.

pub mod keys;
pub mod level;
pub mod repos;

pub use level::{LevelDb, LevelError, LevelResult, Sublevel};
pub use repos::{
    DownloadSourcesSublevel, DownloadsSublevel, GamesSublevel, ThemesSublevel,
    UserPreferencesSublevel,
};
