use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ok, Future};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// Responder
impl Responder for MyObj {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

pub fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn object_index() -> impl Responder {
    MyObj { name: "user" }
}
//fn index(data: web::Data<AppState>) {}
