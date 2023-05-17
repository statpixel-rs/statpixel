-- This file should undo anything in `up.sql`

ALTER TABLE snapshot DROP COLUMN update_flag;
