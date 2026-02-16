//! Config file format detection and parse/stringify.

use crate::error::Error;
use c12_parser::{
    parse_json, parse_toml, parse_yaml, stringify_json, stringify_toml, stringify_yaml, Formatted,
};
use serde_json::Value;
use std::path::Path;

/// Supported config format (detected from file extension).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
}

impl ConfigFormat {
    /// Infer format from file path extension.
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
