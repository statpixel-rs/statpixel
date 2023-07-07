-- Your SQL goes here

CREATE TABLE metrics (
	-- guild_id or user_id
	id BIGINT NOT NULL,
	-- 0 = GuildJoin
	-- 1 = GuildLeave
	-- 2 = ProfileCreate
	-- 3 = CommandRun
	kind SMALLINT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

	PRIMARY KEY (id, kind)
);
