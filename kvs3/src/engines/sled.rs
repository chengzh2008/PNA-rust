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
    fn set(&mut self, key: String, value: String) -> Result<()> {
        todo!();
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        todo!();
    }

    fn remove(&mut self, key: String) -> Result<()> {
        todo!();
    }
}
