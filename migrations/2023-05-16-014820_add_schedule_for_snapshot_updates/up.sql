-- Your SQL goes here

CREATE TABLE IF NOT EXISTS schedule (
	uuid UUID NOT NULL PRIMARY KEY,
	update_at TIMESTAMP WITH TIME ZONE NOT NULL,

	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
