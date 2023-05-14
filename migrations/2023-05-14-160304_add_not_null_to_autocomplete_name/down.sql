-- This file should undo anything in `up.sql`

ALTER TABLE autocomplete ALTER COLUMN name DROP NOT NULL;
