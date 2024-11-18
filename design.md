# Overall architecture

At the highest level, the system consists of two subsystems: an *fhEVM blockchain* and a *KMS*. The KMS in turn consists of a *KMS blockchain* and a *KMS core*. These are in turn composed of multiple components, which we illustrate in the following picture, where we use conjoined boxes to mean that components are part of the same Docker image.

![Centralized KMS system](assets/central.png "Centralized KMS system")

We now briefly outline each of these components along with their constituents:

- *fhEVM validator*: The validator node running the fhEVM blockchain.

- *Gateway*: Untrusted service that listens for decryption events on the fhEVM blockchain and propagates these as decryption requests to the KMS, and propagates decryption results back to the fhEVM blockchain. Used in a similar fashion to handle reencryption requests from a user.

- *Gateway KMS Connector*: A simple translation service that offers a gRPC interface for the gateway to communicate with the KMS blockchain. Calls from the gateway are submitted as transactions to the KMS blockchain, and result events from the KMS blockchain are returned to the gateway.

- *KV-store*: A simple storage service that holds the actual FHE ciphertexts on behalf of the KMS blockchain (which instead stores a hash digest of the ciphertext).

- *KMS Validator*: The validator node running the KMS blockchain.

- *KMS Connector*: A simple translation service that listens for request events from the KMS blockchain and turn these into gRPC calls to the KMS Core. Likewise, results from the KMS Core are submitted as transactions back to the KMS blockchain.

- *KMS Core*: Trusted gRPC service that implements the actual cryptographic operations such as decryption and reencryption. All results are signed.

On the fhEVM blockchain the following smart contracts are deployed:

- *ACL smart contract*: Smart contract deployed on the fhEVM blockchain to manage access control of ciphertexts. dApp contracts use this to persists their own access rights and to delegate access to other contracts.

- *Gateway smart contract*: Smart contract deployed on the fhEVM blockchain that is used by a dApp smart contract to request a decrypt. This emits an event that triggers the gateway.

- *KMS smart contract*: Smart contract running on the fhEVM blockchain that is used by a dApp contract to verify decryption results from the KMS. To that end, it contains the identity of the KMS and is used to verify its signatures.

On the KMS blockchain the following smart contracts are deployed:

- *fhEVM ASC*: Smart contract to which transaction from the gateway (connector) are submitted to. This contract contains all customization logic required to work with the specific fhEVM blockchain.

Finally, dApp smart contracts use the *TFHE* Solidity library to perform operations on encrypted data on the fhEVM blockchain. This library is embedded into the dApp smart contract, and calls an executor smart contract under the hood.

## Notes

The gateway is designed _not_ to be required to be trusted, thus a malicious gateway will _not_ be able to compromise correctness or privacy of the system, but at most be able to block requests and responses between the fhEVM and the KMS. However, this can be prevented by simply deploying multiple gateways.

While the KMS protects secret material, selective failure attacks may allow an adversary to extract secret keys by submitting malformed ciphertexts for decryption and reencryption. The KMS itself has no built in mechanism for protecting against this, so there is an implicit trust assumption that only well-formed ciphertexts are submitted to the KMS for decryption and reencryption. This in turn means that there is an implicit trust assumption that whoever produced the ciphertexts did so "honestly", which must be ensured externally (e.g. by the fhEVM).
