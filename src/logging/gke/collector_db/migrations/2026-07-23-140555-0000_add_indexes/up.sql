-- Your SQL goes here

CREATE INDEX gcp_id_index ON logs (google_project_id);

CREATE INDEX logs_project_host_idx ON logs (google_project_id, project_id, region, host);
