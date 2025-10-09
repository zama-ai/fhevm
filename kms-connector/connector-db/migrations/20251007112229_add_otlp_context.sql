ALTER TABLE public_decryption_requests ADD COLUMN otlp_context BYTEA;
ALTER TABLE user_decryption_requests ADD COLUMN otlp_context BYTEA;
ALTER TABLE prep_keygen_requests ADD COLUMN otlp_context BYTEA;
ALTER TABLE keygen_requests ADD COLUMN otlp_context BYTEA;
ALTER TABLE crsgen_requests ADD COLUMN otlp_context BYTEA;

ALTER TABLE public_decryption_responses ADD COLUMN otlp_context BYTEA;
ALTER TABLE user_decryption_responses ADD COLUMN otlp_context BYTEA;
ALTER TABLE prep_keygen_responses ADD COLUMN otlp_context BYTEA;
ALTER TABLE keygen_responses ADD COLUMN otlp_context BYTEA;
ALTER TABLE crsgen_responses ADD COLUMN otlp_context BYTEA;
