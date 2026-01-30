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

## Smoke runner (inputFlow)

Runs a single on-chain smoke flow (input + add42 + decrypt) using Hardhat as a runtime
and hardened transaction handling.

### Prereqs

**For sepolia/mainnet**, most config is auto-populated from the SDK (`SepoliaConfig`/`MainnetConfig`).
You only need:

- `RPC_URL` (or `SEPOLIA_ETH_RPC_URL` / `MAINNET_ETH_RPC_URL`)
- `MNEMONIC`
- `ZAMA_FHEVM_API_KEY` (mainnet only)

**For devnet**, use the pre-configured `.env.devnet` (all addresses included):

```shell
DOTENV_CONFIG_PATH=./.env.devnet npx hardhat run --network devnet scripts/smoke-inputflow.ts
```

**For other networks** (staging, custom), set all variables manually - see `.env.example`.

Network-specific RPC URLs:

- staging/zwsDev: `RPC_URL` (defaults to localhost:8545)
- sepolia: `SEPOLIA_ETH_RPC_URL` (falls back to `RPC_URL`)
- mainnet: `MAINNET_ETH_RPC_URL` (falls back to `RPC_URL`)

For pod deployments, just set `RPC_URL` - it works for all networks.

Set `TEST_INPUT_CONTRACT_ADDRESS` to reuse an existing contract (requires `SMOKE_DEPLOY_CONTRACT=0`).

Hardhat loads env from `test-suite/e2e/.env` by default; override with `DOTENV_CONFIG_PATH`.
You can also store secrets with Hardhat vars, e.g. `npx hardhat vars set SEPOLIA_ETH_RPC_URL` (it will prompt for the value).
For devnet, `test-suite/e2e/.env.devnet` provides a ready baseline (use `DOTENV_CONFIG_PATH=./.env.devnet`).

### Signer configuration

The smoke runner uses HD wallet signers derived from `MNEMONIC`. By default, it uses indices `0,1,2`
for automatic failover - if one signer has a stuck transaction, it falls back to another.

**Important:** All configured signers should be funded for maximum resilience. The script logs all
available signers at startup with their balances and warns if any have low balance (< 0.1 ETH).

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
- `SMOKE_MAX_BACKLOG` (`3`)
- `SMOKE_CANCEL_BACKLOG` (`1`) - set to `0` to disable auto-cancel of pending transactions
- `SMOKE_DEPLOY_CONTRACT` (`1`) - set to `0` to attach to existing contract via `TEST_INPUT_CONTRACT_ADDRESS`
- `SMOKE_RUN_TESTS` (`1`) - set to `0` to deploy contract only without running tests
- `SMOKE_DECRYPT_TIMEOUT_SECS` (`120`) - timeout for decryption operations
- `BETTERSTACK_HEARTBEAT_URL` (optional) - if set, pings BetterStack on success; reports error with exit code on failure

### Run

```shell
cd test-suite/e2e
npx hardhat run --network zwsDev scripts/smoke-inputflow.ts
npx hardhat run --network sepolia scripts/smoke-inputflow.ts
npx hardhat run --network mainnet scripts/smoke-inputflow.ts
```
