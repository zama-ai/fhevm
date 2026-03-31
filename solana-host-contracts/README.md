# Solana Host Contracts

This package contains a local Rust foundation for porting the FHEVM host contracts to Solana.

The implementation focuses on behavioral parity for the host-side responsibilities:

- symbolic FHE executor handle generation
- ACL and delegated user decryption semantics
- input verifier context and proof-shape validation
- KMS verifier context management
- HCU block and transaction metering
- normalized event types suitable for a Solana-specific host-listener

The shared host logic still keeps proof verification abstract at the trait boundary, but the
on-chain wrapper now ships with a concrete secp256k1 recovery backend for input admission and
KMS decryption-proof verification.

For local development, the workspace now uses an Anchor-driven localnet workflow plus `make`
wrappers for bootstrapping the Solana host program: build the SBF artifact, start a local
validator, load the program at a stable address, initialize the host state PDA, and write address
artifacts under `addresses/`.

## Current Scope

- Implemented as a small Cargo workspace with:
  - `core`: shared Rust host logic
  - `program`: deployable Solana `solana-program` wrapper
  - `local-cli`: local initialization helper used by the Anchor bootstrap to create the PDA state and write address artifacts
- Includes a program-facing instruction and state dispatcher layer
- Includes an actual Solana entrypoint and processor for program-owned state accounts
- Uses Borsh-encoded instructions and state for the on-chain wrapper
- Includes a deterministic PDA helper for the state account using seed `b"host-state"`
- Includes a deterministic PDA helper for the transient session account using seed `b"host-session"`
- Creates the state PDA on chain through `system_program::create_account` when `InitializePda` is invoked against a blank PDA
- Resizes the program-owned state account on demand through `system_program` CPI plus rent top-up when serialized state growth exceeds the current account size
- Uses fixed-width serialized threshold fields (`u32`) instead of `usize` in on-chain-facing state and instruction types
- Wraps program state with an explicit account discriminator and layout-version header before deserializing
- Includes an on-chain secp256k1 proof backend built on `solana_program::secp256k1_recover`
- Enforces explicit upper bounds for batch size, verifier signer sets, proof payloads, and decrypted-result sizes
- Writes local deployment artifacts to `addresses/.env.host` and `addresses/localnet.json`
- No direct integration with the existing coprocessor listener yet
- HCU pricing is seeded from the Solidity host-contract tariff table and enforced by the shared dispatcher
- Input verification keeps the Solidity-compatible EIP-712 field layout for the gateway-facing verifier context
- KMS decryption verification keeps the existing Ethereum-style typed-data layout

## Project Layout

- `core/src/executor.rs`: symbolic FHE operations and deterministic handle generation
- `core/src/acl.rs`: ACL, allow-for-decryption, deny-list, pausers, delegation/revocation
- `core/src/input_verifier.rs`: input-proof parsing, context validation, cached verification
- `core/src/kms_verifier.rs`: KMS context lifecycle and public decryption proof validation
- `core/src/hcu.rs`: HCU caps, pricing, whitelisting, and transaction metering
- `core/src/instructions.rs`: shared instruction/config types
- `core/src/program.rs`: `HostProgramState`, transaction-local sessions, and instruction dispatch
- `core/src/onchain_interface.rs`: shared PDA helpers and Borsh instruction encoding for the on-chain wrapper
- `core/src/secp256k1_verifier.rs`: concrete secp256k1 recovery backend for proof verification
- `core/tests/host_contracts.rs`: parity-oriented tests for executor, ACL, verifiers, HCU, and admin flows
- `core/tests/secp256k1_proof_verifier.rs`: proof-verifier tests using real secp256k1 signatures
- `program/src/onchain.rs`: Solana instruction decoding, account load/save, batch execution, and event logging
- `program/src/entrypoint.rs`: Solana program entrypoint
- `program/src/onchain_program_tests.rs`: low-level on-chain processor tests using real `AccountInfo` values
- `program/src/program_test.rs`: BPF-backed `solana-program-test` coverage for PDA initialize + resize + execute flows
- `local-cli/src/main.rs`: local initialization helper used by the Anchor bootstrap flow to create the PDA state and write address artifacts
- `Anchor.toml`: Anchor localnet configuration, genesis program loading, and pre-test SBF build hook
- `tests/anchor-localnet.mjs`: Anchor bootstrap helper that can either initialize the disposable local verification flow or start a persistent `anchor localnet` session and write address artifacts
- `tests/fixtures/anchor-authority.json`: deterministic local authority wallet used by Anchor

