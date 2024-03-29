-- Your SQL goes here

CREATE TABLE IF NOT EXISTS guild_schedule (
	uuid UUID NOT NULL PRIMARY KEY,
	snapshots INTEGER NOT NULL DEFAULT 0,
	hash BIGINT NOT NULL,
	prev_hash BIGINT,

	update_at TIMESTAMP WITH TIME ZONE NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
