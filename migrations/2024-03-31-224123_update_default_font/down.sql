-- This file should undo anything in `up.sql`
ALTER TABLE "user" ALTER COLUMN "display" SET DEFAULT 1;
ALTER TABLE "user" ALTER COLUMN "font" SET DEFAULT 0;
