use crate::config::db_config::Pool;
use crate::db;
use crate::models::{ListResult, PaginationQuery, TrainingsResponse};
use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::future::Future;
use futures::future::{err, Either};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::fmt::{self, Display, Formatter};

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
