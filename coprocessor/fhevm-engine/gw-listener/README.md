# Gateway Listener

The **gw-listener** service listens for events from the GW and dispatches them to respective components in the coprocessor.

## Input Proof Verification Events

**gw-listener** listens for input proof verification events from the InputVerification contract and inserts them into the DB into the `verify_proofs` table.

The gw-listener will notify **zkproof-worker** services that work is available over the `event_zkpok_new_work` DB channel (configurable, but this is the default one).

Once a ZK proof request is verified, a zkproof-worker should set:
 * `verified = true or false`
 * `verified_at = NOW()` 
 * `handles = concatenated 32-byte handles` (s.t. the length of the handles field in bytes is a multiple of 32)

Then, zkproof-worker should notify the **transaction-sender** on the **verify_proof_responses** DB channel (configurable, but this is the default one).

### Note on Missed Events

**gw-listener** polls the gateway over HTTP JSON-RPC using `get_block_number` and
`get_logs` for input proof verification events. Processed block progress is
stored in the database, so after a restart the listener resumes from the last
stored block instead of relying on an active WebSocket subscription.

For **gw-listener** to tolerate transient gateway RPC failures, the following
configuration options should be set to high enough values:

```rust
    #[arg(long, default_value = "1000000")]
    provider_max_retries: u32,

    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    provider_retry_interval: Duration,
```
