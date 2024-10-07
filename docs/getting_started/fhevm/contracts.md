# Deploy initial contracts

0/ Prerequisites: First, git clone `fhevm` repo main branch, install dependencies with `npm i`(node version at least 20), then create a`.env` file in root of the repo.

1/ Edit correct values in the `.env` file taking inspiration from the [`.env.example.deployment` file](https://github.com/zama-ai/fhevm/blob/main/.env.example.deployment).
For instance, your `.env` file should have the following format, while replacing the 2 private keys values, and taking the KMS addresses to be aligned with what is used by KMS, as well as the Gateway relayer address to be aligned with the Gateway service, and the coprocessor account address to be aligned with the coprocessor service:

```
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
export SEPOLIA_RPC_RUL="https://sepolia.infura.io/v3/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
export ETHERSCAN_API_KEY="XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
```

For the `SEPOLIA_RPC_RUL` env variable, you can either get one from a service provider for free like Infura, or use your own RPC URL if you are running a node yourself. For `ETHERSCAN_API_KEY` it is needed to verify source code of smart contracts on Sepolia Etherscan (if you deploy coprocessor on Sepolia for instance) and you can get a free Etherscan API key from: [https://docs.etherscan.io/getting-started/viewing-api-usage-statistics](https://docs.etherscan.io/getting-started/viewing-api-usage-statistics).

**Important** : the PRIVATE_KEY_FHEVM_DEPLOYER and PRIVATE_KEY_GATEWAY_DEPLOYER are expected to have a nonce of `0` initially (i.e never sent any tx before with those) for the deployment scripts to succeed later. If you have [foundry](https://book.getfoundry.sh/getting-started/installation) installed, you can generate fresh Ethereum private key / address pairs locally with this command:
`cast wallet new`

2/ Then run the precompute addresses tasks -> this will write on disk the correct addresses needed to launch the modified Geth node (TFHEExecutor) and the Gateway service (GatewayContract one) and I think the ACL and KMSVerifier addresses which would be needed to setup some values inside the ASC contract on KMS chain, in files such as: `lib/.env.acl`, `lib/.env.kmsverifier`, `gateway/env.gateway` etc. This script typically contains first part of [`./launch-fhevm.sh`](https://github.com/zama-ai/fhevm/blob/main/launch-fhevm.sh) :

```
#!/bin/bash
npx hardhat clean

PRIVATE_KEY_GATEWAY_DEPLOYER=$(grep PRIVATE_KEY_GATEWAY_DEPLOYER .env | cut -d '"' -f 2)
PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)

npx hardhat task:computeGatewayAddress --private-key "$PRIVATE_KEY_GATEWAY_DEPLOYER"
npx hardhat task:computeACLAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeTFHEExecutorAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeKMSVerifierAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeInputVerifierAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:computeFHEPaymentAddress --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
```

3/ Launch and setup the Geth node, the Gateway service, setup the KMS, all these using previously precomputed addresses.

4/ Use the faucet to give some funds to the `FHEVM_DEPLOYER` address, and the and the ` GATEWAY_DEPLOYER` addresses. Note that you can compute the address corresponding to a private key, if you have [foundry](https://book.getfoundry.sh/getting-started/installation) installed, via this command: `cast wallet address [PRIVATE_KEY]`.

**Important**: Later after deployment, the `GATEWAY_RELAYER` should also receive a LOT more funds than the other addresses since this is the account which will send tx one each decryption fulfilment. I would advise for eg 3 ETH for the `FHEVM_DEPLOYER` account, 1 ETH for the `GATEWAY_DEPLOYER` to be safe, and at least 10-20 ETH for the `GATEWAY_RELAYER` at first (to be refilled later if its balance becomes low) to be on the safe side. Another advise is to check current gas price on Sepolia before deployment and avoiding periods where gas prices are spiking to huge values (>100 GWei), for eg by consulting

<!-- markdown-link-check-disable -->

[https://sepolia.beaconcha.in/gasnow](https://sepolia.beaconcha.in/gasnow).

<!-- markdown-link-check-enable-->

The funding of `GATEWAY_RELAYER` account is the only part which is not strictly needed during deployment, and this account could be funded later, after all deployment steps will be completed.

5/ Run the deployment scripts after the commented line. **Important:** at this stage the openzeppelin-upgrades plugin will write on disk an `.openzeppelin/` folder with some files in it (for eg typically a `sepolia.json` file if you deploy on Sepolia). It will be critical to keep this folder saved, to be able to do safe upgrades of smart contracts if needed some day. Typically this script contains the second part of `./launch-fhevm.sh` :

```
#!/bin/bash
npx hardhat clean

PRIVATE_KEY_GATEWAY_DEPLOYER=$(grep PRIVATE_KEY_GATEWAY_DEPLOYER .env | cut -d '"' -f 2)
PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)
IS_COPROCESSOR=$(grep IS_COPROCESSOR .env | cut -d '"' -f 2)

npx hardhat compile:specific --contract lib
npx hardhat compile:specific --contract gateway

npx hardhat task:deployACL --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployTFHEExecutor --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployKMSVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployInputVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia
npx hardhat task:deployFHEPayment --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --network sepolia

npx hardhat task:addSigners --num-signers $NUM_KMS_SIGNERS --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --use-address true

npx hardhat task:launchFhevm --skip-get-coin true --use-address true --network sepolia

echo "Waiting 2 minutes before contract verification... Please wait..."
sleep 120 # makes sure that contracts bytescode propagates on Etherscan, otherwise contracts verification might fail in next step
npx hardhat task:verifyContracts --network sepolia
```

Note that in previous example, we supposed deployment would happen on Sepolia, this is why we used the `--network sepolia` flag. Note also the last line which is responsible to verify all contracts which have just been deployed on Sepolia, using the Etherscan API key, after waiting 2 minutes to make sure that the contracts bytecode were correctly propagated. This last command only makes sense if you plan to deploy on a network with Etherscan support, such as Sepolia or Ethereum mainnet.
