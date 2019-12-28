use crate::models::Pool;
use crate::models::Trainings;
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::future::{err, Either};
use futures::{Future, Stream};

use crate::handler::PaginationQuery;
use crate::pagination::Paginate;

//
pub fn get_exercises(
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
