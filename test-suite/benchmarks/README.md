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
