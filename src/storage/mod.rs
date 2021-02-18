use serde::ser::{Serialize};
use types::Hashable;

pub trait Storage {
    fn add<S>(&self, event_type: String, data: S) where  S: Serialize + Hashable;
}
pub mod elastic_storage;