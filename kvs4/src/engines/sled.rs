use super::KvsEngine;
use crate::{KvsError, Result};
use sled::{Db, Tree};

#[derive(Clone)]
pub struct SledStore {
    db: Db,
}

impl SledStore {
    pub fn new(db: Db) -> Self {
        SledStore { db }
    }
}

impl KvsEngine for SledStore {
    fn set(&self, key: String, value: String) -> Result<()> {
        let tree: &Tree = &self.db;
        tree.insert(key, value.into_bytes()).map(|_| ())?;
        self.db.flush()?;
        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        let tree: &Tree = &self.db;
        let r = tree
            .get(key)?
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?;
        Ok(r)
    }

    fn remove(&self, key: String) -> Result<()> {
        let tree: &Tree = &self.db;
        tree.remove(key)?.ok_or(KvsError::KeyNotFound)?;
        tree.flush()?;
        Ok(())
    }
}
