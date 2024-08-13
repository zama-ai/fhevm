# Contracts

The fhEVM employs symbolic execution - essentially, inputs to FHE operations are symbolic values (also called handles) that refer to ciphertexts. We check constraints on these handles, but ignore their actual values.

On the Executor, we actually execute the FHE operations on the ciphertexts the handles refer to. If a new ciphertext is generated in the Executor as a result of an FHE operation, it is inserted into the blockchain (into the ciphertext storage contract, see [Storage](storage.md)) under a handle that is deterministically generated in the TFHEExecutor contract.

## TFHEExecutor Contract

Symbolic execution on the blockchain is implemented via the [TFHEExecutor](https://github.com/zama-ai/fhevm/blob/main/lib/TFHEExecutor.sol) contract. One of the main responsibilites of the TFHEExecutor contract is to deterministically generate ciphertext handles. For this, we hash the FHE operation requested and the inputs to produce the result handle H:

```
H = keccak256(fheOperation, input1, input2, ..., inputN)
```

Inputs can either be other handles or plaintext values.

_Note:_ As of now, TFHEExecutor emloys precompiles and not symbolic execution. It will soon be migrated to symbolic execution.

## ACL Contract

The [ACL](https://github.com/zama-ai/fhevm/blob/main/lib/ACL.sol) contract enforces access control for ciphertexts. The model we adopt is very simple - a ciphertext is either allowed for an address or not. An address can be any address - either an EOA address or a contract address. Essentially, it is a mapping from handle to a set of addresses that are allowed to use the handle.

Access control applies to passing ciphertexts from one contract to another, for FHE computation on ciphertexts, for decryption and for reencryption of a ciphertext to a user-provided key.

### Garbage Collection of Allowed Ciphertexts Data

Data in the ACL contract grows indefinitely as new ciphertexts are produced. We might want to expose ways for developers to reclaim space by marking that certain ciphertexts are no longer needed and, consequently, zeroing the slot in the ACL. A future effort will look into that.

## Gateway Contract

The [Gateway](https://github.com/zama-ai/fhevm/blob/main/gateway/GatewayContract.sol) contract is an onchain contract designed to interact with an offchain Gateway component that handles decryption requests. When a dApp calls the `requestDecryption` function, the Gateway contract emits an event that is caught by the Gateway service.

_Note_: It is possible to have multiple Gateways, so multiple Gateway contracts can also be deployed.

## KMSVerifier Contract

The [KMSVerifier](https://github.com/zama-ai/fhevm/blob/main/lib/KMSVerifier.sol) contract allows any dApp to verify a received decryption. This contract exposes a function `verifySignatures` which receives the decryption and signatures coming from the TKMS.

Verifier addresses are stored and updated in the contract.
