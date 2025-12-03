# FHEVM Metrics

This document lists and describes metrics supported by FHEVM services. Intention is for it to help operators monitor these services, configure alarms based on the metrics, and act on those in case of issues.

We also recommend alarm thresholds for each metric, where applicable. Thresholds suggested are conservative and can be adjusted based on the operator's environment and requirements.

Note that recommendations assume a smoke test that runs transactions/requests at a rate of approximately 1 per 30 seconds. These include verify proofs, FHE computation, ACL updates and decryptions.

## coprocessor

### transaction-sender

#### Metric Name: `coprocessor_txn_sender_verify_proof_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful verify or reject proof transactions in the transaction-sender.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `coprocessor_txn_sender_verify_proof_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed verify or reject proof transactions in the transaction-sender.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `coprocessor_txn_sender_add_ciphertext_material_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful add ciphertext material transactions in the transaction-sender.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `coprocessor_txn_sender_add_ciphertext_material_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed add ciphertext material transactions in the transaction-sender.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.
 
#### Metric Name: `coprocessor_txn_sender_allow_handle_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful allow handle transactions in the transaction-sender.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `coprocessor_txn_sender_allow_handle_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed allow handle transactions in the transaction-sender.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `coprocessor_allow_handle_unsent_gauge`
 - **Type**: Gauge
 - **Description**: Tracks the number of unsent allow handle transactions in the transaction-sender.
 - **Alarm**: If the gauge value exceeds a predefined threshold.
    - **Recommendation**: more than 100 unsent over 2 minutes, i.e. `min_over_time(gauge[2m]) > 100`.

#### Metric Name: `coprocessor_add_ciphertext_material_unsent_gauge`
 - **Type**: Gauge
 - **Description**: Tracks the number of unsent add ciphertext material transactions in the transaction-sender.
 - **Alarm**: If the gauge value exceeds a predefined threshold.
    - **Recommendation**: more than 100 unsent over 2 minutes, i.e. `min_over_time(gauge[2m]) > 100`.

#### Metric Name: `coprocessor_verify_proof_resp_unsent_txn_gauge`
 - **Type**: Gauge
 - **Description**: Tracks the number of unsent verify proof response transactions in the transaction-sender.
 - **Alarm**: If the gauge value exceeds a predefined threshold.
    - **Recommendation**: more than 100 unsent over 2 minutes, i.e. `min_over_time(gauge[2m]) > 100`.

#### Metric Name: `coprocessor_verify_proof_pending_gauge`
 - **Type**: Gauge
 - **Description**: Tracks the number of pending verify proofs (pending on the zkproof-worker).
 - **Alarm**: If the gauge value exceeds a predefined threshold.
    - **Recommendation**: more than 100 pending over 2 minutes, i.e. `min_over_time(gauge[2m]) > 100`.

### gw-listener

#### Metric Name: `coprocessor_gw_listener_verify_proof_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful verify proof request events in GW listener.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `coprocessor_gw_listener_verify_proof_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed verify proof request events in GW listener.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `coprocessor_gw_listener_get_block_num_fail_counter`
- **Type**: Counter
- **Description**: Counts the number of failed get block number requests in GW listener.
- **Alarm**: If the counter increases over a period of time.
   - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `coprocessor_gw_listener_get_logs_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful get logs requests in GW listener.
 - **Alarm**: If the counter is a flat line over a period of time.

#### Metric Name: `coprocessor_gw_listener_get_logs_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed get logs requests in GW listener.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `coprocessor_gw_listener_activate_crs_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful activate CRS requests in GW listener.
 - **Alarm**: N/A - no alarm needed as activate CRS is an infrequent event.

#### Metric Name: `coprocessor_gw_listener_activate_crs_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed activate CRS requests in GW listener.
 - **Alarm**: If the counter increases from 0. Activate CRS is an important event that should not fail.
    - **Recommendation**: alarm on any failures over a 1 minute period, i.e. `increase(counter[1m]) > 0`.

#### Metric Name: `coprocessor_gw_listener_crs_digest_mismatch_counter`
 - **Type**: Counter
 - **Description**: Counts the number of CRS digest mismatches in GW listener.
 - **Alarm**: If the counter increases from 0. CRS digest mismatch is not something that is supposed to happen in normal circumstances.
    - **Recommendation**: alarm on any failures over a 1 minute period, i.e. `increase(counter[1m]) > 0`.

