CREATE TABLE autocomplete (id UUID PRIMARY KEY, name VARCHAR(16));
CREATE INDEX autocomplete_name_idx ON autocomplete (name varchar_pattern_ops);
