extern crate serde;
extern crate serde_json;
extern crate tonic;

use crate::storage::base::{Factory, Storage, StorageErr, StorageRes};
use async_trait::async_trait;
use logproto::pusher_client::PusherClient;
use logproto::{EntryAdapter, PushRequest, StreamAdapter};
use serde::ser::Serialize;

use std::time::SystemTime;

pub mod logproto {
    tonic::include_proto!("logproto");
}

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;

#[derive(Clone)]
pub struct LokiStorage {
    client: Option<PusherClient<tonic::transport::channel::Channel>>,
    url: String,
}

pub async fn connect(url: String) -> Result<LokiStorage, tonic::transport::Error>
where
{
    let url_cp = url.clone();
    let client = PusherClient::connect(String::from(url)).await?;

    Ok(LokiStorage {
        client: Some(client),
        url: url_cp.clone(),
    })
}

#[async_trait]
impl Storage for LokiStorage {
    async fn add<T>(&mut self, event_type: String, doc: T) -> Result<StorageRes, StorageErr>
    where
        T: Serialize + Send,
    {
        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        let request = tonic::Request::new(PushRequest {
            streams: vec![StreamAdapter {
                labels: format!("{{job=\"greebo-events/{}\"}}", event_type).into(),
                entries: vec![EntryAdapter {
                    timestamp: serde::__private::Some(::prost_types::Timestamp {
                        seconds: timestamp.as_secs() as i64,
                        nanos: timestamp.subsec_nanos() as i32,
                    }),
                    line: serde_json::to_string(&doc).unwrap(),
                }],
            }],
        });
        match self.client {
            Some(ref mut c) => match c.push(request).await {
                Ok(_r) => Ok(StorageRes { code: 200 }),
                Err(e) => Err(StorageErr {
                    message: e.message().to_string(),
                }),
            },
            None => {
                return Err(StorageErr {
                    message: "not connected".into(),
                })
            }
        }
    }
}

impl Factory for LokiStorage {
    fn new(self) -> Self {
        let url = self.url.clone();
        let channel = tonic::transport::Channel::from_shared(url)
            .unwrap()
            .connect_lazy()
            .unwrap();
        LokiStorage {
            client: Some(PusherClient::new(channel)),
            url: self.url.clone(),
        }
    }
}