## How To Compile

From the repository root, build the whole workspace:

```bash
make -C solana-host-contracts build
```

Release build:

```bash
make -C solana-host-contracts build-release
```

Build only the shared logic crate:

```bash
make -C solana-host-contracts build-core
```

Build only the deployable Solana program crate:

```bash
make -C solana-host-contracts build-program
```

If you are already inside `solana-host-contracts/`, you can also run:

```bash
make build
```

If the Solana toolchain is installed locally and you want to produce the SBF artifact, build the `program` crate:

```bash
make -C solana-host-contracts build-sbf
```

If `cargo build-sbf` is not installed yet, install it with:

```bash
cargo install cargo-build-sbf --version 4.0.0
```

The SBF build was verified in this repository. The output artifact is:

- `solana-host-contracts/target/deploy/solana_host_contracts.so`
- Splitting the workspace into `core` and `program` removes the previous mixed `rlib` plus `cdylib` warning from the SBF build

## Anchor Localnet

The local workflow now follows Anchor conventions instead of custom shell orchestration, and the
recommended entry points are the `make` targets in this directory.

`Anchor.toml` does three important things:

- compiles the SBF artifact before tests through a `pre-test` hook
- starts `solana-test-validator` on a stable local RPC/WS/faucet port set
- loads the Solana host program at genesis at a fixed program ID:
  - `5TeWSsjg2gbxCyWVniXeCmwM7UtHTCK7svzJr5xYJzHf`

Then the Anchor bootstrap helper:

- funds the Anchor authority wallet
- initializes the host state PDA
- writes `addresses/.env.host`
- writes `addresses/localnet.json`
- copies `addresses/.env.host` to `addresses/.env.local`

## How To Run A Local Validator

The local workflow follows the same broad pattern as the EVM host contracts:

1. start a disposable local chain
2. deploy the contracts/program
3. write address artifacts under `addresses/`
4. run smoke tests or attach other local tooling

### Prerequisites

You need the Solana CLI, Anchor CLI, Node.js, and Rust toolchain installed locally:

```bash
solana --version
solana-keygen --version
anchor --version
cargo build-sbf --help
node --version
```

If your Solana or Anchor installation is exposed through your shell startup files, source them
before using the `make` targets:

```bash
source ~/.zshrc
```

The default one-command local bootstrap is:

```bash
cd solana-host-contracts
source ~/.zshrc
make localnet
```

That command:

- runs `make build-sbf`
- runs `node tests/anchor-localnet.mjs live`
- starts `anchor localnet`
- waits for the validator to come up and the program to be deployed
- funds the Anchor authority wallet
- initializes the state PDA
- writes `addresses/.env.host`
- writes `addresses/localnet.json`
- copies `addresses/.env.host` to `addresses/.env.local`
- keeps the validator running until you stop it with `Ctrl-C`

When you stop `make localnet` with `Ctrl-C`, the validator shuts down as expected. `make` may
still exit with a non-zero status because the interrupt is propagated to the parent process.

If the validator is already running on the configured RPC port, `make localnet` now exits early
with an explicit message telling you to use `make localnet-test-existing` or
`make localnet-bootstrap` instead of surfacing the raw Anchor port-binding error.

If you want a disposable one-shot verification run instead, use:

```bash
source ~/.zshrc
make localnet-test
```

That target first builds the SBF artifact and then runs the disposable Anchor verification flow.

Both `make localnet` and `make localnet-test` expect the configured local ports to be free:

- RPC `18999`
- WS `19000`
- faucet `19900`

Do not run `make localnet-test` while `make localnet` is still active.

If you want to bootstrap the address artifacts against an already-running local validator without
starting a new one, use:

```bash
source ~/.zshrc
make localnet-bootstrap
```

### Two-Terminal Flow

If you want to keep a local validator running in one terminal and run verification against that
existing localnet from another terminal, use this split:

Terminal 1:

```bash
cd /Users/panagiotisganelis/Projects/zama/fhevm/solana-host-contracts
source ~/.zshrc
make localnet
```

Terminal 2:

```bash
cd /Users/panagiotisganelis/Projects/zama/fhevm/solana-host-contracts
source ~/.zshrc
make localnet-test-existing
```

`make localnet-test-existing` runs `anchor test --skip-local-validator`, so it reuses the already
running validator instead of trying to start another one on the same ports.

### Address Artifacts

The Anchor setup step writes the same kind of machine-readable address output that the EVM
host-contract tooling uses for local development:

- `addresses/.env.host`
  - primary env-style artifact for local tooling
