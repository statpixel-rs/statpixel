ALTER TABLE users ADD COLUMN uuid UUID;

CREATE INDEX users_uuid ON users (uuid);
