-- This file should undo anything in `up.sql`

DROP INDEX autocomplete_name_idx;
CREATE INDEX autocomplete_name_idx ON autocomplete (name varchar_pattern_ops);
