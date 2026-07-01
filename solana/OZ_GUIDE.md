# OpenZeppelin Solana PoC Guide

This is the short handoff guide for OpenZeppelin work on the Solana PoC branch.
The central technical guide is [README.md](./README.md), and durable design rationale lives in
[docs/DESIGN_DECISIONS.md](./docs/DESIGN_DECISIONS.md). Update those permanent docs whenever the
program shape, tests, ACL model, or event semantics change. Development pitfalls, failed
approaches, and unresolved implementation notes are tracked in the temporal
[DEVELOPMENT_ISSUES.md](./DEVELOPMENT_ISSUES.md) ledger.

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
  Mollusk behavior tests
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
  -> trivial_encrypt_and_bind creates initial total-supply handle + ACL record
  -> trivial_encrypt_and_bind creates zero initial balance handle + ACL record
  -> nonzero confidential supply enters through wrap or a future explicit mint flow

random handle birth
  -> fhe_rand_and_bind derives a random seed from slot context plus output nonce metadata
  -> fhe_rand_bounded_and_bind additionally validates a power-of-two upper bound for the FHE type
  -> emits FheRandEvent for worker ciphertext creation
  -> emits FheRandBoundedEvent for bounded worker ciphertext creation
  -> creates the first durable ACL record for the random handle

create_random_amount / create_random_bounded_amount
  -> token-level amount birth for Transfer or Burn amount labels
  -> records app_account = owner and domain = confidential mint
  -> grants compute_signer compute/use access for later token FHE graphs
  -> consumes ConfidentialTokenAccount.next_amount_nonce_sequence internally
  -> emits RandomAmountCreatedEvent for token-aware indexers

wrap_usdc
  -> SPL transfer_checked into vault
  -> trivial_encrypt_and_bind amount
  -> fhe_binary_op_and_bind_output Add into current balance
  -> fhe_binary_op_and_bind_output Add into encrypted total supply

confidential_transfer
  -> fhe_binary_op_and_bind_output Ge for encrypted sufficient-balance check
  -> fhe_binary_op_and_bind_output Sub for debit candidate
  -> fhe_ternary_op_and_bind_output IfThenElse for all-or-zero sender balance
  -> fhe_binary_op_and_bind_output Sub for encrypted transferred amount
  -> fhe_binary_op_and_bind_output Add for receiver balance
  -> rotates balance handle + ACL record for each changed token account

### SUPERSEDED (issue #1593): the transfer-and-call callback flow below was removed.
# The 3 callback instructions + confidential-token-receiver(-sdk) are deleted. Solana-native
# composition replaces them: a receiving app exposes its own `deposit` that CPIs
# confidential_transfer with the user as sole signer (authority flows through the CPI — no operator,
# no callback, no refund leg). See the confidential-deposit-app reference program. Kept for context.

confidential_call_transfer_receiver
  -> owner-gated split instruction that invokes a receiver hook program with caller-provided
     instruction data and remaining accounts
  -> no operator/delegated receiver-hook continuation is exposed in the production token surface
  -> verifies return data binding the encrypted callback-success witness to the prior transfer
  -> creates a one-shot TransferReceiverHookCall marker keyed by mint and sent_handle before any
     later callback settlement, blocking duplicate receiver CPIs for the same transferred handle
  -> confidential-token-receiver-sdk fixes the return magic, field order, encoder, decoder, and
     return-data setter
  -> programs/confidential-token-receiver is the external sample receiver using the SDK in runtime
     tests

confidential_prepare_transfer_callback / confidential_finalize_transfer_callback
  -> prepare requires the prior TransferReceiverHookCall marker and rejects settlement without a
     verified receiver return for the same sent handle and callback-success witness
  -> consumes the verified encrypted callback success bit with a recipient-side ACL witness
  -> prepare computes requested_refund, refund_success, recipient debit candidate, recipient balance,
     and encrypted refund
  -> finalize credits the sender with refund and computes final_transferred = sent - refund
  -> replay marker tracks prepared/finalized status

confidential_burn
  -> fhe_binary_op_and_bind_output Ge for encrypted sufficient-balance check
  -> fhe_binary_op_and_bind_output Sub for debit candidate
  -> fhe_ternary_op_and_bind_output IfThenElse for all-or-zero holder balance
  -> fhe_binary_op_and_bind_output Sub for encrypted burned amount
  -> fhe_binary_op_and_bind_output Sub from encrypted total supply
  -> rotates holder balance handle + mint total-supply handle

