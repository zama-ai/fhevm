-- Create `already_sent` column for every requests tables.

ALTER TABLE public_decryption_requests ADD COLUMN already_sent BOOLEAN DEFAULT FALSE;
ALTER TABLE public_decryption_requests ALTER COLUMN already_sent SET NOT NULL;

ALTER TABLE user_decryption_requests ADD COLUMN already_sent BOOLEAN DEFAULT FALSE;
ALTER TABLE user_decryption_requests ALTER COLUMN already_sent SET NOT NULL;

ALTER TABLE prep_keygen_requests ADD COLUMN already_sent BOOLEAN DEFAULT FALSE;
ALTER TABLE prep_keygen_requests ALTER COLUMN already_sent SET NOT NULL;

ALTER TABLE keygen_requests ADD COLUMN already_sent BOOLEAN DEFAULT FALSE;
ALTER TABLE keygen_requests ALTER COLUMN already_sent SET NOT NULL;

ALTER TABLE crsgen_requests ADD COLUMN already_sent BOOLEAN DEFAULT FALSE;
ALTER TABLE crsgen_requests ALTER COLUMN already_sent SET NOT NULL;
