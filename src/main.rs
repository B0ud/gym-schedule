#![allow(warnings)]
// Extern Crate
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate r2d2;

use actix_web::get;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use api::trainings;
use env_logger::Builder;
use futures::future::Future;
use listenfd::ListenFd;
use log::Level;
use std::env;
use std::sync::Mutex;

mod api;
mod config;
mod db;
mod models;
mod pagination;
mod schema;

// This struct represents state
struct AppState {
    app_name: String,
}

fn main() {
    //INIT LOGGER
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    //env_logger::init();

    Builder::new()
        .parse_filters(&std::env::var("MY_APP_LOG").unwrap_or("info".parse().unwrap()))
        .init();

    //
    info!("Application Start ");
    // Hot Reload
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .wrap(Logger::default())
            .data(config::db_config::manage_database().clone())
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .service(web::resource("/db").route(web::get().to_async(trainings::get_all_trainings)))
    });

    // Hot Reload
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };
    server.run().unwrap();

    /* .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();*/
}
