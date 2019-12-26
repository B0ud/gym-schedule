use crate::database;
use crate::models::{ListResult, Pool, TrainingsResponse};
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
        Ok((trainings_list_and_total)) => {
            // let mut list: Vec<TrainingsResponse> = Vec::new();
            let total = trainings_list_and_total
                .get(0)
                .map(|&(_, t)| t)
                .unwrap_or(0);

            let list: Vec<TrainingsResponse> = trainings_list_and_total
                .into_iter()
                .map(|(tr, total)| TrainingsResponse::from(tr))
                .collect();

            Ok(HttpResponse::Ok().json(ListResult {
                offset: 0,
                total: total as u32,
                items: list,
            }))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
