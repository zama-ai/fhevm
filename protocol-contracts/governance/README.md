## Governance OApp

1/ Fill the `.env` file (see `.env.example`), use a `PRIVATE_KEY` from an account funded on both chains.

2/ Deploy the `GovernanceOAppSender` contract by running:

```bash
DST_EID=<YOUR_DST_EID> npx hardhat lz:deploy --tags GovernanceOAppSender --network <SRC_CHAIN>
```

Replace `<YOUR_DST_EID>` in the command above by the correct destination eid value : [click here](https://docs.layerzero.network/v2/deployments/deployed-contracts) to fetch the eid value from the destination chain (eg: `40267` for Polygon Amoy Testnet).

`<SRC_CHAIN>` should be either `ethereum-testnet` or `ethereum-mainnet`.

3/ Deploy the `GovernanceOAppReceiver` contract by running:

```bash
npx hardhat lz:deploy --tags GovernanceOAppReceiver --network <DST_CHAIN>
```

4/ Wire contracts:

Make sure to double check the correct security parameters are used inside your `LZ_CONFIG_FILE` (required and optional DVNs, threshold, number of block confirmations) then run:

```bash
npx hardhat lz:oapp:wire --oapp-config <LZ_CONFIG_FILE>
```

5/ After the Safe and AdminModule have been deployed, run:

```bash
npx hardhat task:setAdminSafeModule --module <ADMIN_MODULE_ADDRESS> --network <DST_CHAIN>
```

6/ Verify contracts by running this command on all the networks - replace `<network>` by the network name:

```bash
pnpm verify:etherscan:<network>:testnet
```
