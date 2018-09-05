-- Your SQL goes here
CREATE TABLE gradients (
  id bigserial PRIMARY KEY,
  message_id bigint NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
  color_id bigint NOT NULL REFERENCES colors(id) ON DELETE CASCADE,
  position integer NOT NULL
);
