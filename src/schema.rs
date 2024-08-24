// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "reason"))]
    pub struct Reason;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Reason;

    songrecords (id) {
        id -> Int4,
        ts -> Timestamptz,
        username -> Nullable<Varchar>,
        platform -> Nullable<Varchar>,
        ms_played -> Int4,
        conn_country -> Nullable<Varchar>,
        user_agent -> Nullable<Varchar>,
        shuffle -> Nullable<Bool>,
        offline -> Nullable<Bool>,
        incognito_mode -> Nullable<Bool>,
        track_name -> Nullable<Text>,
        artist_name -> Nullable<Text>,
        album_name -> Nullable<Text>,
        spotify_track_uri -> Nullable<Text>,
        reason_start -> Nullable<Reason>,
        reason_end -> Nullable<Reason>,
        id_addr_decrypted -> Nullable<Text>,
    }
}
