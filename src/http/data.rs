use crate::core::database::{DB};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct Data {
    inner: Arc<DataInner>,
}
impl Data {
    pub fn new(db_path: impl AsRef<str>) -> Self {
        Data {
            inner: Arc::new(DataInner::new(&db_path)),
        }
    }
}
impl Deref for Data {
    type Target = DataInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Clone)]
pub struct DataInner {
    pub db: Arc<DB>,
    pub db_path: String,
}
impl DataInner {
    pub fn new(db_path: impl AsRef<str>) -> Self {
        DataInner {
            db: Arc::new( DB::new() ),
            db_path: db_path.as_ref().to_string(),
        }
    }
}