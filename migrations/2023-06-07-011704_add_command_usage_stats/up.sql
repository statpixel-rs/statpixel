-- Your SQL goes here

CREATE TABLE usage (
	user_id BIGINT NOT NULL,
	command_id BIGINT NOT NULL,
	count INT NOT NULL DEFAULT 0,

	PRIMARY KEY (user_id, command_id)
);

CREATE INDEX usage_command_id ON usage (command_id);
CREATE INDEX user_id ON usage (user_id);
