# Block Computer — Transaction Type Encoding Specification

**Status**: Specification
**Author**: nboisde
**Date**: 2026-04-14
**File**: `crates/listener_core/src/blockchain/evm/evm_block_computer.rs`

---

## 1. Problem Statement

The block computer verifies block integrity by recomputing the transaction trie root
from parsed RPC data and comparing it to the header's `transactionsRoot`. This requires
RLP-encoding every transaction in the block.

The current implementation only handles:
- Standard Ethereum types 0–4 (via `AnyTxEnvelope::Ethereum` + alloy's `encoded_2718()`)
- Optimism/Base deposit type 126 (0x7E)
- Arbitrum internal type 106 (0x6A)

Any other type falls into the `_` branch and returns `UnsupportedTransactionType`, which
currently blocks cursor processing. As of block 85,523,136 on Polygon mainnet, type 127
(0x7F — PIP-74 state sync) causes this failure.

## 2. Complete Transaction Type Inventory

### 2.1 Standard Ethereum (types 0–4)

Handled by alloy natively via `AnyTxEnvelope::Ethereum` → `encoded_2718()`.

| Type | EIP | Name | Encoding |
|------|-----|------|----------|
| 0 | — | Legacy | RLP([nonce, gasPrice, gasLimit, to, value, data, v, r, s]) |
| 1 | 2930 | Access list | 0x01 + RLP([chainId, nonce, gasPrice, gasLimit, to, value, data, accessList, signatureYParity, signatureR, signatureS]) |
| 2 | 1559 | Dynamic fee | 0x02 + RLP([chainId, nonce, maxPriorityFeePerGas, maxFeePerGas, gasLimit, to, value, data, accessList, signatureYParity, signatureR, signatureS]) |
| 3 | 4844 | Blob | 0x03 + RLP([chainId, nonce, maxPriorityFeePerGas, maxFeePerGas, gasLimit, to, value, data, accessList, maxFeePerBlobGas, blobVersionedHashes, signatureYParity, signatureR, signatureS]) |
| 4 | 7702 | Set code | 0x04 + RLP([chainId, nonce, maxPriorityFeePerGas, maxFeePerGas, gasLimit, to, value, data, accessList, authorizationList, signatureYParity, signatureR, signatureS]) |

**No action needed** — alloy handles these.

### 2.2 Optimism / Base (type 126)

**Status**: Implemented.

| Type | Name | Encoding |
|------|------|----------|
| 126 (0x7E) | Deposit transaction | 0x7E + RLP([sourceHash, from, to, mint, value, gas, isSystemTx, data]) |

Source: OP Stack specification. `from` comes from the recovered signer, not from `OtherFields`.

### 2.3 Arbitrum (types 100–106)

Source: [OffchainLabs/go-ethereum — core/types/arb_types.go](https://github.com/OffchainLabs/go-ethereum/blob/master/core/types/arb_types.go)

Arbitrum's modified Geth marshals all custom fields to JSON, so all types below **can** be
encoded from `OtherFields`.

#### Type 100 (0x64) — ArbitrumDepositTx

L1→L2 ETH deposit via the bridge. Appears in every block that processes a bridge deposit.

```
0x64 + RLP([chainId, l1RequestId, from, to, value])
```

| Field | JSON key | Rust type | Notes |
|-------|----------|-----------|-------|
| chainId | `"chainId"` | u64 | hex string |
| l1RequestId | `"requestId"` | B256 | |
| from | `"from"` | Address | use recovered signer |
| to | `"to"` | Address | always present (not optional) |
| value | `"value"` | U256 | |

```rust
fn encode_arbitrum_deposit_transaction(
    unknown: &UnknownTxEnvelope,
    from: Address,
    index: usize,
) -> Result<Vec<u8>, BlockVerificationError> {
    let fields = &unknown.inner.fields;
    let chain_id: u64 = extract_u64(fields, "chainId", index)?;
    let l1_request_id: B256 = extract_field(fields, "requestId", index)?;
    let to: Address = extract_field(fields, "to", index)?;
    let value: U256 = extract_field(fields, "value", index)?;

    let mut payload = Vec::new();
    chain_id.encode(&mut payload);
    l1_request_id.encode(&mut payload);
    from.encode(&mut payload);
    to.encode(&mut payload);
    value.encode(&mut payload);

    encode_typed_tx(0x64, payload)
}
```

#### Type 101 (0x65) — ArbitrumUnsignedTx

L1 user calling an L2 contract via the bridge (unsigned, no signature).

```
0x65 + RLP([chainId, from, nonce, gasFeeCap, gas, to, value, data])
```

| Field | JSON key | Rust type | Notes |
|-------|----------|-----------|-------|
| chainId | `"chainId"` | u64 | |
| from | `"from"` | Address | use recovered signer |
| nonce | `"nonce"` | u64 | |
| gasFeeCap | `"maxFeePerGas"` | U256 | Arbitrum maps gasFeeCap to maxFeePerGas in JSON |
| gas | `"gas"` | u64 | |
| to | `"to"` | Option\<Address\> | can be None for contract creation |
| value | `"value"` | U256 | |
| data | `"input"` | Bytes | "input" not "data" |

```rust
fn encode_arbitrum_unsigned_transaction(
    unknown: &UnknownTxEnvelope,
    from: Address,
    index: usize,
) -> Result<Vec<u8>, BlockVerificationError> {
    let fields = &unknown.inner.fields;
    let chain_id: u64 = extract_u64(fields, "chainId", index)?;
    let nonce: u64 = extract_u64(fields, "nonce", index)?;
    let gas_fee_cap: U256 = extract_field(fields, "maxFeePerGas", index)?;
    let gas: u64 = extract_u64(fields, "gas", index)?;
    let to: Option<Address> = extract_optional_address(fields, "to");
    let value: U256 = extract_field(fields, "value", index)?;
    let data: Bytes = extract_field(fields, "input", index)?;

    let mut payload = Vec::new();
    chain_id.encode(&mut payload);
    from.encode(&mut payload);
    nonce.encode(&mut payload);
    gas_fee_cap.encode(&mut payload);
    gas.encode(&mut payload);
    encode_optional_address(to, &mut payload);
    value.encode(&mut payload);
    data.encode(&mut payload);

    encode_typed_tx(0x65, payload)
}
```

#### Type 102 (0x66) — ArbitrumContractTx

L1 contract calling an L2 contract. Similar to ArbitrumUnsignedTx but with `requestId`
instead of `nonce`.

```
0x66 + RLP([chainId, requestId, from, gasFeeCap, gas, to, value, data])
```

| Field | JSON key | Rust type | Notes |
|-------|----------|-----------|-------|
| chainId | `"chainId"` | u64 | |
| requestId | `"requestId"` | B256 | |
| from | `"from"` | Address | use recovered signer |
| gasFeeCap | `"maxFeePerGas"` | U256 | |
| gas | `"gas"` | u64 | |
| to | `"to"` | Option\<Address\> | |
| value | `"value"` | U256 | |
| data | `"input"` | Bytes | |

```rust
fn encode_arbitrum_contract_transaction(
    unknown: &UnknownTxEnvelope,
    from: Address,
    index: usize,
) -> Result<Vec<u8>, BlockVerificationError> {
    let fields = &unknown.inner.fields;
    let chain_id: u64 = extract_u64(fields, "chainId", index)?;
    let request_id: B256 = extract_field(fields, "requestId", index)?;
    let gas_fee_cap: U256 = extract_field(fields, "maxFeePerGas", index)?;
    let gas: u64 = extract_u64(fields, "gas", index)?;
    let to: Option<Address> = extract_optional_address(fields, "to");
    let value: U256 = extract_field(fields, "value", index)?;
    let data: Bytes = extract_field(fields, "input", index)?;

    let mut payload = Vec::new();
    chain_id.encode(&mut payload);
    request_id.encode(&mut payload);
    from.encode(&mut payload);
    gas_fee_cap.encode(&mut payload);
    gas.encode(&mut payload);
    encode_optional_address(to, &mut payload);
    value.encode(&mut payload);
    data.encode(&mut payload);

    encode_typed_tx(0x66, payload)
}
```

#### Type 104 (0x68) — ArbitrumRetryTx

Retry execution of a failed L1→L2 retryable ticket.

```
0x68 + RLP([chainId, nonce, from, gasFeeCap, gas, to, value, data, ticketId, refundTo, maxRefund, submissionFeeRefund])
```

| Field | JSON key | Rust type | Notes |
|-------|----------|-----------|-------|
| chainId | `"chainId"` | u64 | |
| nonce | `"nonce"` | u64 | |
| from | `"from"` | Address | use recovered signer |
| gasFeeCap | `"maxFeePerGas"` | U256 | |
| gas | `"gas"` | u64 | |
| to | `"to"` | Option\<Address\> | |
| value | `"value"` | U256 | |
| data | `"input"` | Bytes | |
| ticketId | `"ticketId"` | B256 | Arbitrum-specific field |
| refundTo | `"refundTo"` | Address | Arbitrum-specific field |
| maxRefund | `"maxRefund"` | U256 | Arbitrum-specific field |
| submissionFeeRefund | `"submissionFeeRefund"` | U256 | Arbitrum-specific field |

```rust
fn encode_arbitrum_retry_transaction(
    unknown: &UnknownTxEnvelope,
    from: Address,
    index: usize,
) -> Result<Vec<u8>, BlockVerificationError> {
    let fields = &unknown.inner.fields;
    let chain_id: u64 = extract_u64(fields, "chainId", index)?;
    let nonce: u64 = extract_u64(fields, "nonce", index)?;
    let gas_fee_cap: U256 = extract_field(fields, "maxFeePerGas", index)?;
    let gas: u64 = extract_u64(fields, "gas", index)?;
    let to: Option<Address> = extract_optional_address(fields, "to");
    let value: U256 = extract_field(fields, "value", index)?;
    let data: Bytes = extract_field(fields, "input", index)?;
    let ticket_id: B256 = extract_field(fields, "ticketId", index)?;
    let refund_to: Address = extract_field(fields, "refundTo", index)?;
    let max_refund: U256 = extract_field(fields, "maxRefund", index)?;
    let submission_fee_refund: U256 = extract_field(fields, "submissionFeeRefund", index)?;

    let mut payload = Vec::new();
    chain_id.encode(&mut payload);
    nonce.encode(&mut payload);
    from.encode(&mut payload);
    gas_fee_cap.encode(&mut payload);
    gas.encode(&mut payload);
    encode_optional_address(to, &mut payload);
    value.encode(&mut payload);
    data.encode(&mut payload);
    ticket_id.encode(&mut payload);
    refund_to.encode(&mut payload);
    max_refund.encode(&mut payload);
    submission_fee_refund.encode(&mut payload);

    encode_typed_tx(0x68, payload)
}
```

#### Type 105 (0x69) — ArbitrumSubmitRetryableTx

Creates a retryable ticket with L1→L2 fee escrow.

```
0x69 + RLP([chainId, requestId, from, l1BaseFee, depositValue, gasFeeCap, gas, retryTo, retryValue, beneficiary, maxSubmissionFee, feeRefundAddr, retryData])
```

| Field | JSON key | Rust type | Notes |
|-------|----------|-----------|-------|
| chainId | `"chainId"` | u64 | |
| requestId | `"requestId"` | B256 | |
| from | `"from"` | Address | use recovered signer |
| l1BaseFee | `"l1BaseFee"` | U256 | Arbitrum-specific field |
| depositValue | `"depositValue"` | U256 | Arbitrum-specific field |
| gasFeeCap | `"maxFeePerGas"` | U256 | |
| gas | `"gas"` | u64 | |
| retryTo | `"retryTo"` | Option\<Address\> | destination on L2 |
| retryValue | `"retryValue"` | U256 | Arbitrum-specific field |
| beneficiary | `"beneficiary"` | Address | Arbitrum-specific field |
| maxSubmissionFee | `"maxSubmissionFee"` | U256 | Arbitrum-specific field |
| feeRefundAddr | `"feeRefundAddr"` | Address | Arbitrum-specific field |
| retryData | `"retryData"` | Bytes | Arbitrum-specific field |

```rust
fn encode_arbitrum_submit_retryable_transaction(
    unknown: &UnknownTxEnvelope,
    from: Address,
    index: usize,
) -> Result<Vec<u8>, BlockVerificationError> {
    let fields = &unknown.inner.fields;
    let chain_id: u64 = extract_u64(fields, "chainId", index)?;
    let request_id: B256 = extract_field(fields, "requestId", index)?;
    let l1_base_fee: U256 = extract_field(fields, "l1BaseFee", index)?;
    let deposit_value: U256 = extract_field(fields, "depositValue", index)?;
    let gas_fee_cap: U256 = extract_field(fields, "maxFeePerGas", index)?;
    let gas: u64 = extract_u64(fields, "gas", index)?;
    let retry_to: Option<Address> = extract_optional_address(fields, "retryTo");
    let retry_value: U256 = extract_field(fields, "retryValue", index)?;
    let beneficiary: Address = extract_field(fields, "beneficiary", index)?;
    let max_submission_fee: U256 = extract_field(fields, "maxSubmissionFee", index)?;
    let fee_refund_addr: Address = extract_field(fields, "feeRefundAddr", index)?;
    let retry_data: Bytes = extract_field(fields, "retryData", index)?;

    let mut payload = Vec::new();
    chain_id.encode(&mut payload);
    request_id.encode(&mut payload);
    from.encode(&mut payload);
    l1_base_fee.encode(&mut payload);
    deposit_value.encode(&mut payload);
    gas_fee_cap.encode(&mut payload);
    gas.encode(&mut payload);
    encode_optional_address(retry_to, &mut payload);
    retry_value.encode(&mut payload);
    beneficiary.encode(&mut payload);
    max_submission_fee.encode(&mut payload);
    fee_refund_addr.encode(&mut payload);
    retry_data.encode(&mut payload);

    encode_typed_tx(0x69, payload)
}
```

#### Type 106 (0x6A) — ArbitrumInternalTx

**Status**: Implemented.

```
0x6A + RLP([chainId, data])
```

### 2.4 Polygon Bor (type 127)

Source: [PIP-74 — Canonical Inclusion of StateSync Transactions](https://forum.polygon.technology/t/pip-74-canonical-inclusion-of-statesync-transactions-in-block-bodies/21331)

Introduced by the **Madhugiri hardfork** (Dec 2024, activation block 80,084,800).

| Type | Name | Encoding |
|------|------|----------|
| 127 (0x7F) | State sync (PIP-74) | 0x7F + RLP([\[encStateSyncData, ...]]) |

Each `encStateSyncData`:

| Field | Type | Description |
|-------|------|-------------|
| ID | uint64 | State sync event ID |
| Contract | address | Receiver contract on L2 |
| Data | bytes | ABI-encoded payload |
| TxHash | bytes32 | L1 transaction hash |

**Cannot be encoded from JSON RPC** — the inner payload is not in the transaction fields.
The RPC returns a normalized view: `from=0x0, to=0x0, gas=0, input=0x, v/r/s=0`.
The actual state sync data is only available via `eth_getRawTransactionByHash`.

State sync txs appear on **sprint blocks** (every 16 blocks on Polygon, e.g. block numbers
divisible by 16).

Example raw bytes for block 85,523,136 tx at index 375:
```
0x7f                        <- type byte
f9025f                      <- RLP list header (607 bytes)
  f9013d                    <- first encStateSyncData
    83 30364e               <- ID: 3159630
    94 a6fa...c0aa          <- Contract: 0xa6fa4fb5f76172d178d61b04b0ecd319c5d1c0aa
    b90100 87a7811f...      <- Data: 256 bytes
    a0 378a5f6c...1e1e      <- TxHash: 0x378a5f6c...
  f9011c                    <- second encStateSyncData
    83 30364f               <- ID: 3159631
    94 8397...8a28a         <- Contract: 0x8397259c983751daf40400790063935a11afa28a
    b8e0 0000...            <- Data: 224 bytes
    a0 458eea10...a51f      <- TxHash: 0x458eea10...
```

#### Options for supporting type 127

**Option A — Skip tx root verification (current approach)**

When encountering type 0x7F, skip the entire transaction root verification for this block.
Block hash + receipt root verification still run. Block hash verification alone proves header
authenticity (including the `transactionsRoot` stored in the header).

Tradeoff: a malicious RPC could serve tampered transactions for state-sync-containing blocks
without detection. Acceptable for trusted RPC endpoints.

**Option B — Fetch raw bytes via `eth_getRawTransactionByHash`**

Add a `get_raw_transaction_by_hash` method to `SemEvmRpcProvider`. In the fetcher, before
calling verification, scan for unknown types and fetch raw bytes. Pass them to the block
computer via a `HashMap<B256, Vec<u8>>`.

Changes required:
- `sem_evm_rpc_provider.rs`: add `get_raw_transaction_by_hash`
- `evm_block_fetcher.rs`: add `fetch_raw_unknown_txs`, make `build_fetched_block` async
- `evm_block_computer.rs`: accept `raw_txs` parameter in `verify_block`/`verify_transaction_root`/`safe_encode_transaction`

Tradeoff: extra RPC call per state-sync block, architectural changes to fetcher.

**Option C — Reconstruct from receipt logs**

The receipt logs contain state sync event IDs (in topics of 0x0000...1001 contract events)
and contract addresses, but NOT the full payload data or L1 tx hash. **Not viable.**

### 2.5 Other chains

| Chain | Custom types? | Notes |
|-------|--------------|-------|
| BNB/BSC | No | Standard 0–2 only |
| Avalanche C-Chain | No | Standard 0–2 (custom block header handled in `verify_block_hash`) |
| Monad | No | Standard 0–4 |
| Base | Type 126 only | Same as Optimism (OP Stack) |
| zkSync | Type 113 (0x71) | EIP-712 zkSync tx. Not relevant unless we add zkSync support. |
| Polygon zkEVM | No | Standard types only |

## 3. Shared Helper

Extract a shared `encode_typed_tx` to reduce duplication across all encoders:

```rust
/// Build a typed transaction envelope: type_byte + RLP list wrapping the payload.
fn encode_typed_tx(type_byte: u8, payload: Vec<u8>) -> Result<Vec<u8>, BlockVerificationError> {
    let mut result = vec![type_byte];
    let header = alloy::rlp::Header {
        list: true,
        payload_length: payload.len(),
    };
    header.encode(&mut result);
    result.extend_from_slice(&payload);
    Ok(result)
}
```

Refactor the existing `encode_deposit_transaction` (0x7E) and
`encode_arbitrum_internal_transaction` (0x6A) to use this helper too.

## 4. Skip Mechanism Design

The `_` fallback and type 127 should both use a non-blocking skip. The current code at
`verify_block` (lines 296–305) already catches `verify_transaction_root` errors and continues
— but the error still propagates from `safe_encode_transaction` through `verify_transaction_root`.

Proposed flow:

```
safe_encode_transaction
  └── type 127 or _ → Err(UnsupportedTransactionType { tx_type, index })

verify_transaction_root
  └── on UnsupportedTransactionType:
        error!(tx_type, index, block_number, "Skipping tx root verification: ...")
        return Ok(())  // skip, don't propagate

verify_block (unchanged)
  └── verify_transaction_root  → Ok (skipped) or Ok (verified)
  └── verify_receipt_root      → must pass
  └── verify_block_hash        → must pass
```

The ERROR log ensures visibility. A `listener_block_verification_skipped_total` metric
counter (labels: `chain_id`, `reason`) should be incremented for alerting.

## 5. Receipt Encoding for Custom Types

Receipt encoding uses `AnyReceiptEnvelope::encoded_2718()` for all non-0x7E types, which
produces `type_byte || rlp(status, cumulativeGasUsed, bloom, logs)`. This works for:

- Arbitrum types 100–106: standard receipt format, just different type byte
- Polygon type 127: standard receipt format (status=1, gasUsed=0, standard logs)

**No receipt encoding changes needed** for any of the types above. The Optimism-specific
deposit receipt handling (type 0x7E with `depositNonce`/`depositReceiptVersion`) is already
implemented.

## 6. Testing

### Polygon state sync block

Block **85,523,136** (0x518FAC0) on Polygon mainnet. 376 transactions, index 375 is type 0x7F.
Sprint blocks occur every 16 blocks.

```
TX hash: 0x4f8e7a02f12c3573bf9a7c83ac19f77f0537d614a1f05f68b39278cc02d652e5
RPC: https://polygon-bor-rpc.publicnode.com
```

Test should verify:
1. `verify_block` does NOT error (tx root skipped, receipt root + block hash pass)
2. `verify_receipt_root` passes independently
3. `verify_block_hash` passes independently

### Arbitrum

Enable `Arbitrum One` in `report_block_verification` test (already commented out at line 118).
Use `https://arb1.arbitrum.io/rpc`. Most blocks contain type 106 (internal) and
occasionally types 100, 104, 105.

### Field mapping validation

For each new Arbitrum encoder, validate by picking a real transaction of that type from
Arbiscan and comparing:
1. Fetch via `eth_getBlockByNumber(..., true)` — check JSON field names match spec
2. Fetch via `eth_getRawTransactionByHash` — compare our encoding against raw bytes

## 7. References

- [PIP-74 forum post](https://forum.polygon.technology/t/pip-74-canonical-inclusion-of-statesync-transactions-in-block-bodies/21331)
- [Bor v2.5.0 release (PIP-74 implementation)](https://github.com/0xPolygon/bor/releases/tag/v2.5.0)
- [Arbitrum Nitro arb_types.go](https://github.com/OffchainLabs/go-ethereum/blob/master/core/types/arb_types.go)
- [Arbitrum inside Nitro docs](https://docs.arbitrum.io/how-arbitrum-works/inside-arbitrum-nitro)
- [EIP-2718: Typed Transaction Envelope](https://eips.ethereum.org/EIPS/eip-2718)
