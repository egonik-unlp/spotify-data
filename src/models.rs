use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Clone, PartialEq, Identifiable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::songrecords)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SongRecord {
    id: i32,
    ms_played: i32,
    #[serde(alias = "master_metadata_track_name")]
    track_name: Option<String>,
    #[serde(alias = "master_metadata_album_artist_name")]
    artist_name: Option<String>,
    #[serde(alias = "master_metadata_album_album_name")]
    album_name: Option<String>,
    spotify_track_uri: Option<String>,
    reason_start: Option<String>,
    reason_end: Option<String>,
    id_addr_decrypted: Option<String>,
}

#[derive(Insertable, Default, AsChangeset, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::songrecords)]
pub struct NewSongRecord {
    ms_played: i32,
    #[serde(alias = "master_metadata_track_name")]
    track_name: Option<String>,
    #[serde(alias = "master_metadata_album_album_name")]
    album_name: Option<String>,
    #[serde(alias = "master_metadata_album_artist_name")]
    artist_name: Option<String>,
    spotify_track_uri: Option<String>,
    reason_start: Option<String>,
    reason_end: Option<String>,
    id_addr_decrypted: Option<String>,
}
