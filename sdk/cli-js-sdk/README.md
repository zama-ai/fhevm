# cli-fhevm-sdk

Command-line examples for `@fhevm/sdk` viem flows against the `FHETest` contract.

The CLI can:

- create encrypted inputs and input proofs
- public decrypt FHETest handles
- user decrypt private FHETest handles
- delegated user decrypt handles owned by another account
- initialize, inspect, and run operation demos on FHETest
- send ERC-7984 confidential token transfers and read confidential balance handles
- install shell completion for commands, options, and choice values

Progress logs go to stderr. Final results go to stdout as JSON, so commands are pipeable.

## Workspace Layout

This repository is a pnpm workspace:

- `packages/toolkit` — importable library layer wrapping `@fhevm/sdk`: config, encryption, decrypt flows, FHETest helpers. No CLI dependencies.
- `packages/cli` — the `fhevm-sdk` commander CLI, consuming the toolkit via `workspace:*`.
- `packages/load-test` — private relayer load-test application with validated scenarios, durable pools, collectors, reports, and explicit baselines. It is not an SDK library or published package.

## Quick Start

```bash
pnpm install
cp .env.example .env
pnpm run build
cd packages/cli && pnpm add -g .
fhevm-sdk --help
```

`pnpm run build` compiles the TypeScript CLI to `packages/cli/dist/` with `tsdown`; the linked `fhevm-sdk` binary runs that compiled output. Without linking, replace `fhevm-sdk` with:

```bash
pnpm --silent run cli
```

For source-mode development without rebuilding, use:

```bash
pnpm --silent run cli:dev
```

The separate operator load-test runs from source:

```bash
pnpm load-test --help
```

See [`packages/load-test/README.md`](packages/load-test/README.md) before generating pools or running a suite.

To remove the global link later, remove the globally linked package:

```bash
pnpm remove --global cli-fhevm-sdk
```

The repository `.env` is loaded automatically, even when you run the CLI from another directory. Shell variables override `.env` values.
Explicit credential flags override environment credentials.

## First Commands

These are good smoke tests once your RPC and wallet environment are set:

```bash
fhevm-sdk fhe-test info
fhevm-sdk input-proof --type uint64 --value 42
fhevm-sdk fhe-test init --type uint32
fhevm-sdk public-decrypt fresh --type uint8
fhevm-sdk user-decrypt fresh --type uint16
```

Global options such as `-n devnet`, `--rpc-url`, and `--relayer-url` can be placed before or after subcommands.

## Command Map

| Command                              | What it does                                                                                                              | Needs wallet?                                       |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| `input-proof`                        | Encrypts clear values and requests a verified input proof. Does not write to FHETest.                                     | No                                                  |
| `public-decrypt direct`              | Public decrypts existing `--handle` values from any contract (`--contract` sets the ACL pairing, default FHETest).        | Only when the pairing contract needs a caller       |
| `public-decrypt fresh`               | Encrypts a value, stores it in FHETest with `makePublic=true`, then public decrypts it.                                   | Yes                                                 |
| `public-decrypt stored`              | Public decrypts the FHETest handles stored in explicit `--account`/`--type` slots, or the wallet's default slot.          | Only when using the wallet default                  |
| `public-decrypt make-public`         | Marks the caller's stored FHETest handle public, then public decrypts it.                                                 | Yes                                                 |
| `user-decrypt direct`                | User decrypts existing `--handle` values from any contract (`--contract` sets the ACL pairing, default FHETest).          | Yes                                                 |
| `user-decrypt fresh`                 | Encrypts a value, stores it in FHETest with `makePublic=false`, then decrypts it as the owner.                            | Yes                                                 |
| `user-decrypt stored`                | User decrypts the caller's stored FHETest handles for `--type` slots.                                                     | Yes                                                 |
| `delegated-user-decrypt direct`      | Delegate decrypts existing `--handle` values from any contract (`--contract` sets the ACL pairing, default FHETest).      | Delegate; delegator only if creating ACL permission |
| `delegated-user-decrypt fresh`       | Delegator creates a private handle; delegate gets ACL permission and decrypts it.                                         | Delegate and delegator                              |
| `delegated-user-decrypt stored`      | Delegate decrypts a delegator's stored FHETest handles for `--type` slots.                                                | Delegate; delegator only if creating ACL permission |
| `verify-user-decrypt`                | Decrypts and compares a relayer user-decrypt GET response using a saved validation artifact; derives the GET URL from the artifact. | No wallet; needs RPC                                |
| `fhe-test info`                      | Shows resolved network, host chain, relayer, and FHETest metadata.                                                        | No                                                  |
| `fhe-test inspect`                   | Reads FHETest state for a raw handle, an explicit account/type slot, or the wallet's default account/type slot.           | Only when using the wallet default                  |
| `fhe-test init`                      | Creates public FHETest handles for one, several, or all supported types.                                                  | Yes                                                 |
| `fhe-test op <operation>`            | Runs an FHETest operation against the caller's stored handle.                                                             | Yes                                                 |
| `token transfer`                     | Encrypts an amount and runs an ERC-7984 confidential transfer, or `--from` for `confidentialTransferFrom`; `--verify` decrypts the sender balance before/after to confirm the transfer. | Yes                                                 |
| `token balance`                      | Reads the confidential ERC-7984 balance handle for the wallet or an explicit account.                                     | Only when using the wallet default                  |
| `completion install`                 | Installs shell completion.                                                                                                | No                                                  |
| `completion uninstall`               | Uninstalls shell completion.                                                                                              | No                                                  |

