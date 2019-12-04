use crate::models::Pool;
use crate::models::Trainings;
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::future::{err, Either};
use futures::{Future, Stream};

//
pub fn get_exercises(pool: web::Data<Pool>) -> Result<Vec<Trainings>, diesel::result::Error> {
    use crate::schema::trainings;

    let conn: &PgConnection = &pool.get().unwrap();
    // diesel::select()
    // let trainings_result = Trainings::
    let result = trainings::table.load::<Trainings>(conn);

    result
}
