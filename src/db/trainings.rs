use crate::config::db_config::Pool;
use crate::models::{NewTrainings, PaginationQuery, Trainings};
use crate::pagination::Paginate;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::future::{err, Either};
use futures::{Future, Stream};
use std::process::id;
use uuid::Uuid;

/// Get All trainings
/// Optional pagination  passed to diesel dsl orm
pub fn get_trainings(
    pool: web::Data<Pool>,
    pagination: PaginationQuery,
) -> Result<(Vec<Trainings>, i64), diesel::result::Error> {
    use crate::schema::trainings;
    let conn: &PgConnection = &pool.get().unwrap();
    let mut query = trainings::table.into_boxed().paginate(
        pagination.limit.map(|l| l as i64),
        pagination.offset.map(|o| o as i64),
    );

    query.load_and_count_total::<(Trainings)>(conn)
}

pub fn get_training_by_id(
    pool: web::Data<Pool>,
    id_param: Uuid,
) -> Result<Trainings, diesel::result::Error> {
    use crate::schema::trainings;
    let conn: &PgConnection = &pool.get().unwrap();
    let training_result = trainings::table.find(id_param).first(conn);
    training_result
}

pub fn create_new_training(
    pool: web::Data<Pool>,
    new_training: NewTrainings,
) -> Result<Trainings, diesel::result::Error> {
    use crate::schema::trainings;
    let conn: &PgConnection = &pool.get().unwrap();

    let training_result = insert_into(trainings::table)
        .values(&new_training)
        .get_result::<Trainings>(conn);

    training_result
}
