-- Your SQL goes here

ALTER TABLE pod_metrics ADD COLUMN IF NOT EXISTS reason TEXT;
