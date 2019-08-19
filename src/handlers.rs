extern crate serde_json;
extern crate base64;
use serde::ser::{Serialize};

use actix_web::{
    http, HttpRequest, HttpResponse, Responder
};
use std::collections::HashMap;
use self::http::StatusCode;
use std::str;

use greebo;

#[derive(Serialize, Deserialize)]
struct OkResponse {
    created: bool
}

impl Default for OkResponse {
    fn default() -> OkResponse {
        OkResponse {
            created: true
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ErrResponse {
    created: bool,
    error_msg: String
}
impl ErrResponse {
    fn msg<S>(message: S) -> ErrResponse
        where S: Into<String> {
        ErrResponse {
            error_msg: message.into(),
            created: false
        }
    }
}

fn prepare_response<T>(sc: http::StatusCode, res: T,  query: &HashMap<String,String>) -> HttpResponse
    where
        T: Serialize,
{
    if query.contains_key("jsonp") {
        let jsonp = &query["jsonp"];
        return HttpResponse::build(sc)
            .header("content-type", "application/javascript")
            .body(format!("{}({})", jsonp, serde_json::to_string(&res).unwrap()));
    }

    return HttpResponse::build(sc).json(res);
}


pub fn handle_keen(req: &HttpRequest<greebo::AppState>) -> impl Responder {
    let query = &req.query();
    if !query.contains_key("data") || !query.contains_key("api_key") {
        return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST, ErrResponse::msg("invalid query params"), query);
    }

    let data_buf = match base64::decode(&query["data"]) {
        Ok(d) => d,
        Err(_) => return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST, ErrResponse::msg( "unable to decode b64"), query),
    };
    let parts: Vec<&str> = req.path().split("/").collect();
    if parts.len() < 5 {
        return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST, ErrResponse::msg("invalid path"), query);
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
        return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST, ErrResponse::msg("invalid key"), query);
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
    let msg = greebo::Msg {
        event_type: index_name,
        data: data.to_string(),
        user_agent: ua.to_string(),
        ip: ip.to_string()
    };

    req.state().sender.send(msg);
    return prepare_response::<OkResponse>(StatusCode::ACCEPTED, OkResponse::default(), query);
}
