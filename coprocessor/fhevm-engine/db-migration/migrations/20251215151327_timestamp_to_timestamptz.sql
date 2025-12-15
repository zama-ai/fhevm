BEGIN;

SET TIMEZONE TO 'UTC';

ALTER TABLE computations
    ALTER COLUMN created_at TYPE TIMESTAMPTZ,
    ALTER COLUMN created_at SET DEFAULT NOW(),
    ALTER COLUMN completed_at TYPE TIMESTAMPTZ,
    ALTER COLUMN schedule_order TYPE TIMESTAMPTZ,
    ALTER COLUMN schedule_order SET DEFAULT NOW();

ALTER TABLE ciphertexts
    ALTER COLUMN created_at TYPE TIMESTAMPTZ,
    ALTER COLUMN created_at SET DEFAULT NOW();

ALTER TABLE pbs_computations
    ALTER COLUMN created_at TYPE TIMESTAMPTZ,
    ALTER COLUMN created_at SET DEFAULT NOW(),
    ALTER COLUMN completed_at TYPE TIMESTAMPTZ;

ALTER TABLE ciphertext_digest
    ALTER COLUMN created_at TYPE TIMESTAMPTZ,
    ALTER COLUMN created_at SET DEFAULT NOW(),
    ALTER COLUMN txn_last_error_at TYPE TIMESTAMPTZ,
    ALTER COLUMN txn_last_error_at SET DEFAULT NULL;

ALTER TABLE allowed_handles
    ALTER COLUMN txn_last_error_at TYPE TIMESTAMPTZ,
    ALTER COLUMN txn_last_error_at SET DEFAULT NULL,
    ALTER COLUMN allowed_at TYPE TIMESTAMPTZ,
    ALTER COLUMN allowed_at SET DEFAULT NOW();

DROP INDEX IF EXISTS idx_allowed_handles_allowed_at;
CREATE INDEX IF NOT EXISTS idx_allowed_handles_unsent
    ON allowed_handles (txn_is_sent, allowed_at);

ALTER TABLE host_listener_poller_state
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ,
    ALTER COLUMN updated_at SET DEFAULT NOW();

COMMIT;
