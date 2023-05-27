-- This file should undo anything in `up.sql`

ALTER TABLE snapshot ALTER COLUMN version DEFAULT 0;
ALTER TABLE guild_snapshot ALTER COLUMN version DEFAULT 0;
