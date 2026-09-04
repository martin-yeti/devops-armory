-- This file should undo anything in `up.sql`

ALTER TABLE pod_metrics DROP COLUMN reason;
