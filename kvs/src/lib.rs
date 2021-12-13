use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        panic!();
    }

    pub fn set(&mut self, key: String, val: String) {
        panic!();
    }

    pub fn get(&self, key: String) -> Option<String> {
        panic!();
    }

    pub fn remove(&mut self, key: String) -> KvStore {
        panic!();
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
