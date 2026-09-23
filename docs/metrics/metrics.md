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

#### Metric Name: `coprocessor_gw_listener_drift_detected_counter`
 - **Type**: Counter
 - **Description**: Number of handles where coprocessor digests diverged. Does not discriminate whether divergence comes from the local coprocessor or another coprocessor in the network.

#### Metric Name: `coprocessor_gw_listener_consensus_timeout_counter`
 - **Type**: Counter
 - **Description**: Number of handles that timed out without a consensus event. This includes both handles where no consensus was ever observed and handles where all expected coprocessors submitted but the gateway never emitted a consensus event.

#### Metric Name: `coprocessor_gw_listener_missing_submission_counter`
 - **Type**: Counter
 - **Description**: Number of handles where consensus was reached but some expected coprocessors never submitted their ciphertext material before the post-consensus grace period expired.

#### Metric Name: `coprocessor_gw_listener_consensus_latency_blocks`
 - **Type**: Histogram
 - **Description**: Block distance between the first observed submission and the consensus event for a handle. Diagnostic metric for understanding on-chain latency; timeouts are wall-clock based and configured via `--drift-no-consensus-timeout`. Bucket boundaries: 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144.

#### Metric Name: `coprocessor_gw_listener_post_consensus_completion_blocks`
 - **Type**: Histogram
 - **Description**: Block distance between the consensus event and seeing all expected submissions for a handle. Diagnostic metric for understanding on-chain completion latency; the grace window is wall-clock based and configured via `--drift-post-consensus-grace`. Bucket boundaries: 0, 1, 2, 3, 5, 8, 13, 21, 34.

#### Metric Name: `coprocessor_drift_revert_signal_created_counter`
 - **Type**: Counter (labeled by `host_chain_id`)
 - **Description**: Number of drift-revert signals created, per host chain. Each signal represents one detected consensus drift that triggered the auto-recovery mechanism. Emitted by gw-listener only.
 - **Alarm**: Any non-zero increase is unusual — drift should be rare.
    - **Recommendation**: alarm on `increase(counter[5m]) > 0`.

#### Metric Name: `coprocessor_drift_revert_success_counter`
 - **Type**: Counter (labeled by `host_chain_id`)
 - **Description**: Number of drift reverts that completed successfully per host chain (SQL ran to completion and signal marked Done). Emitted by gw-listener only.

#### Metric Name: `coprocessor_drift_revert_failure_counter`
 - **Type**: Counter (labeled by `host_chain_id`)
 - **Description**: Number of drift reverts that failed during SQL execution per host chain (signal marked Failed). Recovery did not complete and operator intervention may be required. Emitted by gw-listener only.
 - **Alarm**: Any non-zero increase.
    - **Recommendation**: alarm on `increase(counter[1m]) > 0`.

#### Metric Name: `coprocessor_drift_revert_too_many_attempts_counter`
 - **Type**: Counter (labeled by `host_chain_id`)
 - **Description**: Number of times the revert runner refused to revert because too many successful reverts already happened in the recent window for this host chain. Indicates a deterministic loop where reverts succeed but drift keeps recurring (e.g. a tfhe-worker bug). The signal is marked Failed and the service refuses to start until an operator intervenes. Emitted by gw-listener only.
 - **Alarm**: Any non-zero increase.
    - **Recommendation**: alarm on `increase(counter[1m]) > 0`.

### solana-host-listener

The listener resumes from its checkpoint only while the Yellowstone provider can still replay it, about 24 hours for a hosted provider. Past that window it stops and needs manual recovery, so the lag alarm fires within minutes. A fatal ingestion error exits the process, so it shows up as container restarts, not as a metric.

#### Metric Name: `coprocessor_solana_host_listener_applied_block_timestamp_seconds`
 - **Type**: Gauge (labeled by `host_chain_id`)
 - **Description**: Unix time the cluster assigned to the last block the listener committed. `time()` minus this value is the ingestion lag in seconds. It grows both when the stream stalls and when the listener applies blocks slower than the cluster produces them. After a restart the series is missing until the listener commits a block.
 - **Alarm**: If the lag stays high, or the series is missing (the listener is down or not applying blocks).
    - **Recommendation**: more than 2 minutes behind for 2 minutes, i.e. `min_over_time((time() - gauge)[2m:]) > 120`, and `absent_over_time(gauge[5m])`.

#### Metric Name: `coprocessor_solana_host_listener_applied_slot`
 - **Type**: Gauge (labeled by `host_chain_id`)
 - **Description**: Slot of the last block the listener committed with its compute rows, leaves and checkpoint. On a restart it starts at the resumed checkpoint.

#### Metric Name: `coprocessor_solana_host_listener_confirmed_slot`
 - **Type**: Gauge (labeled by `host_chain_id`)
 - **Description**: The cluster's confirmed slot, polled over RPC every 10 seconds. Minus `applied_slot`, it is the lag in slots, which compares directly with the provider's replay window, including while a restarted listener has not yet applied a block. Between polls it reads up to about 25 slots low, so a healthy lag hovers around zero and can dip below it.
 - **Alarm**: If the RPC poll stops updating the gauge. Chart the slot lag against the provider's window rather than paging on it; the time lag above pages first.
    - **Recommendation**: `changes(confirmed_slot[5m]) == 0`.

