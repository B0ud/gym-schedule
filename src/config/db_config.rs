use diesel::{r2d2::ConnectionManager, PgConnection};
use std::env;

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn manage_database() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 00 must be set");
    info!("Db connection pool : {}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