Use `--help` on any command for exact options:

```bash
fhevm-sdk public-decrypt stored --help
fhevm-sdk delegated-user-decrypt fresh --help
fhevm-sdk fhe-test op --help
```

## Decrypting Existing Handles

Each decrypt family has a `direct` subcommand that decrypts existing handles directly, plus `fresh` and `stored` FHETest demo subcommands.

Direct mode lives in the family's `direct` subcommand. It accepts repeated `--handle` flags and sends all provided handles in one SDK decrypt request. It works for **any** contract's handles, not just FHETest, so you can decrypt token handles or handles from your own contracts:

```bash
fhevm-sdk public-decrypt direct --handle 0x... --handle 0x...
fhevm-sdk user-decrypt direct --handle 0x... --handle 0x...
fhevm-sdk delegated-user-decrypt direct --delegator 0x... --handle 0x... --handle 0x...
```

`--contract <address>` is the contract paired with the handles for ACL verification; it defaults to the FHETest contract. All handles in one command must belong to the same pairing contract, so pass `--contract` explicitly when decrypting handles that belong to a token or another contract. For user decrypt the handles must be decryptable by the wallet owner; for delegated user decrypt they must be decryptable through the same delegator/delegate relationship. Running a family root with no subcommand prints its help.

Stored slot mode is the `stored` subcommand. It accepts repeated `--type` flags, reads one FHETest handle per account/type slot, and sends those handles in one SDK decrypt request. When no `--type` is given it defaults to the `bool` stored slot:

```bash
fhevm-sdk public-decrypt stored --type uint16 --type uint32
fhevm-sdk user-decrypt stored --type uint16 --type uint32
fhevm-sdk delegated-user-decrypt stored --delegator 0x... --type uint16 --type uint32
```

`fresh` commands create one new stored FHETest handle and decrypt that handle. They are intentionally single-value flows.

## Networks And Global Options

| Option                      | Meaning                                                                                              |
| --------------------------- | ---------------------------------------------------------------------------------------------------- |
| `-n, --network testnet`     | FHETest `0x94B9d3aF050687D1F76251aD7D09a1F216a19845` on Ethereum Sepolia. Default.                   |
| `-n, --network devnet`      | FHETest `0xf56a7990E63a63eC75aD9Aa07De8cB6bF7baa805` on Ethereum Sepolia with devnet relayer config. |
| `-n, --network devnet-amoy` | FHETest `0x7553CB9124f974Ee475E5cE45482F90d5B6076BC` on Polygon Amoy with devnet relayer config.     |
| `-n, --network mainnet`     | FHeTest `0xf56a7990E63a63eC75aD9Aa07De8cB6bF7baa805`on Ethereum Mainnet with mainnet relayer config. |
| `--rpc-url <url>`           | Host chain RPC override. Otherwise uses the matching env var, then a public fallback.                |
| `--relayer-url <url>`       | Relayer base URL override. `localhost:3000` becomes `http://localhost:3000`.                         |
| `--contract <address>`      | FHETest contract override. This is command-specific, not a global option.                            |

