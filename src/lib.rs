//! # Extendable Assets
//!
//! An asset framework for graphics and games that provides flexible asset management,
//! loading, and saving capabilities.

mod asset;
mod asset_type;
mod filesystem;
mod loader;
mod manager;
mod saver;
mod util;

/// Third-party re-exports for external crates used by this library.
pub mod third_party;

pub use asset::*;
pub use asset_type::*;
pub use filesystem::*;
pub use loader::*;
pub use manager::*;
pub use saver::*;
