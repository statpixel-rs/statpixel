ALTER TABLE history
	DROP CONSTRAINT IF EXISTS history_pkey;

ALTER TABLE history
	ADD COLUMN id SERIAL;

CREATE SEQUENCE IF NOT EXISTS history_id_seq OWNED BY history.id;

ALTER TABLE history
	ALTER COLUMN id
	SET DEFAULT nextval('history_id_seq');

ALTER TABLE history ADD CONSTRAINT history_pkey PRIMARY KEY (id);

ALTER TABLE history
	DROP COLUMN game_id,
	DROP COLUMN game_mode,
	DROP COLUMN update_frequency;

ALTER TABLE history
	ADD COLUMN data BYTEA NOT NULL;