Supported FHETest value types are `bool`, `uint8`, `uint16`, `uint32`, `uint64`, `uint128`, `uint256`, and `address`.

## Environment

| Variable                | Used for                                                                                                       |
| ----------------------- | -------------------------------------------------------------------------------------------------------------- |
| `MAINNET_RPC_URL`       | RPC for `mainnet`.                                                                                             |
| `SEPOLIA_RPC_URL`       | RPC for `testnet` and `devnet`.                                                                                |
| `POLYGON_AMOY_RPC_URL`  | RPC for `devnet-amoy`.                                                                                         |
| `ZAMA_FHEVM_API_KEY`    | Optional SDK relayer auth when the target environment requires an API key.                                     |
| `PRIVATE_KEY`           | Default wallet private key. Used by transaction commands, user decrypt, and delegated decrypt as the delegate. |
| `MNEMONIC`              | Default wallet mnemonic when `PRIVATE_KEY` is not set.                                                         |
| `DELEGATOR_PRIVATE_KEY` | Encrypted data owner private key for delegated decrypt flows.                                                  |
| `DELEGATOR_MNEMONIC`    | Encrypted data owner mnemonic when `DELEGATOR_PRIVATE_KEY` is not set.                                         |

For delegated flows:

- delegate credentials are `--private-key`/`--mnemonic` or `PRIVATE_KEY`/`MNEMONIC`
- delegator credentials are `--delegator-private-key`/`--delegator-mnemonic` or `DELEGATOR_PRIVATE_KEY`/`DELEGATOR_MNEMONIC`
- `--delegator <address>` identifies the encrypted data owner when credentials are not needed or are supplied separately

## Workflow Examples

### Input Proof

`input-proof` only talks to the SDK/relayer. It does not write a transaction.

```bash
fhevm-sdk input-proof
fhevm-sdk input-proof --type uint32
fhevm-sdk input-proof --type uint64 --value 42 --user 0x0000000000000000000000000000000000000002
```

### Public Decrypt

Use `fresh` to create a new public handle and immediately decrypt it:

```bash
fhevm-sdk public-decrypt fresh --type uint8
fhevm-sdk public-decrypt fresh --type uint64 --value 42
```

Decrypt an existing public handle with the `direct` subcommand, or read FHETest slots with `stored`:

```bash
fhevm-sdk public-decrypt direct --handle 0x... --handle 0x...
fhevm-sdk public-decrypt direct --handle 0x... --contract 0x<token or other contract>
fhevm-sdk public-decrypt stored --type uint8
fhevm-sdk public-decrypt stored --account 0x... --type uint8
fhevm-sdk public-decrypt stored --type uint16 --type uint32
```

Use `make-public` when the caller already has a stored FHETest handle and wants to mark it publicly decryptable:

```bash
fhevm-sdk public-decrypt make-public --type uint64
```

### User Decrypt

Use `fresh` to create a new private handle and decrypt it as the owner:

```bash
fhevm-sdk user-decrypt fresh --type uint8
fhevm-sdk user-decrypt fresh --type uint64 --value 42 --duration-days 7
fhevm-sdk user-decrypt fresh --type uint64 --value 42 --artifact ./artifacts/user-decrypt.json
```

Decrypt an existing private handle owned by the wallet with the `direct` subcommand, or read FHETest slots with `stored`:

```bash
fhevm-sdk user-decrypt direct --handle 0x... --handle 0x...
fhevm-sdk user-decrypt direct --handle 0x... --contract 0x<token or other contract>
fhevm-sdk user-decrypt stored --type uint8
fhevm-sdk user-decrypt stored --type uint16 --type uint32
```

`--artifact <path>` writes a sensitive validation artifact containing the
transport private key for that decrypt request. Use it only for debugging
duplicated relayer responses and protect it like key material.

