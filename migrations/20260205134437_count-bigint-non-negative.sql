ALTER TABLE measurements
ALTER COLUMN count TYPE BIGINT;

ALTER TABLE measurements
DROP CONSTRAINT IF EXISTS measurements_count_non_negative;

ALTER TABLE measurements
ADD CONSTRAINT measurements_count_non_negative CHECK (count >= 0);