BalanceHandleUpdatedEvent
  -> emitted by confidential-token for app/frontend indexing
  -> not consumed by the generic coprocessor listener

user decrypt model tests
  -> signed authorization + handle entry + ACL record verification
  -> current and historical handles
```

Self-transfer is a no-op. It must not rotate handles or create output ACL records.

Transient sessions are one-shot. A valid session has exactly one capability, that capability can be
consumed once, and transient capabilities cannot enable public decrypt.

## Important Boundaries

```text
Token app checks token semantics:
  owner signed
  token account owner/mint match
  output ACL account belongs to the token account being updated

ZamaHost checks FHEVM semantics:
  host_config gates pause/test/mock behavior
  compute_subject is signer
  encrypted operand ACL records are canonical
  encrypted operands allow compute_subject with compute/use roles
  output ACL record PDA matches supplied nonce metadata
  persistent grants require ACL_ROLE_GRANT
  public decrypt requires ACL_ROLE_PUBLIC_DECRYPT

Host listener consumes only generic ZamaHost events:
  FheBinaryOpEvent
  TrivialEncryptEvent
  FheRandEvent
  AclAllowedEvent
  InputVerifiedEvent

Those event decoders are generated at host-listener build time from the checked-in ZamaHost Anchor
IDL snapshot. Use `solana/scripts/check-zama-host-idl.sh` to catch drift and
`solana/scripts/sync-zama-host-idl.sh` when a ZamaHost IDL change is intentional. The listener must
not parse confidential-token events.

Solana events are discovery and indexing signals, not authorization evidence. Unlike the EVM-style
listener flow, production Solana decrypt authorization must be checked against host-owned
ACL/material/delegation/replay account witnesses fetched at an accepted commitment level.
Anchor `emit_cpi!` is useful for the PoC listener, but each emitted event adds a ZamaHost self-CPI
frame; complex token -> host -> hook/settlement flows can run into Solana's hard CPI nesting limit.

App indexers consume token-local events:
  BalanceHandleUpdatedEvent
  TotalSupplyHandleUpdatedEvent
  ConfidentialTransferEvent
  ConfidentialBurnEvent
  BalanceDisclosureRequestedEvent
  AmountDisclosureRequestedEvent
  BalanceDisclosedEvent
  AmountDisclosedEvent
  BurnRedeemedEvent

KMS-style tests check decrypt semantics:
  signature scope
  ACL record owner
  canonical ACL record PDA
  handle match
  owner subject is allowed inline or through an overflow permission PDA
```

KMS does not parse token state and does not prove that a handle is the current balance. Currentness
comes from app state. ACL authorization is durable and can also apply to historical handles.

## Current Caveats

```text
# Superseded (reconciliation, June 2026): the input/decrypt caveats below were Ed25519/verifier-set
# shaped. The current model is on-chain secp256k1 over the coprocessor attestation (input) and over
# the KMS cert (decrypt), against a witness-pinned kms_context. See DESIGN_DECISIONS.md DD-007,
# DD-012, DD-021, DD-022, DD-026.

Input path:
  mock_input_verified_and_bind is a test short-circuit (chain-id confined, DD-014).
  The production path is the fhe_eval FheEvalOperand::VerifiedInput operand (the Solana
    FHE.fromExternal analog): consuming it re-verifies the coprocessor EIP-712 CiphertextVerification
    attestation on-chain via secp256k1 (recovers + threshold-checks the EVM coprocessor signers) and
    transient-allows the input for that eval only — NO persistent ACL, matching EVM verifyInput +
    allowTransient(input, msg.sender). The caller-is-contract gate is enforced at consumption
    (attestation.contract_address == compute_subject); derived durable outputs are unconstrained.
    confidential_transfer / confidential_burn consume an attested external amount via this operand.
  The redundant standalone verify_coprocessor_input instruction + InputVerifiedEvent were removed;
    the shared verifier lives in zama_host::instructions::input_verification.
  The removed verify_input_and_bind Ed25519 verifier-set path is the superseded design in DD-007.
  Real ZKPoK / transciphering still lives behind the attestation (PoC shortcut).