Permit summaries report the SDK permit `version` and `durationSeconds`.
Validation artifacts use schema version 2 so protocol-versioned permits from
`@fhevm/sdk` 0.13.2-1 are not confused with older request material. The
CLI keeps `--duration-days` as a convenience and converts it to seconds before
calling the toolkit.

### Delegated User Decrypt

In delegated decrypt, the delegator owns the encrypted data and the delegate signs the decrypt permit.

Use `fresh` to create delegator-owned data, create ACL delegation if needed, and decrypt as the delegate:

```bash
fhevm-sdk delegated-user-decrypt fresh --type uint8
fhevm-sdk delegated-user-decrypt fresh --type uint64 --value 42 --duration-days 7 --delegation-duration-days 30
fhevm-sdk delegated-user-decrypt fresh --type uint64 --artifact ./artifacts/delegated-user-decrypt.json
```

Decrypt raw handles with the `direct` subcommand, or read the delegator's FHETest slots with `stored`:

```bash
fhevm-sdk delegated-user-decrypt direct --delegator 0x... --handle 0x... --handle 0x...
fhevm-sdk delegated-user-decrypt stored --delegator 0x... --type uint8
fhevm-sdk delegated-user-decrypt stored --delegator 0x... --type uint16 --type uint32
```

### Relayer Result Verification

Use a validation artifact from `user-decrypt` or `delegated-user-decrypt` to
verify a terminal relayer GET response later:

```bash
fhevm-sdk verify-user-decrypt --artifact ./artifacts/user-decrypt.json
```

The GET URL is derived from the artifact: the relayer base URL from its
`network`, the path segment from its `flow` (`v2/user-decrypt` or
`v2/delegated-user-decrypt`), and the job id from its `relayer.jobId`. Override
the base with the global `--relayer-url`, the job with `--job-id <id>`, or the
whole URL with `--url <full-url>` for exotic relayers. When
`ZAMA_FHEVM_API_KEY` is set it is sent as the `x-api-key` GET header (devnet is
keyless; mainnet requires it).

The verifier fetches the GET URL, restores the transport key pair, validates the
saved permit, decrypts the KMS signcrypted shares, and compares plaintexts when
the artifact contains expected values. Artifacts created from raw `--handle`
decrypts may not contain expected plaintexts, so those runs report decrypted
values without `valuesMatch`.

The result's `provenance` field distinguishes cryptographic verification from
artifact assertions. KMS shares, request handles, transport key, and permit
signature are cryptographically checked. Expected plaintexts are debugging
values supplied by the local artifact, not independently authenticated truth.
For protocol-v2 delegated permits, the saved owner/delegation label is also an
artifact assertion because the serialized permit does not retain historical ACL
state. The verifier rejects artifact fields that disagree with derivable signed
permit material.

The verifier also binds relayer response identity wherever the protocol exposes
it: it compares the response `requestId` with the artifact and checks the saved
`jobId` against either a response field or the final URL path segment. The
returned `responseIdentity` field explicitly reports any identity dimension that
the available artifact, response, and URL could not bind as `unbound`.

Saved-response reconstruction uses an exact-version compatibility seam for
`@fhevm/sdk@0.13.2-1` private branded objects. It is supported only in the
repository's unbundled Node ESM execution model; bundling it or mixing CJS and
ESM SDK instances is unsupported and may fail the brand checks.

If an active ACL delegation already exists, delegated decrypt (root or `stored`) only needs delegate credentials plus `--delegator`. If not, provide delegator credentials so the CLI can create the delegation.

### FHETest Utilities

Inspect network and contract configuration:

```bash
fhevm-sdk fhe-test info
```

Initialize stored FHETest handles:

```bash
fhevm-sdk fhe-test init --type uint64
fhevm-sdk fhe-test init --type uint64 --type uint128
fhevm-sdk fhe-test init --bulk
fhevm-sdk fhe-test init --type uint256 --force
```

The init JSON includes `transactionHashes`, because non-bulk initialization may write one transaction per initialized type.

Inspect FHETest state:

```bash
fhevm-sdk fhe-test inspect --type uint64
fhevm-sdk fhe-test inspect --account 0x... --type uint64
fhevm-sdk fhe-test inspect --handle 0x...
```

