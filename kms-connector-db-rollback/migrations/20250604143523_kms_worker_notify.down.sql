-- Down for 20250604143523_kms_worker_notify.
--
-- Drops the seven triggers that notify the kms-worker of new requests, and the
-- functions they called. No data is involved.

DROP TRIGGER trigger_from_public_decryption_requests_insertions ON public_decryption_requests;
DROP TRIGGER trigger_from_user_decryption_requests_insertions ON user_decryption_requests;
DROP TRIGGER trigger_from_preprocess_keygen_requests_insertions ON preprocess_keygen_requests;
DROP TRIGGER trigger_from_preprocess_kskgen_requests_insertions ON preprocess_kskgen_requests;
DROP TRIGGER trigger_from_keygen_requests_insertions ON keygen_requests;
DROP TRIGGER trigger_from_kskgen_requests_insertions ON kskgen_requests;
DROP TRIGGER trigger_from_crsgen_requests_insertions ON crsgen_requests;

DROP FUNCTION notify_public_decryption_request();
DROP FUNCTION notify_user_decryption_request();
DROP FUNCTION notify_preprocess_keygen_request();
DROP FUNCTION notify_preprocess_kskgen_request();
DROP FUNCTION notify_keygen_request();
DROP FUNCTION notify_kskgen_request();
DROP FUNCTION notify_crsgen_request();
