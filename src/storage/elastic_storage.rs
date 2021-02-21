extern crate serde;
extern crate serde_json;
use crate::storage::base::{Storage, StorageRes, StorageErr};

use crate::storage::base;
use serde::ser::Serialize;
use std::ops::{Deref, DerefMut};
use async_trait::async_trait;


#[derive(Clone)]
pub struct ElasticStorage {
    prefix: String,
}

pub fn new<I>(url: I, prefix: String) -> ElasticStorage
where
    I: Into<String>,
{
    ElasticStorage { prefix }
}

#[async_trait]
impl base::Storage for ElasticStorage {
   async fn add<T>(&self, event_type: String, doc: T) -> Result<base::StorageRes, base::StorageErr>
    where
        T: Serialize + Send,
    {
        Ok(base::StorageRes {
            code: 200
        })
    }
}
