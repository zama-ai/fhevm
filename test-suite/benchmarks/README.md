# FHEVM benchmarks

This folder contains utilities to benchmark and stress test the FHEVM.



## Configuration

```
npm i
npx hardhat compile
```

- Edit hardhat config
- `cp .env.example .env` and edit it accordingly
- Verify contract addresses in `./contracts/E2EFHEVMConfigSepolia.sol`

## Launch 


```bash
npx hardhat <task-name>

```

### cERC-20 mint, transfer, deploy

```bash
# Deploys cERC-20 contract and bench
npx hardhat cerc-20-multi-transfer-decrypt --network sepolia
# Re-use cERC-20 contract and bench
npx hardhat cerc-20-multi-transfer-decrypt --network sepolia --cerc20-address 0xBC9ead1EeA82C8e07391c72c7f552D2226e9e21c
```


### Decryption benchmarks

Benchmark default values are set in `Makefile`. Testnet settings are set in `.env`, with `.env.example` as reference.

```bash
make public-decrypt-benchmark
make user-decrypt-benchmark
```

#### Testnet settings

| Setting | Description | Default |
|---------|-------------|---------|
| NETWORK | Testnet network | `sepolia` |
| MNEMONIC | Mnemonic to use for running the benchmark | `adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer` |
| RELAYER_URL | Relayer URL | `https://eth-sepolia.public.blastapi.io` |
| RPC_URL | RPC URL | `https://relayer.testnet.zama.cloud` |
| DECRYPTION_CONTRACT_ADDRESS | Deployed decryption contract address | `0xb6E160B1ff80D67Bfe90A85eE06Ce0A2613607D1` |

#### Benchmark settings

| Setting | Description | Default |
|---------|-------------|---------|
| DECRYPTIONS_PER_BATCH | Number of decryptions to do per batch | 10 |
| N_BATCH | Number of batches to do | 1 |
| SLEEP_BETWEEN_DECRYPTIONS | Sleep (milliseconds) between decryption requests | 0 |
| SLEEP_BETWEEN_BATCHES | Sleep (milliseconds) between batches | 1000 |
| DEPLOY_NEW_CONTRACT | Deploy a new contract. Use cache if false | false |
| GENERATE_NEW_HANDLES | Generate new handles (only for user decrypt). Use cache if false | false |
| DEBUG_LOGS | Display debug logs | false |


#### Example

100 public decryptions per second for 10 seconds:
```bash
make public-decrypt-benchmark DECRYPTIONS_PER_BATCH=100 N_BATCH=10 
```

300 user decryptions in a row (after `npx hardhat compile`):
```bash
make user-decrypt-benchmark DECRYPTIONS_PER_BATCH=100 N_BATCH=3 SLEEP_BETWEEN_BATCHES=0
```