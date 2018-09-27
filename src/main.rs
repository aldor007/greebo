extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate crossbeam_channel;
#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate base64;
extern crate elastic;
extern crate config;
extern crate clap;
#[macro_use]
extern crate log;

use actix_web::{
    http, middleware, server, App
};
use actix_web::middleware::cors::Cors;

use http::header;
use config::*;

mod storage;
mod types;
mod handlers;
mod greebo;
mod worker;

fn main() {
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
    let state = greebo::AppState{sender: worker.get_sender(), config: greebo_config_cpy.clone()};
    server::new( move || {
        App::with_state(state.clone()).configure(
            |app| {
                Cors::for_app(app)   // <- Construct CORS builder
                    .allowed_origin("*")
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::ORIGIN, header::USER_AGENT, header::CONTENT_TYPE])
                    .max_age(3600)
                    .resource("/3.0/projects/{key}/events/{event}", |r| r.method(http::Method::GET).f(handlers::handle_keen))
                    .register()
            })
            .middleware(middleware::Logger::default())
    }).bind(&listen)
        .unwrap()
        .shutdown_timeout(1)
        .start();

    info!("Started http server: {}", listen);
    let _ = sys.run();
}