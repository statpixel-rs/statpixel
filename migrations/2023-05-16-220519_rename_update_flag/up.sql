-- Your SQL goes here

ALTER TABLE snapshot
	RENAME COLUMN update_flag TO did_update;
