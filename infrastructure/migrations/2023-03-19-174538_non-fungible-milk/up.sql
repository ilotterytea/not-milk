-- Your SQL goes here
CREATE TABLE "non_fungible_milks" (
	"id"	INTEGER NOT NULL UNIQUE,
	"consumer_id"	INTEGER NOT NULL,
	"hash_sum"	TEXT NOT NULL UNIQUE,
	"created_at"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("consumer_id") REFERENCES "consumers"("id")
);
