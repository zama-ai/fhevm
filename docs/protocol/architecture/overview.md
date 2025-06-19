# FHE on Blockchain

The FHEVM protocol brings encrypted computation to smart contracts using Fully Homomorphic Encryption (FHE), while\
preserving composability, auditability, and EVM compatibility. It is made up of five key components:

* [FHEVM Solidity Library](library.md) – lets developers write confidential smart contracts using encrypted types and\
  operations, all in plain Solidity.
* [Host Contracts](hostchain.md) - trusted contracts deployed on EVM chains that manage access control and trigger\
  off-chain execution.
* [Coprocessors](coprocessor.md) – decentralized services that verify encrypted inputs, run FHE computations, and commit\
  results.
* [Gateway](gateway.md) – orchestrates the protocol: it validates inputs, manages ACLs, bridges ciphertexts across\
  chains, and coordinates coprocessors and the KMS.
* [Key Management Service (KMS)](kms.md) – a threshold MPC network that generates and rotates FHE keys, and handles\
  secure, verifiable decryption.
* [Relayer & oracle](relayer_oracle.md) – A lightweight off-chain service that helps users interact with the Gateway by\
  forwarding encryption or decryption requests.

-[test page](../../examples/fhe-counter.md)



[test 2](https://app.gitbook.com/s/UTmYJ1UQyasGNx2K8Aqd/smart-contract-examples/use-case-examples/fhe-counter)
