-- Down for 20250606125742_tx_sender_notify.
--
-- Drops the two triggers that notify the tx-sender of new decryption
-- responses, and the functions they called. No data is involved.

DROP TRIGGER trigger_from_public_decryption_responses_insertions ON public_decryption_responses;
DROP TRIGGER trigger_from_user_decryption_responses_insertions ON user_decryption_responses;

DROP FUNCTION notify_public_decryption_response();
DROP FUNCTION notify_user_decryption_response();
