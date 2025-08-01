ALTER TABLE tenants
  ALTER COLUMN chain_id TYPE BIGINT,
  ADD CONSTRAINT tenants_chain_id_check CHECK (chain_id >= 0);