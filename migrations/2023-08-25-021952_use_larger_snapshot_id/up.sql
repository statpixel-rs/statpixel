-- Your SQL goes here

ALTER TABLE snapshot
	ALTER COLUMN id TYPE BIGINT;

ALTER SEQUENCE history_id_seq AS BIGINT;
ALTER SEQUENCE history_id_seq RENAME TO snapshot_id_seq;
