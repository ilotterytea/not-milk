-- Your SQL goes here
CREATE TABLE "suspensions" (
	"consumer_id"	INTEGER NOT NULL UNIQUE,
	"reason"	TEXT,
	"duration"	INTEGER NOT NULL DEFAULT -1,
	"timestamp"	INTEGER NOT NULL,
	PRIMARY KEY("consumer_id"),
	FOREIGN KEY("consumer_id") REFERENCES "channels"("id")
);

ALTER TABLE consumers DROP COLUMN is_suspended;
