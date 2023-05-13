-- Your SQL goes here
ALTER TABLE autocomplete ALTER COLUMN name SET DATA TYPE VARCHAR(16) COLLATE "default";

DROP COLLATION english_ci;

DROP INDEX autocomplete_name_idx;
CREATE INDEX autocomplete_name_idx ON autocomplete (LOWER(name) varchar_pattern_ops);
