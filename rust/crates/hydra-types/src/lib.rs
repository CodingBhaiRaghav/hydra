//! Rust port of Hydra's domain models.
//!
//! Source of truth: `src/types/*.ts` and `src/shared/constants.ts`.
//! Field names and JSON shapes match the TypeScript types so that existing
//! level databases and API payloads stay readable by both implementations.

pub mod catalogue;
pub mod download;
pub mod emulator;
pub mod enums;
pub mod game;
pub mod level;
pub mod theme;

pub use catalogue::*;
pub use download::*;
pub use emulator::*;
pub use enums::*;
pub use game::*;
pub use level::*;
pub use theme::*;

/// Timestamps that TypeScript types as `Date` are persisted as ISO-8601
/// strings, because level stores values with `JSON.stringify`.
pub type IsoDateTime = String;
