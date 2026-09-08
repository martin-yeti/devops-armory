-- Your SQL goes here

-- `id` was created as SERIAL (int4), so `logs_id_seq` stops at 2147483647
-- and every subsequent insert fails with
--   nextval: reached maximum value of sequence "logs_id_seq"
--
-- The counter runs out well before the table holds 2.1B rows: the nextval
-- default is evaluated before conflict detection, so every log line that
-- `ON CONFLICT DO NOTHING` discards as a duplicate still burns an id.
--
-- ALTER COLUMN TYPE rewrites the whole table under an ACCESS EXCLUSIVE
-- lock, which on a large logs table is minutes-to-hours of blocked reads
-- and writes plus a second copy of the table on disk. Run this by hand in
-- a maintenance window first; the guard makes it a no-op afterwards, so
-- the migration stays safe to apply on deploy.

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = 'logs'
          AND column_name = 'id'
          AND data_type <> 'bigint'
    ) THEN
        ALTER TABLE logs ALTER COLUMN id TYPE BIGINT;
    END IF;
END
$$;

-- Widening the column does not widen the sequence it draws from, so the
-- sequence has to be altered separately or nextval keeps failing.
ALTER SEQUENCE logs_id_seq AS BIGINT MAXVALUE 9223372036854775807;
