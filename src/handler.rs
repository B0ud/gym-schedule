use crate::database;
use crate::models::{ListResult, Pool, TrainingsResponse};
use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::future::Future;
use futures::future::{err, Either};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::fmt::{self, Display, Formatter};

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct PaginationQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
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
    query: web::Query<PaginationQuery>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let pagination = query.into_inner();

    // Thread Blocking
    web::block(move || database::get_exercises(pool, pagination)).then(move |res| {
        match res {
            Ok((trainings_list, total)) => {
                // let mut list: Vec<TrainingsResponse> = Vec::new();

                let list: Vec<TrainingsResponse> = trainings_list
                    .into_iter()
                    .map(|tr| TrainingsResponse::from(tr))
                    .collect();

                Ok(HttpResponse::Ok().json(ListResult {
                    offset: pagination.offset.unwrap_or(0),
                    total: total as u32,
                    items: list,
                }))
            }
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }
    })
}
