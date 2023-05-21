-- This file should undo anything in `up.sql`

ALTER TABLE guild_autocomplete RENAME COLUMN uuid TO id;
