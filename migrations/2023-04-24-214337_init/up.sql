CREATE TABLE users (
	-- Snowflake from Discord
	id BIGINT PRIMARY KEY,
	-- Whether to send text or image responses
	text BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE history (
	uuid UUID NOT NULL,
	game_id SMALLINT NOT NULL,
	game_mode SMALLINT NOT NULL,

	update_frequency SMALLINT,

	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

	PRIMARY KEY (uuid, game_id, game_mode)
);
