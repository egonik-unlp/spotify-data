use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[serde(rename_all_fields = "kebab-case")]
#[derive(diesel_derive_enum::DbEnum, Hash, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::Reason"]
pub enum Reason {
    #[serde(alias = "app_load")]
    Appload,
    #[serde(alias = "backbtn")]
    Backbtn,
    #[serde(alias = "click-row")]
    Clickrow,
    #[serde(alias = "clickside")]
    Clickside,
    #[serde(alias = "endplay")]
    Endplay,
    #[serde(alias = "fwdbtn")]
    Fwdbtn,
    #[serde(alias = "logout")]
    Logout,
    #[serde(alias = "persisted")]
    Persisted,
    #[serde(alias = "playbtn")]
    Playbtn,
    #[serde(alias = "popup")]
    Popup,
    #[serde(alias = "remote")]
    Remote,
    #[serde(alias = "trackdone")]
    Trackdone,
    #[serde(alias = "trackerror")]
    Trackerror,
    #[serde(alias = "unexpected-exit")]
    UnexpectedExit,
    #[serde(alias = "unexpected-exit-while-paused")]
    UnexpectedExitWhilePaused,
    #[serde(alias = "unknown")]
    Unknown,
    #[serde(other)]
    Void,
}

#[derive(Queryable, Selectable, Debug, Clone, PartialEq, Identifiable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::songrecords)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SongRecord {
    id: i32,
    #[serde(with = "my_date_format")]
    ts: DateTime<Utc>,
    username: Option<String>,
    platform: Option<String>,
    ms_played: i32,
    conn_country: Option<String>,
    id_addr_decrypted: Option<String>,
    #[serde(alias = "user_agent_decrypted")]
    user_agent: Option<String>,
    shuffle: Option<bool>,
    offline: Option<bool>,
    incognito_mode: Option<bool>,
    #[serde(alias = "master_metadata_track_name")]
    pub track_name: Option<String>,
    #[serde(alias = "master_metadata_album_artist_name")]
    pub artist_name: Option<String>,
    #[serde(alias = "master_metadata_album_album_name")]
    pub album_name: Option<String>,
    pub spotify_track_uri: Option<String>,
    reason_start: Option<Reason>,
    reason_end: Option<Reason>,
}

#[skip_serializing_none]
#[derive(Insertable, Default, AsChangeset, Deserialize, Clone, Debug)]
#[serde(default)]
#[diesel(table_name = crate::schema::songrecords)]
pub struct NewSongRecord {
    #[serde(with = "my_date_format")]
    ts: DateTime<Utc>,
    username: Option<String>,
    platform: Option<String>,
    ms_played: i32,
    conn_country: Option<String>,
    #[serde(alias = "user_agent_decrypted")]
    user_agent: Option<String>,
    shuffle: Option<bool>,
    offline: Option<bool>,
    incognito_mode: Option<bool>,
    #[serde(alias = "master_metadata_track_name")]
    track_name: Option<String>,
    #[serde(alias = "master_metadata_album_artist_name")]
    artist_name: Option<String>,
    #[serde(alias = "master_metadata_album_album_name")]
    album_name: Option<String>,
    spotify_track_uri: Option<String>,
    reason_start: Option<Reason>,
    reason_end: Option<Reason>,
    id_addr_decrypted: Option<String>,
}

mod my_date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%SZ";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}
