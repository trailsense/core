ALTER TABLE nodes
DROP CONSTRAINT IF EXISTS nodes_send_frequency_seconds_valid_range;

ALTER TABLE nodes
ADD CONSTRAINT nodes_send_frequency_seconds_valid_range
CHECK (send_frequency_seconds BETWEEN 1 AND 86400);
