-- Your SQL goes here

CREATE INDEX IF NOT EXISTS gcp_id_index ON logs (google_project_id);

CREATE INDEX IF NOT EXISTS logs_project_host_idx ON logs (google_project_id, project_id, region, host);
