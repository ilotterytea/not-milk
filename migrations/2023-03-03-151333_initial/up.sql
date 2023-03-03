-- Your SQL goes here

CREATE TABLE "users" (
	"id"	INTEGER NOT NULL,
	"alias_id"	TEXT NOT NULL,
	"platform"	INTEGER NOT NULL DEFAULT 0,
	"points"	INTEGER NOT NULL DEFAULT 0,
	"inventory"	TEXT DEFAULT '',
	"created_timestamp"	INTEGER NOT NULL,
	"last_timestamp"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
