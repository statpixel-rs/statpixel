-- Your SQL goes here

DROP INDEX IF EXISTS history_uuid_idx;
CREATE INDEX IF NOT EXISTS history_uuid_created_idx ON history (uuid, created_at DESC);
CREATE INDEX IF NOT EXISTS history_created_idx ON history (created_at DESC);
