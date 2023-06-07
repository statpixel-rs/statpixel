-- Your SQL goes here

DROP TABLE commands;
DROP TABLE usage;

CREATE TABLE usage (
	user_id BIGINT NOT NULL,
	command_name TEXT NOT NULL,
	count INT NOT NULL DEFAULT 0,

	PRIMARY KEY (user_id, command_name)
);

CREATE INDEX usage_command_name ON usage (command_name);
CREATE INDEX user_id ON usage (user_id);
