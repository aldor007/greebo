extern crate serde;
extern crate serde_json;
extern crate url;
use crate::storage::base::{Storage, StorageErr, StorageRes};

use crate::storage::base;
use async_trait::async_trait;
use serde::ser::Serialize;
use elasticsearch::{
    Elasticsearch, Error,
    http::transport::{TransportBuilder,SingleNodeConnectionPool},
};
use url::Url;


#[derive(Clone)]
pub struct ElasticStorage {
    prefix: String,
}

pub fn new<I>(url: I, prefix: String) -> ElasticStorage
where
    I: Into<String>,
{
    let url = Url::parse(url)?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool).disable_proxy().build()?;
    let client = Elasticsearch::new(transport);
    ElasticStorage { prefix }
}

#[async_trait]
impl base::Storage for ElasticStorage {
    async fn add<T>(&self, event_type: String, doc: T) -> Result<base::StorageRes, base::StorageErr>
    where
        T: Serialize + Send,
    {
        Ok(base::StorageRes { code: 200 })
    }
}
