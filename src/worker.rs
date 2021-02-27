extern crate crossbeam_channel;
extern crate serde_json;
extern crate threadpool;

use crate::greebo;
use crate::storage::base::Storage;
use crate::types::{Clicks, Pageviews};
use crossbeam_channel::{Receiver, Sender};

use std::sync::Arc;
use tokio;

#[derive(Clone)]
struct WorkerInner<S>
where
    S: Storage + Send + Clone + Sync + 'static,
{
    sender: Sender<greebo::Msg>,
    receiver: Receiver<greebo::Msg>,
    storage: Arc<S>,
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
        let (s, r) = crossbeam_channel::unbounded::<greebo::Msg>();
        Worker {
            inner: Arc::new(WorkerInner {
                sender: s,
                receiver: r,
                storage: Arc::new(storage),
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
            let local_th = local_self.clone();
            let mut self_cp = self.clone();
            tokio::spawn(async move {
                loop {
                    if let Ok(msg) = local_th.receiver.recv() {
                        self_cp.process_message(msg).await;
                    }
                }
            });
        }
    }

    pub async fn process_message(&mut self, msg: greebo::Msg) {
        info!("Processing event {}", msg.event_type);
        let storage = self.inner.storage.clone();
        if msg.event_type == "pageviews" {
            let mut doc: Pageviews = serde_json::from_str::<Pageviews>(msg.data.as_str()).unwrap();
            doc.ip_address = msg.ip;
            doc.user_agent = msg.user_agent;
            let result = storage.add(msg.event_type, doc).await;
            match result {
                Ok(s) => info!("Event added {}", s.code),
                Err(e) => warn!("Error {}", e.message),
            }
        } else if msg.event_type == "clicks" {
            let mut doc: Clicks = serde_json::from_str::<Clicks>(msg.data.as_str()).unwrap();
            doc.ip_address = msg.ip;
            doc.user_agent = msg.user_agent;
            let result = storage.add(msg.event_type, doc).await;
            match result {
                Ok(s) => info!("Event added {}", s.code),
                Err(e) => warn!("Error {}", e.message),
            }
        } else {
            warn!("Unknown event type {}", msg.event_type)
        }
    }
}
