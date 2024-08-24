use chrono::Local;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods,
};
use spotify_data::{establish_postgres_connection, models::SongRecord, schema::songrecords};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Data {
    song_id: Option<String>,
    artist_name: Option<String>,
    album_name: Option<String>,
    track_name: Option<String>,
}
impl From<SongRecord> for Data {
    fn from(value: SongRecord) -> Self {
        return Data {
            song_id: value.spotify_track_uri,
            artist_name: value.artist_name,
            album_name: value.album_name,
            track_name: value.track_name,
        };
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut counter = HashMap::<String, Vec<Data>>::new();
    let conn = &mut establish_postgres_connection();
    let date = Local::now()
        .checked_sub_months(chrono::Months::new(8))
        .unwrap();

    let songs_period = songrecords::table
        .filter(songrecords::ts.gt(date))
        .select(SongRecord::as_select())
        .get_results::<SongRecord>(conn)?;

    songs_period.into_iter().for_each(|song_record| {
        let key = song_record.clone().album_name.unwrap_or_default();
        let query = counter.contains_key(&key);
        if query {
            let value = counter.get_mut(&key).unwrap();
            value.push(song_record.into());
        } else {
            counter.insert(key, vec![song_record.into()]);
        }
    });
    let mut counter_vec = counter.into_iter().collect::<Vec<(String, Vec<Data>)>>();
    counter_vec.sort_by_key(|(_v, k)| k.len());
    let crvr = &counter_vec[0..counter_vec.len()]
        .into_iter()
        .filter_map(|(alb, dta)| {
            let album = alb.to_owned();
            dta.first().map(|sr| (album, sr.song_id.clone(), dta.len()))
        })
        .filter_map(|(alb, dta, cnt)| dta.map(|dt| (alb, dt, cnt)))
        // .filter_map(|()|)
        .collect::<Vec<(String, String, usize)>>();
    println!("{:#?}", crvr);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("dump_albumes_all.txt")
        .unwrap();
    let mut cntr = 0usize;
    crvr.into_iter().rev().for_each(|(album, id, cnt)| {
        writeln!(&mut file, "{} -> {}", album, id).unwrap();
        cntr += 1
    });
    println!("data for {} albums", cntr);
    Ok(())
}
