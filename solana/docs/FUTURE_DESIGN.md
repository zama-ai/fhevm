# Future Design Requirements

Forward requirements and decisions the Solana port defers. Each item states what is built today and
what production needs, phrased as a requirement or an open decision — not a narrative. Cross-refs are
to [`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md).

## 1. Coprocessor signer set: single signer → registered n-of-m

Today `HostConfig` holds one `coprocessor_signer: [u8; 20]` verified at threshold 1
(`state/host_config.rs`). Input-attestation verification recovers that single EVM address and checks
it (`eip712::verify_coprocessor_input`).

**Requirement:** move input trust to a registered n-of-m coprocessor signer set. The threshold
machinery already exists — `eip712::verify_threshold` (distinct-signer counting, high-s rejection) is
used for the KMS cert path and should back input verification too — and the shape to follow is the
`KmsContext` PDA (`state/kms_context.rs`: a `context_id`-keyed signer set + per-operation thresholds,
synced from the EVM registry). Decision needed: whether input signers get their own context PDA or
share the KMS context surface.

## 2. Canonicalize the compute-authority-PDA binding convention

The host enforces only `attestation.contract_address == compute_subject` (the msg.sender analog).
The convention that `compute_subject` is an app compute-authority PDA (e.g. `[b"fhe-compute", mint]`)
is **app policy**, not protocol-enforced. `EvalBuilder` cannot assert it because `compute_subject` is
only known at eval-execution time.

**Decision needed:** lift the PDA-binding discipline to a protocol-level assertion, or codify it as an
SDK guardrail (documented convention + `zama-fhe` helper), or leave it as app responsibility. If
protocol-level, define how the host recognizes an app's canonical compute PDA without coupling to a
specific seed layout.

## 3. Operator / delegated-transfer model

EVM `setOperator` / `confidentialTransferFrom` are **deliberately absent** (DD-009): one owner-signed
transfer authority, no operator rows. This is an intentional ERC7984 parity gap, not a Solana
constraint.

**Revisit** for RFQ / third-party settlement use cases. Any reintroduction must be a separate,
signature-backed authority design — not hidden operator compatibility in the token surface.

## 4. Arbitrary-receiver push payments

There is **no Solana analog by design**. The EVM transfer-and-call callback (a contract can't observe
an incoming transfer, so the token calls it back) was removed (DD-011); Solana apps drive their own
atomic `deposit` that CPIs `confidential_transfer` (see `confidential-deposit-app`).

**Requirement, if ever needed:** the only Solana idiom for token-driven receiver logic is a
Token-2022-style transfer hook, which is a **veto-only** primitive (it can reject a transfer, not run
privileged receiver logic). It is not a receiver callback and must not be documented as one.

## 5. Gateway RFC-021 reconciliation and host-listener event surface

The Solana input path uses the gateway `InputVerification.verifyProofRequestSolana` +
`VerifyProofRequestSolana` bytes32 entrypoint (kept, not renamed to V2 — DD-030). User-decrypt uses the
typed `userDecryptionRequestSolana` entrypoint (DD-026).

**Requirement:** keep the PoC ↔ RFC-021 mirror in sync as the gateway evolves. The Solana
host-listener reconstructs from confirmed Yellowstone instructions and inserts directly, while KMS
revalidates confirmed live authorization before plaintext release (DD-024, DD-025, DD-028). Wiring
the listener into the EVM block-status substrate (`host_chain_blocks_valid` +
`cmd/block_history.rs`) is optional resource-recovery work, not a release-authorization gate.

## Standing open decisions

Carried from `DESIGN_DECISIONS.md` "Open Product Decisions":

- Replace the PoC sentinel `chain_id` (`SOLANA_POC_CHAIN_ID`) with the repository-wide high-bit
  Solana chain-id convention.
- Durable archival / compaction policy for ACL, material, delegation, and replay evidence (no
  `close_acl_record` today).
- Confidential-balance profile: keep the immediate available-balance profile or move to staged
  inbound-credit (DD-016).
- Full production KMS-connector wiring and real ZKPoK / transciphering behind the input attestation
  (both are PoC shortcuts today — DD-028).
