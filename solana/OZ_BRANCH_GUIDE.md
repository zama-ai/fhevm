# OpenZeppelin Solana PoC Branch Guide

This branch already has the shared Solana PoC wiring. New ACL/token work should extend that wiring instead of creating a second Anchor workspace.

## Work In These Files

```text
solana/programs/zama-host
  Protocol-side host program.
  Owns ACL record accounts, FHE event emission, and compute-time ACL checks.

solana/programs/confidential-token
  App-side PoC program.
  Models the current cUSDC-like flow: wrap, transfer, rotate balance handles.

solana/runtime-tests/tests/host_events.rs
  Fast LiteSVM tests.
  Add most Solana behavior tests here first.

coprocessor/fhevm-engine/host-listener/src/solana_adapter.rs
  Solana event normalization into the existing coprocessor DB shape.

coprocessor/fhevm-engine/tfhe-worker/src/tests/solana_poc.rs
  Worker/KMS-shaped tests.
  Update this when event payloads, ACL semantics, or decrypt checks change.
```

Avoid adding a separate `library-solana/...` Anchor workspace unless the team explicitly decides to split the repo layout. A detached workspace will not exercise the branch's host listener, worker, KMS-shaped checks, or Rust Solana runtime tests.

## Current ACL Shape

The current record is:

```text
AclRecord PDA = PDA("acl-record", nonce_key, nonce_sequence)

nonce_key = H(
  "zama-acl-nonce-key-v1",
  acl_domain_key,
  app_account,
  encrypted_value_label
)

AclRecord data:
  handle
  nonce_key
  nonce_sequence
  acl_domain_key
  app_account
  encrypted_value_label
  subjects[]
  public_decrypt
```

For the confidential token PoC:

```text
acl_domain_key        = cUSDC mint pubkey
app_account           = Alice confidential token account
encrypted_value_label = "balance"
subjects              = [Alice, compute_signer]
compute_signer        = PDA("fhe-compute", cUSDC mint)
```

The handle is stored in the ACL record, but the handle is not a PDA seed. This is intentional because computed handles are opaque and may not be known early enough to derive Solana accounts from them.

## How To Port A New ACL Idea

Map new concepts into the existing names first:

```text
old "permission list" / "handler permissions" -> AclRecord
old "initial_key"                           -> nonce_key inputs, not a raw seed
old "output_index"                          -> nonce_sequence
old "context_key"                           -> subject or acl_domain_key, depending on meaning
old "fhe_authority"                         -> compute_signer only if it is app-controlled
old "external_input_authority"              -> future input verifier authority, not current token flow
```

Then update the smallest affected layer:

```text
Change ACL storage/checking?
  update zama-host + solana/runtime-tests

Change token behavior?
  update confidential-token + solana/runtime-tests

Change emitted host events?
  update zama-host + host-listener solana_adapter + worker tests

Change user decrypt semantics?
  update runtime KMS model tests + tfhe-worker solana_poc tests
```

## Test Loop

Run this before handing work back:

```bash
cd solana
anchor build
cargo test --workspace
```

If event shapes or worker/KMS-shaped logic changed, also run:

```bash
cd coprocessor/fhevm-engine
SQLX_OFFLINE=true cargo test -p tfhe-worker solana_user_decrypt_acl_invariants_match_evm_semantics --no-run
```

## PR Checklist

```text
No second Anchor workspace.
No TypeScript-only test path for core behavior.
No new standalone ACL program unless discussed first.
No new ACL terminology without mapping it to the glossary in solana/README.md.
Every authorization happy path has a negative test.
Any event field change updates listener and worker tests.
Any decrypt-relevant field can be verified by KMS from account data plus the signed request.
```

