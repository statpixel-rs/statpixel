-- Your SQL goes here

ALTER TABLE "user" ADD COLUMN "votes" SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE "bazaar_items" RENAME TO "bazaar_item";
