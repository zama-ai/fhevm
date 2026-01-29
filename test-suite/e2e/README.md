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

The smoke runner uses the same environment variables as the e2e tests:

- `MNEMONIC`
- `RELAYER_URL`
- `KMS_VERIFIER_CONTRACT_ADDRESS`, `ACL_CONTRACT_ADDRESS`, `INPUT_VERIFIER_CONTRACT_ADDRESS`
- `CHAIN_ID_GATEWAY`, `CHAIN_ID_HOST`
- `DECRYPTION_ADDRESS`, `INPUT_VERIFICATION_ADDRESS`

Network-specific RPC URLs:

- staging/zwsDev: `RPC_URL` (defaults to localhost:8545)
- sepolia: `SEPOLIA_ETH_RPC_URL` (falls back to `RPC_URL`)
- mainnet: `MAINNET_ETH_RPC_URL` (falls back to `RPC_URL`)

For pod deployments, just set `RPC_URL` - it works for all networks.

Mainnet additionally requires `ZAMA_FHEVM_API_KEY`. Set `TEST_INPUT_CONTRACT_ADDRESS` to reuse an existing contract (requires `SMOKE_DEPLOY_CONTRACT=0`).

Hardhat loads env from `test-suite/e2e/.env` by default; override with `DOTENV_CONFIG_PATH`.
You can also store secrets with Hardhat vars, e.g. `npx hardhat vars set SEPOLIA_ETH_RPC_URL` (it will prompt for the value).
For devnet, `test-suite/e2e/.env.devnet` provides a ready baseline (use `DOTENV_CONFIG_PATH=./.env.devnet`).

### Smoke-specific knobs (defaults in parentheses)

- `SMOKE_SIGNER_INDICES` (`0`)
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
