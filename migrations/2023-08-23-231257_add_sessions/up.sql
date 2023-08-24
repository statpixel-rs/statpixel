-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE sessions (
	-- session id
	id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
	-- the snapshot id
	snapshot_id INT NOT NULL,
	-- user that created the session
	user_id BIGINT NOT NULL,
	-- player or guild that the session is for
	uuid UUID NOT NULL,
	-- the type of the session
	-- 0: player
	-- 1: guild
	kind SMALLINT NOT NULL
);
