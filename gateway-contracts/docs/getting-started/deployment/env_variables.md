# Environment variables

This section describes the environment variables used for deployment. A complete example of an expected `.env` file is given in the [`.env.example`](../../../.env.example) file.

Environment variables can be separated in 3 categories:

- [`GatewayConfig` values](./env_variables.md#gatewayconfig-values)
- [Deployment settings](./env_variables.md#deployment-settings)

Except for deployment settings, the values are then stored in the deployed contracts and are not always allowed to be updated after. In the following, the values are given as examples. Most of them are from the `.env.example` file and are used for local testing. The expected types are also given as comments and should be respected, else the deployment is expected to fail.

Besides, the accounts found in the `.env.example` file are already-funded hardhat accounts generated with the following command:

```bash
make get-accounts
```

## Summary

Here's the complete list of environment variables used for deploying the FHEVM gateway. More detailed information can be found in [this section](#in-details) below. Solidity types are defined in [Solidity's documentation](https://docs.soliditylang.org/en/latest/types.html).

| Environment Variable                | Description                                | Solidity Type | Default                                                                                             | Comment                                                                          |
| ----------------------------------- | ------------------------------------------ | ------------- | --------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `PROTOCOL_NAME`                     | Name of the protocol to display            | string        | -                                                                                                   | -                                                                                |
| `PROTOCOL_WEBSITE`                  | Website of the protocol to display         | string        | -                                                                                                   | -                                                                                |
| `MPC_THRESHOLD`                     | MPC threshold (cryptographic parameter)    | uint256       | -                                                                                                   | Must be strictly less than the number of KMS nodes registered                    |
| `PUBLIC_DECRYPTION_THRESHOLD`       | Public decryption threshold                | uint256       | -                                                                                                   | Must be non-null and less than or equal to the number of KMS nodes registered    |
| `USER_DECRYPTION_THRESHOLD`         | User decryption threshold                  | uint256       | -                                                                                                   | Must be non-null and less than or equal to the number of KMS nodes registered    |
| `KMS_GENERATION_THRESHOLD`          | KMS public material generation threshold   | uint256       | -                                                                                                   | Must be non-null and less than or equal to the number of KMS nodes registered    |
| `COPROCESSOR_THRESHOLD`             | Coprocessor threshold                      | uint256       | -                                                                                                   | Must be non-null and less than or equal to the number of coprocessors registered |
| `NUM_KMS_NODES`                     | Number of KMS nodes to register            | -             | -                                                                                                   | Must be at least the number of KMS nodes registered below                        |
| `KMS_TX_SENDER_ADDRESS_{i}`         | Address of the KMS node `i`                | address       | -                                                                                                   | If `i` >= `NUM_KMS_NODES`, the variable is ignored                               |
| `KMS_SIGNER_ADDRESS_{i}`            | Signer address of the KMS node `i`         | address       | -                                                                                                   | If `i` >= `NUM_KMS_NODES`, the variable is ignored                               |
| `KMS_NODE_IP_ADDRESS_{i}`           | IP address of the KMS node `i`             | string        | -                                                                                                   | If `i` >= `NUM_KMS_NODES`, the variable is ignored                               |
| `NUM_COPROCESSORS`                  | Number of coprocessors to register         | -             | -                                                                                                   | Must be at least the number of coprocessors registered below                     |
| `COPROCESSOR_TX_SENDER_ADDRESS_{j}` | Address of the coprocessor `j`             | address       | -                                                                                                   | If `j` >= `NUM_COPROCESSORS`, the variable is ignored                            |
| `COPROCESSOR_SIGNER_ADDRESS_{j}`    | Signer address of the coprocessor `j`      | address       | -                                                                                                   | If `j` >= `NUM_COPROCESSORS`, the variable is ignored                            |
| `COPROCESSOR_S3_BUCKET_URL_{j}`     | S3 bucket URL of the coprocessor `j`       | string        | -                                                                                                   | If `j` >= `NUM_COPROCESSORS`, the variable is ignored                            |
| `NUM_HOST_CHAINS`                   | Number of host chains to register          | -             | -                                                                                                   | Must be at least the number of host chains registered below                      |
| `HOST_CHAIN_CHAIN_ID_{k}`           | Chain ID of the host chain `k`             | uint256       | -                                                                                                   | If `k` >= `NUM_HOST_CHAINS`, the variable is ignored                             |
| `HOST_CHAIN_FHEVM_EXECUTOR_{k}`     | FHEVM executor of the host chain `k`       | address       | -                                                                                                   | If `k` >= `NUM_HOST_CHAINS`, the variable is ignored                             |
| `HOST_CHAIN_ACL_ADDRESS_{k}`        | ACL address of the host chain `k`          | address       | -                                                                                                   | If `k` >= `NUM_HOST_CHAINS`, the variable is ignored                             |
| `HOST_CHAIN_NAME_{k}`               | Name of the host chain `k`                 | string        | -                                                                                                   | If `k` >= `NUM_HOST_CHAINS`, the variable is ignored                             |
| `HOST_CHAIN_WEBSITE_{k}`            | Website of the host chain `k`              | string        | -                                                                                                   | If `k` >= `NUM_HOST_CHAINS`, the variable is ignored                             |
| `NUM_PAUSERS`                       | Number of pausers to register              | -             | -                                                                                                   | Must be at least the number of pausers registered below                          |
| `PAUSER_ADDRESS_{l}`                | Address of the pauser `l`                  | address       | -                                                                                                   | If `l` >= `NUM_PAUSERS`, the variable is ignored                                 |
| `INPUT_VERIFICATION_PRICE`          | Price of an input verification             | address       | -                                                                                                   | The price is in $ZAMA base units (using 18 decimals)                             |
| `PUBLIC_DECRYPTION_PRICE`           | Price of a public decryption               | address       | -                                                                                                   | The price is in $ZAMA base units (using 18 decimals)                             |
| `USER_DECRYPTION_PRICE`             | Price of a user decryption                 | address       | -                                                                                                   | The price is in $ZAMA base units (using 18 decimals)                             |
| `ZAMA_OFT_ADDRESS`                  | Address of the ZamaOFT contract            | address       | -                                                                                                   | When using a real environment, the contract should already be deployed.          |
| `FEES_SENDER_TO_BURNER_ADDRESS`     | Address of the FeesSenderToBurner contract | address       | -                                                                                                   | When using a real environment, the contract should already be deployed.          |
| `DEPLOYER_PRIVATE_KEY`              | Private key for contract deployment        | bytes32       | -                                                                                                   | -                                                                                |
| `HARDHAT_NETWORK`                   | Network to deploy contracts on             | string        | "hardhat"                                                                                           | Possible values: `hardhat`, `localGateway`, `staging`, `zwsDev`, `testnet`       |
| `CHAIN_ID_GATEWAY`                  | Chain ID of the gateway network            | uint256       | 31337                                                                                               | It should be consistent with the `HARDHAT_NETWORK` value                         |
| `MNEMONIC`                          | "Mnemonic phrase for address generation    | string        | "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" | -                                                                                |
| `RPC_URL`                           | URL of the RPC node                        | string        | "http://127.0.0.1:8757"                                                                             | -                                                                                |
| `GATEWAY_CONFIG_ADDRESS`            | Address of the GatewayConfig contract      | address       | -                                                                                                   | Only for production settings                                                     |
| `KMS_GENERATION_ADDRESS`            | Address of the KmsGeneration contract      | address       | -                                                                                                   | Only for production settings                                                     |
| `PAUSER_SET_ADDRESS`                | Address of the PauserSet contract          | address       | -                                                                                                   | Only for production settings                                                     |
| `NEW_OWNER_PRIVATE_KEY`             | Private key for new owner acceptance       | bytes32       | -                                                                                                   | Only used in task that transfers ownership of the gateway to a new EAO owner     |

## In details

### GatewayConfig values

These values are crucial for the FHEVM Gateway protocol and are set in the `GatewayConfig` contract at deployment for most of them. To understand what each value is used for, please refer to the [GatewayConfig](../contracts/gateway_config.md) documentation.

#### At deployment

The following values are set at deployment.

- Protocol metadata:

```bash
PROTOCOL_NAME="Protocol" # (string)
PROTOCOL_WEBSITE="https://protocol.com" # (string)
```

- KMS Thresholds:

```bash
MPC_THRESHOLD="1" # (uint256)
```

`MPC_THRESHOLD` must be less or equal to the number of KMS nodes registered below.

```bash
PUBLIC_DECRYPTION_THRESHOLD="3" # (uint256)
USER_DECRYPTION_THRESHOLD="3" # (uint256)
```

`PUBLIC_DECRYPTION_THRESHOLD` and `USER_DECRYPTION_THRESHOLD` must be non-null and less or equal to the number of KMS nodes registered below.

In practice in the FHEVM protocol, these thresholds are set using the following formulas:

- public decryption threshold: `t + 1`
- user decryption threshold: `2*t + 1`

With `t` the MPC threshold.

These values might change in the future.

```bash
KMS_GENERATION_THRESHOLD="3" # (uint256)
```

`KMS_GENERATION_THRESHOLD` must be non-null and less or equal to the number of KMS nodes registered below.

- Coprocessor Thresholds:

```bash
COPROCESSOR_THRESHOLD="2" # (uint256)
```

`COPROCESSOR_THRESHOLD` must be non-null and less or equal to the number of coprocessors registered below.

- KMS Nodes:

```bash
NUM_KMS_NODES="1" # (number)
```

`NUM_KMS_NODES` is the number of KMS nodes to register in the `GatewayConfig` contract. It it not stored in it and is only used within the deployment script. The following metadata variables must be set for each KMS node, indexed by a node number starting from 0. If not enough variables are set, the deployment will fail. If, on the contrary, too many variables are set, the deployment will succeed but the extra ones will be ignored.

```bash
KMS_TX_SENDER_ADDRESS_0="0xc1d91b49A1B3D1324E93F86778C44a03f1063f1b" # (address)
KMS_SIGNER_ADDRESS_0="0x305F1F471e9baCFF2b3549F9601f9A4BEafc94e1" # (address)
KMS_NODE_IP_ADDRESS_0="127.0.0.1" # (string)
KMS_NODE_STORAGE_URL_0="s3://kms-bucket-1" # (string)
```

- Coprocessors:

```bash
NUM_COPROCESSORS="3" # (number)
```

`NUM_COPROCESSORS` is the number of coprocessors to register in the `GatewayConfig` contract. It it not stored in it and is only used within the deployment script. The following metadata variables must be set for each coprocessor, indexed by a coprocessor number starting from 0. If not enough variables are set, the deployment will fail. If, on the contrary, too many variables are set, the deployment will succeed but the extra ones will be ignored.

```bash
COPROCESSOR_TX_SENDER_ADDRESS_0="0x6518D50aDc9036Df37119eA465a8159E34417E2E" # (address)
COPROCESSOR_SIGNER_ADDRESS_0="0xa5eE8292dA52d8234248709F3E217ffEBA5E8312" # (address)
COPROCESSOR_S3_BUCKET_URL_0="s3://coprocessor-bucket-1" # (string)
```

#### After deployment

The following values are set after deployment in a separate script. However, they are still necessary for the FHEVM Gateway protocol to be fully functional.

- Host chains:

```bash
NUM_HOST_CHAINS="1" # (number)
```

`NUM_HOST_CHAINS` is the number of host chains to register in the `GatewayConfig` contract. It it not stored in it and is only used within the deployment script. The following metadata variables must be set for each host chain, indexed by a host chain number starting from 0. If not enough variables are set, the script will fail. If, on the contrary, too many variables are set, the script will succeed but the extra ones will be ignored.

```bash
HOST_CHAIN_CHAIN_ID_0="2025" # (uint256)
HOST_CHAIN_FHEVM_EXECUTOR_0="0xbb8ab3d75fd306ce85c90e899a2db850490cd697" # (address)
HOST_CHAIN_ACL_ADDRESS_0="0xabcdef1234567890abcdef1234567890abcdef12" # (address)
HOST_CHAIN_NAME_0="Host chain 2025" # (string)
HOST_CHAIN_WEBSITE_0="https://host-chain-2025.com" # (string)
```

`HOST_CHAIN_CHAIN_ID` must be different for all host chains, else the script will fail.

- Pausers:

```bash
NUM_PAUSERS="1" # (number)
```

`NUM_PAUSERS` is the number of pausers to register in the `GatewayConfig` contract. It it not stored in it and is only used within the deployment script. The following metadata variables must be set for each pauser, indexed by a pauser number starting from 0. If not enough variables are set, the script will fail. If, on the contrary, too many variables are set, the script will succeed but the extra ones will be ignored.

The number of pausers should correspond to the total number of registered operators (the number of KMS nodes + coprocessors registered in the protocol).

```bash
PAUSER_ADDRESS_0="0x6591319B97979Acc59b7191A8B4Ec381375bFc92" # (address)
```

### ProtocolPayment values

The initial price of each operation needs defined when deploying the `ProtocolPayment` contract.

```bash
INPUT_VERIFICATION_PRICE="10000000000000000000" # (uint256, 10 $ZAMA)
PUBLIC_DECRYPTION_PRICE="10000000000000000000"  # (uint256, 10 $ZAMA)
USER_DECRYPTION_PRICE="1000000000000000000" # (uint256, 1 $ZAMA)
```

The prices are in `$ZAMA`, using 18 decimals. They can be updated later by the owner.

### Deployment settings

The following settings are required for deploying the contracts through hardhat:

- Payment bridging contract addresses

```bash
ZAMA_OFT_ADDRESS="0xc1D733116990ce3D9e54F9eCf48a1cdD441Af4f9"
FEES_SENDER_TO_BURNER_ADDRESS="0xa50F5243C70c80a8309e3D39d8c9d958cDa83979"
```

In a real environment, before deploying the usual gateway contracts, several contracts should have already been deployed on :

- host chain:
  - `ZamaERC20`: the `$ZAMA` token
  - `FeesBurner`: used for burning the operation fees
- gateway chain:

  - `ZamaOFT`: the LayerZero OFT contract used to interact with the `ZamaERC20`
  - `FeesSenderToBurner`: contract with a LayerZero endpoint used for sending fees to `FeesBurner`

- Deployer private key

```bash
DEPLOYER_PRIVATE_KEY="0x7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f" # (bytes32)
```

This is the private key of the deployer account, used to deploy the contracts.

- Hardhat network

```bash
HARDHAT_NETWORK="hardhat" # (string)
```

This is the network on which the contracts are expected to be deployed. Possible values are: `hardhat`, `localGateway`, `staging`, `devnet`, `testnet`.

- Chain ID

```bash
CHAIN_ID_GATEWAY="31337" # (uint256)
```

This is the chain ID of the network on which the contracts are expected to be deployed. It should be consistent with the `HARDHAT_NETWORK` value as such:

- `hardhat`: "31337
- `localGateway`: 123456
- `staging`: 54321
- `devnet`: 10900
- `testnet`: 10901

- Mnemonic

```bash
MNEMONIC="adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" # (string)
```

This is the mnemonic used to generate the addresses and public keys of the deployer.

- RPC URL

```bash
RPC_URL="http://127.0.0.1:8757" # (string)
```

This is the URL of the RPC node for the FHEVM gateway network.

#### After deploying contracts in production

Some additional settings are needed after deploying the contracts in a production setting.

- GatewayConfig address

```bash
GATEWAY_CONFIG_ADDRESS="0xC7D45661a345eC5cA0e8521CFEF7e32FDA0Daa68" # (address)
```

This (static) address is needed for adding host chains to the GatewayConfig contract separately. In a proper production setting, this environment variable needs to be dynamically set after deploying the contracts.

- KMSGeneration address

```bash
KMS_GENERATION_ADDRESS="0x87A5b1152AA51728258dbc1AA54B6a83DCd1d3dd" # (address)
```

This (static) address is needed for generating the FHE key and CRS through the KMSGeneration contract. In a proper production setting, this environment variable needs to be dynamically set after deploying the contracts.

- PauserSet address

```bash
PAUSER_SET_ADDRESS="0xc1D733116990ce3D9e54F9eCf48a1cdD441Af4f9" # (address)
```

This (static) address is needed for managing pausers in the PauserSet contract separately. In a proper production setting, this environment variable needs to be dynamically set after deploying the contracts.

- New owner private key

```bash
NEW_OWNER_PRIVATE_KEY="0x7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f" # (bytes32)
```

This is the private key of the targeted new owner EAO, used to accept the ownership of the gateway contracts in case it is not directly transferred to a multisig account.
