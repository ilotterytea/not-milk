-- This file should undo anything in `up.sql`
ALTER TABLE points_history RENAME TO activities;

ALTER TABLE activities DROP COLUMN caused_by_consumer_id;
ALTER TABLE activities DROP COLUMN difference;
ALTER TABLE activities DROP COLUMN points_before_difference;

ALTER TABLE activities ADD COLUMN action_id INTEGER NOT NULL DEFAULT 0;
