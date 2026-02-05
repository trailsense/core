ALTER TABLE measurement
ADD COLUMN count INTEGER NOT NULL DEFAULT 0;

UPDATE measurement
SET count = wifi + bluetooth;

ALTER TABLE measurement
DROP COLUMN wifi,
DROP COLUMN bluetooth;
