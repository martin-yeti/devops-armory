-- Your SQL goes here

-- message is unbounded TEXT and can exceed the btree index row size limit
-- (2704 bytes), so the unique index is built on a hash of message instead
-- of the raw column.
CREATE UNIQUE INDEX IF NOT EXISTS logs_host_time_message_key
ON logs (host, time, md5(message));
