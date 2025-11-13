ALTER TABLE public_decryption_requests ADD COLUMN error_counter SMALLINT DEFAULT 0;
ALTER TABLE public_decryption_requests ALTER COLUMN error_counter SET NOT NULL;

ALTER TABLE user_decryption_requests ADD COLUMN error_counter SMALLINT DEFAULT 0;
ALTER TABLE user_decryption_requests ALTER COLUMN error_counter SET NOT NULL;
