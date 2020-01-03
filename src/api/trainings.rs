use crate::config::db_config::Pool;
use crate::db;
use crate::models::{ListResult, NewTrainings, PaginationQuery, TrainingsResponse};
use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::future::Future;
use futures::future::{err, Either};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::fmt::{self, Display, Formatter};
use uuid::Uuid;

pub async fn get_all_trainings(
    query: web::Query<PaginationQuery>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let pagination = query.into_inner();

    // Thread Blocking
    web::block(move || db::get_trainings(pool, pagination))
        .await
        .map(|(trainings_list, total)| {
            // let mut list: Vec<TrainingsResponse> = Vec::new();
            let list: Vec<TrainingsResponse> = trainings_list
                .into_iter()
                .map(|tr| TrainingsResponse::from(tr))
                .collect();

            HttpResponse::Ok().json(ListResult {
                offset: pagination.offset.unwrap_or(0),
                total: total as u32,
                items: list,
            })
        })
        .map_err(|_| HttpResponse::InternalServerError().into())
}

pub async fn get_training(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    match req.match_info().get("id").unwrap().parse() {
        Ok(id) => {
            web::block(move || db::get_training_by_id(pool, id))
                .await
                .map(|(training)| {
                    // let mut list: Vec<TrainingsResponse> = Vec::new();
                    let response: TrainingsResponse = TrainingsResponse::from(training);
                    HttpResponse::Ok().json(response)
                })
                .map_err(|_| HttpResponse::InternalServerError().into())
        }
        Err(e) => Err(HttpResponse::BadRequest().into()),
    }
}

pub async fn create_new_training(
    req: HttpRequest,
    pool: web::Data<Pool>,
    new_training: web::Json<NewTrainings>,
) -> Result<HttpResponse, Error> {
    web::block(move || db::create_new_training(pool, new_training.into_inner()))
        .await
        .map(|(training)| {
            // let mut list: Vec<TrainingsResponse> = Vec::new();
            let response: TrainingsResponse = TrainingsResponse::from(training);
            HttpResponse::Created().json(response)
        })
        .map_err(|_| HttpResponse::InternalServerError().into())
}
