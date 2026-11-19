-- Down for 20250604141515_decryption_response.
--
-- Drops the two decryption response tables with their triggers, and the
-- functions that removed a request when its response was inserted. The stored
-- responses are lost.

DROP TABLE public_decryption_responses;
DROP TABLE user_decryption_responses;

DROP FUNCTION delete_from_public_decryption_requests();
DROP FUNCTION delete_from_user_decryption_requests();
