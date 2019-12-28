use crate::config::db_config::Pool;
use crate::models::{PaginationQuery, Trainings};
use crate::pagination::Paginate;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::future::{err, Either};
use futures::{Future, Stream};

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