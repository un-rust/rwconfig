//! # rwconfig — Read/Write Config with Dirty Tracking
//!
//! A lightweight config file reader/writer that tracks modifications via `get`/`set`
//! and flushes all changes in one go with `save()`. Like getters/setters with dirty-tracking.
//!
//! ## Features
//!
//! - **Dot-path access** — Use paths like `"server.port"` or `"a.b.c"` for nested values
//! - **Dirty tracking** — Every `set()` marks the config dirty; `save()` writes only when needed
//! - **Multi-format** — JSON, JSON5, JSONC, YAML, TOML; format is inferred from file extension
//!
//! ## Example
//!
//! ```ignore
//! let mut cfg = RWConfig::from_file("config.json")?;
//! let port = cfg.get("server.port").and_then(|v| v.as_i64()).unwrap_or(8080);
//! cfg.set("server.port", json!(9090))?;
//! cfg.save()?;
//! ```

mod config;
mod error;
mod format;
mod path;

/// Main config struct: load from file, get/set by path, save when dirty.
pub use config::RWConfig;

/// Unified error type (Io, Parse, Path variants).
pub use error::Error;

/// Config format enum (Json, Yaml, Toml).
pub use format::ConfigFormat;

/// Raw path operations on `serde_json::Value`.
pub use path::{get as get_path, set as set_path};

/// Re-export for convenience when working with config values.
pub use serde_json::Value;