- `addresses/.env.local`
  - compatibility copy for ad hoc local scripts
- `addresses/localnet.json`
  - structured JSON artifact for future listener or service wiring

The env file currently contains:

- Solana RPC and WS URLs
- `SOLANA_HOST_KIND=solana`
- deployed program ID
- state PDA
- session PDA for the configured authority
- authority pubkey
- host chain ID
- gateway chain ID
- gateway-facing verifier contract addresses
- coprocessor and KMS signer addresses plus thresholds

This is the file shape you should plan to feed into a future Solana host-listener bootstrap step.

### Make Targets

- `make -C solana-host-contracts build`
- `make -C solana-host-contracts build-release`
- `make -C solana-host-contracts build-core`
- `make -C solana-host-contracts build-program`
- `make -C solana-host-contracts build-sbf`
- `make -C solana-host-contracts test`
- `make -C solana-host-contracts test-core`
- `make -C solana-host-contracts test-core-nocapture`
- `make -C solana-host-contracts test-program`
- `make -C solana-host-contracts test-no-run`
- `make -C solana-host-contracts localnet`
- `make -C solana-host-contracts localnet-live`
- `make -C solana-host-contracts localnet-bootstrap`
- `make -C solana-host-contracts localnet-test`
- `make -C solana-host-contracts localnet-test-existing`
- `make -C solana-host-contracts show-addresses`

`localnet-live` is currently just an alias of `localnet`.

## Framework Options

This workspace now uses Anchor as the localnet framework.

Other useful tools, but not a full replacement for this Anchor/localnet setup:

- Surfpool
  - good local validator developer experience and cluster-loading workflow
  - more useful when you want a richer Solana dev sandbox than a parity-oriented local listener target
- Mollusk
  - great for fast deterministic instruction-level tests
  - not a replacement for a persistent validator plus deploy flow

## How To Run Tests

Run the full test suite:

```bash
make -C solana-host-contracts test
```

Run only the shared-logic parity tests:

```bash
make -C solana-host-contracts test-core
```

Show test output:

```bash
make -C solana-host-contracts test-core-nocapture
```

Build the SBF artifact and then run the validator-backed BPF test:

```bash
make -C solana-host-contracts build-sbf
make -C solana-host-contracts test-program
```

Compile tests without running them:

```bash
make -C solana-host-contracts test-no-run
```

Run the disposable local-validator verification flow:

```bash
source ~/.zshrc
make -C solana-host-contracts localnet-test
```

Use that when you want an Anchor-driven deployment/bootstrap verification in addition to the Rust test suite.

Run the verification flow against an already-running local validator:

```bash
source ~/.zshrc
make -C solana-host-contracts localnet-test-existing
```

Current tests cover:

- executor result-handle generation and transient ACL access
- ACL delegation and revocation semantics
- input-proof parsing and session caching
- KMS context selection and context destruction rules
- HCU transaction and block-cap enforcement
- owner and upgrade-authority checks in the shared dispatcher
- pauser and HCU admin flows through the dispatcher
- PDA initialization through the actual Solana processor
- PDA creation and account-growth resize through `system_program` CPI plus explicit rent-sysvar handling
- on-chain `VerifyInput` and `VerifyDecryptionSignatures` flows through the actual Solana processor
- a BPF-backed `solana-program-test` flow from the `program` crate that creates the PDA, resizes state through admin growth, and executes `FheRand`
- real secp256k1 signature verification for both input and KMS proofs
- local CLI compilation as part of the workspace build

## Upgrade Model

This crate currently models upgrades in two layers.

### 1. Local state-model upgrade

`HostProgramState` stores:

- `owner`
- `upgrade_authority`
- `state_version`

The program-facing dispatcher exposes a `Migrate { new_state_version }` instruction. The current behavior is:

- only `upgrade_authority` may call `Migrate`
- migrations are monotonic: `new_state_version` must be strictly greater than the current version
- the migration updates state version only; it does not yet transform serialized accounts because the crate is still pure Rust logic

This models the contract/account migration step the on-chain wrapper will need once versioned account migrations are added.

### 2. Intended real Solana deployment model

For real deployment of the on-chain program, the expected upgrade path is:

- deploy with Solana's upgradeable loader
- keep program-binary control under the Solana upgrade authority
- store explicit version fields in program-owned state accounts
- introduce versioned migration instructions such as `migrate_v2`, `migrate_v3`
- preserve owner-governed functional admin flows separately from upgrade authority

