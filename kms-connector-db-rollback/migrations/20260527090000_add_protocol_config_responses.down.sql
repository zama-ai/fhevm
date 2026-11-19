-- Down for 20260527090000_add_protocol_config_responses.
--
-- Drops the two ProtocolConfig response tables with their triggers, and the
-- functions the triggers called.
--
-- Data impact: the stored responses are lost. A response not yet submitted by
-- the tx-sender is never submitted.

DROP TABLE new_kms_context_responses;
DROP TABLE epoch_result_responses;

DROP FUNCTION refresh_updated_at_new_kms_context_responses();
DROP FUNCTION refresh_updated_at_epoch_result_responses();
DROP FUNCTION notify_new_kms_context_response();
DROP FUNCTION notify_epoch_result_response();
DROP FUNCTION complete_new_kms_context();
DROP FUNCTION complete_new_kms_epoch();
