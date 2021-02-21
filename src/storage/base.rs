use serde::ser::Serialize;

pub unsafe trait Storage {
    fn add<S>(&self, event_type: String, data: S)
    where
        S: Serialize;
}
