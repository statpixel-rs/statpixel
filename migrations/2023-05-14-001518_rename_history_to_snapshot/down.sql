-- This file should undo anything in `up.sql`

ALTER TABLE snapshot
  RENAME TO history;
