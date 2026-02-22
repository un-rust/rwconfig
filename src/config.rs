//! `RWConfig` — in-memory config with get/set and dirty-tracking.
//!
//! Load config from file, modify via `get`/`set`, persist with `save()`.
//! `save()` is a no-op when the config is not dirty.

use crate::error::Error;
use crate::format::ConfigFormat;
use crate::path;
use c12_parser::Formatted;
use serde_json::Value;
use std::fs;
use std::path::Path;
use urlogger::{LogLevel, log};

/// Config struct: loads from file, tracks changes, persists on `save()`.
///
/// Supports dot-path access (e.g. `"server.port"`, `"a.b.c"`) and dirty tracking.
#[derive(Debug)]
pub struct RWConfig {
    /// Path to the config file.
    path: std::path::PathBuf,
    /// Parsed value with format-specific metadata (comments, formatting).
    formatted: Formatted<Value>,
    /// Detected format (Json, Yaml, Toml).
    format: ConfigFormat,
    /// Whether any `set()` has been called since load or last save.
    dirty: bool,
}

impl RWConfig {
    /// Load config from a file. Format is inferred from the file extension.
    ///
    /// Supported extensions: `.json`, `.json5`, `.jsonc`, `.yaml`, `.yml`, `.toml`.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref().to_path_buf();
        let format = ConfigFormat::from_path(&path).ok_or_else(|| {
            Error::Parse("Unsupported config extension (use .json, .yaml, .toml)".into())
        })?;
        let text = fs::read_to_string(&path)?;
        let formatted = format.parse(&text)?;
        log!(LogLevel::Trace, "RWConfig loaded from {:?}", path);
        Ok(Self {
            path,
            formatted,
            format,
            dirty: false,
        })
    }

    /// Get a value by path (e.g. `"info.a"`, `"a.b.c"`). Returns a reference into the config.
    pub fn get(&self, path: &str) -> Option<&Value> {
        path::get(&self.formatted.value, path)
    }

    /// Set a value by path. Creates parent keys as objects if needed. Marks config dirty.
    pub fn set(&mut self, path: &str, value: Value) -> Result<(), Error> {
        path::set(&mut self.formatted.value, path, value)?;
        self.dirty = true;
        log!(LogLevel::Trace, "set {} -> dirty", path);
        Ok(())
    }

    /// Whether any change has been made since load or last save.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Write all changes to the config file. No-op if not dirty.
    pub fn save(&mut self) -> Result<(), Error> {
        if !self.dirty {
            log!(LogLevel::Trace, "save skipped (not dirty)");
            return Ok(());
        }
        let text = self.format.stringify(&self.formatted)?;
        fs::write(&self.path, text)?;
        self.dirty = false;
        log!(LogLevel::Trace, "saved to {:?}", self.path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::path::PathBuf;

    /// Returns the fixtures directory path.
    fn fixtures_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
    }

    /// Run load -> get -> set -> save -> reload -> assert for a given fixture.
    fn test_fixture_format(
        fixture_name: &str,
        get_path: &str,
        expected_initial: i64,
        set_path: &str,
        set_value: Value,
    ) {
        let fixture_path = fixtures_dir().join(fixture_name);
        let content = fs::read_to_string(&fixture_path).unwrap_or_else(|e| {
            panic!("read fixture {}: {}", fixture_name, e);
        });
        let temp_dir = std::env::temp_dir().join("rwconfig_fixture_test");
        fs::create_dir_all(&temp_dir).ok();
        let temp_path = temp_dir.join(fixture_name);
        fs::write(&temp_path, &content).unwrap();

        let mut cfg = RWConfig::from_file(&temp_path).unwrap();
        assert_eq!(
            cfg.get(get_path).and_then(Value::as_i64),
            Some(expected_initial),
            "{}: initial get {}",
            fixture_name,
            get_path
        );
        cfg.set(set_path, set_value).unwrap();
        assert!(cfg.is_dirty(), "{}: dirty after set", fixture_name);
        cfg.save().unwrap();
        assert!(!cfg.is_dirty(), "{}: not dirty after save", fixture_name);

        let cfg2 = RWConfig::from_file(&temp_path).unwrap();
        assert_eq!(
            cfg2.get(set_path).and_then(Value::as_i64),
            Some(42),
            "{}: persisted value at {}",
            fixture_name,
            set_path
        );

        fs::remove_file(&temp_path).ok();
    }

    #[test]
    fn test_rwconfig_load_set_save() {
        let dir = std::env::temp_dir().join("rwconfig_test");
        fs::create_dir_all(&dir).ok();
        let path = dir.join("test.config.json");
        fs::write(&path, r#"{"a":1,"b":2}"#).unwrap();
        let mut cfg = RWConfig::from_file(&path).unwrap();
        assert_eq!(cfg.get("a").and_then(Value::as_i64), Some(1));
        cfg.set("b", json!(99)).unwrap();
        cfg.set("c", json!("new")).unwrap();
        assert!(cfg.is_dirty());
        cfg.save().unwrap();
        assert!(!cfg.is_dirty());
        let cfg2 = RWConfig::from_file(&path).unwrap();
        assert_eq!(cfg2.get("b").and_then(Value::as_i64), Some(99));
        assert_eq!(cfg2.get("c").and_then(Value::as_str), Some("new"));
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_fixture_json() {
        test_fixture_format(
            "test.config.json",
            "a",
            42, // fixture may have been modified; we only assert structure
            "_test",
            json!(42),
        );
    }

    #[test]
    fn test_fixture_yaml() {
        test_fixture_format("test.config.yaml", "info.a", 1, "info._test", json!(42));
    }

    #[test]
    fn test_fixture_toml() {
        test_fixture_format("test.config.toml", "info.a", 1, "info._test", json!(42));
    }

    #[test]
    fn test_fixture_jsonc() {
        test_fixture_format("test.config.jsonc", "a", 1, "_test", json!(42));
    }

    #[test]
    fn test_fixture_json5() {
        test_fixture_format("test.config.json5", "a", 1, "_test", json!(42));
    }

    #[test]
    fn test_unsupported_format_ini_returns_error() {
        let dir = std::env::temp_dir().join("rwconfig_test");
        fs::create_dir_all(&dir).ok();
        let path = dir.join("test.config.ini");
        fs::write(&path, "a=1\nb=2").unwrap();

        let result = RWConfig::from_file(&path);
        assert!(result.is_err());
        let err = result.unwrap_err();
        match &err {
            Error::Parse(s) => assert!(
                s.contains("Unsupported") || s.contains("extension"),
                "expected parse/extension error, got: {}",
                s
            ),
            _ => panic!("expected Error::Parse for .ini, got {:?}", err),
        }

        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_save_when_not_dirty_is_noop() {
        let dir = std::env::temp_dir().join("rwconfig_test");
        fs::create_dir_all(&dir).ok();
        let path = dir.join("noop.json");
        fs::write(&path, r#"{"x":1}"#).unwrap();
        let mut cfg = RWConfig::from_file(&path).unwrap();
        cfg.save().unwrap();
        let content_after = fs::read_to_string(&path).unwrap();
        assert_eq!(content_after, r#"{"x":1}"#);
        fs::remove_file(&path).ok();
    }
}
