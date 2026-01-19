# How to add ZamaOFT on BNB Chain

Currently, we have `ZamaERC20` and `ZamaOFTAdapter` deployed on Ethereum mainnet, and `ZamaOFT` deployed on Gateway mainnet. The `ZamaOFTAdapter` contract's owner and delegate are already setup to be an Aragon DAO contract.

The goal of this runbook is to guide you step by step on how to deploy a `ZamaOFT` instance on BNB Chain, and how to wire it to the already deployed `ZamaOFTAdapter` on Ethereum, via the Aragon DAO. We only add a single bidirectional pathway: `BNB <-> Ethereum` (i.e no step to wire `BNB <-> Gateway`).

## Step 1 : Recreating deployments

The first step is to recreate deployments artifacts for the already deployed contract. This is inspired from the official [LayerZero V2 docs](https://docs.layerzero.network/v2/tools/create-lz-oapp-cli/recreating-deployments).

First, run `pnpm install` and setup your `.env` file with all required values (see [`.env.example`](./.env.example)).

Create a `/deployments` folder in the root of the [/token] directory. The eventual structure would look like this:

```
/deployments
    /ethereum-mainnet
      .chainId
      ZamaERC20.json
      ZamaOFTAdapter.json
    /gateway-mainnet
      .chainId
      ZamaOFT.json
```

`/deployments/ethereum-mainnet/.chainId` - this file should contain the chain ID for the network, i.e `1` for Ethereum mainnet.

`/deployments/ethereum-mainnet/ZamaERC20.json` - the only key that is necessary in the JSON file is address. Insert your ERC20 address into the address field.

```
{
  "address": "<ZamaERC20Address>"
}
```

Follow the same similar steps for the remaining files, i.e `/deployments/ethereum-mainnet/ZamaOFTAdapter.json`, `/deployments/gateway-mainnet/.chainId` and `/deployments/gateway-mainnet/ZamaOFT.json`.

Then modifiy `hardhat.config.ts` by replacing `0x0` by the `ZamaERC20` address under the `ethereum-mainnet` field:

```typescript
oftAdapter: {
    tokenAddress: '0x0', // Replace `0x0` with the address of the ERC20 token you want to adapt to the OFT functionality.
}
```

Finally, run `npx hardhat compile` to ensure relevant artifacts that are required by Hardhat helper tasks involving the EVM OFT are generated.

## Step 2 : Deploy ZamaOFT on BNB Chain

Run `npx hardhat deploy --network bnb-mainnet --tags ZamaOFT` command.

You can then verify the contract by running `pnpm verify:etherscan:bnb:mainnet`.

## Step 3 : Wire the BNB OFT

This can be done easily, since your deployer hot wallet is still the owner and delegate of the `ZamaOFT` instance on BNB Chain - later, after full wiring on both chains, ownership and delegate roles should be transferred to governance on BNB Chain, which should be a Safe multisig deployed on BNB mainnet.

You just have to run: 

```npx hardhat lz:oapp:wire --oapp-config layerzero.config.mainnet.bnb.ts --skip-connections-from-eids <EID_ETHEREUM_V2_MAINNET>,<EID_ZAMA_V2_MAINNET>```

In previous command, replace `<EID_ETHEREUM_V2_MAINNET>` by `30101` and `EID_ZAMA_V2_MAINNET` by `30397`.