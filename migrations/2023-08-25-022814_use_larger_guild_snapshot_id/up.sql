-- Your SQL goes here

ALTER TABLE guild_snapshot
	ALTER COLUMN id TYPE BIGINT;

ALTER SEQUENCE guild_snapshot_id_seq AS BIGINT;
