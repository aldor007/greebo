use serde::ser::{Serialize};
use elastic::prelude::DocumentType;
use types::Hashable;

pub trait Storage {
    fn add<S>(&self, event_type: String, data: S) where  S: Serialize + DocumentType + Hashable;
}
pub mod elastic_storage;