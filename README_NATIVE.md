<p align="center">
<!-- product name logo -->
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../assets/KMS-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="../assets/KMS-light.png">
  <img width=600 alt="Zama fhEVM & KMS">
</picture>
</p>

---

<p align="center">
  <a href="./fhevm-whitepaper.pdf"> 📃 Read white paper</a> |<a href="https://docs.zama.ai/fhevm"> 📒 Documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-BSD--3--Clause--Clear-%23ffb243?style=flat-square"></a>
  <a href="https://github.com/zama-ai/bounty-program"><img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-%23ffd208?style=flat-square"></a>
</p>

## Table of Contents

- **[Getting Started](#getting-started)**
  - [Key generation](#key-generation)
  - [Fast run and test](#fast-run-and-test)
  - [Trouble shooting](#trouble-shooting)
  - [Init fhEVM-native](#init-fhevm-native)
  - [Run fhEVM-native + KMS components](#run-fhevm-native--kms-components)
  - [Stop fhEVM-native + KMS](#stop-fhevm-native--kms)
  - [Fresh start](#fresh-start)
  - [Test using fhevm](#test-using-fhevm)
    <br></br>

## Getting started

### Prerequisite

Ensure that Docker (at least version 27) is installed and running.

_Optionally_ you may update `KEY_GEN` value in `.env`. Default is `false`

| KEY_GEN | Purpose                                                                                                        |
| ------- | -------------------------------------------------------------------------------------------------------------- |
| true    | FHE keys are generated on the fly in `res/keys`. Old keys are overwritten. This requires at elast 15GB of RAM. |
| false   | FHE keys are copied from the `kms-service-dev` image in `res/keys`                                             |

### Fast run and test

Execute the following commands:

```bash
# Run fhEVM + full KMS components
make run-full
# Deploy ACL, Gateway ..., please wait until the end before testing!!!
make prepare-e2e-test
# This test could fail (first event catch is buggy - we are on it)
make run-async-test
# This one is working
make run-async-test
# A non trivial test
make run-true-input-async-test
# Manual test
cd work_dir/fhevm & npx hardhat test --grep 'test async decrypt uint32'
cd work_dir/fhevm & npx hardhat test --grep 'test async decrypt uint64'
cd work_dir/fhevm & npx hardhat test --grep 'test async decrypt several addresses'
```

> [!TIP]
> If one of the tests is blocked after a few seconds, check the logs of the gateway with `docker logs zama-dev-gateway-1 -f`. If you do not see any progress after a line like
> `🍊 Waiting for callback from KMS, txn_id: "85fa7..."`; **stop the test and retry**. This is a known issue and we will fix it soon!

<details><summary>Docker logs</summary>
<p>

```bash
# Check logs for Gateway
docker logs zama-dev-gateway-1 -f

# On the second try you should see
# 2024-07-04T09:29:06.649194Z  INFO gateway::events::manager: ⭐ event_decryption: 1
# 2024-07-04T09:29:06.649215Z  INFO gateway::events::manager: Handled event decryption: 1
# 2024-07-04T09:29:06.649255Z  INFO gateway::blockchain::handlers: 🧵 decrypt thread started
# 2024-07-04T09:29:06.654205Z  INFO gateway::blockchain::ciphertext_provider: Getting ciphertext for ct_handle: "aa9f8f90ebf0fa8e30caee92f0b97e158f1ec659b363101d07beac9b0cc90200"
# 2024-07-04T09:29:06.667907Z  INFO gateway::blockchain::kms_blockchain: 📦 Stored ciphertext, handle: 00008138b65173b5c57fc98d0fce54e5ff10635127e526144ffbe21d7099e3a1e1516574
# 2024-07-04T09:29:06.667927Z  INFO gateway::blockchain::kms_blockchain: 🍊 Decrypting ciphertext of size: 33080
# 2024-07-04T09:29:06.670033Z  INFO execute_contract: kms_blockchain_client::client: Body Raw bytes length: 609


# Check the logs for the node
docker logs zama-dev-fhevm-validator-1 -f
```

</p>
</details>

<details><summary>Pre deployment</summary>
<p>
You should see the pre-processing steps, i.e. deployment of ACL, Gateway, KMSVerifier ...

```bash
Generating typings for: 22 artifacts in dir: types for target: ethers-v6
Successfully generated 50 typings!
Compiled 22 Solidity files successfully (evm target: cancun).
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
gatewayContractAddress written to gateway/.env.gateway successfully!
gateway/lib/PredeployAddress.sol file has been generated successfully.
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
ACL address 0x2Fb4341027eb1d2aD8B5D9708187df8633cAFA92 written successfully!
./lib/ACLAddress.sol file generated successfully!
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
TFHE Executor address 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c written successfully!
./lib/FHEVMCoprocessorAddress.sol file generated successfully!
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
KMS Verifier address 0x12B064FB845C1cc05e9493856a1D637a73e944bE written successfully!
./lib/KMSVerifierAddress.sol file generated successfully!
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
ACL was deployed at address: 0x2Fb4341027eb1d2aD8B5D9708187df8633cAFA92
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
TFHEExecutor was deployed at address: 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
KMSVerifier was deployed at address: 0x12B064FB845C1cc05e9493856a1D637a73e944bE
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
privateKey 717fd99986df414889fd8b51069d4f90a50af72e542c58ee065f5883779099c6
ownerAddress 0x305F1F471e9baCFF2b3549F9601f9A4BEafc94e1
GatewayContract was deployed at address:  0xc8c9303Cd7F337fab769686B593B87DC3403E0ce
Account 0x97F272ccfef4026A1F3f0e0E879d514627B84E69 was succesfully added as an gateway relayer

```

</p>
</details>

<br />

### Trouble shooting

If you encounter

```
Error: The nonce of the deployer account is not null. Please use another deployer private key or relaunch a clean instance of the fhEVM
```

Then something went wrong in a step and you will need to run `make clean` and then start over the flow described [above](#fast-run-and-test).

### Init fhEVM-native

```bash
make init-ethermint-node
```

Initialize and generate/copy FHE keys based on `KEY_GEN` value in `.env`.

> [!NOTE]
> If `KEY_GEN` is set to `true`, ensure to have at least 15 GB of empty RAM to generate the keys. On Mac, do not forget to increase the allocated RAM to the docker process.

### Run fhEVM-native + KMS components

```bash
make run-full
# Check the logs for the node
docker logs zama-dev-fhevm-validator-1 -f
# Check logs for Gateway
docker logs zama-dev-gateway-1 -f
```

You should see the following docker images:

```
zama-dev-gateway-1	ghcr.io/zama-ai/kms-blockchain-gateway-dev:v0.8.1-rc4
zama-dev-connector-1	ghcr.io/zama-ai/kms-blockchain-connector-dev:v0.8.1-rc4
zama-dev-fhevm-validator-1	ghcr.io/zama-ai/ethermint-node:v0.5.1
zama-dev-kms-core-1	ghcr.io/zama-ai/kms-service-dev:v0.8.1-rc4
zama-dev-kms-validator-1	ghcr.io/zama-ai/kms-blockchain-asc-dev:v0.8.1-rc4
zama-dev-gateway-store-1	ghcr.io/zama-ai/kms-blockchain-gateway-dev:v0.8.1-rc4
```

### Stop fhEVM-native + KMS

```bash
make stop-full
```

### Fresh start

```bash
make clean
```

> [!NOTE]
> FHE keys are in res/keys folder, delete them to regenerate new keys at `make run-full` step.

### Test using fhevm

```bash
# if not executed before
make run-full
# In new terminal
make run-e2e-test
```

or in one command

```bash
make e2e-test
```
