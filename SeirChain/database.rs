use std::path::Path;
use rocksdb::{DB, Options};

pub struct Database {
    db: DB,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, rocksdb::Error> {
        let path = Path::new(path);
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(Database { db })
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), rocksdb::Error> {
        self.db.put(key, value)
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, rocksdb::Error> {
        self.db.get(key)
    }

    pub fn delete(&self, key: &[u8]) -> Result<(), rocksdb::Error> {
        self.db.delete(key)
    }
}
