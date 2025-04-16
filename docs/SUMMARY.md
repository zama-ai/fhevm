# Table of contents

- [Welcome to HTTPZ](README.md)
- [White paper](https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper-v2.pdf)

## Getting Started

- [Overview](getting-started/overview.md)
- [Quick Start](getting-started/overview-1/overview.md)
  - [Remix](getting-started/overview-1/remix/README.md)
    - [1. Setting up Remix](getting-started/overview-1/remix/remix.md)
    - [2. Connect your wallet to Remix](getting-started/overview-1/remix/connect_wallet.md)
    - [3. Deploying ConfidentialERC20](getting-started/overview-1/remix/deploying_cerc20.md)
    - [4. Interacting with the contract](getting-started/overview-1/remix/interact.md)
  - [Hardhat](getting-started/overview-1/hardhat/README.md)
    - [Prerequisites](getting-started/overview-1/hardhat/prerequisites.md)
    - [1. Setting up Hardhat](getting-started/overview-1/hardhat/1.-setting-up-hardhat.md)
    - [2. Writing contracts](getting-started/overview-1/hardhat/2.-writing-contracts.md)
    - [3. Testing in mocked mode](getting-started/overview-1/hardhat/3.-testing-in-mocked-mode.md)
    - [4. Deploying the contract](getting-started/overview-1/hardhat/4.-deploying-the-contract.md)
    - [5. Interacting with the contract](getting-started/overview-1/hardhat/5.-interacting-with-the-contract.md)

## Tutorials

- [See all tutorials](tutorials/see-all-tutorials.md)

## Smart contract

- [Key features](smart_contracts/key_concepts.md)
- [Configuration](smart_contracts/configure.md)
- [HTTPZ contracts](smart_contracts/contracts.md)
- [Supported types](smart_contracts/types.md)
- [Operations on encrypted types](smart_contracts/operations.md)
- [Access Control List](smart_contracts/acl/README.md)
  - [ACL examples](smart_contracts/acl/acl_examples.md)
- [Encrypted Inputs](smart_contracts/inputs.md)
- [Decryption](smart_contracts/decryption/README.md)
  - [Decryption](smart_contracts/decryption/decrypt.md)
  - [Decryption in depth](smart_contracts/decryption/decrypt_details.md)
  - [Re-encryption](smart_contracts/decryption/reencryption.md)
- [Dealing with branches and conditions](smart_contracts/loop.md)
- [Branching in FHE](smart_contracts/conditions.md)
- [AsEbool, asEuintXX, asEaddress and asEbytesXX operations](smart_contracts/asEXXoperators.md)
- [Generate random numbers](smart_contracts/random.md)
- [Error handling](smart_contracts/error_handling.md)
- [Gas estimation](smart_contracts/gas.md)
- [Debug decrypt](smart_contracts/debug_decrypt.md)
- [Using Foundry](smart_contracts/write_contract/foundry.md)
- [Mocked mode](smart_contracts/mocked.md)

## Frontend

- [Setup](frontend/setup.md)
- [Build a web application](frontend/webapp.md)
- [Using React.js](https://github.com/zama-ai/fhevm-react-template)
- [Using Next.js](https://github.com/zama-ai/fhevm-next-template)
- [Using Vue.js](https://github.com/zama-ai/fhevm-vue-template)
- [Using Node or Typescript](frontend/node.md)
- [Using the CLI](frontend/cli.md)
- [Common webpack errors](frontend/webpack.md)

## Explanations

- [Architectural overview](smart_contracts/architecture_overview.md)
- [FHE on blockchain](smart_contracts/architecture_overview/fhe-on-blockchain.md)
- [HTTPZ components](smart_contracts/architecture_overview/fhevm-components.md)
- [Encryption, decryption, re-encryption, and computation](smart_contracts/d_re_ecrypt_compute.md)

## References

- [Table of all addresses](references/table_of_addresses.md)
- [Smart contracts - HTTPZ API](references/functions.md)
- [Frontend - HTTPZ SDK](references/fhevmjs.md)
- [Repositories](references/repositories.md)

## Developer

- [Contributing](developer/contribute.md)
- [Development roadmap](developer/roadmap.md)
- [Release note](https://github.com/zama-ai/fhevm/releases)
- [Feature request](https://github.com/zama-ai/fhevm/issues/new?assignees=&labels=enhancement&projects=&template=feature-request.md&title=)
- [Bug report](https://github.com/zama-ai/fhevm/issues/new?assignees=&labels=bug&projects=&template=bug_report_fhevm.md&title=)
- [Status](https://status.zama.ai/)
