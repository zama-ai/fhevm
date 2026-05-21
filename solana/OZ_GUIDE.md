# OpenZeppelin Solana PoC Guide

This is the short handoff guide for OpenZeppelin work on the Solana PoC branch.
The central technical guide is [README.md](./README.md); update it whenever the program shape,
tests, ACL model, or event semantics change.

## Current Direction

```text
confidential-token
  app-side PoC program
  SPL-like cUSDC wrapper
  safe area for token design work

zama-host
  protocol-side host program
  owns ACL records
  checks operand ACL before emitting FHE events
  creates durable output ACL records through explicit allow/bind calls

runtime-tests
  LiteSVM behavior tests
  first place to prove or disprove a design change
```

The v0 ACL direction is keyed-nonce records:

```text
nonce_key = H("zama-acl-nonce-key-v1", acl_domain_key, app_account, encrypted_value_label)
acl_record = PDA("acl-record", nonce_key, nonce_sequence)
```

The handle is stored inside the ACL record. It is not used as a PDA seed.

## What Is Working

```text
initialize mint/account
  -> trivial_encrypt_and_bind creates initial balance handle + ACL record

wrap_usdc
  -> SPL transfer_checked into vault
  -> trivial_encrypt_and_bind amount
  -> fhe_binary_op_and_bind_output Add into current balance

confidential_transfer
  -> fhe_binary_op_and_bind_output Sub for sender balance
  -> fhe_binary_op_and_bind_output Add for receiver balance
  -> rotates balance handle + ACL record for each changed token account

BalanceHandleUpdatedEvent
  -> emitted by confidential-token for app/frontend indexing
  -> not consumed by the generic coprocessor listener

user decrypt model tests
  -> signed authorization + handle entry + ACL record verification
  -> current and historical handles
```

Self-transfer is a no-op. It must not rotate handles or create output ACL records.

## Important Boundaries

```text
Token app checks token semantics:
  owner signed
  token account owner/mint match
  output ACL account belongs to the token account being updated

ZamaHost checks FHEVM semantics:
  compute_subject is signer
  encrypted operand ACL records are canonical
  encrypted operands allow compute_subject
  output ACL record PDA matches supplied nonce metadata

Host listener consumes only generic ZamaHost events:
  FheBinaryOpEvent
  TrivialEncryptEvent
  FheRandEvent
  AclAllowedEvent
  InputVerifiedEvent

Those event decoders are generated at host-listener build time from the checked-in ZamaHost Anchor
IDL snapshot. The listener must not parse confidential-token events.

App indexers consume token-local events:
  BalanceHandleUpdatedEvent

KMS-style tests check decrypt semantics:
  signature scope
  ACL record owner
  canonical ACL record PDA
  handle match
  owner subject is allowed
```

KMS does not parse token state and does not prove that a handle is the current balance. Currentness
comes from app state. ACL authorization is durable and can also apply to historical handles.

## Current Caveats

```text
Input path:
  mock_input_verified_and_bind is a test short-circuit.
  It deliberately trusts the caller-supplied input handle.
  Its nonce sequence is explicit in tests and must not come from handle bytes.

Public decrypt:
  allow_for_decryption is modeled, but the production authority rule is not final.
  A compute_signer should not automatically be able to flip public_decrypt.

Persistent grants:
  allow_acl_subjects mutates the existing canonical ACL record.
  It does not create a second ACL record for the same handle.

Subjects:
  subjects[] is a plain Pubkey list.
  There is no Compute/UserDecrypt permission enum in the PoC.
  Overflow/chunking is not designed yet.

Scalar RHS:
  scalar = false means RHS is an encrypted handle and needs an ACL record.
  scalar = true means RHS is plaintext scalar bytes and does not need a RHS ACL record.
```

## How To Contribute Safely

```text
1. Start with a LiteSVM test in solana/runtime-tests/tests/host_events.rs.
2. Change confidential-token for app behavior.
3. Change zama-host only when host semantics need to change.
4. Keep README.md in sync with any behavior or API change.
5. Preserve negative tests for authorization-sensitive paths.
```

Prefer extending the canonical wrap/transfer/decrypt scenario over adding a second disconnected
demo flow.

## Commands

```bash
cd solana
NO_DNA=1 anchor build --ignore-keys
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
