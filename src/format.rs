//! Config file format detection and parse/stringify.
//!
//! Supports JSON (including JSON5, JSONC), YAML, and TOML.
//! Format is inferred from the file extension.

use crate::error::Error;
use c12_parser::{
    Formatted, parse_json, parse_toml, parse_yaml, stringify_json, stringify_toml, stringify_yaml,
};
use serde_json::Value;
use std::path::Path;

/// Supported config format (detected from file extension).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    /// JSON format (`.json`, `.json5`, `.jsonc`).
    Json,
    /// YAML format (`.yaml`, `.yml`).
    Yaml,
    /// TOML format (`.toml`).
    Toml,
}

impl ConfigFormat {
    /// Infer format from file path extension (case-insensitive).
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|e| e.to_str())
            .and_then(|e| match e.to_lowercase().as_str() {
                "json" | "json5" | "jsonc" => Some(ConfigFormat::Json),
                "yaml" | "yml" => Some(ConfigFormat::Yaml),
                "toml" => Some(ConfigFormat::Toml),
                _ => None,
            })
    }

    /// Parse text into a formatted value.
    pub fn parse(&self, text: &str) -> Result<Formatted<Value>, Error> {
        match self {
            ConfigFormat::Json => parse_json(text, None).map_err(|e| Error::Parse(e.to_string())),
            ConfigFormat::Yaml => parse_yaml(text, None).map_err(|e| Error::Parse(e.to_string())),
            ConfigFormat::Toml => parse_toml(text, None).map_err(|e| Error::Parse(e.to_string())),
        }
    }

    /// Stringify a formatted value back to text.
    pub fn stringify(&self, formatted: &Formatted<Value>) -> Result<String, Error> {
        match self {
            ConfigFormat::Json => {
                stringify_json(formatted, None).map_err(|e| Error::Parse(e.to_string()))
            }
            ConfigFormat::Yaml => {
                stringify_yaml(formatted, None).map_err(|e| Error::Parse(e.to_string()))
            }
            ConfigFormat::Toml => {
                stringify_toml(formatted, None).map_err(|e| Error::Parse(e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn from_path_json_extensions() {
        assert_eq!(
            ConfigFormat::from_path(Path::new("x.json")),
            Some(ConfigFormat::Json)
        );
        assert_eq!(
            ConfigFormat::from_path(Path::new("x.json5")),
            Some(ConfigFormat::Json)
        );
        assert_eq!(
            ConfigFormat::from_path(Path::new("x.JSONC")),
            Some(ConfigFormat::Json)
        );
    }

    #[test]
    fn from_path_yaml_extensions() {
        assert_eq!(
            ConfigFormat::from_path(Path::new("x.yaml")),
            Some(ConfigFormat::Yaml)
        );
        assert_eq!(
            ConfigFormat::from_path(Path::new("x.yml")),
            Some(ConfigFormat::Yaml)
        );
    }

    #[test]
    fn from_path_toml() {
        assert_eq!(
            ConfigFormat::from_path(Path::new("x.toml")),
            Some(ConfigFormat::Toml)
        );
    }

    #[test]
    fn from_path_unsupported() {
        assert_eq!(ConfigFormat::from_path(Path::new("x.ini")), None);
        assert_eq!(ConfigFormat::from_path(Path::new("x.txt")), None);
        assert_eq!(ConfigFormat::from_path(Path::new("no_ext")), None);
    }

    #[test]
    fn roundtrip_json() {
        let text = r#"{"a":1,"b":2}"#;
        let formatted = ConfigFormat::Json.parse(text).unwrap();
        let out = ConfigFormat::Json.stringify(&formatted).unwrap();
        let formatted2 = ConfigFormat::Json.parse(&out).unwrap();
        assert_eq!(formatted.value.get("a"), formatted2.value.get("a"));
        assert_eq!(formatted.value.get("b"), formatted2.value.get("b"));
    }

    #[test]
    fn roundtrip_yaml() {
        let text = "info:\n  a: 1\n  b: 2";
        let formatted = ConfigFormat::Yaml.parse(text).unwrap();
        let out = ConfigFormat::Yaml.stringify(&formatted).unwrap();
        let formatted2 = ConfigFormat::Yaml.parse(&out).unwrap();
        assert_eq!(
            formatted.value.get("info").and_then(|o| o.get("a")),
            formatted2.value.get("info").and_then(|o| o.get("a"))
        );
    }

    #[test]
    fn roundtrip_toml() {
        let text = "[info]\na = 1\nb = 2";
        let formatted = ConfigFormat::Toml.parse(text).unwrap();
        let out = ConfigFormat::Toml.stringify(&formatted).unwrap();
        let formatted2 = ConfigFormat::Toml.parse(&out).unwrap();
        assert_eq!(
            formatted.value.get("info").and_then(|o| o.get("a")),
            formatted2.value.get("info").and_then(|o| o.get("a"))
        );
    }
}
