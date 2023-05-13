-- This file should undo anything in `up.sql`

CREATE COLLATION english_ci (
	PROVIDER = icu,
	LOCALE = 'en-US-u-ks-level2',
	DETERMINISTIC = TRUE
);

ALTER TABLE autocomplete ALTER COLUMN name SET DATA TYPE VARCHAR(16) COLLATE english_ci;

DROP INDEX autocomplete_name_idx;
CREATE INDEX autocomplete_name_idx ON autocomplete (name varchar_pattern_ops);
