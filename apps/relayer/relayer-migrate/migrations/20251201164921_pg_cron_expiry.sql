-- Add migration script here
-- 1. Index for User Decrypt Retention
CREATE INDEX idx_user_decrypt_req_created_at ON user_decrypt_req (created_at);
CREATE INDEX idx_user_decrypt_share_created_at ON user_decrypt_share (created_at);

-- 2. Index for Public Decrypt Retention
CREATE INDEX idx_public_decrypt_req_created_at ON public_decrypt_req (created_at);

-- 3. Index for Input Proof Retention
CREATE INDEX idx_input_proof_req_created_at ON input_proof_req (created_at);

-- JOB 4: Clean Public Decrypt Requests (Older than 365 days, Runs Daily at 02:00 PM france winter timezone and 03:00 PM france summer timezone)
SELECT cron.schedule(
    'clean_public_decrypt_job',
    '0 13 * * *', 
    $$
    DELETE FROM public_decrypt_req 
    WHERE created_at < NOW() - INTERVAL '365 days';
    $$
);

-- JOB 5: Clean User Decrypt Requests & Shares (Older than 24h, Runs every 12h at 7am and 7pm france winter timezone and 8am and 8pm france summer timezone)
-- We delete shares first, then requests, to maintain logical consistency 
-- (though without FK constraints, order technically doesn't strictly matter for DB integrity)
SELECT cron.schedule(
    'clean_user_decrypt_job',
    '0 6,18 * * *', 
    $$
    BEGIN;
        DELETE FROM user_decrypt_share WHERE created_at < NOW() - INTERVAL '24 hours';
        DELETE FROM user_decrypt_req WHERE created_at < NOW() - INTERVAL '24 hours';
    COMMIT;
    $$
);

-- JOB 6: Clean Input Proofs (Older than 24h, Runs every 12h at 7:30am and 7:30pm france winter timezone and 8:30am and 8:30pm france summer timezone)
SELECT cron.schedule(
    'clean_input_proof_job',
    '30 6,18 * * *', 
    $$
    DELETE FROM input_proof_req 
    WHERE created_at < NOW() - INTERVAL '24 hours';
    $$
);