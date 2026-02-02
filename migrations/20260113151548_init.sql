CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE measurement (
    node_id UUID    NOT NULL,
    created_at  TIMESTAMPTZ   NOT NULL,
    wifi    INTEGER NOT NULL,
    bluetooth   INTEGER NOT NULL
) WITH (
    tsdb.hypertable,
    tsdb.segmentby = 'node_id',
    tsdb.orderby = 'created_at DESC'
);