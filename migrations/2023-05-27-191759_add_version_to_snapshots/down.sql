-- This file should undo anything in `up.sql`

ALTER TABLE snapshot DROP COLUMN version;
ALTER TABLE guild_snapshot DROP COLUMN version;
