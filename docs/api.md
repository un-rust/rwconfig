---
title: API Reference
outline: deep
---

# API Reference

## RWConfig

In-memory config with get/set and dirty-tracking. Call `save()` to write back to the file.

### Methods

#### `from_file(path: impl AsRef<Path>) -> Result<Self, Error>`

Load config from a file. Format is inferred from the file extension. Fails for unsupported extensions or invalid content.

#### `get(&self, path: &str) -> Option<&Value>`

Get a value by dot path (e.g. `"info.a"`, `"a.b.c"`). Returns a reference into the config. Missing path segments or keys yield `None`.

#### `set(&mut self, path: &str, value: Value) -> Result<(), Error>`

Set a value at the given path. Creates parent keys as objects if needed. Marks the config dirty. Returns `Error::Path` for empty path or when a path segment is not an object.

#### `is_dirty(&self) -> bool`

Returns whether any change has been made since load or last save.

#### `save(&mut self) -> Result<(), Error>`

Write all changes to the config file. No-op if not dirty. Clears the dirty flag on success.

---

## Error

Unified error type for the crate.

| Variant | Description |
|---------|-------------|
| `Error::Io(io::Error)` | I/O error (e.g. file not found, permission denied). |
| `Error::Parse(String)` | Parse error or unsupported file extension. |
| `Error::Path(String)` | Invalid path (empty path, or segment not an object). |

Implements `std::error::Error`, `Display`, `From<io::Error>`, and `From<Error>` for `io::Error` (so `?` works with `io::Result` when your function returns `Error`).

---

## ConfigFormat

Supported config format (detected from file extension).

| Variant | Extensions |
|---------|------------|
| `ConfigFormat::Json` | `.json`, `.json5`, `.jsonc` |
| `ConfigFormat::Yaml` | `.yaml`, `.yml` |
| `ConfigFormat::Toml` | `.toml` |

### Method

#### `from_path(path: &Path) -> Option<Self>`

Infer format from the path’s extension. Returns `None` for unsupported extensions.

---

## Path helpers

### `get_path(root: &Value, path: &str) -> Option<&Value>`

Get a value by dot path from a `serde_json::Value`. Same path rules as `RWConfig::get`.

### `set_path(root: &mut Value, path: &str, value: Value) -> Result<(), Error>`

Set a value by dot path on a `serde_json::Value`. Same path and error rules as `RWConfig::set`.

---

## Re-exports

- **Value** — Re-exported from `serde_json::Value` for convenience.
