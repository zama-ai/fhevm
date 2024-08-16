# Architecture

![Threshold architecture](../../assets/threshold.png)
![Central architecture](../../assets/threshold.png)

The KMS system consists of a frontend, backend and temporary storage components.
One big usage-case of the KMS system is to facilitate key generation and decryption for one or more fhEVMs.

We now briefly outline each of these components along with their constituents:

- *fhEVM validator*: The validator node running the fhEVM blockchain.

- *Gateway*: Untrusted service that listens for decryption events on the fhEVM blockchain and propagates these as decryption requests to the KMS, and propagates decryption results back to the fhEVM blockchain. Used in a similar fashion to handle reencryption requests from a user.

- *Gateway KMS Connector*: A simple translation service that offers a gRPC interface for the gateway to communicate with the KMS blockchain. Calls from the gateway are submitted as transactions to the KMS blockchain, and result events from the KMS blockchain are returned to the gateway.

- *KV-store*: A simple storage service that temporarily holds the actual FHE ciphertexts on behalf of the KMS blockchain (which instead stores a hash digest of the ciphertext).

- *KMS Validator*: The validator node running the KMS blockchain.

- *KMS Connector*: A simple translation service that listens for request events from the KMS blockchain and turn these into gRPC calls to the KMS Core. Likewise, results from the KMS Core are submitted as transactions back to the KMS blockchain.

- *KMS Core*: Trusted gRPC service that implements the actual cryptographic operations such as decryption and reencryption. All results are signed.

- *KMS Engine*: The actual computational engine carrying out the FHE cryptographic key operations in a Nitro enclave

- *S3*: Public S3 instance storing the public keys for the FHE schemes for easy access.

## Frontend

The Frontend consists of the KMS blockchain. More specifically through an ASC contract. Each of which is unique for each application (e.g. each layer 1 blockchain).

The frontend makes up the public interface of the KMS, through which all requests are going. It consists of the KMS blockchain together with a collection of smart contracts. This gives it several desirable properties:

- Decentralized enforcement of policies.
- Trustable audit log of all actions performed by the KMS.
- Support for payments of the operators.
- Total ordering of requests (which is useful for some backends).

It consists of the following components:

- Smart contracts: ISC, ASC and Config SC.
  - Responsible for receiving, validating and processing requests and updates from the fhEVM. Including decryption, reencryption, validator updates, key generation and setup.
- KMS validators (realized through CometBFT).
  - The entities realizing the KMS blockchain. There may, or may not, be a 1-1 mapping between each validator and a threshold party in the KMS backend.

Multiple ISCs are deployed on the blockchain, typically one for each application (e.g. fhEVM blockchain) or application type (e.g. EVM blockchain). Each of these can keep application-specific state in order to verify requests from the application. For instance, an ISC for an fhEVM blockchain holds the identity of the current set of validators, so that access controls lists (ACLs) in decryption and reencryption requests can be validated by checking state inclusion proofs against the state roof of the fhEVM blockchain.

All decryption and reencryption requests are submitted as transactions to an ASC. The ASC performs universal validation and forwards ACL validation to the appropriate ISC. If all validations are ok then the ASC calls the backend by emitting an event that will trigger the backend to actually fulfill the request. Once the request has been fulfilled, the backend submits a fulfillment transaction back to the ASC.

All payments to the KMS is also handled through the ASC to which the transaction is submitted. These payments are used to incentivize the KMS operators.

Note that the KMS blockchain may be operated by a single validator if decentralization is not needed, or either in a permissioned or permissionless fashion for the decentralized setting.

### Backend

The backend consists of the KMS core.
It is the most security critical component of the entire system and a compromise of this could lead to breakage of both correctness, confidentiality and robustness.
Because of this we have designed it to support threshold security and Enclave support, along with isolation of the security critical _Engine_ from the general Internet.

The backend fulfills the requests as determined by the frontend. It comes in two flavors:

- [Centralized](centralized.md) where sensitive material is kept in its typical form.
- [Threshold](threshold.md) where sensitive material is secret shared

Each backend type is further described in their own document but each _logical_ party in the backend (which will be 1 for the centralized case and n for the threshold case) generally consists of the following components:

- Connector
  - A KMS blockchain client that supports _both_ reading from _and_ posting to the KMS blockchain. It is responsible for relaying information between the Coordinator and the KMS blockchain. Hence it connects the frontend and backend.
- Coordinator
  - A gRPC server which is responsible for load-balancing. It relays each call to an appropriate Core.
  - A gRPC server which is responsible for managing the requests to the KMS. It relays the FHE-related aspects of requests onto a Core.
- Core
  - The part of the system responsible for cryptographic tasks in relation to requests. This includes the FHE operations (which is handled by a sub-component called the `Engine`), along with request validation and signcryption.
  Observe that the `Engine` and `Core` are _not_ connected through a network, but that the `Engine` code is simply imported and called from `Core`.

More specifically the coordinator listens for events from the ASC (received through the Connector) and triggers the Core to fulfill operations. This means that the blockchain is the ground truth of which requests are processed, and each backend instance can independently authenticate these. The backend make use of a vault to keep and share sensitive material.

