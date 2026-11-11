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

The `solana_host_follower_*` metrics come from the Solana host follower. The listener and the `solana-merkle-indexer` each run one and export these metrics with the same names and labels, so scope every recipe below to one of them by the `service` label its ServiceMonitor sets, such as `{service=~".*-solana-host-listener"}` or `{service=~".*-solana-merkle-indexer"}`, and alarm on each. The text below describes the listener; the indexer behaves the same, with its leaves and its own checkpoint in place of the compute rows. The listener resumes from its checkpoint through the stream while the Yellowstone provider can still replay it, about 24 hours for a hosted provider. Past that window it catches up from the archive RPC, with one `getBlock` per slot and one `getTransaction` per host transaction. A day of mainnet takes hours, so the lag alarm fires during a long catch-up too; `archive_catch_up_active` at 1 with the lag falling means it is progressing. A listener that fails the same slot, on the stream or during catch-up, retries it every 2 seconds: `failures_since_commit` keeps rising, and the lag and reconnect alarms fire too. `applied_slot` names the last committed slot and the listener's `ingestion interrupted` log line the error. The `/healthz` route checks only the database. A fatal ingestion error exits the process, so it shows up as container restarts, not as a metric.

On an empty database the indexer starts at `solanaHostListener.merkleIndexer.startSlot`, a block before the first encrypted store, such as the zama-host deployment slot, not at the tip, so a rebuild catches up from the archive over the whole host history. Its lag alarms fire for the duration of that catch-up, and the proof server answers only the stores and leaves the record already holds.

#### Metric Name: `solana_host_follower_applied_block_timestamp_seconds`
 - **Type**: Gauge (labeled by `host_chain_id`)
 - **Description**: Unix time the cluster assigned to the last block the listener committed. `time()` minus this value is the ingestion lag in seconds. It includes the cluster's finality delay, since the listener reads at finalized commitment: under a second on Alpenglow, about 13 seconds on TowerBFT. It grows both when the stream stalls and when the listener applies blocks slower than the cluster produces them. After a restart the series is missing until the listener commits a block.
 - **Alarm**: If the lag stays high, or the series is missing (the listener is down or not applying blocks).
    - **Recommendation**: more than 2 minutes behind for 2 minutes, i.e. `min_over_time((time() - gauge)[2m:]) > 120`, and `absent_over_time(gauge[5m])`.

#### Metric Name: `solana_host_follower_applied_slot`
 - **Type**: Gauge (labeled by `host_chain_id`)
 - **Description**: Slot of the last block the follower committed with its checkpoint. On a restart it starts at the resumed checkpoint.

#### Metric Name: `solana_host_follower_finalized_slot`
 - **Type**: Gauge (labeled by `host_chain_id`)
 - **Description**: The cluster's finalized slot, polled over RPC every 10 seconds. Minus `applied_slot`, it is the lag in slots, which compares directly with the provider's replay window, including while a restarted listener has not yet applied a block. Between polls it reads up to about 25 slots low, so a healthy lag hovers around zero and can dip below it.
 - **Alarm**: If the RPC poll stops updating the gauge. Chart the slot lag against the provider's window rather than paging on it; the time lag above pages first.
    - **Recommendation**: `changes(finalized_slot[5m]) == 0`.

#### Metric Name: `solana_host_follower_archive_catch_up_active`
 - **Type**: Gauge (labeled by `host_chain_id`)
 - **Description**: 1 while the listener rebuilds, from the archive RPC, slots the stream can no longer replay, else 0. The lag gauges above show its progress. A catch-up that keeps failing, such as on an archive missing the slots, shows as the gauge returning to 1 while reconnects rise and the lag stays flat.
 - **Alarm**: None of its own; the time lag pages.

#### Metric Name: `solana_host_follower_reconnects_total`
 - **Type**: Counter (labeled by `host_chain_id`)
 - **Description**: Interruptions the listener resumed from its checkpoint: no block meta for 30 seconds, closed by the server, a transport error, a retryable ingest failure, or a failed archive read during catch-up.
 - **Alarm**: If the counter increases repeatedly.
    - **Recommendation**: more than 3 reconnects in 10 minutes, i.e. `increase(counter[10m]) > 3`.

