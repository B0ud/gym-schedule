use actix_web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use std::sync::Mutex;

// This struct represents state
struct AppState {
    app_name: String,
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
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
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
