use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::get;
use listenfd::ListenFd;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
    let mut server =HttpServer::new(|| {
        App::new()
            .service(index3)
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
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