#### Metric Name: `solana_host_follower_failures_since_commit`
 - **Type**: Gauge (labeled by `host_chain_id`)
 - **Description**: Interruptions the listener resumed from its checkpoint since it last committed a block, on the stream or during catch-up. A commit, or a restart, resets it to 0. A stream that drops now and then moves it up and back to 0; a slot that fails again and again, or a provider that stays unreachable, keeps it rising.
 - **Alarm**: If it reaches 5, the listener has resumed five times without committing a block: a stuck slot or a provider outage. The `ingestion interrupted` log line names the error, and `applied_slot` the last committed slot.
    - **Recommendation**: `gauge >= 5`.

#### Metric Name: `coprocessor_solana_host_listener_handle_check_failures_total`
 - **Type**: Counter (labeled by `host_chain_id`)
 - **Description**: Steps whose emitted result handle did not match the handle the listener re-derived. Each one is held back: its computation and every computation that depends on it end as errors, while the rest of the block is ingested. It means the listener's derivation or step decoding is wrong. The log line `solana handle check failed` names the slot, signature, step and both handles; the repair is in the host-listener README.
 - **Alarm**: Any increase. Page on it.
    - **Recommendation**: `increase(counter[5m]) > 0`.

#### Container restarts
 - **Description**: A fatal ingestion error, such as a block whose ancestry does not match the checkpoint or a provider that cannot replay from any slot, exits the listener, which then resumes from its checkpoint. A restart that catches up quickly never trips the lag alarm, so restarts need their own alarm. The `solana-merkle-indexer` container follows the same stream into the leaf record's own database and exits the same way, resuming from its own checkpoint; it also exits on a Store whose history the record does not hold, and then restarts at the same block until the record is rebuilt. The Merkle proofs come from the separate `solana-merkle-proof-server` container, which keeps serving through an indexer restart. It restarts when it crashes or cannot reach its database at startup. It is not ready until it has read the KMS tx-senders from `ProtocolConfig`, and a database lost later makes it not ready, which removes it from its Service without a restart.
 - **Alarm**: Any restart of the three containers.
    - **Recommendation**: `increase(kube_pod_container_status_restarts_total{container=~"solana-host-listener|solana-merkle-indexer|solana-merkle-proof-server"}[15m]) > 0`.

### solana-merkle-indexer

The store check compares every recorded store with its account on chain, once at start and then every `--store-check-interval-secs` (600). Each store compares at the chain's leaf count: a record holding fewer leaves is behind and is compared once more at the end of the check, and otherwise its peaks at that count must equal the chain's. A store that disagrees is written to `quarantined_stores`, and the proof server answers it as inconsistent until a later check matches or the store is closed. [`RUNBOOK.md`](../../coprocessor/fhevm-engine/solana-merkle-proof-service/RUNBOOK.md) says how to heal it.

#### Metric Name: `solana_merkle_indexer_store_checks_total`
 - **Type**: Counter (labeled by `host_chain_id`, `result`)
 - **Description**: Recorded stores compared with their account on chain, one per store per check, by `result`: `match`, `mismatch` (the peaks differ, or the account is not a valid store), `behind` (the record still holds fewer leaves than the chain at the end of the check) or `absent` (the store was closed).
 - **Alarm**: Covered by `solana_merkle_indexer_quarantined_stores`: every `mismatch` quarantines its store. `behind` on many stores at every check means the indexer lags, which its lag alarm reports.

#### Metric Name: `solana_merkle_indexer_quarantined_stores`
 - **Type**: Gauge (labeled by `host_chain_id`)
 - **Description**: Stores the proof server refuses to prove from, as the last completed store check left them. The indexer also sets it from the database when it starts, so a quarantine left by an earlier run shows before the first check completes. A quarantine lifts when a later check matches or finds the store closed.
 - **Alarm**: Above 0.
    - **Recommendation**: critical, `max(gauge) > 0` for 1 minute, with the runbook as `runbook_url`.

#### Metric Name: `solana_merkle_indexer_store_check_completed_timestamp_seconds`
 - **Type**: Gauge (labeled by `host_chain_id`)
 - **Description**: Unix time the last store check over every recorded store completed. The runbook uses it to confirm a clean check after a restore.
 - **Alarm**: No completed check for three intervals, or no series at all (the first check has not completed since the indexer started).
    - **Recommendation**: warning, `time() - gauge > 1800`, and `absent_over_time(gauge[30m])`.

