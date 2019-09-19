extern crate lmdb_zero as lmdb;

fn main() {
    let path = "./lmdb.db";
    let env = unsafe {
        lmdb::EnvBuilder::new().unwrap().open( path, lmdb::open::Flags::empty(), 0o600).unwrap()
    };

    let db = lmdb::Database::open( &env, None, &lmdb::DatabaseOptions::defaults()).unwrap();
    {
        let txn = lmdb::WriteTransaction::new( &env ).unwrap();
        {
            let mut access = txn.access();

            access.put(&db, "Key-A", "Value-A", lmdb::put::Flags::empty()).unwrap();
            access.put(&db, "Key-B", "Value-B", lmdb::put::Flags::empty()).unwrap();
        }
        txn.commit().unwrap();
    }

    {
        let txn = lmdb::ReadTransaction::new(&env).unwrap();
        let access = txn.access();

        let a: &str = access.get(&db, "Key-A").unwrap();
        assert_eq!("Value-A", a);
    }
}
