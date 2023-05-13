-- Your SQL goes here

DROP INDEX autocomplete_name_idx;
CREATE INDEX autocomplete_name_idx ON autocomplete (LOWER(name) varchar_pattern_ops);
CREATE INDEX autocomplete_name_order_idx ON autocomplete (LENGTH(name) ASC);
