BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "SongRecord" (
	"id"	INTEGER NOT NULL UNIQUE,
	"ms_played"	INTEGER NOT NULL,
	"master_metadata_track_name"	TEXT NOT NULL,
	"master_metadata_album_artist_name"	TEXT NOT NULL,
	"master_metadata_album_album_name"	TEXT NOT NULL,
	"spotify_track_uri"	TEXT,
	"reason_start"	TEXT,
	"reason_end"	TEXT,
	"incognito_mode"	INTEGER NOT NULL,
	"id_addr_decrypted"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
COMMIT;