The design of the backend consisting of multiple components is done to make it possible to isolate the cryptographic _Engine_ from the public Internet and make it completely agnostic to the fhEVM and even the KMS blockchain.
It will simply only communicate with the Core Service and trust its requests blindly.
However, this does not pose a security risk as the Core Service and Connector _must_ be executed on the same machine and will only issue commands if signed and finalized by the KMS blockchain.

Each Core Service holds a signature key which is used to validate the authenticity of the operations which will eventually get passed back down to the fhEVM.
More specifically this key is used to sign fulfillment transactions and fingerprints of public material.

The Core Service and Engine is also AWS-friendly, in the sense that it can take advantage of AWS Nitro and AWS KMS to offer additional security. However, they can also be operated in a "developer mode" where the use of AWS components is bypassed, and the sensitive material is simply kept in clear-text on disc. This mode is useful for developers to run a KMS on for instance their laptops.

A S3-compatible storage system can also be used to store the key material for easy public access. When used with Nitro private material can also be stored in signcrypted form, allowing easy rolling of servers since they can then be stateless.

In case of a horizontal scaling multiple Cores may be launched and managed by the Coordinator. I.e. the Coordinator will be responsible for load-balancing the requests between the Cores it control.
This _may_ be realized based on the underlying event, where the hash value of the event payload is used to determine which Core should process it.
This means that the Coordinator only has relevance in the system when each party has multiple Cores. If there is only a single Core per party, then the Coordinator can be excluded from the system and requests from the Connector goes directly to the Core.

Each logical backend party also holds a signature key but may be shared between each Core in the case of horizontal scaling. This key is used to sign fulfillment transactions and fingerprints of public material.

Note that backends may choose to batch operations across request transactions in order to e.g. optimize the overall network load.

Note that two-way attestation should happen between the Coordinator and Core, along with the Coordinator and Connector to ensure e.g. that the Coordinator is not triggering other operations than those approved by the frontend.

Note that while the backend protects secret material, selective failure attacks may allow an adversary to extract secret keys by submitting malformed ciphertexts for decryption and reencryption. The KMS itself has no built in mechanism for protecting against this, so there is an implicit trust assumption that only well-formed ciphertexts are submitted to the KMS for decryption and reencryption. This in turn means that there is an implicit trust assumption that whoever produced the ciphertexts did so "honestly", which must be ensured externally (e.g. by the fhEVM).

Note also that the threshold assumption used by the threshold backend is not based on PoS but rather on a classic MPC threshold assumption that remains unjustified from an incentive point of view. Future work aims to address this.

#### Overall design choice
All calls on the Coordinator, which are not just simple data retrieval, will be identified with a unique `request_id`, even if a call is conceptually repeated. This `request_id` will be used to uniquely identify the result of call, e.g. preprocessed material, a decrypted ciphertext, etc.
More specifically from each call (posted on the blockchain in `ASC`) the `ASC` will derive a unique `request_id` from the call and map this to a 160 bit hex encoded string. This specific approach to ID generation is used in order to ensure the IDs are human readable and recognizable for blockchain (Ethereum) developers.

The result of key generation and CRS generation will be two chunks of information: One which is large and can be stored insecure in any public medium (this will be the CRS and the different public keys), the other will be a structure containing handles, IDs and signatures, which be stored internally on the coordinator _and_ on the KMS blockchain in `ASC`. This information will be used to validate the keys/CRS' which a client can retrieve through an insecure connection from any public domain.
More specifically the information will be a hash digest of the large element and a signature from the coordinator on this hash digest, along with the `request_id` associated with the large element.

We require that it is possible for clients to uniquely derive a URI for the large material based on the small material (and any auxiliary information stored on the blockchain).
The large data could for example be stored on IPFS, in which case the URI would be uniquely derived purely from the hash digest of the large element.
Alternatively the large data could be stored on S3, a file-system or a webserver s.t. `http://www.<some url>.com/keys/<requestID>/<key_type>.bin`.

### Storage

The storage component is used to make available public material that is not suitable for storing in the frontend fulfillment transactions. This includes public FHE keys and CRSs. Instead, only URIs and signed fingerprints of this material is stored in the fulfillment transactions. The fingerprint is computed using a cryptographic hash function.

The storage component can be entirely untrusted from a security perspective, and comes in two flavors with different availability properties:

- AWS S3 buckets.
- The local file system.

The storage component is expected to have high availability, although all material stored therein can easily be replicated without security risk.

## Security

Secret material is protected by the KMS either through the use of secure enclaves or through threshold secret sharing (see [Backend](#backend)).

While the KMS protects secret material, selective failure attacks may allow an adversary to extract secret keys by submitting malformed ciphertexts for decryption and reencryption. The KMS itself has no built in mechanism for protecting against this, so there is an implicit trust assumption that only well-formed ciphertexts are submitted to the KMS for decryption and reencryption. This in turn means that there is an implicit trust assumption that whoever produced the ciphertexts did so "honestly", which must be ensured externally (e.g. by the fhEVM).

Note also that the threshold assumption used by the threshold backend is not based on PoS but rather on a classic MPC threshold assumption that remains unjustified from an incentive point of view. Future work aims to address this.
