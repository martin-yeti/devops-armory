-- This file should undo anything in `up.sql`

UPDATE pod_metrics SET cpu_usage = 0 WHERE cpu_usage IS NULL;
UPDATE pod_metrics SET ram_usage = 0 WHERE ram_usage IS NULL;
ALTER TABLE pod_metrics ALTER COLUMN cpu_usage SET NOT NULL;
ALTER TABLE pod_metrics ALTER COLUMN ram_usage SET NOT NULL;
