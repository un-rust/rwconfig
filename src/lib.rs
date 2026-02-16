//! RWConfig: read config, track changes via get/set, flush with `save()`.
//!
//! Rust has no language-level getters/setters like TypeScript; this crate provides
//! the same pattern with methods: `get(path)` / `set(path, value)` and `save()`.

use c12_parser::{parse_json, parse_toml, parse_yaml, stringify_json, stringify_toml, stringify_yaml, Formatted};
use log::trace;
use serde_json::Value;
use std::path::Path;
use std::{fs, io};

/// Supported config format (detected from file extension).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
}

impl ConfigFormat {
    fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|e| e.to_str())
            .and_then(|e| match e.to_lowercase().as_str() {
                "json" | "json5" | "jsonc" => Some(ConfigFormat::Json),
                "yaml" | "yml" => Some(ConfigFormat::Yaml),
                "toml" => Some(ConfigFormat::Toml),
                _ => None,
            })
    }
}

/// Config that records every modification; call `save()` to write back to file.
pub struct RWConfig {
    path: std::path::PathBuf,
    formatted: Formatted<Value>,
    format: ConfigFormat,
    dirty: bool,
}

impl RWConfig {
    /// Load config from a file. Format is inferred from the file extension.
    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let format = ConfigFormat::from_path(&path)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Unsupported config extension (use .json, .yaml, .toml)"))?;
        let text = fs::read_to_string(&path)?;
        let formatted = parse_to_value(&text, format)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
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
        get_path(&self.formatted.value, path)
    }

    /// Set a value by path. Creates parent keys as objects if needed. Marks config dirty.
    pub fn set(&mut self, path: &str, value: Value) -> Result<(), String> {
        set_path(&mut self.formatted.value, path, value)?;
        self.dirty = true;
        trace!("set {} -> dirty", path);
        Ok(())
    }

    /// Whether any change has been made since load or last save.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Write all changes to the config file. No-op if not dirty.
    pub fn save(&mut self) -> io::Result<()> {
        if !self.dirty {
            trace!("save skipped (not dirty)");
            return Ok(());
        }
        let text = stringify_value(&self.formatted, self.format)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&self.path, text)?;
        self.dirty = false;
        trace!("saved to {:?}", self.path);
        Ok(())
    }
}

fn get_path<'a>(root: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = root;
    for key in path.split('.') {
        current = current.get(key)?;
    }
    Some(current)
}

fn set_path(root: &mut Value, path: &str, value: Value) -> Result<(), String> {
    let mut keys: Vec<&str> = path.split('.').collect();
    let last = keys.pop().ok_or("empty path")?;
    let mut current = root;
    for key in keys {
        let obj = current.as_object_mut().ok_or("path segment is not an object")?;
        current = obj
            .entry(key)
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
    }
    current
        .as_object_mut()
        .ok_or("path parent is not an object")?
        .insert(last.to_string(), value);
    Ok(())
}

fn parse_to_value(text: &str, format: ConfigFormat) -> Result<Formatted<Value>, String> {
    match format {
        ConfigFormat::Json => parse_json(text, None).map_err(|e| e.to_string()),
        ConfigFormat::Yaml => parse_yaml(text, None).map_err(|e| e.to_string()),
        ConfigFormat::Toml => parse_toml(text, None).map_err(|e| e.to_string()),
    }
}

fn stringify_value(formatted: &Formatted<Value>, format: ConfigFormat) -> Result<String, String> {
    match format {
        ConfigFormat::Json => stringify_json(formatted, None).map_err(|e| e.to_string()),
        ConfigFormat::Yaml => stringify_yaml(formatted, None).map_err(|e| e.to_string()),
        ConfigFormat::Toml => stringify_toml(formatted, None).map_err(|e| e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_set_path() {
        let mut root = json!({"info": {"a": 1, "b": 2}});
        assert_eq!(get_path(&root, "info.a"), Some(&json!(1)));
        set_path(&mut root, "info.c", json!(3)).unwrap();
        assert_eq!(get_path(&root, "info.c"), Some(&json!(3)));
    }

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
