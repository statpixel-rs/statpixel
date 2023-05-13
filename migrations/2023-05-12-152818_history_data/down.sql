-- This file should undo anything in `up.sql`

ALTER TABLE history
	DROP CONSTRAINT history_pkey;

ALTER TABLE history
	DROP COLUMN id;

ALTER TABLE history
	DROP COLUMN data;

ALTER TABLE history
	ADD COLUMN game_id SMALLINT NOT NULL,
	ADD COLUMN game_mode SMALLINT NOT NULL,
	ADD COLUMN update_frequency SMALLINT;

ALTER TABLE history
	ADD CONSTRAINT history_pkey PRIMARY KEY (uuid, game_id, game_mode);
