-- Your SQL goes here

CREATE INDEX IF NOT EXISTS guild_autocomplete_xp_order_idx ON autocomplete (LENGTH(name) ASC);
