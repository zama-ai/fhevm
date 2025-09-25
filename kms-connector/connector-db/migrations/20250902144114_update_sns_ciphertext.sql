ALTER TYPE sns_ciphertext_material DROP ATTRIBUTE coprocessor_tx_sender_addresses CASCADE;
ALTER TYPE sns_ciphertext_material ADD ATTRIBUTE storage_urls TEXT[] CASCADE;
