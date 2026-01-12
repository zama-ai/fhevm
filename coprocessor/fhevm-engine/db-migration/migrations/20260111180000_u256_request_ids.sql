ALTER TABLE input_verification_requests
  DROP CONSTRAINT IF EXISTS input_verification_requests_request_id_check;

ALTER TABLE input_verification_requests
  ALTER COLUMN request_id TYPE BYTEA
  USING decode(lpad(to_hex(request_id), 64, '0'), 'hex');

ALTER TABLE verify_proofs
  DROP CONSTRAINT IF EXISTS verify_proofs_zk_proof_id_check;

ALTER TABLE verify_proofs
  ALTER COLUMN zk_proof_id TYPE BYTEA
  USING decode(lpad(to_hex(zk_proof_id), 64, '0'), 'hex');
