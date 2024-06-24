-- Add migration script here

--- /// create url shortener service TABLE
CREATE TABLE IF NOT EXISTS url_shortener_service (
    id Serial PRIMARY KEY,
    url TEXT NOT NULL,
    short_url TEXT NOT NULL,
    created_at timestamp DEFAULT CURRENT_TIMESTAMP
);