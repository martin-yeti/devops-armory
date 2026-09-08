-- Your SQL goes here

CREATE TABLE pod_metrics (
    id SERIAL PRIMARY KEY,
    google_project_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    region TEXT NOT NULL,
    namespace TEXT NOT NULL,
    pod_name TEXT NOT NULL,
    cpu_request DOUBLE PRECISION NOT NULL,
    ram_request DOUBLE PRECISION NOT NULL,
    cpu_limit DOUBLE PRECISION NOT NULL,
    ram_limit DOUBLE PRECISION NOT NULL,
    healthy BOOLEAN NOT NULL,
    cpu_usage DOUBLE PRECISION NOT NULL,
    ram_usage DOUBLE PRECISION NOT NULL,
    time TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX IF NOT EXISTS pod_metrics_project_pod_idx
ON pod_metrics (google_project_id, project_id, region, namespace, pod_name, time);
