use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods,
};
use itertools::Itertools;
use spotify_data::{establish_postgres_connection, models::SongRecord, schema::songrecords};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
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

fn main() -> Result<(), Box<dyn std::error::Error> > {
	let conn = &mut establish_postgres_connection();
	let ends : Vec<DateTime<Utc>> = songrecords::table.select(songrecords::ts).get_results(conn)?;
	let mut sorted_ends = ends.into_iter().sorted_by_key(|date| {
		date.timestamp()
	}).collect::<Vec<DateTime<Utc>>>();
	let fy = sorted_ends.clone().get(0).unwrap().year();
	let datestring_beg = format!("{} 01 01 00 00", fy);
	let mut date = NaiveDateTime::parse_from_str(&datestring_beg, "%Y %m %d %H %M").unwrap().and_utc();
	let grain = chrono::Months::new(2);
	let mut file = OpenOptions::new().write(true).create(true).append(true).open("dumpissimo.txt")?;
	let foldername = "dump_animacion";
	let mut folder = Path::new(foldername);
	if !folder.exists() {
		std::fs::create_dir(foldername)?
	}

	let mut frame = 1usize;
	while date.lt(&Utc::now().checked_sub_months(grain).unwrap()) {
	    let filename = format!("frame_{}", frame);
	    let fp = folder.join(Path::new(&filename));
            let mut file = OpenOptions::new().write(true).create(true).open(fp).unwrap();
	    let mut counter: HashMap<String, Vec<Data>> = HashMap::<String, Vec<Data>>::new();
	    let new_date = date.checked_add_months(grain).unwrap();
	    let music_data = songrecords::table.filter(songrecords::ts.gt(date)).filter(songrecords::ts.lt(new_date)).select(SongRecord::as_select()).get_results(conn).unwrap();
	    music_data.into_iter().for_each(|song_record| {
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
		    let mut cntr = 0usize;
		    crvr.into_iter().rev().for_each(|(album, id, cnt)| {
			writeln!(&mut file, "{} -> {}", album, id).unwrap();
			cntr += 1
		    });
		    println!("data for {} albums", cntr);
	    
	    
	  
	    date = new_date;
	//     println!("{:?}", date)
	frame +=1;
	}

	Ok(())
}