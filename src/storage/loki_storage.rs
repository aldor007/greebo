extern crate serde;
extern crate serde_json;
extern crate tonic;

use crate::storage::base::Storage;
use serde::ser::Serialize;
use tonic::{transport::Server, Request, Response, Status};
use logproto::{PushRequest, StreamAdapter, EntryAdapter};
use logproto::pusher_client::{PusherClient};
use tonic::transport::Channel;
use tower::timeout::Timeout;
use std::time::Duration;

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

unsafe impl Storage for LokiStorage {
    fn add<T>(&self, event_type: String, doc: T)
    where
        T: Serialize,
    {
        let request = tonic::Request::new(PushRequest{
            streams: vec![StreamAdapter {
                labels: "test".into(),
                entries: vec! [EntryAdapter {
                    timestamp: serde::__private::Some(::prost_types::Timestamp { seconds: 20, nanos: 999 }),
                    line: serde_json::to_string(&doc).unwrap()
                }]
            }

            ]
        });
        let mut client = self.client.clone();
        client.push(request);
    }
}
