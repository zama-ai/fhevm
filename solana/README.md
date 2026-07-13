# Solana FHEVM PoC

This workspace ports the Zama FHEVM host to Solana. It keeps the EVM FHEVM *spirit* — the same ACL,
input-verification, and decrypt trust model — and re-expresses it in Solana idiom (accounts, PDAs,
CPI, signer propagation) instead of transliterating Solidity.

It is a proof of concept: one end-to-end path (input → eval → compute → user/public decrypt) is
real enough to reason about ACL, event listening, worker compute, and decrypt from code and tests.
It does not settle the final product shape — see [`docs/FUTURE_DESIGN.md`](docs/FUTURE_DESIGN.md).

## What is here

```text
programs/zama-host              Protocol host program. Owns canonical `EncryptedValue` ACL accounts
                                (one stable PDA per logical encrypted value, handle updates sealed into
                                an on-account MMR), verifies FHE-op authorization, verifies coprocessor
                                input attestations and KMS certs on-chain (secp256k1), emits generic
                                host events (the ACL lifecycle itself is event-free; see DD-033).
programs/confidential-token     App program. Minimal confidential-token / cUSDC wrapper (ERC7984
                                spirit): wrap, transfer, burn, redeem, disclose. Drives zama-host by CPI.
programs/confidential-deposit-app  Reference app showing Solana-native composition: its `deposit`
                                CPIs `confidential_transfer` with the user as sole signer — the
                                replacement for EVM transfer-and-call callbacks.
crates/zama-fhe                 App-facing SDK: typed `EvalBuilder`, `Encrypted<T>`, `DurableSlot`,
                                `AccessPolicy`, and a `cpi`-feature account resolver for `fhe_eval` plans.
crates/solana-ed25519-instruction  Ed25519 instruction-sysvar helpers.
runtime-tests                   Fast evaluator/plan contracts plus real-SBF Mollusk host/token
                                suites; see docs/TESTING.md for the evidence each layer provides.
scripts/poc                     Live end-to-end scripts against a local validator + fhevm-cli stack.
geyser                          Yellowstone plugin build helpers for the account/event stream.
```

The account boundary is the core design: app state lives in `confidential-token`, canonical ACL
state lives in `zama-host`'s `EncryptedValue` accounts, and opaque FHE handles are stored *inside*
those accounts — never used as PDA seeds — so apps can pre-allocate output ACL accounts before the
compute result is known.

## Build and test

From `solana/`:

```bash
# Verify the production IDL/ABI snapshot, then rebuild the local SBF artifacts the Mollusk
# runtime tests need (built with the `poc` feature that enables the test-only shims).
bash scripts/check-zama-host-idl.sh

cargo test --workspace
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

If the ZamaHost Anchor IDL changes intentionally, refresh the vendored listener snapshot with
`bash scripts/sync-zama-host-idl.sh`. See [`docs/TESTING.md`](docs/TESTING.md) for exact focused
commands, what each test layer proves, Mollusk specifics, and build traps.

Live end-to-end run against a local validator (mainnet-safe, validator pinned to `127.0.0.1:8899`):

```bash
bash scripts/poc/clean-e2e.sh              # bring up fhevm-cli + Solana side-stack
bash scripts/poc/full-vertical.sh          # compute -> public-decrypt -> user-decrypt
bash scripts/poc/adversarial-l4.sh         # negative: relayer-bypass + context-rotation rejection
```

## Integrating an app

An app program drives compute by CPI into `zama-host`, using `crates/zama-fhe`:

- Verify an external encrypted input by consuming it as the `fhe_eval` `FheEvalOperand::VerifiedInput`
  operand (the Solana `FHE.fromExternal` analog): the host re-verifies the coprocessor attestation
  in-frame and transient-allows it for that eval only. Bind the attestation to your program's
  **compute-authority PDA** (in `confidential-token`, `[b"fhe-compute", mint]`) and check the attested
  `user_address` yourself — the host only enforces `attestation.contract_address == compute_subject`.
- Compose atomic multi-account effects (e.g. debit sender + credit receiver) as one `fhe_eval` frame
  with per-output authority signer witnesses, using `EvalBuilder` + `DurableSlot`.
- To receive confidential funds, expose your own instruction that CPIs `confidential_transfer` with
  the user as sole signer (authority propagates through the CPI). See `confidential-deposit-app`.
  There is no receiver-callback / transfer-and-call path — that EVM workaround is not needed on Solana.

## Read before changing behavior

- [`docs/DESIGN_DECISIONS.md`](docs/DESIGN_DECISIONS.md) — the numbered design decisions (DD-001…),
  their status, and rationale. Read this before touching ACL, input verification, decrypt, KMS
  context, event transport, or token-transfer behavior.
- [`docs/EVM_PARITY.md`](docs/EVM_PARITY.md) — capability-by-capability EVM → Solana mapping.
- [`docs/FUTURE_DESIGN.md`](docs/FUTURE_DESIGN.md) — the forward design requirements and open decisions.
- Rustdoc in `programs/*` is authoritative for account layouts, roles, PDA seeds, and instruction invariants.
