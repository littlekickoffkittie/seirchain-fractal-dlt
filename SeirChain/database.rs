use std::path::Path;
use rocksdb::{DB, Options, ColumnFamilyDescriptor};
use self::schema::{CF_DEFAULT, CF_TRIADS, CF_TRANSACTIONS, CF_WALLETS};

pub mod schema;

pub struct Database {
    db: DB,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, rocksdb::Error> {
        let path = Path::new(path);
        let cfs = vec![
            ColumnFamilyDescriptor::new(CF_DEFAULT, Options::default()),
            ColumnFamilyDescriptor::new(CF_TRIADS, Options::default()),
            ColumnFamilyDescriptor::new(CF_TRANSACTIONS, Options::default()),
            ColumnFamilyDescriptor::new(CF_WALLETS, Options::default()),
        ];
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        let db = DB::open_cf_descriptors(&opts, path, cfs)?;
        Ok(Database { db })
    }

    pub fn put(&self, cf: &str, key: &[u8], value: &[u8]) -> Result<(), rocksdb::Error> {
        let cf_handle = self.db.cf_handle(cf).unwrap();
        self.db.put_cf(cf_handle, key, value)
    }

    pub fn get(&self, cf: &str, key: &[u8]) -> Result<Option<Vec<u8>>, rocksdb::Error> {
        let cf_handle = self.db.cf_handle(cf).unwrap();
        self.db.get_cf(cf_handle, key)
    }

    pub fn delete(&self, cf: &str, key: &[u8]) -> Result<(), rocksdb::Error> {
        let cf_handle = self.db.cf_handle(cf).unwrap();
        self.db.delete_cf(cf_handle, key)
    }
}
