use std::collections::hash_map::{Entry, HashMap};
use bytes::Bytes;
use std::sync::{RwLock};

#[derive(Clone)]
pub struct Index {
    pub uid: String,
    pub documents: HashMap<Bytes, Bytes>
}
impl Index {
    pub fn create(uid: impl AsRef<str>) -> Self {
        Index {
            uid: uid.as_ref().to_string(),
            documents: HashMap::new()
        }
    }
}

pub struct DB {
    indexes: RwLock<HashMap<String, Index>>,
}
impl DB {
    pub fn new() -> Self {
        DB {
            indexes: RwLock::new( HashMap::new() ),
        }
    }
}

impl DB {
    pub fn create_index(&self, uid: impl AsRef<str>) -> Result<Index, &'static str> {
        let uid = uid.as_ref();
        let mut indexes_lock = self.indexes.write().unwrap();

        match indexes_lock.entry(uid.to_owned()) {
            Entry::Occupied(_) => {
                Err("正在创建的Index已经存在!")
            }
            Entry::Vacant(entry) => {
                let index = Index::create(&uid);
                entry.insert( index.clone() );

                Ok(index)
            }
        }
    }
}