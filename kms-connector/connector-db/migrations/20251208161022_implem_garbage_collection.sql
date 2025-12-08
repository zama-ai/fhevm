-- Delete events autoremoval triggers/functions
DROP TRIGGER IF EXISTS trigger_public_decryption_requests ON public_decryption_responses;
DROP TRIGGER IF EXISTS trigger_user_decryption_requests ON user_decryption_responses;
DROP TRIGGER IF EXISTS trigger_delete_prep_keygen_requests ON prep_keygen_responses;
DROP TRIGGER IF EXISTS trigger_delete_keygen_requests ON keygen_responses;
DROP TRIGGER IF EXISTS trigger_delete_crsgen_requests ON crsgen_responses;

DROP FUNCTION IF EXISTS delete_from_public_decryption_requests;
DROP FUNCTION IF EXISTS delete_from_user_decryption_requests;
DROP FUNCTION IF EXISTS delete_from_prep_keygen_requests;
DROP FUNCTION IF EXISTS delete_from_keygen_requests;
DROP FUNCTION IF EXISTS delete_from_crsgen_requests;

-- Rename the `under_process` field to `locked` for all events/responses tables.
ALTER TABLE public_decryption_requests RENAME COLUMN under_process TO locked;
ALTER TABLE user_decryption_requests RENAME COLUMN under_process TO locked;
ALTER TABLE prep_keygen_requests RENAME COLUMN under_process TO locked;
ALTER TABLE keygen_requests RENAME COLUMN under_process TO locked;
ALTER TABLE crsgen_requests RENAME COLUMN under_process TO locked;
ALTER TABLE prss_init RENAME COLUMN under_process TO locked;
ALTER TABLE key_reshare_same_set RENAME COLUMN under_process TO locked;

ALTER TABLE public_decryption_responses RENAME COLUMN under_process TO locked;
ALTER TABLE user_decryption_responses RENAME COLUMN under_process TO locked;
ALTER TABLE prep_keygen_responses RENAME COLUMN under_process TO locked;
ALTER TABLE keygen_responses RENAME COLUMN under_process TO locked;
ALTER TABLE crsgen_responses RENAME COLUMN under_process TO locked;

-- Add the `locked_at` field for all events/responses tables.
ALTER TABLE public_decryption_requests ADD COLUMN locked_at TIMESTAMP;
ALTER TABLE user_decryption_requests ADD COLUMN locked_at TIMESTAMP;
ALTER TABLE prep_keygen_requests ADD COLUMN locked_at TIMESTAMP;
ALTER TABLE keygen_requests ADD COLUMN locked_at TIMESTAMP;
ALTER TABLE crsgen_requests ADD COLUMN locked_at TIMESTAMP;
ALTER TABLE prss_init ADD COLUMN locked_at TIMESTAMP;
ALTER TABLE key_reshare_same_set ADD COLUMN locked_at TIMESTAMP;

ALTER TABLE public_decryption_responses ADD COLUMN locked_at TIMESTAMP;
ALTER TABLE user_decryption_responses ADD COLUMN locked_at TIMESTAMP;
ALTER TABLE prep_keygen_responses ADD COLUMN locked_at TIMESTAMP;
ALTER TABLE keygen_responses ADD COLUMN locked_at TIMESTAMP;
ALTER TABLE crsgen_responses ADD COLUMN locked_at TIMESTAMP;

-- Add the `completed_at` field for all events/responses tables.
ALTER TABLE public_decryption_requests ADD COLUMN completed_at TIMESTAMP;
ALTER TABLE user_decryption_requests ADD COLUMN completed_at TIMESTAMP;
ALTER TABLE prep_keygen_requests ADD COLUMN completed_at TIMESTAMP;
ALTER TABLE keygen_requests ADD COLUMN completed_at TIMESTAMP;
ALTER TABLE crsgen_requests ADD COLUMN completed_at TIMESTAMP;
ALTER TABLE prss_init ADD COLUMN completed_at TIMESTAMP;
ALTER TABLE key_reshare_same_set ADD COLUMN completed_at TIMESTAMP;

ALTER TABLE public_decryption_responses ADD COLUMN completed_at TIMESTAMP;
ALTER TABLE user_decryption_responses ADD COLUMN completed_at TIMESTAMP;
ALTER TABLE prep_keygen_responses ADD COLUMN completed_at TIMESTAMP;
ALTER TABLE keygen_responses ADD COLUMN completed_at TIMESTAMP;
ALTER TABLE crsgen_responses ADD COLUMN completed_at TIMESTAMP;

-- TODO: autofill completed at for public_decryption_requests, ..., crsgen_requests?
