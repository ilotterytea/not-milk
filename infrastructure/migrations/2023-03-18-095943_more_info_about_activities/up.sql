-- Your SQL goes here
ALTER TABLE activities RENAME TO points_history;

ALTER TABLE points_history ADD COLUMN caused_by_consumer_id INTEGER REFERENCES consumers(id);
ALTER TABLE points_history ADD COLUMN difference INTEGER NOT NULL;
ALTER TABLE points_history ADD COLUMN points_before_difference INTEGER NOT NULL;
ALTER TABLE points_history DROP COLUMN action_id;

