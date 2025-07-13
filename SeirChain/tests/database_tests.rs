use seirchain::database::Database;
use rocksdb::Options;

#[test]
fn test_database_creation() {
    let path = "_test_db_creation";
    let db = Database::new(path);
    assert!(db.is_ok());
    let _ = rocksdb::DB::destroy(&Options::default(), path);
}

#[test]
fn test_database_put_get_delete() {
    let path = "_test_db_put_get_delete";
    let db = Database::new(path).unwrap();

    let key = b"test_key";
    let value = b"test_value";

    // Test put
    let put_result = db.put("default", key, value);
    assert!(put_result.is_ok());

    // Test get
    let get_result = db.get("default", key).unwrap();
    assert!(get_result.is_some());
    assert_eq!(get_result.unwrap(), value);

    // Test delete
    let delete_result = db.delete("default", key);
    assert!(delete_result.is_ok());

    // Test get after delete
    let get_result_after_delete = db.get("default", key).unwrap();
    assert!(get_result_after_delete.is_none());

    let _ = rocksdb::DB::destroy(&Options::default(), path);
}
