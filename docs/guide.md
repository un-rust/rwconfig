---
title: Guide
outline: deep
---

# Guide

## Quick start

Add the dependency:

```bash
cargo add rwconfig
```

Load a config file, read or change values, then save:

```rust
use rwconfig::{RWConfig, Error};
use serde_json::json;

fn main() -> Result<(), Error> {
    let mut cfg = RWConfig::from_file("config.json")?;

    // Read a value
    let port = cfg.get("server.port").and_then(|v| v.as_i64()).unwrap_or(8080);

    // Write values (each set marks the config dirty)
    cfg.set("server.port", json!(9090))?;
    cfg.set("server.host", json!("0.0.0.0"))?;

    // Write all changes to the file
    cfg.save()?;
    Ok(())
}
```

## Loading config

Format is inferred from the file extension:

- `.json`, `.json5`, `.jsonc` → JSON
- `.yaml`, `.yml` → YAML
- `.toml` → TOML

```rust
let cfg = RWConfig::from_file("app.json")?;   // JSON
let cfg = RWConfig::from_file("app.yaml")?;   // YAML
let cfg = RWConfig::from_file("app.toml")?;   // TOML
```

Unsupported extensions (e.g. `.ini`) return an `Error::Parse`.

## Path syntax

Paths use dot notation for nested keys:

| Path     | Meaning                    |
|----------|----------------------------|
| `"a"`    | Top-level key `a`          |
| `"info.a"` | Object `info`, then key `a` |
| `"a.b.c"` | Nested `a` → `b` → `c`      |

- **get(path)** — Returns `Option<&Value>`. Missing keys or path segments yield `None`.
- **set(path, value)** — Sets the value at path. Creates parent objects if they don’t exist. Empty path returns `Error::Path`.

## Dirty tracking

- After **load** or **save**, the config is not dirty.
- Every **set** marks it dirty.
- **save()** writes the file only when dirty, then clears the flag. Calling `save()` when not dirty is a no-op.

```rust
let mut cfg = RWConfig::from_file("config.json")?;
assert!(!cfg.is_dirty());

cfg.set("x", json!(1))?;
assert!(cfg.is_dirty());

cfg.save()?;
assert!(!cfg.is_dirty());
```

## Error handling

The crate uses a single error type `Error`:

| Variant   | When |
|-----------|------|
| `Error::Io`   | File read/write (e.g. file not found). |
| `Error::Parse`| Unsupported extension or invalid file content. |
| `Error::Path` | Invalid path (e.g. empty path, or setting through a non-object). |

`Error` implements `From<io::Error>`, so you can use `?` with `std::io::Result` in places that also return `Error`.

## Working with `serde_json::Value`

Values are `serde_json::Value`. Use its methods to inspect or build values:

```rust
use serde_json::json;

// Get as number/string/array
let n = cfg.get("port").and_then(|v| v.as_i64());
let s = cfg.get("name").and_then(|v| v.as_str());
let arr = cfg.get("tags").and_then(|v| v.as_array());

// Set with json! macro
cfg.set("port", json!(8080))?;
cfg.set("name", json!("myapp"))?;
cfg.set("tags", json!(["a", "b"]))?;
```

You can also use **get_path** and **set_path** on a raw `Value` (e.g. for custom trees):

```rust
use rwconfig::{get_path, set_path};
use serde_json::json;

let mut root = json!({"info": {"a": 1}});
let v = get_path(&root, "info.a");  // Some(&1)
set_path(&mut root, "info.b", json!(2))?;
```
