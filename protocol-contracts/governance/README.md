## Governance OApp

1/ Fill the `.env` file (see `.env.example`), use a `PRIVATE_KEY` from an account funded on both chains.

2/ Deploy the `GovernanceOAppSender` contract by running:
```bash
npx hardhat lz:deploy 
```
And select `ethereum-testnet` network then enter the `GovernanceOAppSender` script.

3/ Deploy the `GovernanceOAppReceiver` contract by running:
```bash
npx hardhat lz:deploy 
```
And select `gateway-testnet` network then enter the `GovernanceOAppReceiver` script.

4/ Wire contracts:
```bash
npx hardhat lz:oapp:wire --oapp-config layerzero.config.testnet.ts
```

5/ After the Safe and AdminModule have been deployed, run: 
```bash
npx hardhat task:setAdminSafeModule --module <ADMIN_MODULE_ADDRESS> --network gateway-testnet
```

6/ Verify contracts: 
```bash
pnpm verify:etherscan:ethereum:testnet
pnpm verify:etherscan:gateway:testnet
```