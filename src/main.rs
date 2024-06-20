use diesel::{sqlite::SqliteConnection, Connection, RunQueryDsl};
use diesel::pg::{Pg, PgConnection};
use models::{NewSongRecord, SongRecord};
use std::env;
mod models;
mod schema;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    return conn;
}
pub fn establish_postgres_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    return conn;
}


fn main() {
    let conn = &mut establish_connection();
    let pconn = &mut establish_postgres_connection();
    let filestring = std::fs::read_to_string("full_streaming_history.json").unwrap();
    let datas: Vec<NewSongRecord> = serde_json::from_str(&filestring).unwrap();
    let loaded_songs = diesel::insert_into(schema::songrecords::table)
        .values(&datas)
        .execute(conn)
        .unwrap();
    println!("Length of load {}", loaded_songs);
}