#### Metric Name: `coprocessor_solana_host_listener_reconnects_total`
 - **Type**: Counter (labeled by `host_chain_id`)
 - **Description**: gRPC subscriptions the listener dropped and reopened from its checkpoint: a stream idle for 30 seconds, closed by the server, a transport error, or a retryable ingest failure.
 - **Alarm**: If the counter increases repeatedly.
    - **Recommendation**: more than 3 reconnects in 10 minutes, i.e. `increase(counter[10m]) > 3`.

#### Metric Name: `coprocessor_solana_host_listener_handle_check_failures_total`
 - **Type**: Counter (labeled by `host_chain_id`)
 - **Description**: Steps whose emitted result handle did not match the handle the listener re-derived. Each one is held back: its computation and every computation that depends on it end as errors, while the rest of the block is ingested. It means the listener's derivation or step decoding is wrong. The log line `solana handle check failed` names the slot, signature, step and both handles; the repair is in the host-listener README.
 - **Alarm**: Any increase. Page on it.
    - **Recommendation**: `increase(counter[5m]) > 0`.

#### Container restarts
 - **Description**: A fatal ingestion error, such as a block whose ancestry does not match the checkpoint or a replay the provider can no longer serve, exits the listener, which then resumes from its checkpoint. A restart that catches up quickly never trips the lag alarm, so restarts need their own alarm.
 - **Alarm**: Any restart.
    - **Recommendation**: `increase(kube_pod_container_status_restarts_total{container="solana-host-listener"}[15m]) > 0`.

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

#### Metric Name: `kms_connector_gw_listener_event_received_counter`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: can be used to filter by event type (public_decryption_request, user_decryption_request, crsgen_request, ...).
 - **Description**: Counts the number of events received by the GW listener.
 - **Alarm**: If the counter is a flat line over a period of time, only for `event_type` `public_decryption_request` and `user_decryption_request`.
   - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter{event_type="..."}[1m]) == 0`.

#### Metric Name: `kms_connector_gw_listener_event_listening_errors`
 - **Type**: Counter
 - **Labels**:
   - `contract`: can be used to filter by contract (decryption, kmsgeneration).
 - **Description**: Counts the number of errors encountered by the GW listener while listening for events.
 - **Alarm**: If the counter increases over a period of time.
   - **Recommendation**: more than 60 failures in 1 minute, i.e. `sum(increase(counter[1m])) > 60`.

### kms-worker

#### Metric Name: `kms_connector_worker_event_received_counter`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: see [description](#metric-name-kms_connector_gw_listener_event_received_counter)
 - **Description**: Counts the number of events received by the KMS worker.
 - **Alarm**: If the counter is a flat line over a period of time, only for `event_type` `public_decryption_request` and `user_decryption_request`.
   - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter{event_type="..."}[1m]) == 0`.

#### Metric Name: `kms_connector_worker_event_received_errors`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: see [description](#metric-name-kms_connector_gw_listener_event_received_counter)
 - **Description**: Counts the number of errors encountered while listening for events in the KMS worker.
 - **Alarm**: If the counter increases over a period of time.
   - **Recommendation**: more than 60 failures in 1 minute, i.e. `sum(increase(counter[1m])) > 60`.

#### Metric Name: `kms_connector_worker_grpc_request_sent_counter`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: see [description](#metric-name-kms_connector_gw_listener_event_received_counter)
 - **Description**: Number of successful GRPC requests sent by the KMS worker to the KMS Core,
 - **Alarm**: If the counter is a flat line over a period of time, only for `event_type` `public_decryption_request` and `user_decryption_request`.
   - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter{event_type="..."}[1m]) == 0`.

#### Metric Name: `kms_connector_worker_grpc_request_sent_errors`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: see [description](#metric-name-kms_connector_gw_listener_event_received_counter)
 - **Description**: Counts the number of errors encountered by the KMS worker while sending grpc requests to the KMS Core.
 - **Alarm**: If the counter increases over a period of time.
   - **Recommendation**: more than 60 failures in 1 minute, i.e. `sum(increase(counter[1m])) > 60`.

#### Metric Name: `kms_connector_worker_grpc_response_polled_counter`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: see [description](#metric-name-kms_connector_gw_listener_event_received_counter)
 - **Description**: Counts the number of responses successfully polled from the KMS Core via GRPC.
 - **Alarm**: If the counter is a flat line over a period of time, only for `event_type` `public_decryption_request` and `user_decryption_request`.
   - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter{event_type="..."}[1m]) == 0`.

#### Metric Name: `kms_connector_worker_grpc_response_polled_errors`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: see [description](#metric-name-kms_connector_gw_listener_event_received_counter)
 - **Description**: Counts the number of errors encountered by the KMS worker while polling responses from the KMS Core.
 - **Alarm**: If the counter increases over a period of time.
   - **Recommendation**: more than 60 failures in 1 minute, i.e. `sum(increase(counter[1m])) > 60`.

