-- Your SQL goes here

CREATE TABLE IF NOT EXISTS guild_autocomplete (
	id UUID NOT NULL PRIMARY KEY,
	name VARCHAR(32) NOT NULL,
	xp INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS guild_autocomplete_name_idx ON guild_autocomplete (LOWER(name) varchar_pattern_ops);
