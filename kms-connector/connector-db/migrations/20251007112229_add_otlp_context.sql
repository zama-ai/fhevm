-- Create otlp_context column for every requests/responses tables.
-- The default value corresponds to a serialized empty otlp context (only for old requests already in DB).

ALTER TABLE public_decryption_requests ADD COLUMN otlp_context BYTEA;
UPDATE public_decryption_requests SET otlp_context = '\x0000000000000000'::bytea;
ALTER TABLE public_decryption_requests ALTER COLUMN otlp_context SET NOT NULL;

ALTER TABLE user_decryption_requests ADD COLUMN otlp_context BYTEA;
UPDATE user_decryption_requests SET otlp_context = '\x0000000000000000'::bytea;
ALTER TABLE user_decryption_requests ALTER COLUMN otlp_context SET NOT NULL;

ALTER TABLE prep_keygen_requests ADD COLUMN otlp_context BYTEA;
UPDATE prep_keygen_requests SET otlp_context = '\x0000000000000000'::bytea;
ALTER TABLE prep_keygen_requests ALTER COLUMN otlp_context SET NOT NULL;

ALTER TABLE keygen_requests ADD COLUMN otlp_context BYTEA;
UPDATE keygen_requests SET otlp_context = '\x0000000000000000'::bytea;
ALTER TABLE keygen_requests ALTER COLUMN otlp_context SET NOT NULL;

ALTER TABLE crsgen_requests ADD COLUMN otlp_context BYTEA;
UPDATE crsgen_requests SET otlp_context = '\x0000000000000000'::bytea;
ALTER TABLE crsgen_requests ALTER COLUMN otlp_context SET NOT NULL;


ALTER TABLE public_decryption_responses ADD COLUMN otlp_context BYTEA;
UPDATE public_decryption_responses SET otlp_context = '\x0000000000000000'::bytea;
ALTER TABLE public_decryption_responses ALTER COLUMN otlp_context SET NOT NULL;

ALTER TABLE user_decryption_responses ADD COLUMN otlp_context BYTEA;
UPDATE user_decryption_responses SET otlp_context = '\x0000000000000000'::bytea;
ALTER TABLE user_decryption_responses ALTER COLUMN otlp_context SET NOT NULL;

ALTER TABLE prep_keygen_responses ADD COLUMN otlp_context BYTEA;
UPDATE prep_keygen_responses SET otlp_context = '\x0000000000000000'::bytea;
ALTER TABLE prep_keygen_responses ALTER COLUMN otlp_context SET NOT NULL;

ALTER TABLE keygen_responses ADD COLUMN otlp_context BYTEA;
UPDATE keygen_responses SET otlp_context = '\x0000000000000000'::bytea;
ALTER TABLE keygen_responses ALTER COLUMN otlp_context SET NOT NULL;

ALTER TABLE crsgen_responses ADD COLUMN otlp_context BYTEA;
UPDATE crsgen_responses SET otlp_context = '\x0000000000000000'::bytea;
ALTER TABLE crsgen_responses ALTER COLUMN otlp_context SET NOT NULL;
