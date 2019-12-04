use crate::database;
use crate::models::Pool;
use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::future::Future;
use futures::future::{err, Either};
use serde::Serialize;

#[derive(Serialize)]
struct MyObj {
    name: String,
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
    MyObj {
        name: "user".to_string(),
    }
}

pub fn get_all_trainings_2(
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || database::get_exercises(pool)).then(|res| match res {
        Ok(trainings_list) => Ok(HttpResponse::Ok().json(MyObj {
            name: "works".to_string(),
        })),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

/*pub fn get_all_trainings(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    match database::get_exercises(pool) {
        Ok(traingins_list) => ok(HttpResponse::Ok().json(MyObj {
            name: traingins_list.get(0).unwrap().name,
        })),
        Err(err) => Ok(HttpResponse::BadRequest().json(MyObj {
            name: "Erreur".to_string(),
        })),
    }
    // Ok(HttpResponse::Ok())
}*/
//fn index(data: web::Data<AppState>) {}