#### Metric Name: `coprocessor_gw_listener_activate_key_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful activate key requests in GW listener.
 - **Alarm**: N/A - no alarm needed as activate key is an infrequent event.

#### Metric Name: `coprocessor_gw_listener_activate_key_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed activate key requests in GW listener.
 - **Alarm**: If the counter increases from 0. Activate key is an important event that should not fail.
    - **Recommendation**: alarm on any failures over a 1 minute period, i.e. `increase(counter[1m]) > 0`.

#### Metric Name: `coprocessor_gw_listener_key_digest_mismatch_counter`
 - **Type**: Counter
 - **Description**: Counts the number of key digest mismatches in GW listener.
 - **Alarm**: If the counter increases from 0. Key digest mismatch is not something that is supposed to happen in normal circumstances.
    - **Recommendation**: alarm on any failures over a 1 minute period, i.e. `increase(counter[1m]) > 0`.

### zkproof-worker

Metrics for zkproof-worker are to be added in future releases, if/when needed. Currently, the transaction-sender handles ZK proof related metrics, please see its section.

### sns-worker

#### Metric Name: `coprocessor_sns_worker_task_execute_success_counter`
 - **Type**: Counter
 - **Description**: Counts tasks executed by sns-worker successfully.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `coprocessor_sns_worker_task_execute_failure_counter`
 - **Type**: Counter
 - **Description**: Counts tasks errors in sns-worker.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 240 failures in 1 minute, i.e. `increase(counter[1m]) > 240`.

#### Metric Name: `coprocessor_sns_worker_aws_upload_success_counter`
 - **Type**: Counter
 - **Description**: Counts AWS uploads by sns-worker.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `coprocessor_sns_worker_aws_upload_failure_counter`
 - **Type**: Counter
 - **Description**: Counts AWS upload errors in sns-worker.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 240 failures in 1 minute, i.e. `increase(counter[1m]) > 240`.

#### Metric Name: `coprocessor_sns_worker_uncomplete_tasks_gauge`
 - **Type**: Gauge
 - **Description**: Tracks the number of uncomplete tasks in sns-worker.
 - **Alarm**: If the gauge value exceeds a predefined threshold.
    - **Recommendation**: more than 100 uncomplete over 2 minutes, i.e. `min_over_time(gauge[2m]) > 100`.

#### Metric Name: `coprocessor_sns_worker_uncomplete_aws_uploads_gauge`
 - **Type**: Gauge
 - **Description**: Tracks the number of uncomplete AWS uploads in sns-worker.
 - **Alarm**: If the gauge value exceeds a predefined threshold.
    - **Recommendation**: more than 100 uncomplete over 2 minutes, i.e. `min_over_time(gauge[2m]) > 100`.

### tfhe-worker

#### Metric Name: `coprocessor_worker_errors`
 - **Type**: Counter
 - **Description**: Counts TFHE worker errors.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 240 failures in 1 minute, i.e. `increase(counter[1m]) > 240`.

#### Metric Name: `coprocessor_work_items_polls`
 - **Type**: Counter
 - **Description**: Counts work items polled from the database.
 - **Alarm**: N/A - if work usually arrives via notifications, polling is expected to be low.

#### Metric Name: `coprocessor_work_items_notifications`
 - **Type**: Counter
 - **Description**: Counts the number of instant notifications for work items received from the DB.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `coprocessor_work_items_found`
 - **Type**: Counter
 - **Description**: Counts of work items queried from the DB.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `coprocessor_work_items_processed`
 - **Type**: Counter
 - **Description**: Counts of work items successfully processed and stored in the DB.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

## kms-connector

### gw-listener

