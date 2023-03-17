-- Your SQL goes here
CREATE TABLE
  "consumers" (
    "id" INTEGER NOT NULL,
    "alias_id" INTEGER NOT NULL UNIQUE,
    "alias_name" TEXT NOT NULL,
    "alias_pfp" TEXT NOT NULL,
    "is_suspended" INTEGER NOT NULL DEFAULT 0,
    "created_at" INTEGER NOT NULL,
    PRIMARY KEY ("id" AUTOINCREMENT)
  );

CREATE TABLE
  "savegames" (
    "consumer_id" INTEGER NOT NULL,
    "points" INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY ("consumer_id")
  );

CREATE TABLE
  "activities" (
    "id" INTEGER NOT NULL,
    "consumer_id" INTEGER NOT NULL,
    "action_id" INTEGER NOT NULL,
    "timestamp" INTEGER NOT NULL,
    FOREIGN KEY ("consumer_id") REFERENCES "consumers" ("id"),
    PRIMARY KEY ("id" AUTOINCREMENT)
  );
