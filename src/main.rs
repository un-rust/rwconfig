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
