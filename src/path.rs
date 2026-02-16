//! Dot-path get/set on `serde_json::Value`.
//!
//! Paths use dot notation, e.g. `"server.port"` or `"a.b.c"`.
//! `set` creates parent keys as objects if they don't exist.

use crate::error::Error;
use serde_json::{Map, Value};

/// Get a value by dot path (e.g. `"info.a"`, `"a.b.c"`).
///
/// Returns `None` if any segment in the path is missing or not an object.
pub fn get<'a>(root: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = root;
    for key in path.split('.') {
        current = current.get(key)?;
    }
    Some(current)
}

/// Set a value by dot path. Creates parent keys as objects if needed.
///
/// Returns an error if path is empty, or if a non-terminal segment is not an object.
pub fn set(root: &mut Value, path: &str, value: Value) -> Result<(), Error> {
    if path.trim().is_empty() {
        return Err(Error::Path("empty path".into()));
    }
    let mut keys: Vec<&str> = path.split('.').collect();
    let last = keys.pop().ok_or_else(|| Error::Path("empty path".into()))?;

    // Traverse/create parent path segments
    let mut current = root;
    for key in keys {
        let obj = current
            .as_object_mut()
            .ok_or_else(|| Error::Path("path segment is not an object".into()))?;
        // Create parent object if missing
        current = obj.entry(key).or_insert_with(|| Value::Object(Map::new()));
    }

    // Insert or overwrite the final value
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
    fn get_set_nested_path() {
        let mut root = json!({"info": {"a": 1, "b": 2}});
        assert_eq!(get(&root, "info.a"), Some(&json!(1)));
        assert_eq!(get(&root, "info.b"), Some(&json!(2)));
        set(&mut root, "info.c", json!(3)).unwrap();
        assert_eq!(get(&root, "info.c"), Some(&json!(3)));
    }

    #[test]
    fn get_root_key() {
        let root = json!({"a": 1, "b": 2});
        assert_eq!(get(&root, "a"), Some(&json!(1)));
        assert_eq!(get(&root, "b"), Some(&json!(2)));
    }

    #[test]
    fn get_missing_path_returns_none() {
        let root = json!({"a": 1});
        assert_eq!(get(&root, "a.b"), None);
        assert_eq!(get(&root, "missing"), None);
    }

    #[test]
    fn set_creates_parent_objects() {
        let mut root = json!({});
        set(&mut root, "x.y.z", json!(42)).unwrap();
        assert_eq!(get(&root, "x.y.z"), Some(&json!(42)));
    }

    #[test]
    fn set_empty_path_errors() {
        let mut root = json!({"a": 1});
        assert!(set(&mut root, "", json!(1)).is_err());
    }

    #[test]
    fn set_on_non_object_errors() {
        let mut root = json!({"a": 42}); // "a" is number
        assert!(set(&mut root, "a.b", json!(1)).is_err());
    }
}
