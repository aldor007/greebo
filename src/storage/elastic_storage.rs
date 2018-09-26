
extern crate serde_json;
extern crate elastic;
extern crate serde;

use storage::{Storage, Hashable};
use serde::ser::{Serialize};
use elastic::prelude::{SyncClientBuilder, SyncClient, index, id, DocumentType};

#[derive(Clone)]
pub struct ElasticStorage {
    client: SyncClient,
    prefix: String
}

pub fn new<I>(url: I, prefix: String) -> ElasticStorage
        where
            I: Into<String>,
    {
        let client = SyncClientBuilder::new().base_url(url).build().unwrap();
        ElasticStorage {
            client,
            prefix
        }
    }

impl Storage for ElasticStorage {
  fn add<T>(&self, event_type: String, doc: T)
       where T: Serialize + DocumentType + Hashable
   {
        let index_name = format!("{}-{}", self.prefix, event_type);
       match  self.client.document_index(index(index_name), id(doc.hash()), doc).send() {
           Ok(res) => println!("Added created: {:?}", res.created()),
           Err(e) => println!("Error {:?}", e),
       };
    }
}
