ALTER TABLE public_decryption_requests ADD COLUMN error_counter SMALLINT DEFAULT 0 NOT NULL;
ALTER TABLE user_decryption_requests ADD COLUMN error_counter SMALLINT DEFAULT 0 NOT NULL;
