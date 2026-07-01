-- Your SQL goes here

CREATE TABLE logs (
    id SERIAL PRIMARY KEY,
    google_project_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    region TEXT NOT NULL,
    host TEXT NOT NULL,
    message TEXT NOT NULL,
    time TIMESTAMPTZ DEFAULT now()
);

