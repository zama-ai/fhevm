# Deploy initial contracts

Following is an example of how to deploy initial contracts on the Ethereum Sepolia testnet. Deploying on Ethereum mainnet should be almost identical and should be possible by just changing the SEPOLIA_RPC_URL to ETHEREUM_MAINNET_RPC_URL and poiting to a correct RPC node.

0/ Prerequisites: First, git clone `fhevm` repo and checkout the [`0.6.0-0` prerelease branch](https://github.com/zama-ai/fhevm/tree/v0.6.0-0), install dependencies with `npm i` (node.js version should be at least `20`), then create a`.env` file in root of the repo.

1/ Fill correct values in the `.env` file by first copying the [`.env.example.deployment` file](https://github.com/zama-ai/fhevm/blob/v0.6.0-0/.env.example.deployment).
Your `.env` file should have the following format, while replacing the 2 private keys values `PRIVATE_KEY_FHEVM_DEPLOYER` and `PRIVATE_KEY_GATEWAY_DEPLOYER` with your own keys, and taking the KMS addresses to be aligned with what is used by KMS, as well as the `ADDRESS_GATEWAY_RELAYER` relayer address to be aligned with the Gateway service, and the `ADDRESS_COPROCESSOR` coprocessor account address to be aligned with the coprocessor service:

```
export PRIVATE_KEY_FHEVM_DEPLOYER="0c66d8cde71d2faa29d0cb6e3a567d31279b6eace67b0a9d9ba869c119843a5e"
export PRIVATE_KEY_GATEWAY_DEPLOYER="717fd99986df414889fd8b51069d4f90a50af72e542c58ee065f5883779099c6"
export ADDRESS_GATEWAY_RELAYER="0x97F272ccfef4026A1F3f0e0E879d514627B84E69"
export NUM_KMS_SIGNERS="4"
export ADDRESS_KMS_SIGNER_0="0x0971C80fF03B428fD2094dd5354600ab103201C5"
export ADDRESS_KMS_SIGNER_1="0xB68deCb047B5e6Cc82280502A7E2318c6b3E5eC6"
export ADDRESS_KMS_SIGNER_2="0xfe0fB0BCceb872ee7a6ef6c455e6E127Aef55DD7"
export ADDRESS_KMS_SIGNER_3="0x2dac5193bE0AB0eD8871399E6Ae61EAe6cc8cAE1"
export ADDRESS_COPROCESSOR_ACCOUNT="0xc9990FEfE0c27D31D0C2aa36196b085c0c4d456c"
export IS_COPROCESSOR="true"
export SEPOLIA_RPC_URL="https://sepolia.infura.io/v3/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
export ETHERSCAN_API_KEY="XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
```

For the `SEPOLIA_RPC_URL` env variable, you can either get one from a service provider for free like Infura, or use your own RPC URL if you are running a node yourself. For `ETHERSCAN_API_KEY` it is needed to verify source code of smart contracts on Sepolia/mainnet Etherscan (if you deploy coprocessor on Sepolia or mainnet for instance) and you can get a free Etherscan API key from: [https://docs.etherscan.io/getting-started/viewing-api-usage-statistics](https://docs.etherscan.io/getting-started/viewing-api-usage-statistics).

**Important** : the `PRIVATE_KEY_FHEVM_DEPLOYER` and `PRIVATE_KEY_GATEWAY_DEPLOYER` are expected to have a nonce of `0` initially (i.e never sent any tx before with those) for the deployment scripts to succeed later. If you have [foundry](https://book.getfoundry.sh/getting-started/installation) installed, you can generate fresh Ethereum private key / address pairs locally with this command:
`cast wallet new`.

2/ Then run the precomputing contract addresses script [`./precompute-addresses.sh`](https://github.com/zama-ai/fhevm/blob/v0.6.0-0/precompute-addresses.sh) -> this will write on disk the correct contract addresses needed to launch the modified Geth node (`TFHEExecutor` address) and the Gateway service (`GatewayContract` one) and the `ACL` and `KMSVerifier` addresses which would be needed to setup some values inside the ASC contract on KMS chain. The precomputed addresses for the core contracts are located inside:

- `node_modules/fhevm-core-contracts/addresses/.env.acl` for ACL address
- `node_modules/fhevm-core-contracts/addresses/.env.exec` for TFHEExecutor address
- `node_modules/fhevm-core-contracts/addresses/.env.kmsverifier` for KMSVerifier address
- `node_modules/fhevm-core-contracts/addresses/.env.inputverifier` for InputVerifier address
- `node_modules/fhevm-core-contracts/addresses/.env.fhepayment` for FHEPayment address
- `gateway/.env.gateway` for GatewayContract address.

This script is found exactly inside the [`./precompute-addresses.sh` file](https://github.com/zama-ai/fhevm/blob/main/precompute-addresses.sh):

```
#!/bin/bash
npx hardhat clean

PRIVATE_KEY_GATEWAY_DEPLOYER=$(grep PRIVATE_KEY_GATEWAY_DEPLOYER .env | cut -d '"' -f 2)
PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)

npx hardhat task:computeGatewayAddress --private-key "$PRIVATE_KEY_GATEWAY_DEPLOYER"
npx hardhat task:computeACLAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeTFHEExecutorAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeKMSVerifierAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeInputVerifierAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true
npx hardhat task:computeFHEPaymentAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
```

3/ Launch and setup the Geth node, the Gateway service, setup the KMS, all these using previously precomputed addresses.

4/ Use the faucet to give some funds to the `FHEVM_DEPLOYER` address, and the and the `GATEWAY_DEPLOYER` addresses. Note that you can compute the address corresponding to a private key, if you have [foundry](https://book.getfoundry.sh/getting-started/installation) installed, via this command: `cast wallet address [PRIVATE_KEY]`.

**Important**: Later after deployment, the `GATEWAY_RELAYER` should also receive a LOT more funds than the other addresses since this is the account which will send tx one each decryption fulfilment. I would advise for eg `3`ETH for the `FHEVM_DEPLOYER` account, `0.5`ETH for the `GATEWAY_DEPLOYER` to be safe, and at least `10`ETH for the `GATEWAY_RELAYER` at first (to be refilled later if its balance becomes low) to be on the safe side. Another advise is to check current gas price on Sepolia before deployment and avoiding periods where gas prices are spiking to huge values (>100 GWei), for eg by consulting

<!-- markdown-link-check-disable -->

[https://sepolia.beaconcha.in/gasnow](https://sepolia.beaconcha.in/gasnow).

<!-- markdown-link-check-enable-->

The funding of `GATEWAY_RELAYER` account is the only part which is not strictly needed during deployment, and this account could be funded later, after all deployment steps will be completed.

5/ Finally, run the deployment script [`./launch-fhevm-sepolia.sh`](https://github.com/zama-ai/fhevm/blob/v0.6.0-0/launch-fhevm-sepolia.sh).
This script is found exactly inside the [`./launch-fhevm-sepolia.sh` file](https://github.com/zama-ai/fhevm/blob/v0.6.0-0/launch-fhevm-sepolia.sh) :

```
#!/bin/bash
# This script should be launched after precomputing the addresses via `precompute-addresses.sh`, and preferably after setting up the different services - KMS, Geth node, Gateway
npx hardhat clean

PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)

rm -rf fhevmTemp/
mkdir -p fhevmTemp
cp -L -r node_modules/fhevm-core-contracts/ fhevmTemp/
npx hardhat compile:specific --contract fhevmTemp
npx hardhat compile:specific --contract lib
npx hardhat compile:specific --contract gateway

npx hardhat task:deployACL --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployTFHEExecutor --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployKMSVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployInputVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployFHEPayment --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia

npx hardhat task:addSigners --num-signers "$NUM_KMS_SIGNERS" --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true --network sepolia

npx hardhat task:launchFhevm --skip-get-coin true --use-address true --network sepolia

echo "Waiting 2 minutes before contract verification... Please wait..."
sleep 120 # makes sure that contracts bytescode propagates on Etherscan, otherwise contracts verification might fail in next step
npx hardhat task:verifyACL --network sepolia
npx hardhat task:verifyTFHEExecutor --network sepolia
npx hardhat task:verifyKMSVerifier --network sepolia
npx hardhat task:verifyInputVerifier --network sepolia
npx hardhat task:verifyFHEPayment --network sepolia
npx hardhat task:verifyGatewayContract --network sepolia
```

Note that in previous example, we supposed deployment would happen on Sepolia, this is why we used the `--network sepolia` flag. Note also the last 6 lines which are responsible to verify all contracts which have just been deployed on Sepolia, using the Etherscan API key, after waiting 2 minutes to make sure that the contracts bytecode were correctly propagated. This last command only makes sense if you plan to deploy on a network with Etherscan support, such as Sepolia or Ethereum mainnet.

**Important:** at this stage the `openzeppelin-upgrades` plugin will write on disk an `.openzeppelin/` folder with some files in it (for eg typically a `sepolia.json` file if you deploy on Sepolia). It will be critical to keep this folder saved, to be able to do safe upgrades of smart contracts if needed some day. Similarly, you must also keep all the `.env.XXX` files written inside both of `node_modules/fhevm-core-contracts/addresses` and `gateway/` directories saved, because those contains the proxy addresses which might also be needed during the upgrade process.

# After deployment - Optional

## For launching tests

1/ After deployment, if you wish to launch some hardhat test on Sepolia, you first need to add a `MNEMONIC` env variable inside your `.env` file. You can generate this variable using foundry with this command: `cast wallet new-mnemonic -w 15 -a 5`. Your new `.env` file should then looks like this - please replace `MNEMONIC` by your own value:

```
export MNEMONIC="industry cruise album quality blur observe oppose regret federal carbon mean afford thumb galaxy talk"
export PRIVATE_KEY_FHEVM_DEPLOYER="0c66d8cde71d2faa29d0cb6e3a567d31279b6eace67b0a9d9ba869c119843a5e"
export PRIVATE_KEY_GATEWAY_DEPLOYER="717fd99986df414889fd8b51069d4f90a50af72e542c58ee065f5883779099c6"
export ADDRESS_GATEWAY_RELAYER="0x97F272ccfef4026A1F3f0e0E879d514627B84E69"
export NUM_KMS_SIGNERS="4"
export ADDRESS_KMS_SIGNER_0="0x0971C80fF03B428fD2094dd5354600ab103201C5"
export ADDRESS_KMS_SIGNER_1="0xB68deCb047B5e6Cc82280502A7E2318c6b3E5eC6"
export ADDRESS_KMS_SIGNER_2="0xfe0fB0BCceb872ee7a6ef6c455e6E127Aef55DD7"
export ADDRESS_KMS_SIGNER_3="0x2dac5193bE0AB0eD8871399E6Ae61EAe6cc8cAE1"
export ADDRESS_COPROCESSOR="0xc9990FEfE0c27D31D0C2aa36196b085c0c4d456c"
export IS_COPROCESSOR="true"
export SEPOLIA_RPC_URL="https://sepolia.infura.io/v3/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
export ETHERSCAN_API_KEY="XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
```

2/ The previous command will also show you the `5` addresses of accounts (alice, bob, carol, dave, eve) used inside the hardhat tests, that you will need to fund via the faucet. An alternative way to get the addresses of those accounts could be to simply run: `npx hardhat accounts --network sepolia`, then the first 5 addresses logged will be those that need funding.

**Important:** Currently, for all tests, you just need to fund the first 3 accounts (alice, bob, carol), this could help you to avoid wasting gas. For the EncryptedERC20 ones in particular, you only need to fund the first 2 accounts (alice, bob). No need to actually fund all of the first 5 accounts, unless you want later to run more complex tests like the Confidential AMM, etc, which have not been updated for new Solidity API anyways, yet.

3/ Now you can run any hardhat test with the usual command. For instance, if you want to run the EncryptedERC20 related tests you can use: `npx hardhat test test/encryptedERC20/* --network sepolia`.
As another example, you could run the asynchronous decryption tests via: `npx hardhat test test/gatewayDecrypt/* --network sepolia`.

## For upgrading an fhevm contract

1/ For upgrading the ACL contract, save the new implementation in a solidity file somewhere in your local `fhevm` repo, then simply run this command:

```
npx hardhat task:upgradeACL --current-implementation fhevmTemp/contracts/ACL.sol:ACL --new-implementation examples/ACLUpgradedExample.sol:ACLUpgradedExample --private-key [PRIVATE_KEY_FHEVM_DEPLOYER] --verify-contract true --network sepolia
```

In previous command, you should modify `PRIVATE_KEY_FHEVM_DEPLOYER` with your own value, and replace `examples/ACLUpgradedExample.sol:ACLUpgradedExample` with the path and the name of the new ACL implementation contract that you wish to use. The `--verify-contract` is an optional flag, whish will add an Etherscan verification step after deployment of the new implementation, pausing 2 minutes between deployment and Etherscan verification.

You can use similar commands for any contracts that you wish to upgrade, just using any of the following tasks, to replace the `task:upgradeACL` from previous upgrade command :

```

task:upgradeACL
task:upgradeTFHEExecutor
task:upgradeKMSVerifier
task:upgradeInputVerifier
task:upgradeFHEPayment
task:upgradeGatewayContract

```

**Important:** Note that for the `task:upgradeGatewayContract` you should use the `PRIVATE_KEY_GATEWAY_DEPLOYER` as the `private-key` value.
Also, note that those upgrade scripts would only work if the updated implementations do not have a reintializer function, i.e if part of storage does not need to be reinitialized, which should be almost always the case. In the rare case where you need a reinitializer in the new implementation, the upgrade script should be customized according to the arguments needed for the reinitialization.
