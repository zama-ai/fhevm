# Future Design Requirements

Forward requirements and decisions the Solana port defers. Each item states what is built today and
what production needs, phrased as a requirement or an open decision — not a narrative. Cross-refs are
to [`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md).

## 1. Coprocessor signer set: forward work after DD-041

DD-041 settled the registered n-of-m signer set in `HostConfig` with admin-gated rotation. What
remains:

- A gateway-sync authority that mirrors the EVM `GatewayConfig` coprocessor registry into
  `set_coprocessor_signers`, instead of admin-driven rotation.
- The real proof / transciphering service that produces the attested ciphertext behind the signature.
- If a coprocessor quorum ever needs to carry more than a few signatures alongside a deep-history
  public-decrypt proof, the transaction may exceed one packet — see the DD-041 fit table and the
  fhevm-internal#1704 scratch-account two-tx fallback.

## 2. Attested contract naming on the gateway

DD-047 made program verification the protocol rule: every Store output's authority must be a PDA of
the declared `program`, and the attestation's `contract_address` must equal that program. What
remains open is only how the gateway names the attested contract (a program id, not a signing PDA).

## 3. Operator / delegated-transfer model

EVM `setOperator` / `confidentialTransferFrom` are **deliberately absent** (DD-009): one owner-signed
transfer authority, no operator rows. This is an intentional ERC7984 parity gap, not a Solana
constraint. Tracked as zama-ai/fhevm-internal#1692.

**Revisit** for RFQ / third-party settlement use cases. Any reintroduction must be a separate,
signature-backed authority design — not hidden operator compatibility in the token surface.

## 4. Arbitrary-receiver push payments

There is **no Solana analog by design**. The EVM transfer-and-call callback (a contract can't observe
an incoming transfer, so the token calls it back) was removed (DD-011); Solana apps drive their own
atomic join/deposit that CPIs `confidential_transfer` (see `confidential-batcher::join`).

**Requirement, if ever needed:** the only Solana idiom for token-driven receiver logic is a
Token-2022-style transfer hook, which is a **veto-only** primitive (it can reject a transfer, not run
privileged receiver logic). It is not a receiver callback and must not be documented as one.

## 5. Gateway RFC-021 reconciliation and host-listener event surface

The Solana input path uses the gateway `InputVerification.verifyProofRequestSolana` +
`VerifyProofRequestSolana` bytes32 entrypoint (kept, not renamed to V2 — DD-030). User-decrypt uses
`solanaUserDecryptionRequest` with the `solana-srfc38-user-decrypt-v1` payload (DD-026).

**Requirement:** keep the port and RFC-021 in sync as the gateway evolves. The Solana
host-listener reconstructs from confirmed Yellowstone instructions and inserts directly, while KMS
revalidates confirmed live authorization before plaintext release (DD-024, DD-025, DD-028). Wiring
the listener into the EVM block-status substrate (`host_chain_blocks_valid` +
`cmd/block_history.rs`) is optional resource-recovery work, not a release-authorization gate.

## Standing open decisions

The list is kept once, at the end of [`DESIGN_DECISIONS.md`](./DESIGN_DECISIONS.md) under "Open
product decisions". One item lives only here: one coprocessor indexing both zama-devnet and
zama-testnet would need `host_chains` to key by program ID as well as `chain_id` (DD-051).
