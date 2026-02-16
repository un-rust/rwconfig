//! RWConfig: read config, track changes via get/set, flush with `save()`.
//!
//! Rust has no language-level getters/setters like TypeScript; this crate provides
//! the same pattern with methods: `get(path)` / `set(path, value)` and `save()`.

mod config;
mod error;
mod format;
mod path;

pub use config::RWConfig;
pub use error::Error;
pub use format::ConfigFormat;
pub use path::{get as get_path, set as set_path};
pub use serde_json::Value;
