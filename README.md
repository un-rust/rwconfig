# rwconfig

<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/rwconfig)
![Crates.io Total Downloads](https://img.shields.io/crates/d/rwconfig)
![docs.rs](https://img.shields.io/docsrs/rwconfig)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/un-rust/rwconfig)
![GitHub Repo stars](https://img.shields.io/github/stars/un-rust/rwconfig)
<!-- /automdrs -->

Read config files, change values with get/set, then write everything back in one go with `save()`. Like getters and setters with dirty-tracking.

**[Full documentation →](https://betterhyq.github.io/rwconfig/)**

## Features

- **Get/set by path** — Use dot paths (`"info.a"`, `"a.b.c"`) to read and write nested values.
- **Dirty tracking** — Every `set()` marks the config dirty; `save()` writes only when needed.
- **Multiple formats** — JSON, JSON5, JSONC, YAML, TOML; format is inferred from the file extension.

## Installation

<!-- automdrs:cargo-add -->

```sh
cargo add rwconfig
```

<!-- /automdrs -->

## Usage

```rust
use rwconfig::{RWConfig, Error};
use serde_json::json;

fn main() -> Result<(), Error> {
    let mut cfg = RWConfig::from_file("config.json")?;

    // get (read)
    let port = cfg.get("server.port").and_then(|v| v.as_i64()).unwrap_or(8080);

    // set (write) — marks config dirty
    cfg.set("server.port", json!(9090))?;
    cfg.set("server.host", json!("0.0.0.0"))?;

    // write all changes to disk
    cfg.save()?;
    Ok(())
}
```

## Supported formats

| Extension | Format |
|-----------|--------|
| `.json`, `.json5`, `.jsonc` | JSON |
| `.yaml`, `.yml` | YAML |
| `.toml` | TOML |

## API overview

| Item | Description |
|------|-------------|
| `RWConfig::from_file(path)` | Load config from a file. |
| `cfg.get(path)` | Get a value by dot path → `Option<&Value>`. |
| `cfg.set(path, value)` | Set a value; creates parent keys if needed. |
| `cfg.is_dirty()` | Whether there are unsaved changes. |
| `cfg.save()` | Write changes to the file (no-op if not dirty). |
| `Error` | `Io`, `Parse`, `Path` variants. |
| `ConfigFormat` | `Json`, `Yaml`, `Toml`. |
| `get_path` / `set_path` | Work on a raw `serde_json::Value` by path. |

## Contribution

<details>
<summary>Local development</summary>

- Clone this repository
- Install the latest [Rust](https://rust-lang.org/)
- Run tests: `cargo test`
- Run the demo: `cargo run`

</details>

## License

<!-- automdrs:contributors author="YONGQI" license="MIT" -->
Published under the [MIT](./LICENSE) license.
Made by [@YONGQI](https://github.com/un-rust) 💛
<br><br>
<a href="https://github.com/un-rust/rwconfig/graphs/contributors">
<img src="https://contrib.rocks/image?repo=un-rust/rwconfig" />
</a>
<!-- /automdrs -->

<!-- automdrs:with-automdrs -->

---

_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_

<!-- /automdrs -->