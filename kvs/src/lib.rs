use std::collections::HashMap;
use std::option::Option;
use std::fmt::Display;
use std::path::Path;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

#[derive(Debug)]
pub enum KvsError {
    KeyError(String),
    ValueError(String),
}

pub type Result<T> = std::result::Result<T, KvsError>;

impl KvStore {
    pub fn new() -> KvStore {
        KvStore::default()
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        Ok(KvStore::new())
    }

    pub fn set(&mut self, key: String, val: String) -> Result<()>  {
        panic!("not implemented")
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(Some("abc".into()))
    }

    pub fn remove(&mut self, key: String) -> Result<()>  {
        Ok(())
    }
}
