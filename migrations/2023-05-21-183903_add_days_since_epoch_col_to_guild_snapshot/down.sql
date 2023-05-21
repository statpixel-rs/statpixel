-- This file should undo anything in `up.sql`

ALTER TABLE guild_snapshot DROP COLUMN days_since_epoch;
