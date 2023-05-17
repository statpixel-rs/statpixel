-- This file should undo anything in `up.sql`

ALTER TABLE snapshot
	RENAME COLUMN did_update TO update_flag;
