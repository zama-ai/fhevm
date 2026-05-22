# RFC 024 sync notes (PoC branch)

Push these validated choices to tech-spec branch `elias/rfc-024-solana-acl-design` ([PR #448](https://github.com/zama-ai/tech-spec/pull/448)).

## Validated by this PoC

### Opaque handles — host assigns, app points

- Apps store and pass **existing** handles with their ACL account (`EncryptedValue { handle, acl_record }`).
- Apps must **not** precompute `H("FHE_comp", …)` for frame outputs.
- After `execute_frame`, apps read `output_acl.handle` from host-written ACL data.

### Durable ACL birth stores the frame step result handle

During `execute_frame`:

```text
step result h = H("FHE_comp", op, operands, …, previous_bank_hash, timestamp) + metadata
Allow action → output ACL PDA gets A.handle = h
```

There is **no** extra `H("FHE_bound_output", base, nonce_key, seq)` layer in the current implementation. Distinct durable outputs come from distinct `(nonce_key, nonce_sequence)` ACL addresses, not from re-hashing the step result.

### Event vs ACL sources of truth

| Surface | Authority |
|---------|-----------|
| `emit_cpi!` TFHE / ACL events | FHE operation graph for coprocessor compute |
| On-chain `AclRecord` account data | Durable permissions and decrypt policy |

The listener ingests events only; it does not reconcile against ACL account snapshots.

### Frame account indices are builder-internal

`remaining_accounts` + `account_index` operands are host CPI plumbing. App code uses typed Anchor accounts and `fhe::execute`; only the builder maps accounts to indices.

### Handle domain inputs (EVM parity)

Computed handles bind to: op/operands, host program id, chain id, `previous_bank_hash` (slot N−1), and block timestamp — not transaction id. Same-block identical operands yield identical handles; ACL separates authorization.

## Test defaults

LiteSVM fixtures seed `SlotHashes` with `SHA256(b"zama-solana-test-bank-hash-v1")` so handle derivation is production-shaped without per-test setup.

## Still open in RFC / not settled here

- Real external input path (replacing `poc_authorize_transfer_amount`)
- Subject overflow / chunking
- Account cleanup and rent policy
- Production Solana ingestion (Geyser/RPC → listener)
