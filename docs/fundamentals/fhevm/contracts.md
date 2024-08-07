# Contracts

The fhEVM runs totally on chain symbolically. Essentially, inputs to FHE operations are symbolic values (also called handles) that refer to ciphertexts. We check constraints on these handles, but ignore their actual values.

On the coprocessor, we actually execute the FHE operations on the ciphertexts the handles refer to. If a new ciphertext is generated in the coprocessor as a result of an FHE operation, it is inserted into the blockchain under a handle that is deterministically generated on both the blockchain and the coprocessor.

## Coprocessor contract

Symbolic execution on the blockchain is implemented via the [Coprocessor](https://github.com/zama-ai/fhevm/blob/main/lib/TFHEExecutor.sol) contract. One of the main responsibilites of the Coprocessor contract is to deterministically generate ciphertext handles. For this, we hash the FHE operation requested and the inputs to produce the result handle H:

```
H = keccak256(2, fheOperation, input1, input2, ..., inputN)
```

We use 2 as a domain separator for result handles.
Note that inputs can either be other handles or plaintext values. as described in FHE Execution and implemented in the newHandle() functions.

## ACL

The [ACL](https://github.com/zama-ai/fhevm/blob/main/lib/ACL.sol) Contract enforces access control for ciphertexts. The model we adopt is very simple - a ciphertext is either allowed for an address or not. An address can be any address - either an EOA address or a contract address. We implement that via the pairs member variable. Essentially, it is a set of keccak256(handle, address) values, where the handle refers to a ciphertext. If a (handle, address) pair is in the set, the ciphertext the handle refers to can be used by the address. In Solidity, we implement the set as a mapping such that all values already in the mapping are true. Access control applies to both passing ciphertexts from one contract to another, for decryption and for reencryption of a ciphertext to a user-provided key.

We use keccak256(handle, address) in order to both save space by persisting only one value instead of two and, also, to allow for a single storage slot proof for reencryption.

### Garbage Collection of Allowed Ciphertexts Data

The pairs field in the ACL contract grows indefinitely as new ciphertexts are produced. We might want to expose ways for developers to reclaim space by marking that certain ciphertexts are no longer needed and, consequently, zeroing the slot in pairs. A future effort will look into that.

## Gateway contract

The [Gateway](https://github.com/zama-ai/fhevm/blob/main/gateway/GatewayContract.sol) contract is an on-chain contract designed to interact with an off-chain oracle that handles decryption requests. When a dApp calls the `requestDecryption` function, the contract emits an event that is caught by the Gateway service.
Note: It is possible to have multiple Gateways, so multiple Gateway contracts can also be deployed.

## KMSVerifier contract

The [KMSVerifier](https://github.com/zama-ai/fhevm/blob/main/lib/KMSVerifier.sol) contract allows any dapp to verify a received decryption. This contract exposes a function `verifySignatures` which receives the decryption and signatures coming from the KMS.

Verifier addresses are stored and updated in the contract.
