# ethereum_rpc_mock

Deterministic Ethereum JSON-RPC mock server for component tests.

This crate provides the RPC simulation layer only:

- `MockServer`: an HTTP/WebSocket JSON-RPC server.
- Pattern registration for `eth_call` and `eth_sendRawTransaction`.
- Mocked balances, nonce, bytecode, storage, receipts, logs, and blocks.
- Scheduled events that can later emit logs through WebSocket subscriptions.
- Response scripting with `UsageLimit::Once` and `UsageLimit::Unlimited`.

It intentionally does not own FHEVM gateway, host, relayer, or KMS connector setup. Those layers should live with the component that owns the contract bindings used by its tests. In this repository, relayer-specific FHEVM setup lives in `fhevm_relayer::test_support::fhevm_setup`.

## When To Use It

Use `ethereum_rpc_mock` when the component under test talks to Ethereum JSON-RPC and the test should script the behavior of contracts or other components.

This is different from using Anvil with deployed contracts:

- Anvil is better when real EVM contract behavior is part of the test.
- `ethereum_rpc_mock` is better when the component behavior is the target and surrounding contracts or services should be simulated.
- The mock can produce selector/input/output combinations that are awkward or impossible to trigger through deployed contracts.
- Scheduled events can programmatically trigger expected responses from other components without running those components.

That makes tests useful as executable documentation: each test states the relevant RPC input, mocked output, and expected component behavior.

## Basic Usage

```rust,no_run
use alloy::primitives::{address, Bytes};
use ethereum_rpc_mock::{MockConfig, MockServer, Response, UsageLimit};

# async fn example() -> anyhow::Result<()> {
let server = MockServer::new(MockConfig {
    port: 18545,
    ..MockConfig::new()
});

let contract = address!("0000000000000000000000000000000000000001");
let selector = [0x12, 0x34, 0x56, 0x78];

server.on_call(
    move |params| params.to == contract && params.input.starts_with(&selector),
    Response::call_success(Bytes::from_static(&[0x00; 32])),
    UsageLimit::Unlimited,
);

let handle = server.clone().start().await?;

// Point the component under test at handle.url().

handle.shutdown().await?;
# Ok(())
# }
```

Pattern matching is first-match-wins. `UsageLimit::Once` is useful for retry tests where the first call should fail and the second should succeed.

## Component Setup Layers

Keep component-specific setup outside this crate. A setup layer can depend on this crate plus the component's own contract bindings, then expose higher-level helpers that register accurate selectors, receipts, bytecode, readiness calls, and scheduled response events.

For example, relayer tests use:

```rust,no_run
use alloy::primitives::{address, Bytes};
use ethereum_rpc_mock::{MockConfig, MockServer, SubscriptionTarget};
use fhevm_relayer::test_support::fhevm_setup::{
    RelayerFhevmSetup, UserDecryptKind,
};

# async fn example() -> anyhow::Result<()> {
let server = MockServer::new(MockConfig {
    port: 18546,
    ..MockConfig::new()
});

let decryption = address!("B8Ae44365c45A7C5256b14F607CaE23BC040c354");
let input_verification = address!("e61cff9c581c7c91aef682c2c10e8632864339ab");

let fhevm = RelayerFhevmSetup::new(server.clone(), decryption, input_verification);
fhevm.on_input_proof_success(
    address!("742d35Cc6639C3532e776b2c2B2C19b4d8ed8Faa"),
    Bytes::from_static(&[1, 2, 3, 4]),
    1,
    SubscriptionTarget::All,
);

fhevm.on_user_decrypt_revert(UserDecryptKind::Direct, "mocked revert");

let handle = server.clone().start().await?;
handle.shutdown().await?;
# Ok(())
# }
```

The important boundary is:

- `ethereum_rpc_mock` provides JSON-RPC simulation.
- `fhevm_relayer::test_support::fhevm_setup` describes the relayer's FHEVM gateway setup using the same bindings as the relayer crate.
- A future KMS connector setup should live with the KMS connector and use the connector's own binding versions.

## Main API

- `MockServer::new(config)`
- `server.start()`
- `MockServerHandle::url()`
- `MockServerHandle::shutdown()`
- `server.set_balance(...)`
- `server.set_code(...)`
- `server.set_storage(...)`
- `server.set_nonce(...)`
- `server.on_call(...)`
- `server.on_call_dynamic(...)`
- `server.on_transaction(...)`
- `Response::call_success(...)`
- `Response::transaction_success()`
- `Response::revert(...)`
- `Response::error(...)`
- `UsageLimit::Once`
- `UsageLimit::Unlimited`
- `SubscriptionTarget::All`
- `SubscriptionTarget::Only(...)`

## Extending For Another Component

Start from the RPC layer and put the binding-aware setup in the component crate.

1. Identify the RPC method the component calls, such as `eth_call`, `eth_sendRawTransaction`, subscriptions, or log queries.
2. Use that component's contract bindings to get selectors and ABI-encoded return bytes.
3. Register mocked behavior with `on_call`, `on_call_dynamic`, or `on_transaction`.
4. Use scheduled events when another component would normally emit a later response.
5. Keep the setup module next to the component tests so binding versions cannot drift.

## Current Limitations

- This mock focuses on input/output behavior. It does not execute EVM bytecode or validate contract invariants.
- Some JSON-RPC methods and filters are intentionally simplified. Add only behavior needed by tests, keeping it deterministic.
- Negative paths that depend on real EVM execution are still better covered with deployed contracts.

## Examples

- `tests/test_core_rpc.rs`: core RPC behavior.
- `tests/selective_subscription_test.rs`: targeted WebSocket event emission.
- `../../tests/fhevm_setup_test.rs`: relayer-owned FHEVM setup built on this mock.
