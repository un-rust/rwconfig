//! Demo: RWConfig getter/setter pattern — modifications are tracked; save() writes once.

use rwconfig::RWConfig;
use serde_json::json;

fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let path = "fixtures/test.config.json";
    let mut cfg = RWConfig::from_file(path)?;

    // get (like a getter)
    let a = cfg.get("a").and_then(|v| v.as_i64()).unwrap_or(0);
    println!("a = {}", a);

    // set (like a setter) — each set marks config dirty
    cfg.set("a", json!(42)).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    cfg.set("b", json!(100)).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    println!("dirty = {}", cfg.is_dirty());

    // one save() syncs all changes to the file
    cfg.save()?;
    println!("saved.");
    Ok(())
}