Public decrypt:
  allow_for_decryption is modeled and role-gated.
  confidential-token exposes request_disclose_balance / request_disclose_amount / request_burn_redemption,
  which create DisclosureRequest / BurnRedemptionRequest witness PDAs (kms_context_id, request_nonce,
  expires_slot, request_hash) BEFORE the secp consume (DD-022).
  disclose_amount_secp / disclose_balance_secp / redeem_burned_amount_secp require the ACL public-decrypt
  release flag, a sealed material witness, and an on-chain secp256k1 KMS-cert verification against the
  WITNESS-PINNED kms_context (rejects high-s; consume-once; replay/expiry/context-mismatch rejected).
  A compute_signer does not automatically get public_decrypt authority.

Persistent grants:
  allow_acl_subjects mutates the existing canonical ACL record.
  It does not create a second ACL record for the same handle.

Subjects:
  inline subjects store Pubkey + role flags.
  Overflow subjects use PDA("acl-permission", acl_record, subject).
  Durable ACL/material/delegation/replay evidence is intentionally not closeable in this PoC.
  TransientSession accounts are the current host-side reclaimable rent-bearing accounts.

KMS connector:
  chain_kind = "solana" fails closed for decryption ACL checks unless Solana witnesses are present.
  Native witness helpers now decode and verify ACL records, overflow permission PDAs,
  delegation PDAs, public decrypt flags, domain scope, and material commitment PDAs.
  Native-v0 request admission helpers now bind request hashes, native extra data, app-context
  membership, material witnesses, and replay-key decisions.
  Native-v0 request signature helpers verify Ed25519 signatures over the canonical request-hash
  message.
  A durable native-v0 replay reservation table/helper now records signed Solana replay keys and
  request hashes atomically.
  A native-v0 request parser now extracts payload fields, handle entries, raw extra data,
  reencryption keys, request signatures, and the exact account fetch plan before attaching fetched
  account witnesses into the admission request type.
  A native-v0 live-path boundary now checks account-fetch count, RPC response size,
  requested/observed commitment ordering, signed validity windows, and non-finalized reads that
  must be rechecked before release.
  A native-v0 Solana JSON-RPC getMultipleAccounts fetcher now reads one response-context slot,
  decodes base64 account data, and preserves owner pubkeys for witness attachment.
  A native-v0 finality recheck helper now re-fetches accepted non-finalized witness accounts at
  finalized commitment, compares them byte-for-byte against the accepted snapshot, and re-runs
  admission before release.
  A native-v0 admission service invokes witness verification, signature verification, and replay
  reservation for already-fetched request witnesses.
  Native-v0 response helpers now verify accepted-request binding, response body hashes,
  release-pinned signer-set hashes, certificate thresholds, sorted signer identities, and Ed25519
  KMS response signatures.
  A native-v0 live-flow boundary now composes signed request parsing, RPC/account fetch, admission,
  finalized-account recheck, response certificate verification, and response-route metadata.
  Durable native-v0 Solana decryption request/response tables and a store boundary now cover
  parsed request insertion, pending request picking, release metadata, and verified response rows.
  A native-v0 notification/picker boundary now listens for those request/response rows and returns
  native work items without widening the Gateway ProtocolEvent enum.
  tx-sender has a separate native-v0 Solana response row picker/status/publisher boundary for
  verified native response rows, but the concrete Solana publication target is still not selected.
  The branch has Gateway-PoC extraData witness wiring for public, direct user, and delegated user
  decrypt checks. The live connector path still needs KMS Core native work-item dispatch and
  concrete tx-sender/relayer publication.

Scalar RHS:
  scalar = false means RHS is an encrypted handle and needs an ACL record.
  scalar = true means RHS is plaintext scalar bytes and does not need a RHS ACL record.
```

## How To Contribute Safely

```text
1. Start with a Mollusk test in solana/runtime-tests/tests/host_mollusk.rs or
   solana/runtime-tests/tests/token_mollusk.rs.
2. Change confidential-token for app behavior.
3. Change zama-host only when host semantics need to change.
4. Keep README.md and docs/DESIGN_DECISIONS.md in sync with any durable behavior or API change.
5. Preserve negative tests for authorization-sensitive paths.
```

Prefer extending the canonical wrap/transfer/decrypt scenario over adding a second disconnected
demo flow.

## Commands

```bash
cd solana
bash scripts/check-zama-host-idl.sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
