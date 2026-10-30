-- Sender state is independent of local proof verification and S3 publication.
ALTER TABLE verify_proofs
    ADD COLUMN sender_status TEXT NOT NULL DEFAULT 'QUEUED'
        CHECK (sender_status IN ('QUEUED', 'SENDING', 'AWAITING_OUTCOME', 'RETRY_WAIT', 'COMPLETED', 'STOPPED')),
    ADD COLUMN sender_status_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ADD COLUMN sender_attempt_no BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN sender_next_action_at TIMESTAMPTZ,
    ADD COLUMN sender_last_error TEXT,
    ADD COLUMN sender_tx_hash BYTEA,
    ADD COLUMN sender_raw_tx BYTEA,
    ADD CHECK (sender_raw_tx IS NULL OR (sender_tx_hash IS NOT NULL AND octet_length(sender_tx_hash) = 32));

ALTER TABLE ciphertext_digest
    ADD COLUMN sender_status TEXT NOT NULL DEFAULT 'QUEUED'
        CHECK (sender_status IN ('QUEUED', 'SENDING', 'AWAITING_OUTCOME', 'RETRY_WAIT', 'COMPLETED', 'STOPPED')),
    ADD COLUMN sender_status_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ADD COLUMN sender_attempt_no BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN sender_next_action_at TIMESTAMPTZ,
    ADD COLUMN sender_last_error TEXT,
    ADD COLUMN sender_tx_hash BYTEA,
    ADD COLUMN sender_raw_tx BYTEA,
    ADD CHECK (sender_raw_tx IS NULL OR (sender_tx_hash IS NOT NULL AND octet_length(sender_tx_hash) = 32));

-- These rows have already completed their sender work under the old schema.
UPDATE ciphertext_digest SET sender_status = 'COMPLETED' WHERE txn_is_sent;
UPDATE verify_proofs SET sender_status = 'COMPLETED'
WHERE verified = FALSE AND handles IS NULL;

CREATE INDEX verify_proofs_sender_pending ON verify_proofs (sender_next_action_at, zk_proof_id)
WHERE sender_status IN ('QUEUED', 'RETRY_WAIT', 'AWAITING_OUTCOME');
CREATE INDEX ciphertext_digest_sender_pending ON ciphertext_digest (sender_next_action_at, handle)
WHERE sender_status IN ('QUEUED', 'RETRY_WAIT', 'AWAITING_OUTCOME');

-- Expiry runs alongside queue polling; keep it independent of the retained history size.
CREATE INDEX verify_proofs_sender_completed ON verify_proofs (sender_status_updated_at)
WHERE sender_status = 'COMPLETED' AND verified = TRUE AND sender_raw_tx IS NULL;
