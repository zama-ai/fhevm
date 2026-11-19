-- Down for 20261103000000_pad_decryption.
--
-- Drops the pad decryption tables with their indexes and triggers, and the
-- functions the triggers called.
--
-- Data impact: every pad decryption request and response is lost. They are
-- HTTP-originated, so the callers get no answer and have to retry against a
-- version that serves them.

DROP TABLE pad_decryption_responses;
DROP TABLE pad_decryption_requests;

DROP FUNCTION refresh_updated_at_pad_decryption_requests();
DROP FUNCTION notify_pad_decryption_request();
DROP FUNCTION refresh_updated_at_pad_decryption_responses();
DROP FUNCTION notify_http_pad_decryption_response();
