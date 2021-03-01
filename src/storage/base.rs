use async_trait::async_trait;
use serde::ser::Serialize;
use std::fmt;

pub struct StorageRes {
    pub code: i32,
}

impl fmt::Display for StorageRes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Response {}", self.code)
    }
}

pub struct StorageErr {
    pub message: String,
}

impl fmt::Display for StorageErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {}", self.message)
    }
}

#[async_trait]
pub trait Storage {
    async fn add<S>(&mut self, event_type: String, data: S) -> Result<StorageRes, StorageErr>
    where
        S: Serialize + Send;
}

pub trait Factory {
    fn new(self) -> Self;
}

#[async_trait]
pub trait Connect {
    async fn connect(self: &'_ mut Self) -> Result<StorageRes, StorageErr>;
}
