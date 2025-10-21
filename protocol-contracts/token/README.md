# Zama token quickstart

## Step 1 : Initial setup

```bash
cp .env.example .env
```

Then fill the `PRIVATE_KEY` and `SEPOLIA_RPC_URL` in `.env` with your own values.
The values of `INITIAL_SUPPLY_RECEIVER` (account receiving initially minted supply of 1 billion tokens) and `INITIAL_ADMIN` (first admin role, responsible of adding minters and other admins) should also be filled. For easy testing purpose, they could be both set to be the address of the account corresponding to `PRIVATE_KEY` (i.e the deployer).
Also, if you want to do Step 5 (Etherscan verification) which is optional but recommended for easier debugging, get an Etherscan free API key in order to set the value of `ETHERSCAN_API`.

## Step 2 : Add Arbitrum Sepolia to your Metamask wallet (optional)

For better DevX, add the Arbitrum Sepolia network to your Metamask wallet:

1. Open MetaMask and click on the network dropdown (usually shows "Ethereum Mainnet" by default)
1. Click "Add a custom network" at the bottom of the list
1. Enter these network details:
   - Network Name: Arbitrum Sepolia
   - RPC URL: https://sepolia-rollup.arbitrum.io/rpc
   - Chain ID: 421614
   - Currency Symbol: ETH
   - Block Explorer URL: https://sepolia.arbiscan.io
1. Click "Save" to add the network

## Step 3 : Funding deployer wallet on both chains

**Important:** Make sure the wallet address corresponding to your `PRIVATE_KEY` is funded on _both_ chains corresponding to this OFT, meaning on both Ethereum Sepolia and Arbitrum Sepolia. If you already have funds on Ethereum Sepolia, you can bridge part of it to Arbitrum Sepolia by going to this [website](https://bridge.arbitrum.io/?destinationChain=arbitrum-sepolia&sourceChain=sepolia). The cross-chain swap should take around 10 minutes, you can check funds were received in your Metamask wallet.

## Automated Deployment

The deployment of the `ZamaERC20` and `ZamaOFTAdapter` on Ethereum Sepolia as well as the deployment of the `ZamaOFT` on Arbitrum Sepolia and the wiring of both contracts can be done automatically by running the following script:

```bash
./deploy_zama_oft.sh
```

The script loads environment variables from the `.env` file.
The script expects the commands `pnpm`, `npx` and `node` to be available.
The script runs in non-interactive mode, so no manual input is required. It also:

- Confirms `.env` is populated (Step 1) and reminds that the deployer wallet must be funded on Ethereum and Arbitrum Sepolia (Step 3).
- Deploys `ZamaERC20` and `ZamaOFTAdapter` to `ethereum-testnet` using the appropriate deploy script tags.
- Automatically captures the deployed `ZamaERC20` address and writes it to `networks.ethereum-testnet.oftAdapter.tokenAddress` in `hardhat.config.ts`.
- Deploys `ZamaOFT` to `arbitrum-testnet` with the matching deploy script tag.
- Add `--verify` if you want the optional Step 5 verification commands executed before Step 6.

You can still follow the manual instructions below if you prefer.

## Step 4 : Deploy ZamaERC20, ZamaOFTAdapter and ZamaOFT

First, deploy the ZamaERC20 token, which is an ERC20 implementation from OpenZeppelin with few extensions (such as ERC20Permit and the transferAndCall-related ERC1363). Run this command:

```bash
pnpm i
npx hardhat lz:deploy
```

In the instructions of `lz:deploy`, choose both networks (or choose just ethereum-testnet, since arbitrum-testnet will be filtered out in the deployment script of the ERC20). Then enter `ZamaERC20` when asked for the deploy script tag. The ERC20 token will be deployed on Ethereum testnet and 1 billion tokens will be minted initially for the `INITIAL_SUPPLY_RECEIVER` account.

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

You can still select either both networks or alternatively uniquely the `ethereum-testnet` network (Arbitrum testnet network will be filtered out because its config does not contain and `oftAdapter` field). But this time, enter `ZamaOFTAdapter` when asked for the deploy script tag. This will deploy the OFTAdapter contract on Ethereum Sepolia testnet.

Finally, third deployment step is to deploy the `ZamaOFT` contract on the Arbitrum Sepolia testnet. Again, you should run:

```bash
npx hardhat lz:deploy
```

You can still select either both networks or alternatively uniquely the `arbitrum-testnet` network this time. Now enter `ZamaOFT` when requested for the deploy script tag. This will deploy the `ZamaOFT` contract on Arbitrum Sepolia testnet.

## Step 5 : Etherscan verification (Optional)

In order verify the two contracts (`ZamaERC20` and `ZamaOFTAdapter`) on Etherscan for the Ethereum Sepolia network, use this command (don't worry if the following scripts return you an error or an invalid explorer URL link - see the note at the end of this step):

```bash
pnpm verify:etherscan:ethereum:sepolia
```

And to verify the `ZamaOFT` contract deployed on the Arbitrum Sepolia network run:

```bash
pnpm verify:etherscan:arbitrum:sepolia
```

**Note:** Due to a bug in the `verify-contract` task, sometimes those scripts will log an error and/or return a wrong URL for block explorer link, but most of the times, despite those errors, if you check the actual results by searching for corresponding contracts addresses on the block explorer, you will notice that the contracts will actually be succesfully verified after running those commands (i.e [https://sepolia.etherscan.io/](https://sepolia.etherscan.io/) for Ethereum testnet and [https://sepolia.arbiscan.io/](https://sepolia.arbiscan.io/) for Arbitrum testnet).

## Step 6 : Wire contracts

```bash
npx hardhat lz:oapp:wire --oapp-config layerzero.config.ts
```

And follow straightforward instructions to wire the `ZamaOFTAdapter` contract on Ethereum testnet with the `ZamaOFT` on Arbitrum testnet.

## Step 7 : Cross-chain transfer

For example you can send `1.5` ZAMA token from the deployer wallet to a custom receiver address by running this command and following instructions:

```bash
npx hardhat lz:oft:send --src-eid 40161 --dst-eid 40231 --amount 1.5 --to <RECEIVER_ADDRESS>
```

**Note** In the OFTAdapter case here, contrarily to the OFT case, 2 transactions are sent in previous script instead of 1, because the sender must first approve the corresponding amount of the ERC20 token to the OFTAdapter (i.e calling approve method on the ERC20 method and passing the OFTAdapter address and correct amount as parameters), before locking them to the OFTAdapter contract in a second `send` transaction on OFTAdapter (reminder: in the OFT case, initiating a token transfer happens by directly burning an amount of the OFT contract by calling the `send` method of the OFT contract).

Once these transactions are sent, wait around 2 minutes and check the receiver's account on Etherscan Arbitrum Sepolia explorer that the receiver indeed received `1.5` ZAMA token on Arbitrum testnet by clicking on `Token Holdings` there.

You could then also send back the tokens from Arbitrum Sepolia testnet to Ethereum Sepolia chain, by swapping the values of `--src-eid` and `--dst-eid` flags from previous command.
