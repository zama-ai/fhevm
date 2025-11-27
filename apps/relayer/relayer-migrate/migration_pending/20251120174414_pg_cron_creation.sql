-- NOTE: If not able to install pg_cron extension in the DB (or by default on RDS), we can comment this migration and use the timeout_repo.rs alternative solution instead.
-- WARNING: MUST INSTALL pg_cron extension in the database first...
-- In docker-compose.yml for the Postgres service (dev mode), add the following lines:
--       # REQUIRED: Tell pg_cron which DB to run on
--   POSTGRES_HOST_AUTH_METHOD: trust
-- command: ["postgres", "-c", "shared_preload_libraries=pg_cron", "-c", "cron.database_name=your_db_name"]
-- Otherwise there is no way to schedule jobs and we can comment cron and use internals

-- CREATE THE pg_cron jobs for handling timeouts.
-- Enable the extension (if not already enabled)

-- CREATE EXTENSION IF NOT EXISTS pg_cron;

-- -- JOB 1: User Decrypt Request Timeout
-- SELECT cron.schedule(
--     'user_decrypt_timeout_job', -- Unique Job Name
--     '* * * * *',                -- Every minute
--     $$
--     UPDATE user_decrypt_req
--     SET req_status = 'timed_out', 
--         err_reason = 'response timed out'
--     WHERE req_status = 'receipt_received' 
--       AND updated_at < NOW() - INTERVAL '30 minutes';
--     $$
-- );

-- -- JOB 2: Public Decrypt Request Timeout
-- SELECT cron.schedule(
--     'public_decrypt_timeout_job',
--     '* * * * *',
--     $$
--     UPDATE public_decrypt_req
--     SET req_status = 'timed_out', 
--         err_reason = 'response timed out'
--     WHERE req_status = 'receipt_received' 
--       AND updated_at < NOW() - INTERVAL '30 minutes';
--     $$
-- );

-- -- JOB 3: Input Proof Request Timeout
-- SELECT cron.schedule(
--     'input_proof_timeout_job',
--     '* * * * *',
--     $$
--     UPDATE input_proof_req
--     SET req_status = 'timed_out', 
--         err_reason = 'response timed out'
--     WHERE req_status = 'receipt_received' 
--       AND updated_at < NOW() - INTERVAL '30 minutes';
--     $$
-- );


-- TEST REQUESTS for the jobs:
-- Check status of executed jobs
-- SELECT * FROM cron.job_run_details ORDER BY start_time DESC LIMIT 10;

-- List currently scheduled jobs
-- SELECT * FROM cron.job;

-- To unschedule/delete a job if you made a mistake:
-- SELECT cron.unschedule('user_decrypt_timeout_job');