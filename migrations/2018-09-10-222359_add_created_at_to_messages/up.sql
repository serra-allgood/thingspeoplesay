-- Your SQL goes here
ALTER TABLE messages
  ADD COLUMN created_at timestamp NOT NULL DEFAULT NOW();

