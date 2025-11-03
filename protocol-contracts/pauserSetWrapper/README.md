# PauserSetWrapper

This project contains the PauserSetWrapper contract, which allows pausers from a PauserSet contract to call specific functions on target contracts.

## Prerequisites

```bash
cp .env.example .env
```

Then fill the `PRIVATE_KEY` and `SEPOLIA_RPC_URL` in `.env` with your own values.
Fill the `CONTRACT_TARGET`, `FUNCTION_SIGNATURE` and `PAUSER_SET` with the values of the target contract, function signature, and pauser set contract.

## Deployment

```shell
npx hardhat deploy --network <ethereum-testnet|ethereum-mainnet>
```

## Verification

After deployment, verify the contract on Etherscan:

```shell
npx hardhat task:verifyPauserSetWrapper --address <deployed-address> --network <ethereum-testnet|ethereum-mainnet>
```

## Testing

Run the test suite:

```shell
npx hardhat test
```
