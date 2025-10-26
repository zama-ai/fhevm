After filling all values of the `.env` file until the `ZAMA_ERC20_ADDRESS` variable, run this command to deploy first the `ProtocolFeesBurner` contract on Ethereum testnet:
```bash
npx hardhat deploy --tags ProtocolFeesBurner --network ethereum-testnet
```