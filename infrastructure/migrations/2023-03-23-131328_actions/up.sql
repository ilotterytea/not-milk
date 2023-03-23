-- Your SQL goes here
CREATE TABLE "actions" (
	"id"	INTEGER NOT NULL,
	"consumer_id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"body"	TEXT,
	"raw"	TEXT NOT NULL,
	"created_at"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("consumer_id") REFERENCES "consumers"("id")
);
