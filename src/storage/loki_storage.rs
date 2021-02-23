extern crate serde;
extern crate serde_json;
extern crate tonic;

use crate::storage::base::{Storage, StorageErr, StorageRes};
use async_trait::async_trait;
use logproto::pusher_client::PusherClient;
use logproto::{EntryAdapter, PushRequest, StreamAdapter};
use serde::ser::Serialize;
use std::time::Duration;
use std::time::SystemTime;
use tonic::transport::Channel;
use tonic::{transport::Server, Request, Response, Status};
use tower::timeout::Timeout;

pub mod logproto {
    tonic::include_proto!("logproto");
}

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;

#[derive(Clone)]
pub struct LokiStorage {
    client: PusherClient<tonic::transport::Channel>,
}

pub async fn connect<D>(dst: D) -> Result<LokiStorage, tonic::transport::Error>
where
    D: std::convert::TryInto<tonic::transport::Endpoint>,
    D::Error: Into<StdError>,
{
    let client = PusherClient::connect(dst).await?;

    Ok(LokiStorage { client })
}

#[async_trait]
impl Storage for LokiStorage {
    async fn add<T>(&self, event_type: String, doc: T) -> Result<StorageRes, StorageErr>
    where
        T: Serialize + Send,
    {
        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        let request = tonic::Request::new(PushRequest {
            streams: vec![StreamAdapter {
                labels: "{job=\"greebo\"}".into(),
                entries: vec![EntryAdapter {
                    timestamp: serde::__private::Some(::prost_types::Timestamp {
                        seconds: timestamp.as_secs() as i64,
                        nanos: timestamp.subsec_nanos() as i32,
                    }),
                    line: serde_json::to_string(&doc).unwrap(),
                }],
            }],
        });
        let mut client = self.client.clone();
        match client.push(request).await {
            Ok(r) => Ok(StorageRes { code: 200 }),
            Err(e) => Err(StorageErr {
                message: e.message().to_string(),
            }),
        }
    }
}