#### Metric Name: `solana_merkle_indexer_store_check_failures_total`
 - **Type**: Counter (labeled by `host_chain_id`)
 - **Description**: Store checks stopped by an RPC or database error. The check runs again at the next interval.
 - **Alarm**: Covered by the completed-timestamp alarm; a single failure is not actionable.

### solana-merkle-proof-server

#### Metric Name: `solana_merkle_proof_server_requests_total`
 - **Type**: Counter (labeled by `status`)
 - **Description**: Merkle proof requests answered, by `status`: `ok`, `cached` (a copy of a signed request whose first answer was `ok`, served from the answer cache) or the error code (`malformed`, `non_canonical_path`, `auth_missing`, `auth_malformed`, `auth_expired`, `auth_unknown_signer`, `auth_body_mismatch`, `upstream_transient`, `rate_limited`, `overloaded`). A copy of a refused request counts under the refusal's code.
 - **Alarm**: Sustained refusals.
    - **Recommendation**: warning, `sum(rate(counter{status=~"rate_limited|overloaded"}[5m])) > 0` for 15 minutes. `rate_limited` is one KMS tx-sender over its rate or answer-cache budget; `overloaded` is every database connection busy past 200 ms. A rising `cached` share means someone replays signed requests; it costs no work, so it is a signal, not an alarm.

#### Metric Name: `solana_merkle_proof_server_request_duration_seconds`
 - **Type**: Histogram
 - **Description**: Time to answer a Merkle proof request, refusals included. The KMS connector asks the next coprocessor after 250 ms without an answer.

#### Metric Name: `solana_merkle_proof_server_proof_reads_waiting`
 - **Type**: Gauge
 - **Description**: Requests waiting for a database connection: this replica's backlog. No request waits longer than 200 ms (`PROOF_READ_WAIT`). One that would is refused `overloaded`, so 200 ms bounds the age of the oldest waiting request.
 - **Alarm**: Covered by the `overloaded` alarm on `solana_merkle_proof_server_requests_total`.

#### Metric Name: `solana_merkle_proof_server_proof_reads_in_flight`
 - **Type**: Gauge
 - **Description**: Requests reading the leaf record on this replica, at most `--database-pool-size` minus one. Summed over replicas, it is the proof server's read load on the shared database.

#### Metric Name: `solana_merkle_proof_server_requests_by_signer_total`
 - **Type**: Counter (labeled by `signer`)
 - **Description**: Authenticated requests, by the KMS tx-sender that signed them: each KMS node's load on this server. One series per KMS node of the Active contexts.

