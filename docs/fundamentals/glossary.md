# Glossary

## Smart Contracts

### fhEVM

- _ACL smart contract_: Smart contract deployed on the fhEVM blockchain to manage access control of ciphertexts. dApp contracts use this to persists their own access rights and to delegate access to other contracts.

- _Gateway smart contract_: Smart contract deployed on the fhEVM blockchain that is used by a dApp smart contract to request a decrypt. This emits an event that triggers the gateway.

- _KMS smart contract_: Smart contract running on the fhEVM blockchain that is used by a dApp contract to verify decryption results from the KMS. To that end, it contains the identity of the KMS and is used to verify its signatures.

### TKMS

- _fhEVM ASC_: Smart contract to which transaction from the gateway (connector) are submitted to. This contract contains all customization logic required to work with the specific fhEVM blockchain.
