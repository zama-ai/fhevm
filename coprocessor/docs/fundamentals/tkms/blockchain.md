# Blockchain
The KMS blochhain is implemented using the Cosmos framework. More specifically with [Comet BFT](https://cosmos.network/cometbft/).
This is a permissioned blockchain that is based on BFT consensus that allows for high throughput and low latency, but only supports a small number of validators (since consensus requires mutual interaction between all validator).

The blockchain handles all decryption, reencryption, and key management operations between _all_ fhEVM chains, co-processors etc. and the KMS engine.

## Smart contracts

- *ISC (Inclusion proof Smart Contract)*: Smart contract which handles validation of decryption/re-encryption requests for a specific fhEVM. Thus is contains custom logic for validation for a single fhEVM.

- *ASC (Application Smart Contract)*: A single smart contract to which transaction from the gateway (connector) are submitted to for all fhEVM's. All requests will pass through this contract and decryption and re-encryption requests will be validated by the appropriate ISC contract. 

## Payment
All operations must be paid for with tokens. Currently the tokenomics is not implemented and hence tokens can be constructed freely using a focet.

## Deployment
The KMS blockchain is deployed using `n` servers where `n` is the number of MPC parties. Each run their own validator docker image but is depoyed on the same machine as each of the MPC parties.