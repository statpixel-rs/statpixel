-- This file should undo anything in `up.sql`

ALTER TABLE "user" DROP COLUMN tracks;
ALTER TABLE "user" DROP COLUMN max_tracks;
