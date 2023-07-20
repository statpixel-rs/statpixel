-- Your SQL goes here

DROP TABLE track;

-- This table holds players that should be tracked, and
-- where the updates should be sent to.
CREATE TABLE track (
	-- The user that created the tracker entry
	user_id BIGINT NOT NULL,
	-- The guild that it was created for
	guild_id BIGINT,
	-- The channel that it was created for
	channel_id BIGINT NOT NULL,
	-- The user that is being tracked
	uuid UUID NOT NULL,
	-- This is an enum with the reason for the deactivation:
	-- 0: The tracker is active
	-- 1: The user disabled it manually
	-- 2: The player was not found
	-- 3: The channel was not found
	-- 4: The bot has insufficient permissions
	-- 5: The user's limit was reduced and this one was chosen to be deactivated
	"state" SMALLINT NOT NULL DEFAULT 0,

	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

	PRIMARY KEY (channel_id, uuid)
);
