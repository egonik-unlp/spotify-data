-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "songrecords" (
	"id"	INTEGER NOT NULL UNIQUE,
	"ms_played"	INTEGER NOT NULL,
	"track_name"	TEXT ,
	"artist_name"	TEXT ,
	"album_name"	TEXT ,
	"spotify_track_uri"	TEXT,
	"reason_start"	TEXT,
	"reason_end"	TEXT,
	"id_addr_decrypted"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
);