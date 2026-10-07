# Sample Hardhat Project

This project demonstrates a basic Hardhat use case. It comes with a sample contract, a test for that contract, and a Hardhat Ignition module that deploys that contract.

Try running some of the following tasks:

```shell
npx hardhat help
npx hardhat test
REPORT_GAS=true npx hardhat test
npx hardhat node
npx hardhat ignition deploy ./ignition/modules/Lock.ts
```

## Switching the `@fhevm/sdk` source

`test-suite/e2e` doesn't declare `@fhevm/sdk` in `package.json` — it's
installed in place by `scripts/install-sdk.sh`, either from a local build or
from the npm registry, without touching `package.json`/`package-lock.json`.

By default (and in the Docker build), it's built and packed from your local
`sdk/js-sdk` source:

```shell
cd test-suite/e2e
npm run sdk:local
```

Set `SDK_BUILD_PROFILE=dev` before `npm run sdk:local` for a faster,
unminified build while iterating. Re-run it after every change to
`sdk/js-sdk` source — the install is a one-off pack, not a live link.

To install a specific published version from the registry instead, pass it
explicitly — there's no default to fall back to:

```shell
npm run sdk:registry -- 0.13.2
```

Both commands wrap `scripts/install-sdk.sh` (`local`/`registry` modes) — see
its header comment for details.

## Selecting a chain (`run-tests.sh --chain`)

One container can hold the env vars of several chains and run the tests against any of them:

```shell
./run-tests.sh --chain bnb -g "test user decrypt"
```

| `--chain`         | Hardhat network | Chain id |
| ----------------- | --------------- | -------- |
| `eth`             | `sepolia`       | 11155111 |
| `polygon`         | `polygonAmoy`   | 80002    |
| `bnb`             | `bnbTestnet`    | 97       |
| `hoodi`           | `hoodi`         | 560048   |
| `eth-mainnet`     | `mainnet`       | 1        |
| `polygon-mainnet` | `polygon`       | 137      |
| `bnb-mainnet`     | `bnb`           | 56       |

The environment (DevNet, Testnet, Mainnet) comes from the env vars themselves; only mainnet chains need the
`-mainnet` suffix. Each chain reads its own prefixed vars (`ETH_`, `POLYGON_`, `BNB_`, `HOODI_`):

- required: `<PREFIX>_ACL_CONTRACT_ADDRESS`, `<PREFIX>_FHEVM_EXECUTOR_CONTRACT_ADDRESS`,
  `<PREFIX>_KMS_VERIFIER_CONTRACT_ADDRESS`, `<PREFIX>_INPUT_VERIFIER_CONTRACT_ADDRESS`,
  `<PREFIX>_PROTOCOL_CONFIG_CONTRACT_ADDRESS`, `<PREFIX>_RPC_URL`
- optional: `<PREFIX>_HCU_LIMIT_CONTRACT_ADDRESS`
- shared (unprefixed): `CHAIN_ID_GATEWAY`, `DECRYPTION_ADDRESS`, `INPUT_VERIFICATION_ADDRESS`, `RELAYER_URL`, `MNEMONIC`

`scripts/resolve-chain-env.ts` validates them all before anything else runs: if one is missing or invalid, the run
fails and lists every problem. Otherwise it exports the unprefixed vars, the network and the chain id, and enables the
coprocessor config generator (`E2E_COPROCESSOR_CONFIG_FROM_ENV=true`).

Only one `--chain` run is allowed per container at a time: the run holds `e2e.lock` (next to `run-tests.sh`) and a
second run fails immediately, showing who holds it. A lock left by a run that was killed is detected and replaced.
Without `--chain`, `run-tests.sh` behaves as before and does not take the lock, so avoid mixing such runs with `--chain`
runs in the same container. `--chain` cannot be combined with `--no-hardhat-compile`.

Known limitation: other per-chain values that the tests read unprefixed, such as `TEST_INPUT_CONTRACT_ADDRESS` or the
bridge addresses, are not handled by `--chain`.

## Unified user-decryption suites

E2E coverage for ERC-1271 smart-account signature verification and the unified
EIP-712 user-decryption request:

- `test/erc1271UserDecryption/` — smart-account signing modes and every ERC-1271
  rejection path
- `test/unifiedUserDecryption/` — `allowedContracts` modes, validity window,
  mixed direct+delegated batches, `extraData` version matrix
- `test/decryptionSignatureInvalidation/` — on-chain signature invalidation and
  its end-to-end effect, including the multisig-rotation scenario

These suites POST the unified `eip712-unified-user-decrypt-v1` envelope directly
to the relayer's `/v3/user-decrypt` endpoint via
`test/sdk/unified/unifiedUserDecrypt.ts` — the public SDK builds the same
envelope on protocol >= 0.14, but always signs as the connected signer and does
not expose the fields these suites must control (distinct `userAddress`, empty
signatures, extraData versions, malformed shapes). Positives additionally
decrypt through the public SDK and assert the known plaintext where the
scenario is SDK-expressible. The helper's header comment documents the
assertion model. If the relayer is fronted by auth, set `ZAMA_FHEVM_API_KEY`
(sent as `x-api-key`).

