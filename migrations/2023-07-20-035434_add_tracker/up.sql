-- Your SQL goes here

CREATE INDEX track_uuid_idx ON track (uuid);
-- This is used for the DISTINCT ON query to avoid sending the
-- same update more than once to the same channel
CREATE INDEX track_channel_id_idx ON track (channel_id);