`inspect --handle` is mutually exclusive with account/type options. `inspect --type` defaults to the wallet address when `--account` is omitted.

Run operation demos against the caller's stored handle:

```bash
fhevm-sdk fhe-test op add-uint64 --value 42
fhevm-sdk fhe-test op xor-bool --value true --public
fhevm-sdk fhe-test op eq-address --value 0x0000000000000000000000000000000000000001
```

Supported operations are `xor-bool`, `add-uint8`, `add-uint16`, `add-uint32`, `add-uint64`, `add-uint128`, `xor-uint256`, and `eq-address`.

### Token Utilities

`token transfer` and `token balance` target ERC-7984 confidential tokens rather than FHETest, so `--contract` is required on every invocation; there is no per-network default token address.

Transfer an encrypted amount. The amount is base units (0 < amount < 2^64), encrypted client-side as `euint64` with an input proof:

```bash
fhevm-sdk token transfer --contract 0x... --to 0x... --amount 1000
```

Add `--from` to spend an existing operator allowance via `confidentialTransferFrom` instead of transferring from the loaded wallet's own balance:

```bash
fhevm-sdk token transfer --contract 0x... --from 0x... --to 0x... --amount 1000
```

Because ERC-7984 does not revert on insufficient balance, the transfer result includes `transferredHandle`, the encrypted amount actually moved. The token's ACL grants decryption of this handle to the recipient (and any account the token authorizes), not to the sender, so the recipient can decrypt it with `user-decrypt direct`, pairing the handle with the **token** contract via `--contract` (the default pairing is FHETest, which is wrong for token handles):

```bash
fhevm-sdk user-decrypt direct --handle <transferredHandle> --contract 0x<token address>
```

To confirm a transfer from the sender's side, add `--verify`. It decrypts the sender balance before and after the transfer, then adds `balanceBefore`, `balanceAfter`, and a `deltaMatches` boolean (`balanceBefore - balanceAfter === <requested amount>`) to the JSON output. Only the sender's own balance is decryptable by the sender under ACL, so `--verify` is rejected together with `--from` (the operator wallet cannot decrypt the `--from` account's balance). It adds two user-decrypt rounds:

```bash
fhevm-sdk token transfer --contract 0x... --to 0x... --amount 1000 --verify
```

Read a confidential balance handle. `--account` defaults to the wallet address loaded from `PRIVATE_KEY`/`MNEMONIC`:

```bash
fhevm-sdk token balance --contract 0x...
fhevm-sdk token balance --contract 0x... --account 0x...
```

Pipe the returned `balanceHandle` into user-decrypt to see the plaintext balance, pairing it with the token contract:

```bash
TOKEN=0x<token address>
fhevm-sdk user-decrypt direct --contract "$TOKEN" --handle "$(fhevm-sdk token balance --contract "$TOKEN" | jq -r .balanceHandle)"
```

## Shell Completion

```bash
fhevm-sdk completion install
fhevm-sdk completion install --shell zsh
fhevm-sdk completion uninstall
```

Supported shells are `bash`, `zsh`, `fish`, and `pwsh`. Restart the shell or source its profile after installing.

Completion uses a lightweight static resolver so pressing Tab does not load the SDK, connect to networks, or evaluate command flows.

## Not Yet Exposed

`FHETest.sol` has a few capabilities that are not currently available as first-class CLI commands:

| FHETest capability                                               | Possible CLI addition                                                                                                      |
| ---------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `verify(handles, cleartexts, decryptionProof)`                   | Verify public decrypt proof material on-chain.                                                                             |
| `createPublicHandle(inputHandle, inputProof)`                    | Verify an externally created encrypted input and make that handle publicly decryptable without storing it by account/type. |
| Typed getters such as `getEuint64()` and `getEuint64Of(account)` | Read typed encrypted handles directly instead of using only generic account/type inspection.                               |
| `getHandle(type)`                                                | Read the caller's raw handle without passing an account.                                                                   |

The current CLI focuses on SDK demo flows: input proof, public decrypt, user decrypt, delegated user decrypt, FHETest initialization, inspection, and selected operation demos.

## Development

```bash
pnpm run typecheck
```
