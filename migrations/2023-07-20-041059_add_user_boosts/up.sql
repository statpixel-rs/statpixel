-- Your SQL goes here

ALTER TABLE "user" ADD COLUMN boosts SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE "user" ADD COLUMN max_boosts SMALLINT NOT NULL DEFAULT 3;
