
extern crate serde_json;
extern crate serde;

use storage::{Storage, Hashable};
use serde::ser::{Serialize};

#[derive(Clone)]
pub struct ElasticStorage {
    prefix: String
}

pub fn new<I>(url: I, prefix: String) -> ElasticStorage
        where
            I: Into<String>,
    {
        ElasticStorage {
            prefix
        }
    }

impl Storage for ElasticStorage {
  fn add<T>(&self, event_type: String, doc: T)
       where T: Serialize + Hashable
   {

    }
}
