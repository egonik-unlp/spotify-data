pub mod models;
pub mod schema;
use diesel::pg::{Pg, PgConnection};
use diesel::{Connection, RunQueryDsl};
use dotenvy::dotenv;
use std::env;

pub fn establish_postgres_connection() -> PgConnection {
    dotenv().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to database {}", database_url));
    return conn;
}