#### Metric Name: `kms_connector_gw_listener_public_decryption_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful public decryption request events in GW listener.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_gw_listener_public_decryption_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed public decryption request events in GW listener.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `kms_connector_gw_listener_user_decryption_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful user decryption request events in GW listener.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_gw_listener_user_decryption_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed user decryption request events in GW listener.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `kms_connector_gw_listener_keys_generation_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful key and CRS generation request events in GW listener.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_gw_listener_keys_generation_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed key and CRS generation request events in GW listener.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `kms_connector_gw_listener_event_stored_counter`
 - **Type**: Counter
 - **Description**: Counts the number of events successfully stored in the DB by the GW listener.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_gw_listener_event_storage_errors`
 - **Type**: Counter
 - **Description**: Counts the number of errors encountered by the GW listener while storing events in the DB.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

### kms-worker

#### Metric Name: `kms_connector_worker_event_received_counter`
 - **Type**: Counter
 - **Description**: Counts the number of events received by the KMS worker.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_worker_event_received_errors`
 - **Type**: Counter
 - **Description**: Counts the number of errors encountered while listening for events in the KMS worker.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `kms_connector_worker_decryption_request_sent_counter`
 - **Type**: Counter
 - **Description**: Counts the number of decryption requests sent by the KMS worker to the KMS core.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_worker_decryption_request_sent_errors`
 - **Type**: Counter
 - **Description**: Counts the number of errors encountered by the KMS worker while sending decryption requests to the KMS core.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `kms_connector_worker_decryption_response_counter`
 - **Type**: Counter
 - **Description**: Counts the number of decryption responses received by the KMS worker from the KMS core.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_worker_decryption_response_errors`
 - **Type**: Counter
 - **Description**: Counts the number of errors encountered by the KMS worker while receiving decryption responses from the KMS core.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `kms_connector_worker_key_management_request_sent_counter`
 - **Type**: Counter
 - **Description**: Counts the number of key management requests sent by the KMS worker to the KMS core.
 - **Alarm**: N/A - key management requests are infrequent events.

#### Metric Name: `kms_connector_worker_key_management_request_sent_errors`
 - **Type**: Counter
 - **Description**: Counts the number of errors encountered by the KMS worker while sending key management requests to the KMS core.
 - **Alarm**: If the counter increases from 0. Key management is an important event that should not fail.
    - **Recommendation**: alarm on any failures over a 1 minute period, i.e. `increase(counter[1m]) > 0`.

#### Metric Name: `kms_connector_worker_key_management_response_counter`
 - **Type**: Counter
 - **Description**: Counts the number of key management responses received by the KMS worker from the KMS core.
 - **Alarm**: N/A - key management responses are infrequent events.

#### Metric Name: `kms_connector_worker_key_management_response_errors`
 - **Type**: Counter
 - **Description**: Counts the number of errors encountered by the KMS worker while receiving key management responses from the KMS core.
 - **Alarm**: If the counter increases from 0. Key management is an important event that should not fail.
    - **Recommendation**: alarm on any failures over a 1 minute period, i.e. `increase(counter[1m]) > 0`.

#### Metric Name: `kms_connector_worker_s3_ciphertext_retrieval_counter`
 - **Type**: Counter
 - **Description**: Counts the number of ciphertexts retrieved by the KMS worker from S3.
 - **Alarm**: N/A - key management events are infrequent.

#### Metric Name: `kms_connector_worker_s3_ciphertext_retrieval_errors`
 - **Type**: Counter
 - **Description**: Counts the number of errors encountered by the KMS worker while retrieving ciphertexts from S3.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

### tx-sender

#### Metric Name: `kms_connector_tx_sender_response_received_counter`
 - **Type**: Counter
 - **Description**: Counts the number of responses received by the TX sender.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_tx_sender_response_received_errors`
 - **Type**: Counter
 - **Description**: Counts the number of errors encountered by the TX sender while listening for responses.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `kms_connector_tx_sender_public_decryption_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful public decryption response transactions sent to the Gateway by the TX sender.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_tx_sender_public_decryption_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed public decryption response transactions in the TX sender.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `kms_connector_tx_sender_user_decryption_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful user decryption response transactions sent to the Gateway by the TX sender.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_tx_sender_user_decryption_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed user decryption response transactions in the TX sender.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.

#### Metric Name: `kms_connector_tx_sender_keys_generation_success_counter`
 - **Type**: Counter
 - **Description**: Counts the number of successful key and CRS generation response transactions sent to the Gateway by the TX sender.
 - **Alarm**: If the counter is a flat line over a period of time.
    - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_tx_sender_keys_generation_fail_counter`
 - **Type**: Counter
 - **Description**: Counts the number of failed key and CRS generation response transactions in the TX sender.
 - **Alarm**: If the counter increases over a period of time.
    - **Recommendation**: more than 60 failures in 1 minute, i.e. `increase(counter[1m]) > 60`.
