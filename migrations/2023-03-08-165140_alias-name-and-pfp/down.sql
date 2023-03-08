-- This file should undo anything in `up.sql`
ALTER TABLE Users
DROP COLUMN alias_name;

ALTER TABLE Users
DROP COLUMN alias_pfp;
