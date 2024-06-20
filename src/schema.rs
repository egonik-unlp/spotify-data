// @generated automatically by Diesel CLI.

diesel::table! {
    SongRecord (id) {
        id -> Integer,
        ms_played -> Integer,
        master_metadata_track_name -> Text,
        master_metadata_album_artist_name -> Text,
        master_metadata_album_album_name -> Text,
        spotify_track_uri -> Nullable<Text>,
        reason_start -> Nullable<Text>,
        reason_end -> Nullable<Text>,
        incognito_mode -> Integer,
        id_addr_decrypted -> Integer,
    }
}

diesel::table! {
    songrecords (id) {
        id -> Integer,
        ms_played -> Integer,
        track_name -> Nullable<Text>,
        artist_name -> Nullable<Text>,
        album_name -> Nullable<Text>,
        spotify_track_uri -> Nullable<Text>,
        reason_start -> Nullable<Text>,
        reason_end -> Nullable<Text>,
        id_addr_decrypted -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    SongRecord,
    songrecords,
);
