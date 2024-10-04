# Deploy initial contracts

0/ Prerequisites: First, git clone fhevm repo main branch, install dependencies with `npm i` (node version at least 20), then create a `.env` file in root of the repo.

1/ Edit correct values in the `.env` file taking inspiration from the [`.env.example` file](https://github.com/zama-ai/fhevm/blob/main/.env.example). You dont need to setup all those values: you can drop API keys at the end (for now, but we would need to use some Sepolia API for contracts deployment later and Etherscan API for contract verification TODO). You can also drop the MNEMONIC one which is no longer used for deployment, as well as all the PRIVATE_KEY_KMS_SIGNER_XXX ones (only useful for mocked mode).
For instance, your `.env` file should have the following format, while replacing private keys values, and taking the KMS addresses to be aligned with what is used by KMS:

```
export PRIVATE_KEY_FHEVM_DEPLOYER="0c66d8cde71d2faa29d0cb6e3a567d31279b6eace67b0a9d9ba869c119843a5e"
export PRIVATE_KEY_GATEWAY_DEPLOYER="717fd99986df414889fd8b51069d4f90a50af72e542c58ee065f5883779099c6"
export PRIVATE_KEY_GATEWAY_OWNER="717fd99986df414889fd8b51069d4f90a50af72e542c58ee065f5883779099c6"
export PRIVATE_KEY_GATEWAY_RELAYER="7ec931411ad75a7c201469a385d6f18a325d4923f9f213bd882bbea87e160b67"
export NUM_KMS_SIGNERS="4"
export ADDRESS_KMS_SIGNER_0="0x0971C80fF03B428fD2094dd5354600ab103201C5"
export ADDRESS_KMS_SIGNER_1="0xB68deCb047B5e6Cc82280502A7E2318c6b3E5eC6"
export ADDRESS_KMS_SIGNER_2="0xfe0fB0BCceb872ee7a6ef6c455e6E127Aef55DD7"
export ADDRESS_KMS_SIGNER_3="0x2dac5193bE0AB0eD8871399E6Ae61EAe6cc8cAE1"
export PRIVATE_KEY_COPROCESSOR_ACCOUNT="7ec8ada6642fc4ccfb7729bc29c17cf8d21b61abd5642d1db992c0b8672ab901"
export IS_COPROCESSOR="true"
```

**Important notice** : the PRIVATE_KEY_FHEVM_DEPLOYER and PRIVATE_KEY_GATEWAY_DEPLOYER are expected to have a nonce of 0 initially (i.e never sent any tx before with those) for the deployment scripts to succeed later.

2/ Then run the precompute addresses tasks -> this will write on disk the correct addresses needed to launch the modified Geth node (TFHEExecutor) and the Gateway service (GatewayContract one) and I think the ACL and KMSVerifier addresses which would be needed to setup some values inside the ASC contract on KMS chain, in files such as: `lib/.env.acl`, `lib/.env.kmsverifier`, `gateway/lib/GatewayContractAddress.sol` etc. This script typically contains first part of `./launch-fhevm.sh` :

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

3/ Launch the Geth node, the Gateway service, setup the KMS, all these using previously precomputed addresses.

4/ Use the faucet to give some funds to the FHEVM_DEPLOYER address, the GATEWAY_DEPLOYER, the GATEWAY_OWNER and the GATEWAY_RELAYER addresses. **Important**: The GATEWAY_RELAYER should receive a LOT more funds than the other addresses fyi since this is the account which will send tx one each decryption fulfilment. I would advise for eg 0.1 ETH for the 3 other accounts, but more than 10 ETH for the GATEWAY_RELAYER to be safe.

5/ Run the deployment scripts after the commented line. **Important:** at this stage the openzeppelin-upgrades plugin will write on disk an `.openzeppelin/` folder with some files in it. It will be critical to keep this folder saved, to be able to do safe upgrades of smart contracts if needed some day. Typically this script contains the second part of `./launch-fhevm.sh` :

```
#!/bin/bash
npx hardhat clean

PRIVATE_KEY_GATEWAY_DEPLOYER=$(grep PRIVATE_KEY_GATEWAY_DEPLOYER .env | cut -d '"' -f 2)
PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)
IS_COPROCESSOR=$(grep IS_COPROCESSOR .env | cut -d '"' -f 2)

if [ "$IS_COPROCESSOR" = "true" ]; then
    cp lib/InputVerifier.sol.coprocessor lib/InputVerifier.sol
else
    cp lib/InputVerifier.sol.native lib/InputVerifier.sol
fi
npx hardhat compile:specific --contract lib
npx hardhat compile:specific --contract gateway

npx hardhat task:deployACL --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:deployTFHEExecutor --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:deployKMSVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:deployInputVerifier --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"
npx hardhat task:deployFHEPayment --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER"

npx hardhat task:addSigners --num-signers $NUM_KMS_SIGNERS --private-key "$PRIVATE_KEY_FHEVM_DEPLOYER" --useAddress true

npx hardhat task:launchFhevm --skip-get-coin true
```
