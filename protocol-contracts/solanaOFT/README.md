# How to add ZamaOFT on Solana Chain

Currently, we have `ZamaERC20` and `ZamaOFTAdapter` deployed on Ethereum mainnet (and `ZamaOFT` deployed on both Gateway and BNB mainnet). The `ZamaOFTAdapter` contract's owner and delegate are already setup to be an Aragon DAO contract.

The goal of this runbook is to guide you step by step on how to deploy a ZAMA OFT instance on Solana Chain, and how to wire it to the already deployed `ZamaOFTAdapter` on Ethereum, via the Aragon DAO. We only add a single bidirectional pathway: `Solana <-> Ethereum`.

## Step 1 : Deploy the OFT on Solana Chain

First make sure you have installed all needed dependencies via `pnpm i` and filled the `.env` file correctly: see [`.env.example`](./.env.example) file. Default recommendation is to fill only those 3 values: `PRIVATE_KEY`, `RPC_URL_ETHEREUM` and `RPC_URL_SOLANA` values, as this will use the default Solana config and local keypair, which should be setup first via `solana config set --url mainnet-beta` and the local keypair generated with `solana-keygen` command.

For the `RPC_URL_SOLANA` value, we recommend getting one from [Helius](https://www.helius.dev/).

Please make sure to fund your public key with 5 SOL on Solana mainnet, before deploying.

### Prepare the Solana OFT Program keypair

Run `anchor keys sync -p oft` command. This will create the OFT `programId` keypair and also automatically update `Anchor.toml` to use the generated keypair's public key. The default path for the program's keypair will be `target/deploy/oft-keypair.json`. The program keypair is only used for initial deployment of the program.

**Optional:** you can use `anchor keys list` to view the program ID's based on the generated keypairs.

Copy the `oft` program ID value for use in the build step later.

### Building the Solana OFT Program

Ensure you have Docker running before running the build command.

This step could take more than half an hour:

```bash
anchor build -v -e OFT_ID=<OFT_PROGRAM_ID>
```

Where `<OFT_PROGRAM_ID>` is replaced with your OFT Program ID copied from the previous step.

### Deploying with a priority fee

The `deploy` command will run with a priority fee. Read the section on ['Deploying Solana programs with a priority fee'](https://docs.layerzero.network/v2/developers/solana/technical-reference/solana-guidance#deploying-solana-programs-with-a-priority-fee) to learn more.

### Run the deploy command

First make sure your local key has at least `5 SOL`, for e.g you can check it via `solana balance` if you use the keypair at the default path.

```bash
solana program deploy --program-id target/deploy/oft-keypair.json target/verifiable/oft.so -u mainnet-beta --with-compute-unit-price <COMPUTE_UNIT_PRICE_IN_MICRO_LAMPORTS>
```

Usually a value of `200000` is good for `<COMPUTE_UNIT_PRICE_IN_MICRO_LAMPORTS>`. You can get a more accurate value by visiting [this site](https://www.quicknode.com/gas-tracker/solana).

### Create the Solana OFT

Rename the `layerzero.config.mainnet.ts` file at the root of this project as `layerzero.config.ts`. Then run this command:

```bash
pnpm hardhat lz:oft:solana:create --eid 30168 --program-id <OFT_PROGRAM_ID> --only-oft-store true
```

The above command will create a Solana OFT which will have only the OFT Store as the Mint Authority.

## Step 2 : Wire the Solana devnet OFT to Ethereum Sepolia OFTAdapter

This can be done easily, since your deployer hot wallet is still the owner and delegate of the OFT instance on Solana Chain - later, after full wiring on both chains, owner/admin and delegate roles, as well as other Solana-specific roles, should be transferred to governance on Solana Chain, which should be a Squads multisig wallet (see last section of this runbook).

Run the following command to initialize the SendConfig and ReceiveConfig Accounts. This step is unique to pathways that involve Solana.

```bash
npx hardhat lz:oft:solana:init-config --oapp-config layerzero.config.ts
```

Run the wiring task on the Solana side:

```bash
pnpm hardhat lz:oapp:wire --oapp-config layerzero.config.ts --skip-connections-from-eids 30101
```

## Step 3 : Wire the Ethereum OFTAdapter to Solana OFT

This step is more complex, since the delegate of the OFTAdapter is an Aragon DAO, i.e it requires creating, approving and executing a DAO proposal via the Aragon DAO.

First, create an `ethereum-wiring.json` file containing the different transactions needed to be done, by running:

```
npx hardhat lz:oapp:wire --oapp-config layerzero.config.ts --output-filename ethereum-wiring.json
```

When running previous command, select **no** when requested if you would you like to submit the required transactions (otherwise it would fail anyways). You should now have generated a new `ethereum-wiring.json` file in the root of the directory.

Now, run:

```
npx ts-node scripts/convertToAragonProposal.ts ethereum-wiring.json aragonProposal.json
```

This will convert the `ethereum-wiring.json` file to a new `aragonProposal.json` which could be directly uploaded inside the Aragon App UI, when creating an Aragon proposal, to streamling the process.

More precisely, in Aragon App, when you reach "Step 2 of 3" of proposal creation, click on the `Upload` button there, in select the newly created `aragonProposal.json` file to upload it and create the wiring proposal on Ethereum.

After voting and execution of the wiring proposal, your OFT is now successfully setup.

## Step 4 : Test OFT transfers

First, make sure the Solidity contracts are compiled by running `npx hardhat compile`.

Send 1 OFT from **Ethereum** to **Solana**:

```bash
npx hardhat lz:oft:send --src-eid 30101 --dst-eid 30168 --to <SOLANA_ADDRESS>  --amount 1
```

Send 1 OFT from **Solana** to **Ethereum**:

```bash
npx hardhat lz:oft:send --src-eid 30168 --dst-eid 30101 --to <EVM_ADDRESS>  --amount 1
```

Upon a successful send, the script will provide you with the link to the message on LayerZero Scan.

Once the message is delivered, you will be able to click on the destination transaction hash to verify that the OFT was sent.

Congratulations, you have now sent an OFT between Solana and Ethereum!

## Step 5 : Upload the Anchor IDL

Upload the Anchor IDL for easier debugging on block explorers:

```
anchor idl init <OFT_PROGRAM_ID> --filepath target/idl/oft.json --provider.cluster mainnet --provider.wallet ~/.config/solana/id.json
```

You can then check on [`https://explorer.solana.com/`](https://explorer.solana.com/) that the Anchor IDL has been correctly uploaded. By the way, this step creates a new PDA with a new privileged role: the Anchor IDL Authority.

## Step 6 : Transfer all privileged roles

Let's suppose you already have deployed a Squads Multisig on Solana. Carefully note its **Vault id**, not to be confused with the Multisig Account id. Let's note it `SQUADS_VAULT_ID`. This is the value that we will use to transfer privileged roles from our hot wallet to the Squads Multisig.

From the 7 privileged roles of a Solana OFT listed in [Solana docs](https://docs.layerzero.network/v2/developers/solana/technical-reference/solana-guidance#transferring-oft-ownership-on-solana), we will transfer 5 of them because 2 are not applicalbe: mint authority is the OFT Store PDA, so no trasfer is needed, and freeze authority is `None`. You can verify this by using: `npx hardhat lz:oft:solana:debug`.

The 5 applicable roles to be transferred are:

- The 2 LayerZero-specific roles: owner (also called admin) and delegate.
- 3 Solana-specific roles: upgrade authority, token metadata update authority, Anchor IDL authority.

Program verification depends on upgrade authority, so will be done as a last step via a Squads proposal.

### Transfer delegate and owner role

Let's start with the 2 LayerZero-specific roles:

The transfer of both require modifying your LZ Config file and running helper tasks.

Overall, you should carry out these steps:

1/ Modify `layerzero.config.ts` to include **only** the new delegate address. This means that the exported function at the end of the config file should be as:

```
export default async function () {
    // note: pathways declared here are automatically bidirectional
    // if you declare A,B there's no need to declare B,A
    const connections = await generateConnectionsConfig([
        [
            ethereumContract, // Chain A contract
            solanaContract, // Chain B contract
            [['LayerZero Labs'], [['Nethermind', 'Luganodes', 'P2P'], 2]], // [ requiredDVN[], [ optionalDVN[], threshold ] ]
            [15, 32], // [A to B confirmations, B to A confirmations]
            [SOLANA_ENFORCED_OPTIONS, EVM_ENFORCED_OPTIONS], // Chain B enforcedOptions, Chain A enforcedOptions
        ],
    ])

    return {
        contracts: [
            { contract: ethereumContract },
            {
                contract: solanaContract,
                config: {
                    delegate: '<SQUADS_VAULT_ID>',
                },
            },
        ],
        connections,
    }
}
```

Notice from this snippet that only the returned value changed, and that `<SQUADS_VAULT_ID>` should be replaced by its actual value.

2/ Run `pnpm hardhat lz:oapp:wire --oapp-config layerzero.config.ts`.

3/ Modify `layerzero.config.ts` to include the new owner address. This means that the exported function at the end of the config file should be now as:

```
export default async function () {
    // note: pathways declared here are automatically bidirectional
    // if you declare A,B there's no need to declare B,A
    const connections = await generateConnectionsConfig([
        [
            ethereumContract, // Chain A contract
            solanaContract, // Chain B contract
            [['LayerZero Labs'], [['Nethermind', 'Luganodes', 'P2P'], 2]], // [ requiredDVN[], [ optionalDVN[], threshold ] ]
            [15, 32], // [A to B confirmations, B to A confirmations]
            [SOLANA_ENFORCED_OPTIONS, EVM_ENFORCED_OPTIONS], // Chain B enforcedOptions, Chain A enforcedOptions
        ],
    ])

    return {
        contracts: [
            { contract: ethereumContract },
            {
                contract: solanaContract,
                config: {
                    delegate: '<SQUADS_VAULT_ID>',
                    owner: '<SQUADS_VAULT_ID>',
                },
            },
        ],
        connections,
    }
}
```

4/ Run `pnpm hardhat lz:ownable:transfer-ownership --oapp-config layerzero.config.ts`.

You have now transferred both owner and delegate of your Solana OFT. You can check this via `npx hardhat lz:oft:solana:debug`. From those logs, the owner address is the value of the `Admin` key.

### Transfer upgrade authority

You must run this command:

```
solana program set-upgrade-authority --skip-new-upgrade-authority-signer-check <OFT_PROGRAM_ID> --new-upgrade-authority <SQUADS_VAULT_ID>
```

New upgrade authority can be easily checked via any Solana block explorer.

### Transfer token metadata update authority

Use this command by replacing `<MINT_ADDRESS>` by its actual value, which could be read from `deployments/solana-mainnet/OFT.json`:

```
pnpm hardhat lz:oft:solana:set-update-authority --eid 30168 --mint <MINT_ADDRESS> --new-update-authority <SQUADS_VAULT_ID>
```

Result can be checked via `npx hardhat lz:oft:solana:debug` (in the `Update Authority` field).

### Transfer Anchor IDL authority

Use:

```
anchor idl set-authority --program-id <OFT_PROGRAM_ID> --new-authority <SQUADS_VAULT_ID> --provider.cluster mainnet --provider.wallet ~/.config/solana/id.json
```

Result can be checked via `anchor idl authority <OFT_PROGRAM_ID> --provider.cluster mainnet`.

## Step 7 : Verifying OFT program via Squads

Make sure you have `solana-verify 0.4.11` version installed.

### Optional: Compare locally

If you wish to, you can view the program hash of the locally built OFT program:

```
solana-verify get-executable-hash ./target/verifiable/oft.so
```

And then compare it with the on-chain program hash:

```
solana-verify get-program-hash -u mainnet <OFT_PROGRAM_ID>
```

### Program verification

Run the following command to verify against the repo that contains the program source code:

```
solana-verify verify-from-repo -um --program-id <OFT_PROGRAM_ID> --mount-path examples/oft-solana https://github.com/LayerZero-Labs/devtools --library-name oft -b solanafoundation/solana-verifiable-build:2.1.0 -- --config env.OFT_ID=\'<OFT_PROGRAM_ID>\'
```

Then after few minutes, you will see logs such as:

```
Program hash matches ✅
Do you want to upload the program verification to the Solana Blockchain? (y/n)
```

You should answer yes (by entering `yes`), this will send transactions from your local hot wallet, but is not enough for verification, since your local hot wallet is no longer the upgrade authority. You also need following additional step:

```
solana-verify export-pda-tx https://github.com/LayerZero-Labs/devtools --program-id <OFT_PROGRAM_ID> --uploader <SQUADS_VAULT_ID> --mount-path examples/oft-solana --library-name oft -b solanafoundation/solana-verifiable-build:2.1.0 -- --config env.OFT_ID=\'<OFT_PROGRAM_ID>\'
```

After some time, it will return a base58 string that represents the transaction data for uploading the verification PDA. Make sure your Squads Vault is enough funded (should have at least 0.1 SOL).Then import this base58 string into Squads for approval and execution.

Finally run:

```
solana-verify remote submit-job --program-id <OFT_PROGRAM_ID> --uploader <SQUADS_VAULT_ID> --url mainnet
```
