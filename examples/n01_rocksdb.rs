extern crate rocksdb;
use rocksdb::{DB, Writable, WriteBatch};

fn main() {
    let mut db = DB::open_default("./k.db").unwrap();
    db.put(b"key1", b"value1");

    match db.get(b"key1") {
        Ok( Some( v ) ) => println!("retrieved value {}", v.to_utf8().unwrap()),
        Ok( None )      => println!("value not found."),
        Err( e )        => println!("operational problem encountered: {}", e),
    }

    //write batch
    {
        let mut batch = WriteBatch::new();
        batch.put(b"A", b"V1");
        batch.put(b"B", b"V2");

        db.write( &batch ); //atomically commit
    }

    db.delete(b"key1");


}
