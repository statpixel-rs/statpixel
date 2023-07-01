-- This file should undo anything in `up.sql`

ALTER TABLE bazaar ALTER COLUMN created_at TYPE TIMESTAMP;
