-- Your SQL goes here

ALTER TABLE "user" ADD COLUMN premium_until TIMESTAMP WITH TIME ZONE DEFAULT to_timestamp(0);

CREATE TABLE boost (
	user_id BIGINT NOT NULL,
	guild_id BIGINT NOT NULL,

	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

	PRIMARY KEY (user_id, guild_id)
);
