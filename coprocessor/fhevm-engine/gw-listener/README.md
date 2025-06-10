# Gateway Listener

The **gw-listener** service listens for events from the GW and dispatches them to respective components in the coprocessor.

## Input Proof Verification Events

**gw-listener** listens for input proof verification events from the InputVerification contract and inserts them into the DB into the `verify_proofs` table.

The gw-listener will notify **zkproof-worker** services that work is available over the `verify_proof_requests` DB channel (configurable, but this is the default one).

Once a ZK proof request is verified, a zkproof-worker should set:
 * `verified = true or false`
 * `verified_at = NOW()` 
 * `handles = concatenated 32-byte handles` (s.t. the length of the handles field in bytes is a multiple of 32)

Then, zkproof-worker should notify the **transaction-sender** on the **verify_proof_responses** DB channel (configurable, but this is the default one).

### Note on Missed Events

Currently, **gw-listener** uses WebSocket subscriptions via `eth_subscribe` for input proof verification events. If the connection to the node is dropped and then recovered internally in alloy-rs, the subscription of events will start from the head, possibly skipping events. This is acceptable as input proof verification would be retried by the client. Moreover, replaying
old input verification events is unnecessary as input verification is a synchronous request/response interaction on the client side. Finally, no data on the GW will be left in an inconsistent state.

A future version of the **gw-listener** could change that behaviour and could replay these events.

For **gw-listener** to work correctly with above in mind, the assumption is that alloy-rs would retry "indefinitely". Namely, that the following configuration options are set to high
enough values:

```rust
    #[arg(long, default_value = "1000000")]
    provider_max_retries: u32,

    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    provider_retry_interval: Duration,
```
