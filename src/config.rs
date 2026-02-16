//! RWConfig: in-memory config with get/set and dirty-tracking; `save()` writes to file.

use crate::error::Error;
use crate::format::ConfigFormat;
use crate::path;
use c12_parser::Formatted;
use log::trace;
use serde_json::Value;
use std::path::Path;
use std::fs;

/// Config that records every modification; call `save()` to write back to file.
pub struct RWConfig {
    path: std::path::PathBuf,
    formatted: Formatted<Value>,
    format: ConfigFormat,
    dirty: bool,
}

impl RWConfig {
    /// Load config from a file. Format is inferred from the file extension.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref().to_path_buf();
        let format = ConfigFormat::from_path(&path).ok_or_else(|| {
            Error::Parse("Unsupported config extension (use .json, .yaml, .toml)".into())
        })?;
        let text = fs::read_to_string(&path)?;
        let formatted = format.parse(&text)?;
        trace!("RWConfig loaded from {:?}", path);
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
        trace!("set {} -> dirty", path);
        Ok(())
    }

    /// Whether any change has been made since load or last save.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Write all changes to the config file. No-op if not dirty.
    pub fn save(&mut self) -> Result<(), Error> {
        if !self.dirty {
            trace!("save skipped (not dirty)");
            return Ok(());
        }
        let text = self.format.stringify(&self.formatted)?;
        fs::write(&self.path, text)?;
        self.dirty = false;
        trace!("saved to {:?}", self.path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_rwconfig_load_set_save() {
        let dir = std::env::temp_dir().join("rwconfig_test");
        std::fs::create_dir_all(&dir).ok();
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
        std::fs::remove_file(&path).ok();
    }
}
