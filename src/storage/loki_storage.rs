
extern crate serde_json;
extern crate elastic;
extern crate serde;

use storage::{Storage, Hashable};
use serde::ser::{Serialize};
use tonic::{transport::Server, Request, Response, Status};
use logproto::{Pusher, PushReques, PushResponse}
use hello_world::HelloRequest;

pub mod logproto {
    tonic::include_proto!("logproto");
}

#[derive(Clone)]
pub struct LokiStorage {
    client: Client;
    url: string;
}

pub fn new<I>(url: I, prefix: String) -> LokiStorage
        where
            I: Into<String>,
    {
           let client = reqwest::Client::new();
        ElasticStorage {
            client,
            url
        }
    }

impl Storage for LokiStorage {
  fn add<T>(&self, event_type: String, doc: T)
       where T: Serialize + DocumentType + Hashable
   {
    let res = self.client.post(self.url)
    .body("the exact body that is sent")
    .send()
       };
    }
}
