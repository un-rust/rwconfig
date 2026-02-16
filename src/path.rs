//! Dot-path get/set on `serde_json::Value`.

use crate::error::Error;
use serde_json::{Map, Value};

/// Get a value by dot path (e.g. `"info.a"`, `"a.b.c"`).
pub fn get<'a>(root: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = root;
    for key in path.split('.') {
        current = current.get(key)?;
    }
    Some(current)
}

/// Set a value by dot path. Creates parent keys as objects if needed.
pub fn set(root: &mut Value, path: &str, value: Value) -> Result<(), Error> {
    let mut keys: Vec<&str> = path.split('.').collect();
    let last = keys.pop().ok_or_else(|| Error::Path("empty path".into()))?;
    let mut current = root;
    for key in keys {
        let obj = current
            .as_object_mut()
            .ok_or_else(|| Error::Path("path segment is not an object".into()))?;
        current = obj
            .entry(key)
            .or_insert_with(|| Value::Object(Map::new()));
    }
    current
        .as_object_mut()
        .ok_or_else(|| Error::Path("path parent is not an object".into()))?
        .insert(last.to_string(), value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_set_path() {
        let mut root = json!({"info": {"a": 1, "b": 2}});
        assert_eq!(get(&root, "info.a"), Some(&json!(1)));
        set(&mut root, "info.c", json!(3)).unwrap();
        assert_eq!(get(&root, "info.c"), Some(&json!(3)));
    }
}
