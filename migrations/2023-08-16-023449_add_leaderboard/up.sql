-- Your SQL goes here

CREATE TABLE leaderboard (
	uuid UUID PRIMARY KEY NOT NULL,
	data JSONB NOT NULL,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX leaderboard_uuid_idx ON leaderboard (uuid);
