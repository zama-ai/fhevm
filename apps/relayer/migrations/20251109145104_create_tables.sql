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
-- tx_sent means we got a receipt. in-flight means tx is in the helper and proceeding.
CREATE TYPE req_status AS ENUM ('queued', 'in_flight', 'tx_sent', 'completed', 'timed_out', 'failure');

-- Table for user decryption requests.
CREATE TABLE user_decrypt_req(
    id SERIAL PRIMARY KEY,
    ext_req_id UUID NOT NULL,
    internal_decryption_id TEXT NOT NULL,
    gw_decryption_id INTEGER,
    req JSONB NOT NULL,
    res JSONB,
    req_status req_status NOT NULL DEFAULT 'queued',
    tx_hash TEXT,
    consensus_reached BOOLEAN NOT NULL DEFAULT false,
    err_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for user_decrypt_req
CREATE INDEX idx_user_decrypt_req_ext_req_id ON user_decrypt_req USING HASH (ext_req_id);

-- Trigger for updated at field.
CREATE TRIGGER set_user_decrypt_req_updated_at
BEFORE UPDATE ON user_decrypt_req
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Table for user decryption shares.
CREATE TABLE user_decrypt_share (
    id SERIAL PRIMARY KEY,
    gw_decryption_id INTEGER NOT NULL,
    share_index INTEGER NOT NULL,
    share TEXT NOT NULL,
    kms_signature TEXT NOT NULL,
    -- ADD EXTRA_DATA field as well.
    extra_data TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for gw_decryption_id
CREATE INDEX idx_user_decrypt_share_gw_decryption_id ON user_decrypt_share(gw_decryption_id);

-- Trigger for updated at field.
CREATE TRIGGER set_user_decrypt_share_updated_at
BEFORE UPDATE ON user_decrypt_share
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Table for public decryption requests.
CREATE TABLE public_decrypt_req(
    id SERIAL PRIMARY KEY,
    ext_req_id UUID NOT NULL,
    internal_decryption_id TEXT NOT NULL,
    gw_decryption_id INTEGER,
    req JSONB NOT NULL,
    res JSONB,
    req_status req_status NOT NULL DEFAULT 'queued',
    tx_hash TEXT,
    err_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_public_decrypt_req_ext_req_id ON public_decrypt_req USING HASH (ext_req_id);

-- Trigger for updated at field.
CREATE TRIGGER set_public_decrypt_req_updated_at
BEFORE UPDATE ON public_decrypt_req
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Table for input proof requests.
CREATE TABLE input_proof_req(
    id SERIAL PRIMARY KEY,
    ext_req_id UUID NOT NULL,
    internal_input_proof_id UUID NOT NULL, -- uuid v7 here.
    gw_input_proof_id INTEGER,
    req JSONB NOT NULL,
    res JSONB,
    req_status req_status NOT NULL DEFAULT 'queued',
    tx_hash TEXT,
    err_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index with ext_ref_id.
CREATE INDEX idx_input_proof_req_ext_req_id ON input_proof_req USING HASH (ext_req_id);

-- Trigger for updated at field.
CREATE TRIGGER set_input_proof_req_updated_at
BEFORE UPDATE ON input_proof_req
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();