CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE nodes (
    id UUID PRIMARY KEY,
    status TEXT NOT NULL DEFAULT 'pending',
    location GEOGRAPHY(Point, 4326) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX nodes_location_gix ON nodes USING GIST (location);
