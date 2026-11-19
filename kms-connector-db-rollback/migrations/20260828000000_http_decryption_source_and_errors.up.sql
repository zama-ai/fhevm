-- RFC 033 (Direct HTTP Decryption Endpoint) decryption tables update.
--
--   * `source` discriminator on the decryption request and response tables: tells `tx-sender` to
--     skip HTTP-originated work, and drives the HTTP notify triggers below.
--
--   * Nullable `error_code` / `error_details` on the response tables, so worker-side rejections
--     become response rows.
--
--   * A response row is either a payload or an error, never both: payload columns become
--     nullable, a CHECK enforces the exclusive-or.
--
--   * Row-level `pg_notify` triggers on HTTP-originated response inserts, carrying the
--     hex-encoded `decryption_id` for the Connector endpoint's waiting-connection lookup.

CREATE TYPE request_source AS ENUM (
    'onchain',
    'http'
);

--------------------------------------------------------------------------------------------------
--                             `source` on decryption request tables                             --
--------------------------------------------------------------------------------------------------

ALTER TABLE public_decryption_requests ADD COLUMN source request_source DEFAULT 'onchain' NOT NULL;
ALTER TABLE user_decryption_requests ADD COLUMN source request_source DEFAULT 'onchain' NOT NULL;

-- The Connector endpoint only submits RFC016 unified user decryption requests, so a legacy-shaped
-- row (NULL `signature`) can never be HTTP-sourced.
ALTER TABLE user_decryption_requests ADD CONSTRAINT user_decryption_requests_http_is_unified
    CHECK (NOT (source = 'http' AND signature IS NULL));

--------------------------------------------------------------------------------------------------
--                    `source` and error columns on decryption response tables                   --
--------------------------------------------------------------------------------------------------

ALTER TABLE public_decryption_responses ADD COLUMN source request_source DEFAULT 'onchain' NOT NULL;
ALTER TABLE user_decryption_responses ADD COLUMN source request_source DEFAULT 'onchain' NOT NULL;

ALTER TABLE public_decryption_responses ADD COLUMN error_code TEXT;
ALTER TABLE public_decryption_responses ADD COLUMN error_details TEXT;
ALTER TABLE user_decryption_responses ADD COLUMN error_code TEXT;
ALTER TABLE user_decryption_responses ADD COLUMN error_details TEXT;

ALTER TABLE public_decryption_responses ALTER COLUMN decrypted_result DROP NOT NULL;
ALTER TABLE public_decryption_responses ALTER COLUMN signature DROP NOT NULL;
ALTER TABLE user_decryption_responses ALTER COLUMN user_decrypted_shares DROP NOT NULL;
ALTER TABLE user_decryption_responses ALTER COLUMN signature DROP NOT NULL;

ALTER TABLE public_decryption_responses ADD CONSTRAINT public_decryption_responses_payload_or_error
    CHECK (
        (error_code IS NULL AND error_details IS NULL AND decrypted_result IS NOT NULL AND signature IS NOT NULL)
        OR
        (error_code IS NOT NULL AND decrypted_result IS NULL AND signature IS NULL)
    );

ALTER TABLE user_decryption_responses ADD CONSTRAINT user_decryption_responses_payload_or_error
    CHECK (
        (error_code IS NULL AND error_details IS NULL AND user_decrypted_shares IS NOT NULL AND signature IS NOT NULL)
        OR
        (error_code IS NOT NULL AND user_decrypted_shares IS NULL AND signature IS NULL)
    );

--------------------------------------------------------------------------------------------------
--            Restrict the tx-sender notify triggers to onchain-originated responses            --
--------------------------------------------------------------------------------------------------

DROP TRIGGER IF EXISTS trigger_from_public_decryption_responses_insertions ON public_decryption_responses;
DROP TRIGGER IF EXISTS trigger_from_user_decryption_responses_insertions ON user_decryption_responses;

CREATE TRIGGER trigger_from_public_decryption_responses_insertions
    AFTER INSERT
    ON public_decryption_responses
    FOR EACH ROW
    WHEN (NEW.source = 'onchain')
    EXECUTE FUNCTION notify_public_decryption_response();

CREATE TRIGGER trigger_from_user_decryption_responses_insertions
    AFTER INSERT
    ON user_decryption_responses
    FOR EACH ROW
    WHEN (NEW.source = 'onchain')
    EXECUTE FUNCTION notify_user_decryption_response();

--------------------------------------------------------------------------------------------------
--              Notify the Connector endpoint on HTTP-originated response upserts               --
--------------------------------------------------------------------------------------------------

-- Unlike the payload-less tx-sender notifications, these carry the decryption_id of each
-- individual response the endpoint must correlate.
-- Fire on UPDATE too: a retried request overrides its previous error row.
CREATE OR REPLACE FUNCTION notify_http_public_decryption_response()
    RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('http_public_decryption_response_available', encode(NEW.decryption_id, 'hex'));
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION notify_http_user_decryption_response()
    RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify('http_user_decryption_response_available', encode(NEW.decryption_id, 'hex'));
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trigger_from_http_public_decryption_responses_upserts
    AFTER INSERT OR UPDATE
    ON public_decryption_responses
    FOR EACH ROW
    WHEN (NEW.source = 'http')
    EXECUTE FUNCTION notify_http_public_decryption_response();

CREATE OR REPLACE TRIGGER trigger_from_http_user_decryption_responses_upserts
    AFTER INSERT OR UPDATE
    ON user_decryption_responses
    FOR EACH ROW
    WHEN (NEW.source = 'http')
    EXECUTE FUNCTION notify_http_user_decryption_response();

--------------------------------------------------------------------------------------------------
--            Restrict the decryption request completion triggers to payload responses          --
--------------------------------------------------------------------------------------------------

-- The kms-worker now updates the request status itself, but these triggers are kept so an old
-- kms-worker still completes its requests in case of rollback. They are restricted to payload
-- rows because the new error rows must not mark requests as `completed`.
-- TODO(https://github.com/zama-ai/fhevm-internal/issues/1961): drop the triggers for next release.
DROP TRIGGER IF EXISTS complete_public_decryption_request_on_response_insert ON public_decryption_responses;
CREATE TRIGGER complete_public_decryption_request_on_response_insert
    AFTER INSERT
    ON public_decryption_responses
    FOR EACH ROW
    WHEN (NEW.decrypted_result IS NOT NULL)
    EXECUTE FUNCTION complete_public_decryption_request();

DROP TRIGGER IF EXISTS complete_user_decryption_requests_on_response_insert ON user_decryption_responses;
CREATE TRIGGER complete_user_decryption_requests_on_response_insert
    AFTER INSERT
    ON user_decryption_responses
    FOR EACH ROW
    WHEN (NEW.user_decrypted_shares IS NOT NULL)
    EXECUTE FUNCTION complete_user_decryption_request();