This is the Solana equivalent of the EVM proxy-plus-reinitializer lifecycle. The current crate already separates those responsibilities by keeping `owner` and `upgrade_authority` distinct in the local model.

## State Account Initialization

The on-chain wrapper supports two initialization paths:

- `Initialize`
  - Accepts any caller-provided program-owned state account
- `InitializePda`
  - Requires the state account to match the PDA derived from seed `b"host-state"`
  - Can create the PDA on chain when the caller supplies:
    - the payer/authority signer
    - the blank PDA account
    - the `system_program`
    - the rent sysvar account

The helper exported by the crate is:

- `solana_host_contracts::onchain::find_state_pda(program_id)`
- `solana_host_contracts::onchain::required_state_account_len(config)` for the minimum initial serialized size
- `solana_host_contracts::onchain::state_account_len_with_reserve(config, reserve_bytes)` for a safer preallocation target when dynamic maps or sets will grow

State-growth behavior:

- if serialized state exceeds the current account size during `Initialize`, `InitializePda`, or `ExecuteBatch`, the wrapper can top up rent and call `AccountInfo::resize`
- resize requires the caller to provide both the `system_program` account and the rent sysvar account in the instruction account list
- `required_state_account_len` is only the minimum size for the initial state snapshot; extra reserve still reduces the frequency of resize CPIs during admin growth

## Emitted Events

The crate emits normalized `HostEvent` values intended for a future Solana-specific host-listener. These are logical events, not Solana log-format bindings yet.

In the on-chain wrapper, emitted events are logged in two formats:

- `sol_log_data(["HOST_EVENT", payload])` for Solana-native structured log parsing
- `HOST_EVENT:<hex>` for a simple deterministic text parse target during local testing

### Executor and Input Events

- `Operation`
  - Emitted for executor unary, binary, ternary, cast, trivial encrypt, random, and bounded-random operations
  - Includes caller, operator, operands, scalar flag, result type, and result handle
- `VerifyInput`
  - Emitted when an input proof is accepted through the executor path
  - Includes caller, input handle, user address, proof length, input type, and result handle

### ACL Events

- `Allowed`
  - Emitted when persistent ACL permission is granted for a handle/account pair
- `AllowedForDecryption`
  - Emitted when handles are marked decryptable
- `DelegatedForUserDecryption`
  - Emitted when a user-decryption delegation is created or updated
- `RevokedDelegationForUserDecryption`
  - Emitted when a delegation is revoked
- `BlockedAccount`
  - Emitted when the owner blocks an account
- `UnblockedAccount`
  - Emitted when the owner unblocks an account

### Verifier Events

- `InputVerifierContextUpdated`
  - Emitted when the input-verifier signer set or threshold changes
- `KmsContextUpdated`
  - Emitted when a new KMS context is defined
- `KmsContextDestroyed`
  - Emitted when a non-current KMS context is destroyed

`VerifyDecryptionSignatures` currently returns an `InstructionResult` with `verified = Some(true)` on success but does not emit a dedicated `HostEvent`.

### HCU Events

- `HcuPerBlockSet`
  - Emitted when the block HCU cap changes
- `MaxHcuDepthPerTxSet`
  - Emitted when the max per-transaction depth changes
- `MaxHcuPerTxSet`
  - Emitted when the max HCU per transaction changes
- `BlockHcuWhitelistAdded`
  - Emitted when an account is added to the block-HCU whitelist
- `BlockHcuWhitelistRemoved`
  - Emitted when an account is removed from the block-HCU whitelist

### Admin Actions That Are Currently Silent

The current local dispatcher performs some admin actions without producing a `HostEvent`:

- `AddPauser`
- `Pause`
- `Unpause`
- `Migrate`

Those actions still mutate state and are covered by tests, but they do not yet produce normalized events. If we need listener visibility for those flows later, we should add explicit event variants before wiring the crate into a real Solana host-listener.

## Limitations

- The Anchor bootstrap flow depends on the external Solana CLI and Anchor CLI binaries
- `make localnet-test` still stops the validator when the test command exits; use `make localnet` when you want a persistent node for future host-listener work
- `make localnet` shuts down cleanly on `Ctrl-C`, but `make` may still report a non-zero exit status because the interrupt is forwarded
- The BPF-backed `program_test` expects `solana-host-contracts/target/deploy/solana_host_contracts.so`; if you have not run `make build-sbf` yet, the test exits early instead of exercising the runtime path
- The current Anchor bootstrap initializes the host state PDA and writes artifacts, but it does not yet start a Solana-side host-listener; richer listener-driven scenarios should be added once that component work begins
