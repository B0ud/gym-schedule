#![allow(warnings)]
// Extern Crate
extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
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

embed_migrations!();
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    /// Migration Database
    let connection = config::db_config::establish_connection();
    ///
    ///     // This will run the necessary migrations.*/
    /// embedded_migrations::run(&connection);
    ///
    ///     // By default the output is thrown out. If you want to redirect it to stdout, you
    ///     // should call embedded_migrations::run_with_output.
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    //INIT LOGGER
    std::env::set_var("RUST_LOG", "actix_web=debug");

    //env_logger::init();

    Builder::new()
        .parse_filters(&std::env::var("MY_APP_LOG").unwrap_or("debug".parse().unwrap()))
        .init();

    //
    info!("Application Start ");
    // Hot Reload
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .wrap(Logger::default())
            .data(config::db_config::establish_connection_pool().clone())
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .service(
                web::resource("/trainings")
                    .route(web::get().to(trainings::get_all_trainings))
                    .route(web::post().to(trainings::create_new_training)),
            )
            .service(web::resource("/trainings/{id}").route(web::get().to(trainings::get_training)))
    });

    // Hot Reload
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("0.0.0.0:3000").unwrap()
    };
    server.run().await

    /* .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();*/
}
