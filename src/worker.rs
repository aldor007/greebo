extern crate serde_json;
extern crate crossbeam_channel;
extern crate threadpool;
extern crate fasthash;

use crossbeam_channel::{Sender,Receiver};
use std::thread;
use std::sync::Arc;


use greebo;
use storage;

use types::{Pageviews, Clicks};

#[derive(Clone)]
struct WorkerInner<S>  where S: storage::Storage + Send + Clone +  Sync  + 'static {
    sender: Sender<greebo::Msg>,
    receiver: Receiver<greebo::Msg>,
    storage: Arc<S>
}

#[derive(Clone)]
pub struct Worker<S> where S: storage::Storage + Send + Clone+ Sync + 'static  {
    inner: Arc<WorkerInner<S>>,
    count: usize
}

impl <S> Worker<S> where S: storage::Storage + Send + Clone + Sync  + 'static {
    pub fn new(count: usize, storage: S) -> Worker<S> {
        let (s, r) = crossbeam_channel::unbounded::<greebo::Msg>();
        Worker {
            inner: Arc::new(WorkerInner {
                sender: s,
                receiver: r,
                storage: Arc::new(storage)
            }),
            count
        }

    }

    pub fn get_sender(&self) -> Sender<greebo::Msg> {
        self.inner.sender.clone()
    }

    pub  fn run(& mut self) {
        let local_self = self.inner.clone();
        for _ in 0..self.count {
            let local_th = local_self.clone();
            let mut self_cp = self.clone();
            thread::spawn( move || {
                loop {
                    if let Some(msg) = local_th.receiver.recv() {
                        self_cp.process_message(msg);
                    }
                }
            });

        }
    }

    pub fn process_message(& mut self, msg: greebo::Msg)
    {
        info!("Processing event {}", msg.event_type);
        if msg.event_type == "pageviews" {
            let mut doc: Pageviews = serde_json::from_str::<Pageviews>(msg.data.as_str()).unwrap();
            doc.ip_address = msg.ip;
            doc.user_agent = msg.user_agent;
            doc.hash = fasthash::murmur3::hash128(&msg.data).to_string();
            self.inner.storage.add(msg.event_type, doc);
        } else if msg.event_type  == "clicks" {
            let mut doc: Clicks = serde_json::from_str::< Clicks > (msg.data.as_str()).unwrap();
            doc.ip_address = msg.ip;
            doc.user_agent = msg.user_agent;
            doc.hash = fasthash::murmur3::hash128(&msg.data).to_string();
            self.inner.storage.add(msg.event_type, doc);
        } else {
            warn!("Unknown event type {}", msg.event_type)
        }
    }
}
