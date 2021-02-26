use async_trait::async_trait;
use serde::ser::Serialize;

pub struct StorageRes {
    pub code: i32,
}

pub struct StorageErr {
    pub message: String,
}
#[async_trait]
pub trait Storage {
    async fn add<S>(&self, event_type: String, data: S) -> Result<StorageRes, StorageErr>
    where
        S: Serialize + Send;
}
