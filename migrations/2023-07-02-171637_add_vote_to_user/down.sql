-- This file should undo anything in `up.sql`

ALTER TABLE "user" DROP COLUMN "votes";
ALTER TABLE "bazaar_item" RENAME TO "bazaar_items";
