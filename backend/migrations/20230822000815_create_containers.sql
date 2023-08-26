-- Add migration script here

CREATE TABLE IF NOT EXISTS container (
    id VARCHAR(36) PRIMARY KEY,
    label TEXT UNIQUE NOT NULL,
    port INT UNIQUE NOT NULL
);
