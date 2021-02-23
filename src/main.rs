extern crate actix_web;
extern crate bytes;
extern crate crossbeam_channel;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate clap;
extern crate config;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate actix_cors;
extern crate http;
extern crate tokio;

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpRequest, HttpServer};
use tokio::runtime::Runtime;
use tokio::task::LocalSet;

use config::*;

mod greebo;
mod handlers;
pub mod storage;
mod types;
mod worker;

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
// #[actix_web::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    ::std::env::set_var("RUST_LOG", "greebo=debug,actix_web=trace");
    env_logger::init();
    let sys = actix_web::rt::System::new("greebo");

    let matches = clap::App::new("greebo")
        .version(greebo::VERSION)
        .author("Marcin Kaciuba <marcin.kaciuba@gmail.com>")
        .about("real time monitoring service ")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config.yml");

    let mut settings = Config::default();
    settings.merge(File::with_name(config_path)).unwrap();

    let greebo_config = settings.try_into::<greebo::GreeboConfig>().unwrap();
    let greebo_config_cpy = greebo_config.clone();

    let storage = storage::loki_storage::connect(greebo_config.storage.url)
        .await
        .unwrap();

    let mut worker = worker::Worker::new(4, storage);
    worker.run();

    let listen = greebo_config_cpy.clone().listen;
    let state = greebo::AppState {
        sender: worker.get_sender(),
        config: greebo_config_cpy.clone(),
    };
    let local = LocalSet::new();
    let rt = Runtime::new().unwrap();

    info!("Started http server: {}", listen);
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::ORIGIN,
                        header::USER_AGENT,
                        header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(
                web::resource("/3.0/projects/{project}/events/{event}")
                    .route(web::post().to(handlers::handle_keen_post)),
            )
            .service(
                web::resource("/3.0/projects/{project}/events/{event}")
                    .route(web::get().to(handlers::handle_keen_get)),
            )
    })
    .bind(&listen)?
    .shutdown_timeout(1)
    .workers(4)
    .run()
    .await

}
