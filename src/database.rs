use super::{DbExecutor, PooledConn};
use crate::models::Pool;
use crate::models::Trainings;
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::future::{err, Either};
use futures::{Future, Stream};

fn get_exercises(pool: web::Data<Pool>) -> Result<Trainings, diesel::result::Error> {
    use crate::schema::trainings::dsl::*;

    let conn: &PgConnection = &pool.get().unwrap();
    // diesel::select()
    // let trainings_result = Trainings::
    trainings::
}
