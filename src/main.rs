extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate crossbeam_channel;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate base64;
extern crate config;
extern crate clap;
#[macro_use]
extern crate log;

use actix_web::{http, web, App, HttpServer, Responder};
use actix_cors::Cors;

use http::header;
use config::*;

mod storage;
mod types;
mod handlers;
mod greebo;
mod worker;

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "greebo=info,actix_web=info" );
    env_logger::init();
    let sys = actix::System::new("greebo");

    let matches = clap::App::new("greebo")
        .version(greebo::VERSION)
        .author("Marcin Kaciuba <marcin.kaciuba@gmail.com>")
        .about("real time monitoring service ")
        .arg(clap::Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config.yml");

    let mut settings = Config::default();
    settings
        .merge(File::with_name(config_path)).unwrap();

    let greebo_config = settings.try_into::<greebo::GreeboConfig>().unwrap();
    let greebo_config_cpy = greebo_config.clone();

    let storage = storage::elastic_storage::new(greebo_config.storage.url, greebo_config.prefix);

    let mut worker = worker::Worker::new(4, storage);
    worker.run();

    let listen = greebo_config_cpy.clone().listen;
    info!("Started http server: {}", listen);
    let state = greebo::AppState{sender: worker.get_sender(), config: greebo_config_cpy.clone()};
    HttpServer::new( move || {
        App::new()
            .data(state.clone())
            .service(web::resource("/3.0/projects/{key}/events/{event}").route(web::get().to(handlers::handle_keen)))
            .wrap(
                Cors::default()
                    .allowed_origin("*")
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::ORIGIN, header::USER_AGENT, header::CONTENT_TYPE])
                    .max_age(3600)
            )

    }).bind(&listen)
        .unwrap()
        .shutdown_timeout(1)
        .run()
        .await
}