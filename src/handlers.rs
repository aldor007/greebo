extern crate serde_json;
extern crate base64;
extern crate base64_url;
use serde::ser::{Serialize};
use log::{info, warn};

use actix_web::{dev, error, http, web, Error, HttpResponse, Result, HttpRequest};


use std::collections::HashMap;
use self::http::StatusCode;
use std::str;

use crate::greebo;

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

#[derive(Deserialize)]
pub struct KeenParams {
    project: String,
    event: String
}

#[derive(Deserialize)]
pub struct QueryKeen {
    jsonp: String,
    api_key: String,
    data: String
}

fn prepare_response<T>(sc: http::StatusCode, res: T,  query: web::Query<QueryKeen>) -> HttpResponse
    where
        T: Serialize
{
    if sc.as_u16() > 399 {
        warn!("client bad request {}",  serde_json::to_string(&res).unwrap())
    }

    if query.jsonp != "" {
        return HttpResponse::build(sc)
            .header("content-type", "application/javascript")
            .body(format!("{}({})", query.jsonp, serde_json::to_string(&res).unwrap()));
    }

    return HttpResponse::build(sc).json(&res);
}



pub async fn handle_keen_get(state: web::Data<greebo::AppState>,  params: web::Path<KeenParams>,  query:  web::Query<QueryKeen>, req: HttpRequest) -> HttpResponse {
    if params.project.is_empty() || params.event.is_empty() {
        return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST, ErrResponse::msg("invalid query params"), query);
    }

    let data_buf = match base64_url::decode(&query.data) {
        Ok(d) => d,
        Err(err) => match err {
            base64::DecodeError::InvalidByte(size, offset) =>
                return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST,
                                                       ErrResponse::msg( format!("{} size {} offet {}", "unable to decode b64, invalid byte", size, offset)), query),
            base64::DecodeError::InvalidLength => return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST,
                                                       ErrResponse::msg(  "unable to decode b64, invalid length"), query),
            base64::DecodeError::InvalidLastSymbol(size, offset) =>
                return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST,
                                                       ErrResponse::msg( format!("{} size {} offet {}", "unable to decode b64, invalid last symbol", size, offset)), query),
        }
    };
    let parts: Vec<&str> = req.path().split("/").collect();
    if parts.len() < 5 {
        return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST, ErrResponse::msg("invalid path"), query);
    }

    let mut found_key = false;
    for c in &state.config.clients {
        if c.project == params.project && c.key == query.api_key {
            found_key = true;
        }
    }

    if !found_key {
        return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST, ErrResponse::msg("invalid key"), query);
    }

    let index_name = parts[5].to_string();

    let data = str::from_utf8(&data_buf).unwrap();
    let connection_info = req.connection_info();
    let ip = match connection_info.realip_remote_addr() {
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

    state.sender.send(msg);
    return prepare_response::<OkResponse>(StatusCode::ACCEPTED, OkResponse::default(), query);
}

pub async fn handle_keen_post(state: web::Data<greebo::AppState>,  body: web::Bytes, params: web::Path<KeenParams>,  query:  web::Query<QueryKeen>, req: HttpRequest) -> HttpResponse {
    warn!("Post");
    if params.project.is_empty() || params.event.is_empty() {
        return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST, ErrResponse::msg("invalid query params"), query);
    }

    let mut found_key = false;
    for c in &state.config.clients {
        if c.project == params.project && c.key == query.api_key {
            found_key = true;
        }
    }

    if !found_key {
        return prepare_response::<ErrResponse>(StatusCode::BAD_REQUEST, ErrResponse::msg("invalid key"), query);
    }


    let data = str::from_utf8(&body).unwrap();
    let connection_info = req.connection_info();
    let ip = match connection_info.realip_remote_addr() {
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
        event_type: params.event.to_string(),
        data: data.to_string(),
        user_agent: ua.to_string(),
        ip: ip.to_string()
    };

    state.sender.send(msg);
    return prepare_response::<OkResponse>(StatusCode::ACCEPTED, OkResponse::default(), query);
}