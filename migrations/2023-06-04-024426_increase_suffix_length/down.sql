-- This file should undo anything in `up.sql`

ALTER TABLE "user" ALTER COLUMN "suffix" SET DATA TYPE CHAR(1);
