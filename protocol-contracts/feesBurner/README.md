After filling all values of the `.env` file until the `ZAMA_ERC20_ADDRESS` variable (i.e all variables except from `PROTOCOL_FEES_BURNER_ADDRESS`), run this command to deploy first the `ProtocolFeesBurner` contract on Ethereum testnet:

# Deployment
## Setup
Copy the environment file: `cp .env.example .env`.

Then fill all the environment variables till `ZAMA_ERC20_ADDRESS` for the wanted networks (`MAINNET` or `TESTNET` RPC urls).

## ProtocolFeesBurner Deployment

Deploy the ProtocolFeesBurner contract on Ethereum

```bash
npx hardhat deploy --tags ProtocolFeesBurner --network ethereum-testnet
```

Then fill the `PROTOCOL_FEES_BURNER_ADDRESS` env variable with the address of your deployment.

## FeesSenderToBurner Deployment

```bash
npx hardhat deploy --tags FeesSenderToBurner --network gateway-testnet
```

## Verification

You can verify the contracts on Etherscan with the following tasks:

```bash
npx hardhat --network ethereum-testnet task:verifyProtocolFeesBurner --protocol-fees-burner <PROCOTOL_FEES_BURNER_ADDRESS>
```

```bash
npx hardhat --network gateway-testnet task:verifyFeesSenderToBurner --fees-sender-to-burner <FEES_SENDER_TO_BURNER_ADDRESS>
```