#### Metric Name: `kms_connector_worker_s3_ciphertext_retrieval_counter`
 - **Type**: Counter
 - **Description**: Counts the number of ciphertexts retrieved by the KMS worker from S3.
 - **Alarm**: If the counter is a flat line over a period of time.
   - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter[1m]) == 0`.

#### Metric Name: `kms_connector_worker_s3_ciphertext_retrieval_errors`
 - **Type**: Counter
 - **Description**: Counts the number of errors encountered by the KMS worker while retrieving ciphertexts from S3.
 - **Alarm**: If the counter increases over a period of time.
   - **Recommendation**: more than 60 failures in 1 minute, i.e. `sum(increase(counter[1m])) > 60`.

#### Metric Name: `kms_connector_worker_request_check_errors`
 - **Type**: Counter
 - **Labels**:
   - `check_type`: the family of pre-flight check that rejected the request:
     - `acl`: ACL authorization checks and errors that prevent the ACL check from running (malformed handles, missing contract config...).
     - `signature`: RFC-012/016 signature & request-validity checks (EIP-712/ERC-1271 signature, validity window, signature invalidation).
     - `copro_consensus`: RFC-023 off-chain ciphertext-attestation consensus check.
     - `kms_context`: KMS context/epoch validity check.
     - `network`: network error (on-chain call or DB query) encountered while running any of the above checks.
 - **Description**: Counts request pre-flight check failures. Mostly decryption checks (ACL, signature, copro consensus), but the `kms_context` family also covers key-management requests.
 - **Alarm**: If the counter increases over a period of time.
   - **Recommendation**: more than 60 failures in 1 minute, i.e. `sum(increase(counter[1m])) > 60`.

#### Metric Name: `kms_connector_worker_decryption_latency_seconds`
 - **Type**: Histogram
 - **Labels**:
   - `event_type`: see [description](#metric-name-kms_connector_gw_listener_event_received_counter)
 - **Description**: Measures the latency of decryptions at the KMS worker level, from event creation to processing. Only applies to `public_decryption_request` and `user_decryption_request` event types. Bucket boundaries (in seconds): 0.01, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0.
 - **Alarm**: None for now. Need more experience with this metric first.

### tx-sender

#### Metric Name: `kms_connector_tx_sender_response_received_counter`
 - **Type**: Counter
 - **Labels**:
   - `response_type`: can be used to filter by response type (public_decryption_response, user_decryption_response, crsgen_response, ...).
 - **Description**: Counts the number of responses received by the TX sender.
 - **Alarm**: If the counter is a flat line over a period of time, only for `response_type` `public_decryption_response` and `user_decryption_response`.
   - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter{response_type = "..."}[1m]) == 0`.

#### Metric Name: `kms_connector_tx_sender_response_received_errors`
 - **Type**: Counter
 - **Labels**:
   - `response_type`: see [description](#metric-name-kms_connector_tx_sender_response_received_counter)
 - **Description**: Counts the number of errors encountered by the TX sender while listening for responses.
 - **Alarm**: If the counter increases over a period of time.
   - **Recommendation**: more than 60 failures in 1 minute, i.e. `sum(increase(counter[1m])) > 60`.

#### Metric Name: `kms_connector_tx_sender_gateway_tx_sent_counter`
 - **Type**: Counter
 - **Labels**:
   - `response_type`: see [description](#metric-name-kms_connector_tx_sender_response_received_counter)
 - **Description**: Counts the number of transactions sent to the Gateway by the TX sender.
 - **Alarm**: If the counter is a flat line over a period of time, only for `response_type` `public_decryption_response` and `user_decryption_response`.
   - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter{response_type = "..."}[1m]) == 0`.

#### Metric Name: `kms_connector_tx_sender_gateway_tx_sent_errors`
 - **Type**: Counter
 - **Labels**:
   - `response_type`: see [description](#metric-name-kms_connector_tx_sender_response_received_counter)
 - **Description**: Counts the number of errors encountered by the TX sender while sending transactions to the Gateway.
 - **Alarm**: If the counter increases over a period of time.
   - **Recommendation**: more than 60 failures in 1 minute, i.e. `sum(increase(counter[1m])) > 60`.

#### Metric Name: `kms_connector_unprocessed_operations`
 - **Type**: Gauge
 - **Labels**:
   - `table`: the table of the operation being counted.
   - `status`: the database status of the operations being counted, either `pending` or `under_process`.
 - **Description**: Tracks the number of operations not yet processed in the kms-connector's DB (i.e. still `pending` or `under_process`). Terminal statuses (`completed`/`failed`) are deliberately excluded so the gauge stays bounded, since operations are now kept in the DB forever.
 - **Alarm**: Need more experience with this metric first.

#### Metric Name: `kms_connector_tx_sender_response_forwarding_latency_seconds`
 - **Type**: Histogram
 - **Labels**:
   - `response_type`: see [description](#metric-name-kms_connector_tx_sender_response_received_counter)
 - **Description**: Measures the latency from response creation in DB to successful blockchain transaction confirmation. Bucket boundaries (in seconds): 0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 15.0, 30.0.
 - **Alarm**: Need more experience with this metric first.
