# GatewayConfig contract

This section describes the `GatewayConfig` contract. It is used to configure settings for the fhevm Gateway protocol.

Several settings are stored in the contract, which can be separated in several categories:

- [Protocol metadata](#protocol-metadata)
- [KMS Nodes](#kms-nodes)
- [Governance accounts](#governance-accounts)
- [Host chains](#host-chains)

Except for host chains, they are all set when deploying the `GatewayConfig` contract. Once set, some (but not all) of them can be updated.

## Protocol metadata

The protocol metadata is used to display information about the protocol. It includes:

- `name` : name of the protocol.
- `website` : website of the protocol.

Currently, it is set at deployment but it is not possible to update them later on.

## Operators

The operators include the Key Management Service (KMS) nodes and coprocessors. Each operator needs to be registered in the `GatewayConfig` contract in order to be able to participate in the fhevm protocol.

They serve different purposes, which are described below.

### KMS Node

A KMS node is part of a set of multiple nodes. Often called the KMS, it refers to a Multi-Party Computation (MPC) protocol that manages Fully Homomorphic Encryption (FHE) keys in a decentralized manner. It is used to :

- decrypt ciphertexts based on requests from the `Decryption` contract.
- generate [KMS public materials](./kms_management.md#public-material-generation) based on requests from the `KmsManagement` contract.

Several metadata are stored for each KMS node:

- `txSenderAddress` : see [Sender and signer](#sender-and-signer) below. In the fhevm protocol, this account is also called the KMS connector.
- `signerAddress` : see [Sender and signer](#sender-and-signer) below.
- `ipAddress` : IP address of the KMS node.

The fhevm Gateway has a single KMS, which must be constituted of at least 1 node.

Currently, they are set at deployment and it is currently _not_ possible to add or remove a KMS node to the fhevm Gateway later on.

### KMS thresholds

#### MPC threshold

Additionally, a MPC threshold needs to be attached to a set of KMS nodes. This `mpcThreshold` is a cryptographic parameter used by the KMS nodes for executing the MPC protocol and thus is accessible through the `getMpcThreshold` view function.

The MPC threshold should be strictly less than the number of registered KMS nodes.

This threshold is set at deployment and can be updated by the owner later on, as long as the above conditions are met.

#### Decryption thresholds

The decryption thresholds are used to determine the minimum number of valid responses from KMS nodes required to validate a decryption, also called "consensus", in the `Decryption` contract.

There are currently two thresholds:

- `publicDecryptionThreshold` : the minimum number of valid responses from KMS nodes required to validate a public decryption :
- `userDecryptionThreshold` : the minimum number of valid responses from KMS nodes required to validate a user decryption

Both thresholds should be :

- non-null: decryption consensuses should require at least one vote
- less or equal to the number of registered KMS nodes: decryption consensuses should not require more than the number of registered KMS nodes

These thresholds are set at deployment and can be updated by the owner later on, as long as the above conditions are met.

### Coprocessor

A coprocessor is also part of a set of multiple coprocessors. They are used to :

- perform FHE computations on ciphertexts
- verify inputs' zero-knowledge proof of knowledge (ZKPoK) based on requests from the `InputVerification` contract
- handle access controls to ciphertexts for all registered [host chains](#host-chains), which are centralized in the `MultichainAcl` contract

Several metadata are stored for each coprocessor:

- `txSenderAddress` : see [Sender and signer](#sender-and-signer) below.
- `signerAddress` : see [Sender and signer](#sender-and-signer) below.
- `s3BucketUrl` : URL of the S3 bucket where the ciphertexts are stored. In the fhevm protocol, this URL is fetched by the KMS connector in order to download the ciphertexts needed for decryption requests.

The fhevm Gateway has a single set of coprocessors, which must be constituted of at least 1 coprocessor.

Currently, they are set at deployment and it is currently _not_ possible to add or remove a coprocessor to the fhevm Gateway later on.

### Sender and signer

A KMS node has both a transaction sender and a signer assigned to it:

- `txSenderAddress` : address of the account that will send transactions to the fhevm Gateway.
- `signerAddress` : address associated to the public key used to sign results sent to the fhevm Gateway.

The current list of transaction senders and signers can be retrieved using the following view functions:

- `getKmsTxSenders()`: get all the KMS nodes' transaction senders.
- `getKmsSigners()`: get all the KMS nodes' signers.

The transaction sender and signer addresses are allowed to be the same for a given KMS node.

Additionally, the transaction sender address is used for identifying an operator and may be referred to its "identity". In particular, these addresses can be used as inputs to following view functions in the `GatewayConfig` contract:

- `getKmsNode(address kmsTxSenderAddress)`: get a KMS node's metadata.

## Governance accounts

The fhevm Gateway protocol is governed by two accounts:

- `owner`: account that can perform restricted actions
- `pauser`: account that can pause contract functions

### Owner

The owner is first set as the account that deploys the contracts (the deployer). It is allowed to perform several restricted actions :

- upgrade the contracts
- update the [pauser](#pauser)
- add [host chains](#host-chains)
- trigger a KMS public material generation (see [KmsManagement](./kms_management.md#public-material-generation))
- update KMS-related parameters (see [KmsManagement](./kms_management.md#store-parameters))

The owner is handled by OpenZeppelin's `Ownable2StepUpgradeable` contract. In particular, this means that the deployer can transfer its ownership to another account in a two-step process.

### Pauser

**Important**: currently, the pauser is not used as the pausing mechanism is not implemented yet.

The pauser is an account that can pause contract functions. A paused function means that any transaction sent to trigger it will be reverted.

Nothing prevents the pauser from being the owner itself. However, in practice, it can be useful to use different accounts. For example, if they are both governed by multi-sig contracts, the pauser can be set to a lower threshold than the owner in order to pause the protocol quicker in case of emergency.

Currently, it is set at deployment and can be updated by the owner later on.

### Host chains

Host chains are host chains registered to the protocol. Only ciphertexts generated on registered host chains are allowed to be verified by coprocessors and decrypted by the KMS.

Several metadata are stored for each host chain:

- `chainId` : unique identifier of the host chain. Unlike typical chain IDs, its value is limited to 64 bits.
- `fhevmExecutorAddress` : address of the `FHEVMExecutor` fhevm contract deployed on the host chain.
- `aclAddress` : address of the `ACL` fhevm contract deployed on the host chain.
- `name` : name associated to the host chain.
- `website` : website associated to the host chain.

Host chains are not set at deployment and are instead added through the `addHostChain` function by the owner. It is however currently not possible to remove them later on.
