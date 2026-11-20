-- Down for 20261105000000_rfc037_signed_confirmations.
--
-- Drops the two destruction response tables with their triggers and functions,
-- and the `signature` / `extra_data` columns the up added to
-- `new_kms_context_responses` and `epoch_result_responses`.
--
-- Data impact: the signed confirmations stored by the kms-worker are lost. The
-- up also marked the pending responses as `failed`; that is not undone, since
-- nothing records which rows it changed.

DROP TABLE kms_context_destruction_responses;
DROP TABLE kms_epoch_destruction_responses;

DROP FUNCTION refresh_updated_at_kms_context_destruction_responses();
DROP FUNCTION refresh_updated_at_kms_epoch_destruction_responses();
DROP FUNCTION notify_kms_context_destruction_response();
DROP FUNCTION notify_kms_epoch_destruction_response();
DROP FUNCTION complete_kms_context_destroyed();
DROP FUNCTION complete_kms_epoch_destroyed();

ALTER TABLE new_kms_context_responses
    DROP COLUMN signature,
    DROP COLUMN extra_data;

ALTER TABLE epoch_result_responses
    DROP COLUMN signature,
    DROP COLUMN extra_data;
