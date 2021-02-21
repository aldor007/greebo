extern crate serde;
extern crate serde_json;

use crate::storage::base;
use serde::ser::Serialize;
use std::ops::{Deref, DerefMut};

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

unsafe impl base::Storage for ElasticStorage {
    fn add<T>(&self, event_type: String, doc: T)
    where
        T: Serialize,
    {
    }
}
