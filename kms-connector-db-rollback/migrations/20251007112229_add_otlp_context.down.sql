-- Down for 20251007112229_add_otlp_context.
--
-- Drops the `otlp_context` column from the ten request and response tables.
-- The tracing contexts are lost; the previous version does not use them.

ALTER TABLE public_decryption_requests DROP COLUMN otlp_context;
ALTER TABLE user_decryption_requests DROP COLUMN otlp_context;
ALTER TABLE prep_keygen_requests DROP COLUMN otlp_context;
ALTER TABLE keygen_requests DROP COLUMN otlp_context;
ALTER TABLE crsgen_requests DROP COLUMN otlp_context;

ALTER TABLE public_decryption_responses DROP COLUMN otlp_context;
ALTER TABLE user_decryption_responses DROP COLUMN otlp_context;
ALTER TABLE prep_keygen_responses DROP COLUMN otlp_context;
ALTER TABLE keygen_responses DROP COLUMN otlp_context;
ALTER TABLE crsgen_responses DROP COLUMN otlp_context;
