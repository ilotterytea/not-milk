-- Your SQL goes here
ALTER TABLE non_fungible_milks ADD COLUMN sold INTEGER NOT NULL DEFAULT 0;
ALTER TABLE non_fungible_milks ADD COLUMN rarity INTEGER NOT NULL DEFAULT 1;
