-- Migration for creating tables and functions.
-- Fucntion for updating updated_at field on update.
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Status enum for all tables.
--- COMPLETED status when we have consensus tx + all the x shares receieved (9 for user, 1 for public)
CREATE TYPE req_status AS ENUM ('queued', 'receipt_received', 'completed', 'timed_out', 'failure');

-- Table for user decryption requests.
CREATE TABLE user_decrypt_req(
    id SERIAL PRIMARY KEY,
    ext_reference_id UUID NOT NULL,
    -- int_indexer_id TEXT NOT NULL,
    int_indexer_id BYTEA NOT NULL,
    gw_reference_id INTEGER,
    req JSONB NOT NULL,
    req_status req_status NOT NULL DEFAULT 'queued',
    gw_req_tx_hash TEXT,
    gw_consensus_tx_hash TEXT,
    err_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for user_decrypt_req
CREATE INDEX idx_user_decrypt_req_ext_reference_id ON user_decrypt_req USING HASH (ext_reference_id);
CREATE UNIQUE INDEX idx_user_decrypt_req_int_indexer_id ON user_decrypt_req (int_indexer_id);
CREATE INDEX idx_user_decrypt_req_gw_reference_id ON user_decrypt_req (gw_reference_id);

-- Trigger for updated at field.
CREATE TRIGGER set_user_decrypt_req_updated_at
BEFORE UPDATE ON user_decrypt_req
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Table for user decryption shares.
CREATE TABLE user_decrypt_share (
    id SERIAL PRIMARY KEY,
    gw_reference_id INTEGER NOT NULL,
    share_index INTEGER NOT NULL,
    share TEXT NOT NULL,
    kms_signature TEXT NOT NULL,
    extra_data TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for user_decrypt_share
-- WE REMOVE THIS INDEX SINCE ITS REDUNDANT WITH THE UNIQUE INDEX BELOW
-- CREATE INDEX idx_user_decrypt_share_gw_decryption_id ON user_decrypt_share(gw_reference_id);
-- Uniqueness of shares index (in case of recieving same share twice++)
CREATE UNIQUE INDEX idx_user_decrypt_share_unique_composite_gw_reference_id_share_index ON user_decrypt_share (gw_reference_id, share_index);

-- Trigger for updated at field.
CREATE TRIGGER set_user_decrypt_share_updated_at
BEFORE UPDATE ON user_decrypt_share
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Table for public decryption requests.
CREATE TABLE public_decrypt_req(
    id SERIAL PRIMARY KEY,
    ext_reference_id UUID NOT NULL,
    -- int_indexer_id TEXT NOT NULL,
    int_indexer_id BYTEA NOT NULL,
    gw_reference_id INTEGER,
    req JSONB NOT NULL,
    res JSONB,
    req_status req_status NOT NULL DEFAULT 'queued',
    gw_req_tx_hash TEXT,
    gw_response_tx_hash TEXT,
    err_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_public_decrypt_req_ext_req_id ON public_decrypt_req USING HASH (ext_reference_id);

-- Trigger for updated at field.
CREATE TRIGGER set_public_decrypt_req_updated_at
BEFORE UPDATE ON public_decrypt_req
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Table for input proof requests.
CREATE TABLE input_proof_req(
    id SERIAL PRIMARY KEY,
    ext_reference_id UUID NOT NULL,
    int_request_id UUID NOT NULL, -- uuid v7 here. -- slight difference, we can have the same proof multiple times.
    gw_reference_id INTEGER,
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
CREATE INDEX idx_input_proof_req_ext_req_id ON input_proof_req USING HASH (ext_reference_id);

-- Trigger for updated at field.
CREATE TRIGGER set_input_proof_req_updated_at
BEFORE UPDATE ON input_proof_req
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();