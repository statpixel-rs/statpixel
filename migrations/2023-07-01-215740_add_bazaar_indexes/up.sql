-- Your SQL goes here

CREATE INDEX bazaar_items_name_asc_idx ON bazaar_items (name ASC);
CREATE INDEX IF NOT EXISTS bazaar_items_name_idx ON bazaar_items (LOWER(name) varchar_pattern_ops);
