-- Your SQL goes here

ALTER TABLE session ADD COLUMN name TEXT;
ALTER TABLE session ADD CONSTRAINT unique_name_user_id UNIQUE (name, user_id);