#### Metric Name: `solana_merkle_proof_server_leaves_total`
 - **Type**: Counter (labeled by `outcome`)
 - **Description**: Queried leaves read from the record, by `outcome`: `found`, `not_found`, `unknown_store`, `quarantined` (the store check quarantined the store), `inconsistent` (the row's fields do not match its commitment, or its path is missing or does not reach the recorded peaks) or `read_failed`. Both `quarantined` and `inconsistent` leaves are answered `inconsistent`. Cached answers are not counted again. The `inconsistent` and `quarantined` series exist at 0 from startup.
 - **Alarm**: Any `inconsistent`. `quarantined` does not page: `solana_merkle_indexer_quarantined_stores` already does.
    - **Recommendation**: critical, `increase(counter{outcome="inconsistent"}[5m]) > 0`, with the runbook as `runbook_url`.

#### Metric Name: `solana_merkle_proof_server_kms_tx_senders`
 - **Type**: Gauge
 - **Description**: KMS tx-senders the last successful read of the canonical `ProtocolConfig` allows; 0 until the first read.
 - **Alarm**: 0 while the server serves. The server refuses every request until its first read.

#### Metric Name: `solana_merkle_proof_server_kms_tx_senders_read_timestamp_seconds`
 - **Type**: Gauge
 - **Description**: Unix time of the last successful poll of the KMS tx-senders. The server polls the finalized block every 5 seconds (`--kms-tx-sender-poll-interval`), reads the set again when the block is newer than the last read, and keeps the last set when a poll fails.
 - **Alarm**: Stale reads.
    - **Recommendation**: warning, `time() - gauge > 600`.

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

### coprocessor-proxy

Metrics are served at `/metrics` on `COPROCESSOR_PROXY_METRICS_ENDPOINT`. The proxy accepts requests signed by the
tx-senders of the Active KMS contexts in ProtocolConfig, read at the latest finalized block.

#### Metric Name: `coprocessor_proxy_kms_tx_senders`
 - **Type**: Gauge
 - **Description**: Number of KMS tx-senders the last successful read of ProtocolConfig allows. At 0, the proxy refuses every request with `auth_unknown_signer`.
 - **Alarm**: If the gauge is 0.
    - **Recommendation**: 0 for more than 1 minute, i.e. `max_over_time(coprocessor_proxy_kms_tx_senders[1m]) == 0`.

#### Metric Name: `coprocessor_proxy_kms_tx_senders_block_timestamp_seconds`
 - **Type**: Gauge
 - **Description**: Unix timestamp of the finalized block whose KMS tx-senders the proxy applies; 0 before the first read. On Ethereum the finalized block is the first block of the epoch two epochs back, so its age normally runs from about 13 to 19 minutes, and a late finalization can push it past 25 minutes with nothing wrong in the proxy. It stops advancing when reads of ProtocolConfig fail, and the proxy keeps the last set it read.
 - **Alarm**: If the applied block is older than finality plus a margin for late finalization.
    - **Recommendation**: more than 30 minutes old, i.e. `time() - coprocessor_proxy_kms_tx_senders_block_timestamp_seconds > 1800`.

#### Metric Name: `coprocessor_proxy_requests_counter`
 - **Type**: Counter
 - **Labels**:
   - `route`: `prf` for the PRF route, `none` for any other path.
   - `status`: HTTP response status; `0` means no response status.
   - `signer`: the authenticated signer, EIP-55 checksummed; empty when the request was not authenticated.
 - **Description**: One count per request the proxy answered. `502` means prf-service could not be reached or failed, `504` that it timed out, `429` that the signer exceeded its request rate (`COPROCESSOR_PROXY_MAX_REQUESTS_PER_SIGNER`).
 - **Alarm**: Sustained prf-service failures. Starting threshold: more than 5 in 5 minutes; tune to traffic volume.
    - **Recommendation**: `sum(increase(coprocessor_proxy_requests_counter{status=~"502|504"}[5m])) > 5`.

### prf-service

#### Metric Name: `coprocessor_prf_service_requests_total`
 - **Type**: Counter
 - **Labels**:
   - `method`: `GET` or `HEAD`.
   - `status`: HTTP response status.
 - **Description**: One count for each PRF request that prf-service answers. `429`: a connector is at one of its limits. `500`: an evaluation failed. `503`: a database error. The `signer` label of `coprocessor_proxy_requests_counter` names the connector of a `429`.
 - **Alarm**: Database errors, or a connector at its limit.
    - **Recommendation**: `sum(increase(coprocessor_prf_service_requests_total{status=~"429|503"}[5m])) > 0`.

#### Metric Name: `coprocessor_prf_service_request_duration_seconds`
 - **Type**: Histogram
 - **Labels**: see [`coprocessor_prf_service_requests_total`](#metric-name-coprocessor_prf_service_requests_total).
 - **Description**: Time from the start of a PRF request to its answer, in seconds. A request waits while the pad-worker computes its row, so a `200` can take seconds to minutes. A `429` is immediate. Bucket bounds: 5 ms to 164 s, above the 120 s default `COPROCESSOR_PROXY_PRF_SERVICE_RESPONSE_TIMEOUT`. A request that the proxy stops at its timeout has no answer, so it is not counted here: the proxy counts it as `504`.
 - **Alarm**: The pad-worker is slow. A stopped pad-worker gives no `200` answers, so this alarm does not see it: the proxy alarm on `504` does (see [`coprocessor_proxy_requests_counter`](#metric-name-coprocessor_proxy_requests_counter)).
    - **Recommendation**: the p95 of `200` answers is above half the proxy response timeout for 10 minutes, i.e. `histogram_quantile(0.95, sum by (le) (rate(coprocessor_prf_service_request_duration_seconds_bucket{status="200"}[5m]))) > 60`. Tune it with the pad-worker benchmarks.

## kms-connector

### HTTP decryption: proxy and endpoint

Metrics are served at `/metrics`. Routes: `/v1/public-decrypt`, `/v1/user-decrypt`, `/version`, `unmatched`.
Exclude `/version` probes when measuring decryption traffic. Proxy metrics start after header parsing;
TLS failures and incomplete headers are not counted. Requests still in progress when the process exits
are not recorded.

#### Metric Name: `kms_connector_proxy_http_requests_counter`
 - **Type**: Counter
 - **Labels**:
   - `route`: the API route, `/v1/public-decrypt`, `/v1/user-decrypt` or `/version`, or `unmatched` for any other path.
   - `status`: HTTP response status; `0` means no response status.
   - `code`: API `ErrorCode` value of an error written by the proxy, `upstream` for responses passed through from the endpoint without body inspection, `not_found`, `method_not_allowed`, `transport_error` or `cancelled`.
 - **Description**: One count per completed or cancelled request.
 - **Alarm**: Sustained decryption 5xx responses; break down by code, especially `timeout` and `upstream_transient`. Starting threshold: more than 5 in 5 minutes; tune to traffic volume.
   - **Recommendation**: `sum by (code) (increase(kms_connector_proxy_http_requests_counter{route=~"/v1/(public|user)-decrypt", status=~"5.."}[5m])) > 5`.

#### Metric Name: `kms_connector_endpoint_http_requests_counter`
 - **Type**: Counter
 - **Labels**:
   - `route`, `status`: see [description](#metric-name-kms_connector_proxy_http_requests_counter)
   - `code`: API `ErrorCode` value, `success`, `not_found`, `method_not_allowed`, `cancelled` or `unclassified`.
 - **Description**: One count per completed or cancelled request. Endpoint completion does not guarantee client delivery.
 - **Alarm**: Sustained decryption 5xx responses; break down by code, especially `overloaded` and `timeout`. Starting threshold: more than 5 in 5 minutes; tune to traffic volume.
   - **Recommendation**: `sum by (code) (increase(kms_connector_endpoint_http_requests_counter{route=~"/v1/(public|user)-decrypt", status=~"5.."}[5m])) > 5`.

#### Metric Name: `kms_connector_proxy_http_request_latency_seconds`
 - **Type**: Histogram
 - **Labels**:
   - `route`: see [description](#metric-name-kms_connector_proxy_http_requests_counter)
 - **Description**: Request duration in seconds, including failures and cancellations. Bucket bounds: 0.01–60 seconds, covering the endpoint's default 30-second decryption timeout and the proxy's default 60-second endpoint response timeout.
 - **Alarm**: p95 above 80% of the endpoint's `decryption_timeout` for 5 minutes, with sufficient traffic. The proxy waits on the endpoint, so the endpoint's threshold applies.
   - **Recommendation**: with the default 30-second timeout, `histogram_quantile(0.95, sum by (le, route) (rate(kms_connector_proxy_http_request_latency_seconds_bucket{route=~"/v1/(public|user)-decrypt"}[5m]))) > 24`.

#### Metric Name: `kms_connector_endpoint_http_request_latency_seconds`
 - **Type**: Histogram
 - **Labels**:
   - `route`: see [description](#metric-name-kms_connector_proxy_http_requests_counter)
 - **Description**: see [description](#metric-name-kms_connector_proxy_http_request_latency_seconds)
 - **Alarm**: see [description](#metric-name-kms_connector_proxy_http_request_latency_seconds)
   - **Recommendation**: with the default 30-second timeout, `histogram_quantile(0.95, sum by (le, route) (rate(kms_connector_endpoint_http_request_latency_seconds_bucket{route=~"/v1/(public|user)-decrypt"}[5m]))) > 24`.

#### Metric Name: `kms_connector_proxy_http_requests_in_flight`
 - **Type**: Gauge
 - **Description**: Active requests after header parsing, of every route; decremented when the request completes or is cancelled.
 - **Alarm**: Diagnostic only; compare with traffic and latency.

#### Metric Name: `kms_connector_endpoint_http_requests_in_flight`
 - **Type**: Gauge
 - **Description**: see [description](#metric-name-kms_connector_proxy_http_requests_in_flight)
 - **Alarm**: Diagnostic only; compare with traffic and latency.

#### Metric Name: `kms_connector_endpoint_decryptions_in_flight`
 - **Type**: Gauge
 - **Description**: Used admission permits, read from the semaphore on each scrape. Duplicate requests each hold a permit.
 - **Alarm**: Utilization above 90% of [`kms_connector_endpoint_max_in_flight_decryptions`](#metric-name-kms_connector_endpoint_max_in_flight_decryptions) for 5 minutes, together with increasing overload responses.
   - **Recommendation**: `max(kms_connector_endpoint_decryptions_in_flight / kms_connector_endpoint_max_in_flight_decryptions) > 0.9` for 5 minutes, and `sum(increase(kms_connector_endpoint_http_requests_counter{code="overloaded"}[5m])) > 0`.

#### Metric Name: `kms_connector_endpoint_max_in_flight_decryptions`
 - **Type**: Gauge
 - **Description**: Total admission permits, i.e. the configured maximum number of in-flight decryption requests.

#### Metric Name: `kms_connector_endpoint_decryption_lookup_counter`
 - **Type**: Counter
 - **Labels**:
   - `route`: `/v1/public-decrypt` or `/v1/user-decrypt`.
   - `outcome`: `served` returns an existing response; `stored` submits new work; `retried` submits work after an old error; `already_in_progress` joins existing work, possibly skipping an old error.
 - **Description**: Counts the committed decryption lookups.
 - **Alarm**: Diagnostic only. Worker metrics cannot isolate HTTP work because they lack a source label.

#### Metric Name: `kms_connector_endpoint_decryption_wait_latency_seconds`
 - **Type**: Histogram
 - **Labels**:
   - `route`: `/v1/public-decrypt` or `/v1/user-decrypt`.
 - **Description**: Seconds waiting for a response and reading its database row, for waits ending with a response (including API errors); existing responses served directly are not waited for. Timeouts, listener disruptions and cancellations are left out so they do not mask the response latency: the request counter counts them by `code`.
 - **Alarm**: Diagnostic only; use request timeout and server-error alarms.

#### Collection and traces

Enable `kmsConnectorProxy.serviceMonitor.enabled` and `kmsConnectorEndpoint.serviceMonitor.enabled`
in deployment overrides, and `kmsConnectorProxy.tracing.enabled` and `kmsConnectorEndpoint.tracing.enabled`
for traces. Verify healthy Prometheus targets and changing counters in Grafana.
There are no exemplars; find traces by time/service and correlate requests using `decryption_id`.

### gw-listener

#### Metric Name: `kms_connector_gw_listener_event_received_counter`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: can be used to filter by event type (public_decryption_request, user_decryption_request, crsgen_request, ...).
 - **Description**: Counts the number of events received by the GW listener and stored in DB. Events already in DB (e.g. re-fetched block range) are not counted.
 - **Alarm**: If the counter is a flat line over a period of time, only for `event_type` `public_decryption_request` and `user_decryption_request`.
   - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter{event_type="..."}[1m]) == 0`.

#### Metric Name: `kms_connector_gw_listener_event_listening_errors`
 - **Type**: Counter
 - **Labels**:
   - `contract`: can be used to filter by listener (decryption, ethereum).
 - **Description**: Counts the number of errors encountered by the GW listener while listening for events.
 - **Alarm**: If the counter increases over a period of time.
   - **Recommendation**: more than 60 failures in 1 minute, i.e. `sum(increase(counter[1m])) > 60`.

#### Metric Name: `kms_connector_gw_listener_rpc_head_block`
 - **Type**: Gauge
 - **Labels**:
   - `chain`: can be used to filter by chain (gateway, ethereum).
 - **Description**: The latest head reported by the chain's RPC node: the latest block for the Gateway, the finalized block for Ethereum.
 - **Alarm**: If the gauge stays the same over a period of time. This catches both a lagging RPC node and a listener that no longer gets the head from it, e.g. because its RPC calls keep failing or it is stuck on a DB query. The Gateway only produces blocks when it receives transactions, so a quiet chain also triggers this alarm.
   - **Recommendation**: unchanged for 10 minutes on the Gateway, i.e. `changes(gauge{chain="gateway"}[10m]) == 0`, and for 20 minutes on Ethereum.

#### Metric Name: `kms_connector_gw_listener_last_polled_block`
 - **Type**: Gauge
 - **Labels**:
   - `chain`: see [description](#metric-name-kms_connector_gw_listener_rpc_head_block)
 - **Description**: The end of the last block range the GW listener has polled and stored in DB. It is only set once the listener has stored a block range since it started. If the listener re-polls older blocks, e.g. with `decryption_from_block_number` set, it can be lower than the last block stored in DB.
 - **Alarm**: If the GW listener falls behind the head reported by the RPC node, e.g. because `eth_getLogs` or the DB keeps failing. A lagging RPC node does not trigger this alarm, as the listener keeps up with the stale head: see [`kms_connector_gw_listener_rpc_head_block`](#metric-name-kms_connector_gw_listener_rpc_head_block) for that case.
   - **Recommendation**: more than 100 blocks behind on the Gateway for 5 minutes, i.e. `kms_connector_gw_listener_rpc_head_block{chain="gateway"} - kms_connector_gw_listener_last_polled_block{chain="gateway"} > 100` for 5m.

### kms-worker

#### Metric Name: `kms_connector_worker_event_received_counter`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: see [description](#metric-name-kms_connector_gw_listener_event_received_counter)
 - **Description**: Counts the number of events picked from DB by the KMS worker. Retried events are counted each time they are picked.
 - **Alarm**: If the counter is a flat line over a period of time, only for `event_type` `public_decryption_request` and `user_decryption_request`.
   - **Recommendation**: 0 for more than 1 minute, i.e. `increase(counter{event_type="..."}[1m]) == 0`.

#### Metric Name: `kms_connector_worker_event_completed_counter`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: see [description](#metric-name-kms_connector_gw_listener_event_received_counter)
 - **Description**: Counts the number of events successfully processed by the KMS worker (response stored in DB).

#### Metric Name: `kms_connector_worker_event_failed_counter`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: see [description](#metric-name-kms_connector_gw_listener_event_received_counter)
   - `reason`: `failed` (irrecoverable error or max attempts reached) or `aborted` (aborted on the KMS Core).
 - **Description**: Counts the number of events the KMS worker gave up on.
 - **Alarm**: If the counter increases over a period of time.
   - **Recommendation**: more than 60 failures in 1 minute, i.e. `sum(increase(counter[1m])) > 60`.

#### Metric Name: `kms_connector_worker_event_retried_counter`
 - **Type**: Counter
 - **Labels**:
   - `event_type`: see [description](#metric-name-kms_connector_gw_listener_event_received_counter)
 - **Description**: Counts the number of events put back to `pending` by the KMS worker after a recoverable error, to be retried.

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

#### Metric Name: `kms_connector_worker_solana_proof_answers_counter`
 - **Type**: Counter
 - **Labels**:
   - `source`: the coprocessor's Merkle proof server, as host and port.
   - `outcome`: its answer for one queried leaf: `verified`, `no_leaf`, `unknown_store`, `inconsistent` (the coprocessor knows its record is wrong for this leaf, and pages on its own), `behind` (its record holds fewer leaves than the chain), `ahead` (a leaf past the count the connector observed), `invalid` (built against as many leaves as the chain or more, yet the proof does not verify) or `read_failed` (the read failed, was refused, or answered a different number of leaves than asked).
 - **Description**: Counts each coprocessor's Merkle proof answers on Solana decryptions. The connector verifies every proof against the chain and asks the next coprocessor when one fails, so a coprocessor counting `invalid` costs no wrong decryption, only its share of the reads. `behind` and `read_failed` from one `source` mean it lags or is down; `inconsistent` means its record is wrong for that store, which it reports itself. The `invalid` series of every configured coprocessor exists at 0 from startup.
 - **Alarm**: Any `invalid`: that coprocessor's leaf record disagrees with the chain. Page and tell the coprocessor's operator; [the Merkle record runbook](../../coprocessor/fhevm-engine/solana-merkle-proof-service/RUNBOOK.md) is the repair.
   - **Recommendation**: critical, `increase(counter{outcome="invalid"}[5m]) > 0`, by `source`.

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