Run via the fhevm-cli profiles `erc1271-user-decryption`,
`unified-user-decryption`, and `decryption-signature-invalidation` (all part of
`standard`) — see `test-suite/fhevm/README.md` — or directly with
`npx hardhat test --grep "<describe title>" --network staging`.

## Smoke runner (inputFlow)

Runs a single on-chain smoke flow (input + add42 + decrypt) using Hardhat as a runtime
and hardened transaction handling.

### Prereqs

**For sepolia/mainnet**, most config is auto-populated from the SDK (`SepoliaConfig`/`MainnetConfig`).
You only need:

- `RPC_URL` (or `SEPOLIA_ETH_RPC_URL` / `MAINNET_ETH_RPC_URL`)
- `MNEMONIC`
- `ZAMA_FHEVM_API_KEY` (mainnet only)

**For devnet** (runs on Sepolia), use the pre-configured `.env.devnet` (all addresses included):

```shell
DOTENV_CONFIG_PATH=./.env.devnet npx hardhat run --network sepolia scripts/smoke-inputflow.ts
```

**For other networks** (staging, custom), set all variables manually - see `.env.example`.

Network-specific RPC URLs:

- staging: `RPC_URL` (defaults to localhost:8545)
- sepolia: `SEPOLIA_ETH_RPC_URL` (falls back to `RPC_URL`)
- mainnet: `MAINNET_ETH_RPC_URL` (falls back to `RPC_URL`)
- polygon / polygonAmoy: `POLYGON_RPC_URL` / `POLYGON_AMOY_RPC_URL` (fall back to `RPC_URL`)
- bnb / bnbTestnet: `BNB_RPC_URL` / `BNB_TESTNET_RPC_URL` (fall back to `RPC_URL`)
- hoodi: `HOODI_RPC_URL` (falls back to `RPC_URL`)

For pod deployments, just set `RPC_URL` - it works for all networks.

Set `TEST_INPUT_CONTRACT_ADDRESS` to reuse an existing contract (requires `SMOKE_DEPLOY_CONTRACT=0`).

Hardhat loads env from `test-suite/e2e/.env` by default; override with `DOTENV_CONFIG_PATH`.
You can also store secrets with Hardhat vars, e.g. `npx hardhat vars set SEPOLIA_ETH_RPC_URL` (it will prompt for the value).
For devnet, `test-suite/e2e/.env.devnet` provides a ready baseline (use `DOTENV_CONFIG_PATH=./.env.devnet`).

### Signer configuration

The smoke runner uses HD wallet signers derived from `MNEMONIC`. By default, it uses indices `0,1,2`
for automatic failover - if one signer has a stuck transaction, it falls back to another.

**Important:** All configured signers should be funded for maximum resilience. The script logs all
available signers at startup with their balances and warns if any have low balance (< 0.005 ETH).
Post-success cleanup will attempt to cancel any remaining pending transactions, but failures in that
cleanup step do not fail the smoke run.

To derive signer addresses from a mnemonic (for funding):

```shell
# Using Foundry's cast
cast wallet address --mnemonic "your mnemonic here" --mnemonic-index 0
cast wallet address --mnemonic "your mnemonic here" --mnemonic-index 1
cast wallet address --mnemonic "your mnemonic here" --mnemonic-index 2
```

### Smoke-specific knobs (defaults in parentheses)

- `SMOKE_SIGNER_INDICES` (`0,1,2`) - comma-separated list of signer indices to use for failover
- `SMOKE_TX_TIMEOUT_SECS` (`48`)
- `SMOKE_TX_MAX_RETRIES` (`2`)
- `SMOKE_FEE_BUMP` (`1.125^4`)
- `SMOKE_MAX_FEE_GWEI` (optional) - fail fast if maxFeePerGas exceeds this cap (unset = no cap)
- `SMOKE_MAX_PRIORITY_FEE_GWEI` (optional) - fail fast if maxPriorityFeePerGas exceeds this cap (unset = no cap)
- `SMOKE_MAX_BACKLOG` (`3`)
- `SMOKE_CANCEL_BACKLOG` (`1`) - set to `0` to disable auto-cancel of pending transactions
- `SMOKE_DEPLOY_CONTRACT` (`1`) - set to `0` to attach to existing contract via `TEST_INPUT_CONTRACT_ADDRESS`
- `SMOKE_RUN_TESTS` (`1`) - set to `0` to deploy contract only without running tests
- `SMOKE_DECRYPT_TIMEOUT_SECS` (`300`) - timeout for decryption operations
- `BETTERSTACK_HEARTBEAT_URL` (optional) - if set, pings BetterStack on success; reports error with exit code on failure

### Run

```shell
cd test-suite/e2e
npx hardhat run --network sepolia scripts/smoke-inputflow.ts
npx hardhat run --network mainnet scripts/smoke-inputflow.ts
```
