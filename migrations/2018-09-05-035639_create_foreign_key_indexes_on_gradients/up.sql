-- Your SQL goes here
begin transaction;

CREATE INDEX gradients_on_message_id_idx ON gradients (message_id);
CREATE INDEX gradients_on_color_id_idx ON gradients (color_id);

commit;
