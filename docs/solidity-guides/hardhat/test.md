# Testing FHEVM Smart Contracts

## The FHEVM Hardhat Plugin

To write FHEVM smart contracts using Hardhat, you need to install the [FHEVM Hardhat Plugin](https://www.npmjs.com/package/@fhevm/hardhat-plugin) in your Hardhat project.

This plugin enables you to develop, test, and interact with FHEVM contracts right out of the box.

It extends Hardhat’s functionality with a complete FHEVM API that allows you:

- Encrypt data
- Decrypt data
- Run tests using various FHEVM execution modes

## Enabling the FHEVM Hardhat Plugin in your Hardhat project

Like any Hardhat plugin, the FHEVM Hardhat Plugin must be enabled by adding the following `import` statement to your `hardhat.config.ts` file:

```typescript
import "@fhevm/hardhat-plugin";
```

{% hint style="warning" %}
Without this import, the Hardhat FHEVM API will **not** be available in your Hardhat runtime environment (HRE).
{% endhint %}

## Accessing the Hardhat FHEVM API

The plugin extends the standard [Hardhat Runtime Environment](https://hardhat.org/hardhat-runner/docs/advanced/hardhat-runtime-environment) (or `hre` in short) with the new `fhevm` Hardhat module.

You can access it in either of the following ways:

```typescript
import { fhevm } from "hardhat";
```

or

```typescript
import * as hre from "hardhat";

// Then access: hre.fhevm
```

## Encrypting Values Using the Hardhat FHEVM API

Suppose the FHEVM smart contract you want to test has a function called `foo` that takes an encrypted `uint32` value as input. The Solidity function `foo` should be declared as follows:

```solidity
function foo(externalEunit32 value, bytes calldata memory inputProof);
```

Where:

- `externalEunit32 value` : is a `bytes32` representing the encrypted `uint32`
- `bytes calldata memory inputProof` : is a `bytes` array representing the zero-knowledge proof of knowledge that validates the encryption

To compute these arguments in TypeScript, you need:

- The **address of the target smart contract**
- The **signer’s address** (i.e., the account sending the transaction)

{% stepper %} 

{% step %}

#### Create a new encryted input

```ts
// use the `fhevm` API module from the Hardhat Runtime Environment
const input = fhevm.createEncryptedInput(contractAddress, signers.alice.address);
```

{% endstep %}

{% step %}

#### Add the value you want to encrypt.

```ts
input.add32(12345);
```

{% endstep %}

{% step %}

#### Perform local encryption.

```ts
const encryptedInputs = await input.encrypt();
```

{% endstep %}

{% step %}

#### Call the Solidity function

```ts
const externalUint32Value = encryptedInputs.handles[0];
const inputProof = encryptedInputs.proof;

const tx = await input.foo(externalUint32Value, inputProof);
await tx.wait();
```

{% endstep %}

{% endstepper %} 

### Encryption examples

- [Basic encryption examples](https://docs.zama.ai/protocol/examples/basic/encryption)
- [FHECounter](https://docs.zama.ai/protocol/examples#an-fhe-counter)

## Decrypting values using the Hardhat FHEVM API

Suppose user **Alice** wants to decrypt a `euint32` value that is stored in a smart contract exposing the following
Solidity `view` function:

```solidity
function getEncryptedUint32Value() public view returns (euint32) { returns _encryptedUint32Value; }
```

{% hint style="warning" %}
For simplicity, we assume that both Alice’s account and the target smart contract already have the necessary FHE permissions to decrypt this value. For a detailed explanation of how FHE permissions work, see the [`initializeUint32()`](https://docs.zama.ai/protocol/examples/basic/decryption/fhe-decrypt-single-value#tab-decryptsinglevalue.sol) function in [DecryptSingleValue.sol](https://docs.zama.ai/protocol/examples/basic/decryption/fhe-decrypt-single-value#tab-decryptsinglevalue.sol).
{% endhint %}

{% stepper %} 

{% step %}

#### Retrieve the encrypted value (a `bytes32` handle) from the smart contract:

```ts
const encryptedUint32Value = await contract.getEncryptedUint32Value();
```

{% endstep %}

{% step %}

#### Perform the decryption using the FHEVM API:

```ts
const clearUint32Value = await fhevm.userDecryptEuint(
  FhevmType.euint32, // Encrypted type (must match the Solidity type)
  encryptedUint32Value, // bytes32 handle Alice wants to decrypt
  contractAddress, // Target contract address
  signers.alice, // Alice’s wallet
);
```

{% hint style="warning" %}
If either the target smart contract or the user does **NOT** have FHE permissions, then the decryption call will fail!
{% endhint %}

{% endstep %}

{% endstepper %} 

### Supported Decryption Types

Use the appropriate function for each encrypted data type:

| Type       | Function                         |
| ---------- | -------------------------------- |
| `euintXXX` | `fhevm.userDecryptEuint(...)`    |
| `ebool`    | `fhevm.userDecryptEbool(...)`    |
| `eaddress` | `fhevm.userDecryptEaddress(...)` |

### Decryption examples

- [Basic decryption examples](https://docs.zama.ai/protocol/examples/basic/decryption)
- [FHECounter](https://docs.zama.ai/protocol/examples#an-fhe-counter)

## Run/Debug FHEVM smart contracts

The FHEVM Hardhat plugin provides three **FHEVM runtime modes** tailored for different stages of contract development and testing. Each mode offers a trade-off between speed, encryption, and persistence.

1. The **Hardhat (In-Memory)** default network: 🧪 _Uses mock encryption._ Ideal for regular tests, CI test coverage, and fast feedback during early contract development. No real encryption is used.

2. The **Hardhat Node (Local Server)** network: 🧪 _Uses mock encryption._ Ideal when you need persistent state - for example, when testing frontend interactions, simulating user flows, or validating deployments in a realistic local environment. Still uses mock encryption.

3. The **Sepolia Testnet** network: 🔐 _Uses real encryption._ Use this mode once your contract logic is stable and validated locally. This is the only mode that runs on the full FHEVM stack with **real encrypted values**. It simulates real-world production conditions but is slower and requires Sepolia ETH.

### Summary

| Mode                                                          | Encryption         | Persistent | Chain     | Speed          | Usage                                             |
| ------------------------------------------------------------- | ------------------ | ---------- | --------- | -------------- | ------------------------------------------------- |
| Hardhat (default)         | 🧪 Mock            | ❌ No      | In-Memory | ⚡⚡ Very Fast | Fast local testing and coverage                   |
| Hardhat Node               | 🧪 Mock            | ✅ Yes     | Server    | ⚡ Fast        | Frontend integration and local persistent testing |
| Sepolia Testnet | 🔐 Real Encryption | ✅ Yes     | Server    | 🐢 Slow        | Full-stack validation with real encrypted data    |

