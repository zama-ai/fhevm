# AdminModule for the Safe MultiSig wallet to be used on Gateway

## Deploy Multisig Safe Wallet with only the deployer as owner

Be sure to start with an `.env` - or copy paste the `.env.example.deploy` and fill its values (not to be confused with `.env.example.test` to be used to run tests in hardhat only!) - with just those filled variables: 
```
PRIVATE_KEY=
RPC_URL_ZAMA_GATEWAY_TESTNET=
BLOCKSCOUT_API=https://xxxxxxxxxxxx.xyz/api # don't forget the /api suffix at the end
``````

Then run: 
```
npx hardhat compile
npx hardhat task:deploySafe --network gateway-testnet
```

This will deploy `SafeL2` singleton contract, as well as `SafeProxyFactory` and `SafeL2Proxy`. The `SafeL2Proxy` is actually the multisig wallet which will become later the owner of `GatewayConfig`.

Then, after waiting for around 1 minute for the blockscout indexing to take into consideration newly deployed contracts, run: 
```
npx hardhat task:verifySafe --network gateway-testnet
```

This will verify on Blockscout the 3 previously deployed contracts.

## Deploy, verify and enable the AdminModule

Add in your `.env` a value for the `ADMIN_ADDRESS`, which should be the address of an already deployed `GovernanceOAppReceiver` contract.

Deploy then the AdminModule with: 

```
npx hardhat task:deployAdminModule --network gateway-testnet
```

And, after waiting for around 1 minute for the blockscout indexing, verify it with: 

```
npx hardhat task:verifyAdminModule --network gateway-testnet
```

Finally, enable the safe module with: 

```
npx hardhat task:enableAdminModule --network gateway-testnet
```

## Accept ownership of GatewayConfig contract

This step supposes that the original owner of GatewayConfig already called the `transferOwnership` function of `GatewayConfig` with the Safe proxy address as a new owner. Since `GatewayConfig` inherits from `Ownable2StepUpgradeable`, the Safe wallet still need to call `acceptOwnership` to effectively become the owner. This can be done using: 

```
npx hardhat task:acceptGatewayConfigOwnership --network gateway-testnet
```

## Change owners and threshold of the Safe proxy

This can be done in the 3 following steps, in order to be careful and limit risk of errors.

