-- Migration for creating tables and functions.
-- Fucntion for updating updated_at field on update.
-- TODO: gw_reference_id uses BIGINT (i64) for U256 gateway reference IDs
-- BIGINT range is 2^63-1, should be sufficient for gateway reference IDs
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Status enum for all tables.
--- COMPLETED status when we have consensus tx + all the x shares receieved (9 for user, 1 for public)
CREATE TYPE req_status AS ENUM ('queued', 'processing', 'tx_in_flight', 'receipt_received', 'completed', 'timed_out', 'failure');

-- Table for user decryption requests.
CREATE TABLE user_decrypt_req(
    id SERIAL PRIMARY KEY,
    ext_job_id UUID NOT NULL UNIQUE,
    int_job_id BYTEA NOT NULL UNIQUE,
    gw_reference_id BYTEA,
    req JSONB NOT NULL,
    req_status req_status NOT NULL DEFAULT 'queued',
    gw_req_tx_hash TEXT,
    gw_consensus_tx_hash TEXT,
    err_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for user_decrypt_req
CREATE INDEX idx_user_decrypt_req_ext_job_id ON user_decrypt_req USING HASH (ext_job_id);
-- limit size with indexes.
CREATE UNIQUE INDEX idx_user_decrypt_req_unique_int_job_id_partial
ON user_decrypt_req (int_job_id)
WHERE req_status NOT IN ('failure', 'timed_out');
CREATE INDEX idx_user_decrypt_req_gw_reference_id ON user_decrypt_req (gw_reference_id);

-- Trigger for updated at field.
CREATE TRIGGER set_user_decrypt_req_updated_at
BEFORE UPDATE ON user_decrypt_req
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Table for user decryption shares.
CREATE TABLE user_decrypt_share (
    id SERIAL PRIMARY KEY,
    gw_reference_id BYTEA NOT NULL,
    tx_hash TEXT,
    share_index INTEGER NOT NULL,
    share TEXT NOT NULL,
    kms_signature TEXT NOT NULL,
    extra_data TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for user_decrypt_share
-- WE REMOVE THIS INDEX SINCE ITS REDUNDANT WITH THE UNIQUE INDEX BELOW
-- CREATE INDEX idx_user_decrypt_share_gw_decryption_id ON user_decrypt_share(gw_reference_id);
-- Uniqueness of shares index (in case of recieving same share twice++)
-- NOTE: Be careful, this index naming is the length max - 1
-- Name was before idx_user_decrypt_share_unique_composite_gw_reference_id_share_index and was truncated by PG
-- to idx_user_decrypt_share_unique_composite_gw_reference_id_share_i
CREATE UNIQUE INDEX idx_user_decrypt_share_unique_comp_gw_reference_id_share_index ON user_decrypt_share (gw_reference_id, share_index);

-- Trigger for updated at field.
CREATE TRIGGER set_user_decrypt_share_updated_at
BEFORE UPDATE ON user_decrypt_share
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Table for public decryption requests.
CREATE TABLE public_decrypt_req(
    id SERIAL PRIMARY KEY,
    ext_job_id UUID NOT NULL UNIQUE,
    -- int_job_id TEXT NOT NULL,
    int_job_id BYTEA NOT NULL UNIQUE,
    gw_reference_id BYTEA,
    req JSONB NOT NULL,
    res JSONB,
    req_status req_status NOT NULL DEFAULT 'queued',
    gw_req_tx_hash TEXT,
    gw_response_tx_hash TEXT,
    err_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_public_decrypt_req_ext_job_id ON public_decrypt_req USING HASH (ext_job_id);
-- [REQUIRED] Create this index to make the search fast and support future ON CONFLICT logic
CREATE UNIQUE INDEX idx_public_decrypt_req_unique_int_job_id_partial
ON public_decrypt_req (int_job_id)
WHERE req_status NOT IN ('failure', 'timed_out');
-- [REQUIRED] Needed for efficient updates by Gateway ID
CREATE INDEX idx_public_decrypt_req_gw_reference_id ON public_decrypt_req (gw_reference_id);

-- Trigger for updated at field.
CREATE TRIGGER set_public_decrypt_req_updated_at
BEFORE UPDATE ON public_decrypt_req
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Table for input proof requests.
CREATE TABLE input_proof_req(
    id SERIAL PRIMARY KEY,
    ext_job_id UUID NOT NULL UNIQUE,
    int_request_id UUID NOT NULL UNIQUE, -- uuid v7 here. -- unlike decryption, each int_request triggers a gw request, hence using int_request_id instead of int_job_id
    gw_reference_id BYTEA,
    accepted BOOLEAN DEFAULT null,
    req JSONB NOT NULL,
    res JSONB,
    req_status req_status NOT NULL DEFAULT 'queued',
    gw_req_tx_hash TEXT,
    gw_response_tx_hash TEXT,
    err_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index with ext_ref_id.
CREATE INDEX idx_input_proof_req_ext_job_id ON input_proof_req USING HASH (ext_job_id);
-- UUID v7 is time-ordered, so B-Tree is very efficient here.
CREATE INDEX idx_input_proof_req_int_request_id ON input_proof_req (int_request_id);
CREATE INDEX idx_input_proof_req_gw_reference_id ON input_proof_req (gw_reference_id);

-- Trigger for updated at field.
CREATE TRIGGER set_input_proof_req_updated_at
BEFORE UPDATE ON input_proof_req
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Relative indexes for crons.
-- Indexes are required even if we comes from internals jobs...
-- 1. User Decrypt Timeout Index
CREATE INDEX idx_user_decrypt_req_timeout_check
ON user_decrypt_req (updated_at)
WHERE req_status = 'receipt_received';

-- 2. Public Decrypt Timeout Index
CREATE INDEX idx_public_decrypt_req_timeout_check
ON public_decrypt_req (updated_at)
WHERE req_status = 'receipt_received';

-- 3. Input Proof Timeout Index
CREATE INDEX idx_input_proof_req_timeout_check
ON input_proof_req (updated_at)
WHERE req_status = 'receipt_received';

-- Table for storing last processed block (supports multiple listener instances)
CREATE TABLE gateway_block_number_store (
    instance_id INTEGER PRIMARY KEY,
    last_block_number BIGINT NOT NULL,
    last_block_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Trigger for updated_at field
CREATE TRIGGER set_gateway_block_number_store_updated_at
BEFORE UPDATE ON gateway_block_number_store
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Indexes for retention policies
-- 1. Index for User Decrypt Retention
CREATE INDEX idx_user_decrypt_req_updated_at ON user_decrypt_req (updated_at);
CREATE INDEX idx_user_decrypt_share_updated_at ON user_decrypt_share (updated_at);

-- 2. Index for Public Decrypt Retention
CREATE INDEX idx_public_decrypt_req_updated_at ON public_decrypt_req (updated_at);

-- 3. Index for Input Proof Retention
CREATE INDEX idx_input_proof_req_updated_at ON input_proof_req (updated_at);