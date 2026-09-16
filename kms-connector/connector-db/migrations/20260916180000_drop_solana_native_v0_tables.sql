-- Remove the unused Solana native-v0 decrypt tables. Live user-decrypt uses
-- `user_decryption_requests.solana_request`. No Rust reader or writer remains
-- for these names. Keep the earlier CREATE migrations; this is a forward drop
-- so databases that already applied them still migrate.
--
-- zama-ai/fhevm-internal#1628 Stream B.

DROP TABLE IF EXISTS solana_native_decryption_responses_v0 CASCADE;
DROP TABLE IF EXISTS solana_native_decryption_requests_v0 CASCADE;
DROP TABLE IF EXISTS solana_native_decryption_replay_v0 CASCADE;
DROP TABLE IF EXISTS solana_user_decrypt_responses_v0 CASCADE;

DROP FUNCTION IF EXISTS refresh_updated_at_solana_native_decryption_requests_v0();
DROP FUNCTION IF EXISTS refresh_updated_at_solana_native_decryption_responses_v0();
DROP FUNCTION IF EXISTS complete_solana_native_decryption_request_v0();
DROP FUNCTION IF EXISTS notify_solana_native_decryption_request_v0();
DROP FUNCTION IF EXISTS notify_solana_native_decryption_response_v0();
