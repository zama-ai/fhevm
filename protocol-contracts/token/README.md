# Zama token quickstart

## Step 1 : Initial setup

```bash
cp .env.example .env
```

Then fill the `PRIVATE_KEY` and `SEPOLIA_RPC_URL` in `.env` with your own values.
The values of `INITIAL_SUPPLY_RECEIVER` (account receiving initially minted supply of 1 billion tokens) and `INITIAL_ADMIN` (first admin role, responsible of adding minters and other admins) should also be filled. For easy testing purpose, they could be both set to be the address of the account corresponding to `PRIVATE_KEY` (i.e the deployer).
Also, if you want to do Step 5 (Etherscan verification) which is optional but recommended for easier debugging, get an Etherscan free API key in order to set the value of `ETHERSCAN_API`.

## Step 2 : Add Gateway Testnet to your Metamask wallet (optional)

For better DevX, you can add the Gateway Testnet network to your Metamask wallet.

## Step 3 : Funding deployer wallet on both chains

**Important:** Make sure the wallet address corresponding to your `PRIVATE_KEY` is funded on _both_ chains corresponding to this OFT, meaning on both Ethereum Sepolia and Gateway Testnet.

## Automated Deployment

The deployment of the `ZamaERC20` and `ZamaOFTAdapter` on Ethereum Sepolia as well as the deployment of the `ZamaOFT` on Gateway Testnet and the wiring of both contracts can be done automatically by running the following Hardhat task:

```bash
npx hardhat deploy:token --preset testnet
```

The task automatically:

- Confirms `.env` is populated (Step 1) and that required environment variables are set.
- Deploys `ZamaERC20` to `ethereum-testnet` using the appropriate deploy script tags.
- Automatically captures the deployed `ZamaERC20` address and sets the `oftAdapter.tokenAddress` key in the Hardhat runtime configuration for the layerzero deployment of the `ZamaOFTAdapter`.
- Deploys `ZamaOFTAdapter` to `ethereum-testnet` with the correct token address configuration.
- Deploys `ZamaOFT` to `gateway-testnet` with the matching deploy script tag.
- Wires both contracts together using LayerZero's `lz:oapp:wire` task.

Add `--verify true` if you want to automatically run the Etherscan verification commands:

```bash
npx hardhat deploy:token --preset testnet --verify true
```

You can still follow the manual instructions below if you prefer.

## Step 4 : Deploy ZamaERC20, ZamaOFTAdapter and ZamaOFT

First, deploy the ZamaERC20 token, which is an ERC20 implementation from OpenZeppelin with few extensions (such as ERC20Permit and the transferAndCall-related ERC1363). Run this command:

```bash
pnpm i
npx hardhat lz:deploy
```

In the instructions of `lz:deploy`, choose both networks (or choose just ethereum-testnet, since gateway-testnet will be filtered out in the deployment script of the ERC20). Then enter `ZamaERC20` when asked for the deploy script tag. The ERC20 token will be deployed on Ethereum testnet and 1 billion tokens will be minted initially for the `INITIAL_SUPPLY_RECEIVER` account.

Please carefully note the ZamaERC20 contract address which has been logged during the run of the previous script. In your `hardhat.config.ts` file, add the following configuration to the `ethereum-testnet` you want to deploy the OFTAdapter to:

```typescript
oftAdapter: {
    tokenAddress: '0x0', // Replace `0x0` with the address of the ERC20 token you want to adapt to the OFT functionality.
}
```

After updating your hardhat.config.ts file accordingly, second step is to deploy the ZamaOFTAdapter to the Ethereum testnet by running again:

```bash
npx hardhat lz:deploy
```

You can still select either both networks or alternatively uniquely the `ethereum-testnet` network (Gateway Testnet network will be filtered out because its config does not contain and `oftAdapter` field). But this time, enter `ZamaOFTAdapter` when asked for the deploy script tag. This will deploy the OFTAdapter contract on Ethereum Sepolia testnet.

Finally, third deployment step is to deploy the `ZamaOFT` contract on the Gateway Testnet. Again, you should run:

```bash
npx hardhat lz:deploy
```

You can still select either both networks or alternatively uniquely the `gateway-testnet` network this time. Now enter `ZamaOFT` when requested for the deploy script tag. This will deploy the `ZamaOFT` contract on Gateway Testnet.

## Step 5 : Etherscan verification (Optional)

