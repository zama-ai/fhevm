-- Down for 20260828000000_http_decryption_source_and_errors.
--
-- Removes the `source` discriminator and the error columns from the decryption
-- tables, makes the response payload columns NOT NULL again, drops the HTTP
-- notify triggers and restores the tx-sender notify and request completion
-- triggers as they were before (statement-level, unconditional).
--
-- Rows that only exist because of what this migration added:
--   * HTTP-originated responses (`source = 'http'`). They are read by the
--     Connector endpoint, which the previous version does not have. Without
--     the column its tx-sender would treat them as on-chain work and try to
--     submit them to the Gateway. They are deleted.
--   * Error responses (`error_code IS NOT NULL`). They have no payload and
--     would violate the restored NOT NULL constraints. They are deleted; the
--     request they answer keeps the status the kms-worker gave it.
--   * Finished HTTP-originated requests. They are never read again and stay.
--   * In-flight HTTP-originated requests (`pending` or `under_process`).
--     Without the column the previous kms-worker would process them and its
--     tx-sender would submit the result to the Gateway, which never asked for
--     it. This fails, without changing anything, while such a request exists.
--
-- Forced rollback (`migrate.sh rollback <target> --force`, which sets the
-- `kms_connector_db_rollback.force` session setting): the in-flight
-- HTTP-originated requests are deleted and the revert goes ahead.

DO $$
DECLARE
    in_flight bigint;
    forced boolean;
BEGIN
    forced := coalesce(current_setting('kms_connector_db_rollback.force', true), '') = 'on';

    SELECT (SELECT count(*) FROM public_decryption_requests
            WHERE source = 'http' AND status IN ('pending', 'under_process'))
         + (SELECT count(*) FROM user_decryption_requests
            WHERE source = 'http' AND status IN ('pending', 'under_process'))
    INTO in_flight;

    IF in_flight > 0 AND NOT forced THEN
        RAISE EXCEPTION
            'Cannot remove the decryption source column: % in-flight HTTP-originated requests would be processed as on-chain ones. Let them finish, or force the rollback to delete them.',
            in_flight;
    END IF;

    IF in_flight > 0 THEN
        DELETE FROM public_decryption_requests
        WHERE source = 'http' AND status IN ('pending', 'under_process');
        DELETE FROM user_decryption_requests
        WHERE source = 'http' AND status IN ('pending', 'under_process');

        RAISE NOTICE 'Forced rollback: deleted % in-flight HTTP-originated requests.', in_flight;
    END IF;
END
$$;

DELETE FROM public_decryption_responses WHERE source = 'http' OR error_code IS NOT NULL;
DELETE FROM user_decryption_responses WHERE source = 'http' OR error_code IS NOT NULL;

-- Request completion triggers: back to unconditional, as created by
-- 20251208161022_implem_garbage_collection.
DROP TRIGGER complete_public_decryption_request_on_response_insert ON public_decryption_responses;
CREATE TRIGGER complete_public_decryption_request_on_response_insert
AFTER INSERT ON public_decryption_responses
FOR EACH ROW
EXECUTE FUNCTION complete_public_decryption_request();

DROP TRIGGER complete_user_decryption_requests_on_response_insert ON user_decryption_responses;
CREATE TRIGGER complete_user_decryption_requests_on_response_insert
AFTER INSERT ON user_decryption_responses
FOR EACH ROW
EXECUTE FUNCTION complete_user_decryption_request();

-- HTTP notify triggers and their functions.
DROP TRIGGER trigger_from_http_public_decryption_responses_upserts ON public_decryption_responses;
DROP TRIGGER trigger_from_http_user_decryption_responses_upserts ON user_decryption_responses;
DROP FUNCTION notify_http_public_decryption_response();
DROP FUNCTION notify_http_user_decryption_response();

-- tx-sender notify triggers: back to statement-level, as created by
-- 20250606125742_tx_sender_notify.
DROP TRIGGER trigger_from_public_decryption_responses_insertions ON public_decryption_responses;
CREATE TRIGGER trigger_from_public_decryption_responses_insertions
    AFTER INSERT
    ON public_decryption_responses
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_public_decryption_response();

DROP TRIGGER trigger_from_user_decryption_responses_insertions ON user_decryption_responses;
CREATE TRIGGER trigger_from_user_decryption_responses_insertions
    AFTER INSERT
    ON user_decryption_responses
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_user_decryption_response();

-- Response tables: payload is mandatory again, error and source columns go.
ALTER TABLE public_decryption_responses DROP CONSTRAINT public_decryption_responses_payload_or_error;
ALTER TABLE user_decryption_responses DROP CONSTRAINT user_decryption_responses_payload_or_error;

ALTER TABLE public_decryption_responses ALTER COLUMN decrypted_result SET NOT NULL;
ALTER TABLE public_decryption_responses ALTER COLUMN signature SET NOT NULL;
ALTER TABLE user_decryption_responses ALTER COLUMN user_decrypted_shares SET NOT NULL;
ALTER TABLE user_decryption_responses ALTER COLUMN signature SET NOT NULL;

ALTER TABLE public_decryption_responses
    DROP COLUMN error_code,
    DROP COLUMN error_details,
    DROP COLUMN source;
ALTER TABLE user_decryption_responses
    DROP COLUMN error_code,
    DROP COLUMN error_details,
    DROP COLUMN source;

-- Request tables.
ALTER TABLE user_decryption_requests DROP CONSTRAINT user_decryption_requests_http_is_unified;
ALTER TABLE public_decryption_requests DROP COLUMN source;
ALTER TABLE user_decryption_requests DROP COLUMN source;

DROP TYPE request_source;
