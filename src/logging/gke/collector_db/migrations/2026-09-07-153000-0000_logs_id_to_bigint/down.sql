-- This file should undo anything in `up.sql`

-- Only reversible while every existing id still fits in int4; both
-- statements error out otherwise, which is the intended outcome rather
-- than silently truncating ids.

ALTER SEQUENCE logs_id_seq AS INTEGER MAXVALUE 2147483647;

ALTER TABLE logs ALTER COLUMN id TYPE INTEGER;
