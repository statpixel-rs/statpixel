-- Your SQL goes here

ALTER TABLE schedule ADD COLUMN vendor_hash BIGINT;
ALTER TABLE schedule ADD COLUMN vendor_prev_hash BIGINT;
