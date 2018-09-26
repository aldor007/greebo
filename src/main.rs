extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate crossbeam_channel;
extern crate threadpool;
#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate base64;
extern crate elastic;
extern crate fasthash;
extern crate config;


use actix_web::{
    http, middleware, server, App,
    HttpRequest, HttpResponse, Responder
};

use std::str;
use std::collections::HashMap;
use crossbeam_channel as channel;
use crossbeam_channel::Sender;
use http::StatusCode;
use threadpool::ThreadPool;
use serde::ser::{Serialize};
use fasthash::murmur3;
use std::thread;
use config::*;


mod storage;
mod types;

use types::{Pageviews, Clicks};

#[derive(Serialize, Deserialize)]
struct OkResponse {
    created: bool
}


#[derive(Serialize, Deserialize)]
struct ErrResponse {
    created: bool,
    error_msg: String
}


fn prepare_response<T>(sc: http::StatusCode, res: T,  query: &HashMap<String,String>) -> HttpResponse
    where
        T: Serialize,
{
    if query.contains_key("jsonp") {
        let jsonp = &query["jsonp"];
        return HttpResponse::build(sc)
            .header("content-type", "application/json")
            .body(format!("{}({})", jsonp, serde_json::to_string(&res).unwrap()));
    }

    return HttpResponse::build(sc).json(res);
}

struct Msg {
    event_type: String,
    data: String,
    user_agent: String,
    ip: String
}

fn handle_keen(req: &HttpRequest<AppState>) -> impl Responder {
    let query = &req.query();
    if !query.contains_key("data") || !query.contains_key("api_key") {
        return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST,ErrResponse{created: false, error_msg:"invalid query params".to_string()}, query);
    }

    let data_buf = match base64::decode(&query["data"]) {
       Ok(d) => d,
        Err(_) => return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST, ErrResponse{created: false, error_msg: "unable to decode b64".to_string()}, query),
    };
    let parts: Vec<&str> = req.path().split("/").collect();
    if parts.len() < 5 {
        return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST,ErrResponse{created: false, error_msg: "invalid path".to_string()}, query);
    }

    let project = parts[3];
    let api_key = (&query["api_key"]).to_string();
    let mut found_key = false;
    let clients = &req.state().config.clients;
    for c in clients {
        if c.project == project && c.key == api_key {
            found_key = true;
        }
    }

    if !found_key {
        return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST,ErrResponse{created: false, error_msg: "invalid key".to_string()}, query);
    }

    let index_name = parts[5].to_string();

    let data = str::from_utf8(&data_buf).unwrap();
    let connection_info = req.connection_info();
    let ip = match connection_info.remote() {
        Some(i) => {
            let ip_parts: Vec<&str> = i.split(":").collect();
            ip_parts[0]
        },
        None => req.headers().get("x-real-ip").unwrap().to_str().unwrap(),
    };

    let ua = match req.headers().get("user-agent") {
        Some(u) => u.to_str().unwrap(),
        None => "unknown"
    };
    let msg = Msg{
        event_type: index_name,
        data: data.to_string(),
        user_agent:  ua.to_string(),
        ip: ip.to_string()
    };

    req.state().sender.send(msg);
    return prepare_response::<OkResponse>(StatusCode::ACCEPTED, OkResponse{created: true}, query);

}

#[derive(Debug, Clone)]
struct AppState {
    sender: Sender<Msg>,
    config: GreeboConfig
}

fn process_message<S>(msg: Msg, storage: S)
    where S :  storage::Storage
{
    if msg.event_type == "pageviews" {
        let mut doc: Pageviews = serde_json::from_str::<Pageviews>(msg.data.as_str()).unwrap();
        doc.ip_address = msg.ip;
        doc.user_agent = msg.user_agent;
        doc.hash = murmur3::hash128(&msg.data).to_string();
        storage.add(msg.event_type, doc);
    } else if msg.event_type  == "clicks" {
        let mut doc: Clicks = serde_json::from_str::< Clicks > (msg.data.as_str()).unwrap();
        doc.ip_address = msg.ip;
        doc.user_agent = msg.user_agent;
        doc.hash = murmur3::hash128(&msg.data).to_string();
        storage.add(msg.event_type, doc);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Clients {
    project: String,
    key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GreeboConfig{
    storage: Storage,
    prefix: String,
    listen: String,
    clients: Vec<Clients>,
    #[serde(default)]
    #[serde(rename = "maxmindPath")]
    maxmind_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Storage {
    url: String,
    #[serde(rename = "type")]
    _type: String,
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("greebo");
    let (s, r) = channel::unbounded::<Msg>();

    let mut settings = Config::default();
    settings
        .merge(File::with_name("config.yml")).unwrap();

    let greebo_config = settings.try_into::<GreeboConfig>().unwrap();
    let greebo_config_cpy = greebo_config.clone();

    let pool = ThreadPool::new(4);
    thread::spawn(move || {
        let storage: storage::elastic_storage::ElasticStorage = storage::elastic_storage::new(greebo_config.storage.url, greebo_config.prefix);
        loop {
            let storage_cpy = storage.clone();
            if let Some(msg) = r.recv() {
                pool.execute(move || {
                    process_message(msg, storage_cpy.clone());
                });
            }
        }
    });

    let listen = greebo_config_cpy.clone().listen;
    let state = AppState{sender: s, config: greebo_config_cpy.clone()};
    server::new( move || {
        App::with_state(state.clone())
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/3.0/projects/{key}/events/{event}",|r| r.method(http::Method::GET).f(handle_keen))
    }).bind(&listen)
        .unwrap()
        .shutdown_timeout(1)
        .start();

    println!("Started http server: {}", listen);
    let _ = sys.run();
}