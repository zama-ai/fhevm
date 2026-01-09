--------------------------------------------------------------------------------------------------
--                V2 Decryption requests: handles/ACL fields and rejection metadata             --
--------------------------------------------------------------------------------------------------

ALTER TABLE public_decryption_requests
    ADD COLUMN IF NOT EXISTS handles BYTEA[] NOT NULL DEFAULT '{}'::bytea[],
    ADD COLUMN IF NOT EXISTS contract_addresses BYTEA[] NOT NULL DEFAULT '{}'::bytea[],
    ADD COLUMN IF NOT EXISTS chain_id BYTEA NOT NULL DEFAULT decode(repeat('00', 64), 'hex'),
    ADD COLUMN IF NOT EXISTS timestamp BYTEA NOT NULL DEFAULT decode(repeat('00', 64), 'hex'),
    ADD COLUMN IF NOT EXISTS epoch_id BYTEA NOT NULL DEFAULT decode(repeat('00', 64), 'hex'),
    ADD COLUMN IF NOT EXISTS rejection_reason TEXT,
    ADD COLUMN IF NOT EXISTS rejection_code TEXT;

ALTER TABLE public_decryption_requests
    DROP COLUMN IF EXISTS sns_ct_materials,
    DROP COLUMN IF EXISTS extra_data;

ALTER TABLE user_decryption_requests
    ADD COLUMN IF NOT EXISTS handles BYTEA[] NOT NULL DEFAULT '{}'::bytea[],
    ADD COLUMN IF NOT EXISTS contract_addresses BYTEA[] NOT NULL DEFAULT '{}'::bytea[],
    ADD COLUMN IF NOT EXISTS signature BYTEA NOT NULL DEFAULT ''::bytea,
    ADD COLUMN IF NOT EXISTS chain_id BYTEA NOT NULL DEFAULT decode(repeat('00', 64), 'hex'),
    ADD COLUMN IF NOT EXISTS timestamp BYTEA NOT NULL DEFAULT decode(repeat('00', 64), 'hex'),
    ADD COLUMN IF NOT EXISTS epoch_id BYTEA NOT NULL DEFAULT decode(repeat('00', 64), 'hex'),
    ADD COLUMN IF NOT EXISTS rejection_reason TEXT,
    ADD COLUMN IF NOT EXISTS rejection_code TEXT;

ALTER TABLE user_decryption_requests
    DROP COLUMN IF EXISTS sns_ct_materials,
    DROP COLUMN IF EXISTS extra_data;
