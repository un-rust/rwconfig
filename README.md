# 📝 rwconfig

<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/rwconfig)
![Crates.io Total Downloads](https://img.shields.io/crates/d/rwconfig)
![docs.rs](https://img.shields.io/docsrs/rwconfig)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/un-rust/rwconfig)
![GitHub Repo stars](https://img.shields.io/github/stars/un-rust/rwconfig)
<!-- /automdrs -->

<!-- automdrs:description -->

Read/write config files with get/set and dirty-tracking; save() writes all changes at once

<!-- /automdrs -->

**[Full documentation →](https://docs.rs/rwconfig/)**

## Quick start

<!-- automdrs:cargo-add -->

```sh
cargo add rwconfig
```

<!-- /automdrs -->

## Usage

<!-- automdrs:file src="./src/main.rs" -->
```rust
//! Demo: RWConfig getter/setter pattern.
//!
//! Loads a config file, reads values via `get()`, modifies via `set()` (which marks dirty),
//! and persists all changes with a single `save()`.

use rwconfig::RWConfig;
use serde_json::json;

fn main() -> std::io::Result<()> {
    let path = "fixtures/test.config.json";
    let mut cfg = RWConfig::from_file(path)?;

    // Read value by dot-path (getter semantics)
    let a = cfg.get("a").and_then(|v| v.as_i64()).unwrap_or(0);
    println!("a = {}", a);

    // Write values (setter semantics — each set marks config dirty)
    cfg.set("a", json!(42))?;
    cfg.set("b", json!(100))?;

    println!("dirty = {}", cfg.is_dirty());

    // Persist all changes to disk; no-op if not dirty
    cfg.save()?;
    println!("saved.");
    Ok(())
}
```
<!-- /automdrs -->

## License

<!-- automdrs:contributors author="UnRUST" license="Apache-2.0" -->
Published under the [Apache-2.0](./LICENSE) license.
Made by [@UnRUST](https://github.com/un-rust) 💛
<br><br>
<a href="https://github.com/un-rust/rwconfig/graphs/contributors">
<img src="https://contrib.rocks/image?repo=un-rust/rwconfig" />
</a>
<!-- /automdrs -->

<!-- automdrs:with-automdrs -->

---

_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_

<!-- /automdrs -->