In order verify the two contracts (`ZamaERC20` and `ZamaOFTAdapter`) on Etherscan for the Ethereum Sepolia network, use this command (don't worry if the following scripts return you an error or an invalid explorer URL link - see the note at the end of this step):

```bash
pnpm verify:etherscan:ethereum:sepolia
```

And to verify the `ZamaOFT` contract deployed on the Gateway Testnet network run:

```bash
pnpm verify:etherscan:gateway:testnet
```

**Note:** Due to a bug in the `verify-contract` task, sometimes those scripts will log an error and/or return a wrong URL for block explorer link, but most of the times, despite those errors, if you check the actual results by searching for corresponding contracts addresses on the block explorer, you will notice that the contracts will actually be succesfully verified after running those commands (i.e [https://sepolia.etherscan.io/](https://sepolia.etherscan.io/) for Ethereum testnet and on the Gateway Testnet block explorer).

## Step 6 : Wire contracts

```bash
npx hardhat lz:oapp:wire --oapp-config layerzero.config.testnet.ts
```

And follow straightforward instructions to wire the `ZamaOFTAdapter` contract on Ethereum testnet with the `ZamaOFT` on Gateway Testnet.

## Step 7 : Cross-chain transfer

Two NPM scripts are available to send ZAMA tokens between Ethereum Sepolia and Gateway Testnet:

- `npm run zama:oft:send:ethToGateway -- <address> <amount>`
- `npm run zama:oft:send:gatewayToEth -- <address> <amount>`

The provided address is verified to be a proper Ethereum address, then tokens are sent using `lz:oft:send` task.

If you want to send tokens from/to other chains, you can directly use `lz:oft:send`, as described in the following section.

For example you can send `1.5` ZAMA token from Ethereum Sepolia to Gateway Testnet from the deployer wallet to a custom receiver address by running this command and following instructions:

```bash
npx hardhat lz:oft:send --src-eid 40161 --dst-eid 40424 --amount 1.5 --to <RECEIVER_ADDRESS> --oapp-config layerzero.config.testnet.ts
```

**Note** In the OFTAdapter case here, contrarily to the OFT case, 2 transactions are sent in previous script instead of 1, because the sender must first approve the corresponding amount of the ERC20 token to the OFTAdapter (i.e calling approve method on the ERC20 method and passing the OFTAdapter address and correct amount as parameters), before locking them to the OFTAdapter contract in a second `send` transaction on OFTAdapter (reminder: in the OFT case, initiating a token transfer happens by directly burning an amount of the OFT contract by calling the `send` method of the OFT contract).

Once these transactions are sent, wait around 2 minutes and check the receiver's account on the Gateway Testnet block explorer that the receiver indeed received `1.5` ZAMA token on Gateway Testnet by clicking on `Token Holdings` there.

You could then also send back the tokens from Gateway Testnet to Ethereum Sepolia chain, by swapping the values of `--src-eid` and `--dst-eid` flags from previous command.

## Step 8 : Administrative tasks

After the contracts are deployed and wired you can manage permissions and ownership without writing custom scripts. The project exposes dedicated Hardhat tasks grouped by contract:

- **Role lifecycle on `ZamaERC20`:** Every supported role has its own grant/revoke command (`zama:erc20:grant:minter_role`, `zama:erc20:revoke:minting_pauser_role`, etc.) plus a `zama:erc20:renounce:*` variant that lets the currently connected signer drop its role. Use these to add minters, pause controllers, or new admins safely from the CLI.
- **`ZamaOFTAdapter` administration:** Set a new delegate on `ZamaOFTAdapter` with `zama:oftadapter:setDelegate --address <new_delegate>` and hand off overall control with `zama:oftadapter:transferOwnership --address <new_owner>`.
- **`ZamaOFT` administration:** Mirror the same actions on the `ZamaOFT` contract using `zama:oft:setDelegate --address <new_delegate>` and `zama:oft:transferOwnership --address <new_owner>`.

Always pass the correct `--network` flag so Hardhat connects to the chain that hosts the relevant deployment and make sure the signer you use already holds the corresponding privileges.

By default, these tasks will load the contract address of the task from environment variables:

- `ZAMAOFT_CONTRACT_ADDRESS` for `zama:oft:*`
- `ZAMAERC20_CONTRACT_ADDRESS` for `zama:erc20:*`
- `ZAMAOFTADAPTER_CONTRACT_ADDRESS` for `zama:oftadapter:*`

Two optional parameters are available in the tasks: `--from-deployment` and `--contract-address`, they are mutually exclusive.

- `--from-deployment` will fetch the contract address from the available deployments for the network the task is run upon.
  Here is an example granting the MINTER_ROLE from the ZamaERC20 deployment on the ethereum-testnet

```bash
npx hardhat zama:erc20:grant:minter_role --address <NEW_MINTER_ADDRESS> --from-deployment true --network ethereum-testnet
```

- `--contract-address` will use the provided address as the contract address.
  Here is an example granting the MINTER_ROLE from the ZamaERC20 deployment on the ethereum-testnet

```bash
npx hardhat zama:erc20:grant:minter_role --address <NEW_MINTER_ADDRESS> --contract-address <ZAMAERC20_CONTRACT_ADDRESS>  --network ethereum-testnet
```
