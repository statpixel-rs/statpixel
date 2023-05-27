-- Your SQL goes here

ALTER TABLE snapshot ALTER COLUMN version DROP DEFAULT;
ALTER TABLE guild_snapshot ALTER COLUMN version DROP DEFAULT;
