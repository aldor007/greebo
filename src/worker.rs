extern crate crossbeam_channel;
extern crate serde_json;
extern crate threadpool;

use crate::greebo;
use crate::storage::base::{Storage, StorageErr, StorageRes};
use crate::types::{Clicks, Pageviews};
use crossbeam_channel::{select, Receiver, Sender};
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

#[derive(Clone)]
struct WorkerInner<S>
where
    S: Storage + Send + Clone + Sync + 'static,
{
    sender: Sender<greebo::Msg>,
    receiver: Receiver<greebo::Msg>,
    storage: Arc<Mutex<S>>,
}

#[derive(Clone)]
pub struct Worker<S>
where
    S: Storage + Send + Clone + Sync + 'static,
{
    inner: Arc<WorkerInner<S>>,
    count: usize,
}

impl<S> Worker<S>
where
    S: Storage + Send + Clone + Sync + 'static,
{
    pub fn new(count: usize, storage: S) -> Worker<S> {
        let (s, r) = crossbeam_channel::bounded::<greebo::Msg>(count * 2);
        Worker {
            inner: Arc::new(WorkerInner {
                sender: s,
                receiver: r,
                storage: Arc::new(Mutex::new(storage)),
            }),
            count,
        }
    }

    pub fn get_sender(&self) -> Sender<greebo::Msg> {
        self.inner.sender.clone()
    }

    pub fn run(&mut self) {
        let local_self = self.inner.clone();
        for _ in 0..self.count {
            let local_recv = local_self.receiver.clone();
            let mut self_cp = self.clone();
            tokio::spawn(async move {
                loop {
                    select! {
                        recv(local_recv) -> msg =>  match msg {
                            Ok (m) => match self_cp.process_message(m.clone()).await {
                                Ok(res) =>  info!("Event added {}", res),
                                Err(err) =>  {
                                    warn!("Error adding event {} retrying ", err);
                                    match self_cp.process_message(m.clone()).await {
                                        Ok(res) =>  info!("Event added {}", res),
                                        Err(err) =>  warn!("Event dropped {}", err)
                                    }
                                }
                            },
                            Err(err) => warn!("Recv error {}", err)
                        }
                    }
                }
            });
        }
    }

    pub async fn process_message(&mut self, msg: greebo::Msg) -> Result<StorageRes, StorageErr> {
        info!("Processing event {}", msg.event_type);
        let mut storage = self.inner.storage.lock().await;
        if msg.event_type == "pageviews" {
            let mut doc: Pageviews = serde_json::from_str::<Pageviews>(msg.data.as_str()).unwrap();
            doc.ip_address = msg.ip;
            doc.user_agent = msg.user_agent;
            storage.add(msg.event_type, doc).await
        } else if msg.event_type == "clicks" {
            let mut doc: Clicks = serde_json::from_str::<Clicks>(msg.data.as_str()).unwrap();
            doc.ip_address = msg.ip;
            doc.user_agent = msg.user_agent;
            storage.add(msg.event_type, doc).await
        } else {
            warn!("Unknown event type {}", msg.event_type);
            Err(StorageErr {
                message: "unknown event type".into(),
            })
        }
    }
}
