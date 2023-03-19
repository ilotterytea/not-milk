-- This file should undo anything in `up.sql`
CREATE TABLE "lines" (
	"id"	INTEGER NOT NULL,
	"line"	TEXT NOT NULL,
	"category_id" INTEGER NOT NULL DEFAULT 1,
	"channel_id"	INTEGER,
	"is_disabled"	INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("channel_id") REFERENCES "channels"("id")
);
