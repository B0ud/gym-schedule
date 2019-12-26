use crate::models::Pool;
use crate::models::Trainings;
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::future::{err, Either};
use futures::{Future, Stream};

use crate::pagination::Paginate;

//
pub fn get_exercises(
    pool: web::Data<Pool>,
) -> Result<Vec<(Trainings, i64)>, diesel::result::Error> {
    use crate::schema::trainings;
    let conn: &PgConnection = &pool.get().unwrap();
    let mut query = trainings::table.into_boxed().paginate(1, 1);
    query.load::<(Trainings, i64)>(conn)
}
