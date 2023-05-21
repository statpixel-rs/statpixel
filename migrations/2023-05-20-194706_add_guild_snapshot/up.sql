-- Your SQL goes here

CREATE TABLE guild_snapshot (
	id SERIAL PRIMARY KEY NOT NULL,
	uuid UUID NOT NULL,

	hash BIGINT NOT NULL,
	did_update BOOLEAN NOT NULL,
	data BYTEA NOT NULL,

	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);