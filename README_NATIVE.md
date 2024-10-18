<p align="center">
<!-- product name logo -->
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/d7c9d88b-fc49-46f4-802b-65d1c944e2d9">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/b50f98a7-4190-492c-969b-7762f522dcf7">
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

## About

> [!Warning]
> This demo is an early beta version.

The purpose of this repository is to demonstrate the integration between fhEVM-native and a fully dockerized centralized KMS.

The KMS encompasses all sub-components, including the gateway, KMS blockchain, and centralized KMS backend. This is still an early version with support for (asynchronous) decryption, and reencryption.


### What is the Zama KMS for fhEVM
The Zama KMS is a full key management solution for TFHE, more specifically [TFHE-rs](https://github.com/zama-ai/tfhe-rs), based on a maliciously secure and robust [MPC protocol](https://eprint.iacr.org/2023/815).

The system facilitates this through a the use of a blockchain which provides a means of fulfilling payments to the MPC parties, along with providing an immutable audit log.

Interaction with the same KMS will happen either through an external Ethereum blockchain (fhEVM), providing an API via a smart contract, or through a gateway service.

### Design
Please consult the [design specification](design.md) for details on the design and the individual components.

### Implementation

The KMS is implemented as a gRPC service using the [tonic](https://github.com/hyperium/tonic) crate.
Communication between full nodes and the KMS service is defined by [protobuf](/proto/kms.proto) messages.
The rest of the communication is defined by existing standards and uses JSON-RPC.
For the light client, we currently use CometBFT's [light](https://pkg.go.dev/github.com/cometbft/cometbft/light) package, which provides a service that connects to any CometBFT full node to serve trusted state roots on-demand.
The light client package handles the logic of sequentially verifying block headers.

  <br></br>

  
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
- **[Resources](#resources)**
  - [Presentations](#presentations)
  - [Theory](#theory)
- **[Working with KMS](#working-with-kms)**
  - [Disclaimers](#disclaimers)
  - [Citations](#citations)
  - [License](#license)
- **[Support](#support)**
  <br></br>
  
## Getting started

### Prerequisite

Ensure that Docker (at least version 27) is installed and running.

_Optionally_ you may update `KEY_GEN` value in `.env`. Default is `false`

| KEY_GEN | Purpose |
| --- | --- |
| true    | FHE keys are generated on the fly in `res/keys`. Old keys are overwritten. This requires at elast 15GB of RAM. |
| false   | FHE keys are copied from the `kms-service-dev` image in `res/keys` |


### Intermediate step

The repo is being updated:

For now, one can run:

```bash
make run-coprocessor
make run-full
# Gateway is not started yet.
```

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
`🍊 Waiting for callback from KMS, txn_id: "85fa7..."`; **stop the test and retry**. This is a known issue and we will fix it soon! 


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
> FHE keys are in res/keys folder, delete them to regenerate new keys at ```make run-full``` step.


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
<br></br>

## Resources

### Presentations

- [EthCC 2024 TKMS presentation](EthCC24-tkms.pdf)

### Theory
- [Noah's Ark: Efficient Threshold-FHE Using Noise Flooding](https://eprint.iacr.org/2023/815)

  <br></br>
  
## Working with KMS

### Disclaimers

#### Audits
The Zama KMS is not yet audited and should be considered in an early alpha stage. Known bugs and security issues are present as reflected by issue tracking.

#### Parameters
The default parameters for the Zama KMS are chosen to ensure a failure probability of 2^-64 and symmetric equivalent security of 132 bits.

#### Side-channel attacks

Mitigations for side-channel attacks have not been implemented directly in the Zama KMS. The smart contract of the blockchain from which calls originate is responsible to ensure the validity of calls. In particular that new ciphertexts are correctly constructed (through a proof-of-knowledge).

### Citations
To cite the KMS in academic papers, please use the following entry:
```
@Misc{zama-kms,
  title={{Zama KMS: A Pure Rust Implementation of a Threshold Key Management System for TFHE}},
  author={Zama},
  year={2024},
  note={\url{https://github.com/zama-ai/kms-core}},
}
```

### License
This software is distributed under the **BSD-3-Clause-Clear** license. Read [this](LICENSE.txt) for more details.

#### FAQ
**Is Zama’s technology free to use?**
>Zama’s libraries are free to use under the BSD 3-Clause Clear license only for development, research, prototyping, and experimentation purposes. However, for any commercial use of Zama's open source code, companies must purchase Zama’s commercial patent license.
>
>Everything we do is open source and we are very transparent on what it means for our users, you can read more about how we monetize our open source products at Zama in [this blog post](https://www.zama.ai/post/open-source).

**What do I need to do if I want to use Zama’s technology for commercial purposes?**
>To commercially use Zama’s technology you need to be granted Zama’s patent license. Please contact us hello@zama.ai for more information.

**Do you file IP on your technology?**
>Yes, all Zama’s technologies are patented.

**Can you customize a solution for my specific use case?**
>We are open to collaborating and advancing the FHE space with our partners. If you have specific needs, please email us at hello@zama.ai.

<br></br>

## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/tfhe-rs/assets/157474013/08656d0a-3f44-4126-b8b6-8c601dff5380">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/tfhe-rs/assets/157474013/1c9c9308-50ac-4aab-a4b9-469bb8c536a4">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.
