-- This file should undo anything in `up.sql`

ALTER TABLE users
	DROP COLUMN IF EXISTS updated_at,
	DROP COLUMN IF EXISTS created_at;
