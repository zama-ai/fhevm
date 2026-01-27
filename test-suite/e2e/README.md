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
- `RPC_URL` (or `SEPOLIA_ETH_RPC_URL` / `MAINNET_ETH_RPC_URL`), `MNEMONIC`
- `RELAYER_URL`
- `KMS_VERIFIER_CONTRACT_ADDRESS`, `ACL_CONTRACT_ADDRESS`, `INPUT_VERIFIER_CONTRACT_ADDRESS`
- `CHAIN_ID_GATEWAY`, `CHAIN_ID_HOST`
- `DECRYPTION_ADDRESS`, `INPUT_VERIFICATION_ADDRESS`

Mainnet additionally requires `ZAMA_FHEVM_API_KEY` and `TEST_INPUT_CONTRACT_ADDRESS` unless `SMOKE_FORCE_DEPLOY=1`.

Hardhat loads env from `test-suite/e2e/.env` by default; override with `DOTENV_CONFIG_PATH`.
You can also store secrets with Hardhat vars, e.g. `npx hardhat vars set SEPOLIA_ETH_RPC_URL` (it will prompt for the value).
For devnet, `test-suite/e2e/.env.devnet` provides a ready baseline (use `DOTENV_CONFIG_PATH=./.env.devnet`).

### Smoke-specific knobs (defaults in parentheses)
- `SMOKE_SIGNER_INDICES` (`0`)
- `SMOKE_TX_TIMEOUT_SECS` (`48`)
- `SMOKE_TX_MAX_RETRIES` (`2`)
- `SMOKE_FEE_BUMP` (`1.125^4`)
- `SMOKE_MAX_BACKLOG` (`3`)
- `SMOKE_CANCEL_BACKLOG` (`1`)
- `SMOKE_FORCE_DEPLOY` (`0`)

### Run
```shell
cd test-suite/e2e
npx hardhat run --network zwsDev scripts/smoke-inputflow.ts
npx hardhat run --network sepolia scripts/smoke-inputflow.ts
npx hardhat run --network mainnet scripts/smoke-inputflow.ts
```
