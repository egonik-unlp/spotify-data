use diesel::pg::{Pg, PgConnection};
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{Connection, ExpressionMethods, RunQueryDsl};
use dotenvy::dotenv;
use diesel::SelectableHelper;
use models::{NewSongRecord, SongRecord};
use schema::songrecords;
use chrono::Local;
// use spotify_data::schema::songrecords;
use std::collections::HashSet;
use std::{env, vec};
mod models;
mod schema;

pub fn establish_postgres_connection() -> PgConnection {
    dotenv().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    return conn;
}

fn load_database(conn: &mut PgConnection) -> () {
    let filestring = std::fs::read_to_string("full_str_redux.json").unwrap();
    let datas: Vec<NewSongRecord> = serde_json::from_str(&filestring).unwrap();
    for (n, row) in datas.clone().into_iter().enumerate() {
        if n % 100_000 == 0 {
            println!("{:?}", row)
        }
    }
    for (n_chunk, data_chunk) in datas.chunks(2_000).enumerate() {
        let loaded_songs = diesel::insert_into(schema::songrecords::table)
            .values(data_chunk)
            .execute(conn)
            .unwrap();
        println!("Length of load {} in chunk {}", loaded_songs, n_chunk);
    }
}
fn load_last() {
    let conn = &mut establish_postgres_connection();
    let songs = songrecords::table.filter(songrecords::artist_name.eq("Air")).select(SongRecord::as_select()).get_results(conn);
    println!("{songs:?}");
}

fn main() {
    let conn = &mut establish_postgres_connection();

    let songs = songrecords::table
        .select((
            songrecords::album_name,
            songrecords::artist_name,
            songrecords::track_name,
        ))
        .get_results::<(Option<String>, Option<String>, Option<String>)>(conn);
    // let mut uniqsongs = HashSet<String>::new();
    let mut hash = HashSet::new();
    match songs {
        Ok(innersongs) if innersongs.is_empty() => load_database(conn),
        Ok(innersongs) => {
            println!("{:?}", innersongs.len());
            let s = innersongs.into_iter().collect::<HashSet<_>>();
            println!("Yupii");
            hash.extend(s);
        }
        Err(_) => panic!("AAAAAAAAAAAAA"),
    }
    println!("{:?}", hash.len());
    let date = Local::now()
    .checked_sub_months(chrono::Months::new(6))
    .unwrap();
    load_last();
}
