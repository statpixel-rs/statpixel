-- This file should undo anything in `up.sql`

ALTER TABLE "user" DROP COLUMN display;
ALTER TABLE "user" ADD COLUMN text bool NOT NULL DEFAULT false;
