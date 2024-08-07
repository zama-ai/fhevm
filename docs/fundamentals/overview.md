# Overview

At the highest level, the system consists of two subsystems: an fhEVM blockchain and a KMS.

An fhEVM blockchain consists of a set of validator nodes running a coprocessor, a configuration referred to as _fhEVM native_. A Key Management System (KMS) is a component that integrates a blockchain as the communication layer and a threshold protocol to handle decryption and reencryption.

These two systems are not directly connected; instead, a component called a gateway handles communication between them.

![Overview](../assets/overview.png)

## fhEVM

A blockchain using fhEVM processes all transactions, including those involving operations on encrypted data types. These transactions are symbolically executed, meaning that any operation on these encrypted types returns a new handle. A handle can be seen as a pointer to a ciphertext, similar to an identifier. After the transactions are executed, the coprocessor performs the operations on the ciphertexts, and the results are stored on-chain.

An fhEVM validator doesn't own the private key; instead, they have a _bootstrap key_ that allows them to perform computations on ciphertext. This key does not permit any node to decrypt anything.

## Gateway

The Gateway handles the decryption and the reencryption process.

- A decryption can be requested from a smart contract. In this case, the Gateway acts as an Oracle: the dApp calls the Gateway contract with the necessary materials for decryption. The contract will then emit an event that is monitored by the Gateway service.

- A user can directly request a reencryption through a HTTP call. In this case, the Gateway acts as a web2 service: the user provides a public key for the reencryption, a signature, and the handle of the ciphertext to be reencrypted.

The Gateway will then emit a transaction on the KMS blockchain, which serves as the communication layer, to request the decryption or reencryption. When the KMS responds with a transaction, the Gateway will transmit the decryption either through a callback function on-chain or the reencryption directly by responding synchronously to the HTTP call.

The gateway is designed not to be required to be trusted, thus a malicious gateway will not be able to compromise correctness or privacy of the system, but at most be able to block requests and responses between the fhEVM and the TKMS. However, this can be prevented by simply deploying multiple gateways.

## (T)KMS

The TKMS is a full key management solution for TFHE, more specifically [TFHE-rs](https://github.com/zama-ai/tfhe-rs), based on a maliciously secure and robust [MPC Protocol](https://eprint.iacr.org/2023/815).
It leverages a blockchain as its communication layer and utilizes a threshold protocol to manage decryption and reencryption requests securely. When a decryption or reencryption is requested, the TKMS processes the request using its cryptographic mechanisms, ensuring that no single entity has access to the full decryption key. Instead, the decryption or reencryption is carried out in a distributed manner, which enhances security by minimizing the risk of key exposure.
