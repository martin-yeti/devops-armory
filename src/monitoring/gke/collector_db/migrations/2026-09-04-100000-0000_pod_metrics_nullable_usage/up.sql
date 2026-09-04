-- Your SQL goes here

-- An unhealthy pod has no running container to exec into, so live cpu/ram
-- usage genuinely can't be measured for it; these columns need to allow
-- that instead of the whole row being dropped.
ALTER TABLE pod_metrics ALTER COLUMN cpu_usage DROP NOT NULL;
ALTER TABLE pod_metrics ALTER COLUMN ram_usage DROP NOT NULL;
