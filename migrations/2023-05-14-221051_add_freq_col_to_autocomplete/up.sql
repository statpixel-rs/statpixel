-- Your SQL goes here

ALTER TABLE autocomplete ADD COLUMN searches INT NOT NULL DEFAULT 0;

CREATE INDEX autocomplete_searches_desc_idx ON autocomplete (searches DESC);
