After filling all values of the `.env` file until the `ZAMA_ERC20_ADDRESS` variable, run this command to deploy first the `ProtocolFeesBurner` contract on Ethereum testnet:

```bash
npx hardhat deploy --tags ProtocolFeesBurner --network ethereum-testnet
```

Then to verify it on Etherscan: 

```bash
npx hardhat --network ethereum-testnet etherscan-verify --api-key [ETHERSCAN_API] --license BSD-3-Clause --force-license
```

Then to deploy the `FeesSenderToBurner` on Gateway-testnet, fill the 2 remaining variables in the `.env` and run: 

```bash
npx hardhat deploy --tags FeesSenderToBurner --network gateway-testnet
```