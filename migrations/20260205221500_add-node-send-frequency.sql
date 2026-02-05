ALTER TABLE nodes
ADD COLUMN send_frequency_seconds BIGINT NOT NULL DEFAULT 3600;

ALTER TABLE nodes
ADD CONSTRAINT nodes_send_frequency_seconds_positive
CHECK (send_frequency_seconds > 0);
