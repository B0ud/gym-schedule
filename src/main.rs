#![allow(warnings)]
use actix_web::get;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use futures::future::Future;
use listenfd::ListenFd;
use std::sync::Mutex;
extern crate r2d2;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod database;
mod handler;
mod models;
mod schema;

// This struct represents state
struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

fn _index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {}!", app_name) // <- response with app_name
}

fn main() {
    println!("Hello, world!");

    //INIT LOGGER
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    let mut server = HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .wrap(Logger::default())
            .register_data(counter.clone())
            .route("/2", web::get().to(_index)) // <- register the created data
            .data(models::manage_database().clone())
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .route("/sync", web::get().to(handler::object_index))
            .service(web::resource("/db").route(web::get().to_async(handler::get_all_trainings_2)))
            // .route("/3", web::get().to(handler::index))
            .route("/", web::get().to(index))
    });

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
