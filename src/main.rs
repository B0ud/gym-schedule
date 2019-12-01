use actix_web::get;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use std::sync::Mutex;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;

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

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[get("/hello")]
fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn main() {
    println!("Hello, world!");
    dotenv::dotenv().ok();
    let mut listenfd = ListenFd::from_env();
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    let mut server = HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .register_data(counter.clone())
            .route("/2", web::get().to(_index)) // <- register the created data
            .data(models::manage_database().clone())
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .service(web::scope("/app").route("/index.html", web::get().to(index)))
            .service(index3)
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(web::scope("/app1").route("/", web::to(|| HttpResponse::Ok())))
            .service(web::scope("/app2").route("/", web::to(|| HttpResponse::Ok())))
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
