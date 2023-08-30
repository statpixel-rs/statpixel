-- Your SQL goes here

ALTER TABLE schedule ADD COLUMN vendor_update_at TIMESTAMP WITH TIME ZONE DEFAULT NOW();
