-- This file should undo anything in `up.sql`
DROP TABLE suspensions;

ALTER TABLE consumers ADD COLUMN is_suspended INTEGER NOT NULL DEFAULT 0;
