---
title: Supported formats
outline: deep
---

# Supported formats

rwconfig infers the config format from the **file extension**. No need to specify the format explicitly.

## By extension

| Extension | Format | Notes |
|-----------|--------|--------|
| `.json`   | JSON  | Standard JSON. |
| `.json5`  | JSON  | JSON5 (trailing commas, comments, etc.). |
| `.jsonc`  | JSON  | JSON with comments. |
| `.yaml`   | YAML  | YAML 1.x. |
| `.yml`    | YAML  | Same as YAML. |
| `.toml`   | TOML  | TOML. |

## Unsupported

Other extensions (e.g. `.ini`, `.cfg`, `.txt`) are not supported. `RWConfig::from_file` will return `Error::Parse` with a message about the unsupported extension.

## Internal representation

All formats are parsed into a single in-memory representation (`serde_json::Value`). Nested structures map to nested objects; you use the same dot-path API regardless of format:

- **JSON** `{"info": {"a": 1}}` → `get("info.a")` → `1`
- **YAML** `info:\n  a: 1` → same structure → `get("info.a")` → `1`
- **TOML** `[info]\na = 1` → same structure → `get("info.a")` → `1`

When you call `save()`, the in-memory tree is serialized back using the format that was inferred at load time (from the same file path).
