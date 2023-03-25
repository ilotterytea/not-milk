-- This file should undo anything in `up.sql`
ALTER TABLE non_fungible_milks DROP COLUMN sold;
ALTER TABLE non_fungible_milks DROP COLUMN rarity;
