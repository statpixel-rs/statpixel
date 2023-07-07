-- Your SQL goes here

DROP TABLE metric;
CREATE TABLE metric (
	id SERIAL NOT NULL PRIMARY KEY,
	-- guild_id or user_id
	discord_id BIGINT NOT NULL,
	-- 0 = GuildJoin
	-- 1 = GuildLeave
	-- 2 = ProfileCreate
	-- 3 = CommandRun
	kind SMALLINT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
