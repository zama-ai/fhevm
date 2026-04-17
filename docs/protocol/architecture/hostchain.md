# Host contracts

This document explains one of the key components of the Zama Protocol - Host contracts.&#x20;

## What are host contracts?

Host contracts are smart contracts deployed on any supported blockchain (EVM or non-EVM) that act as trusted bridges between on-chain applications and the FHEVM protocol. They serve as the minimal and foundational interface that confidential smart contracts use to:

- Interact with encrypted data (handles)
- Perform access control operations
- Emit events for the off-chain components (coprocessors, Gateway)

These host contracts are used indirectly by developers via the FHEVM Solidity library, abstracting away complexity and integrating smoothly into existing workflows.

## Responsibilities of host contracts

### Trusted interface layer

Host contracts are the only on-chain components that:

- Maintain and enforce Access Control Lists (ACLs) for ciphertexts.
- Emit events that trigger coprocessor execution.
- Validate access permissions (persistent, transient, or decryption-related).

They are effectively the on-chain authority for:

- Who is allowed to access a ciphertext
- When and how they can use it
- These ACLs are mirrored on the Gateway for off-chain enforcement and bridging.

### Access Control API

Host contracts expose access control logic via standardized function calls (wrapped by the FHEVM library):

- `allow(handle, address)`: Grants persistent access.
- `allowTransient(handle, address)`: Grants temporary access for a single transaction.
- `allowForDecryption(handle)`: Marks a handle as publicly decryptable.
- `isAllowed(handle, address)`: Returns whether a given address has access.
- `isSenderAllowed(handle)`: Checks if msg.sender is allowed to use a handle.

They also emit:

- `Allowed(handle, address)`
- `AllowedForDecryption(handle)`

These events are crucial for triggering coprocessor state updates and ensuring proper ACL replication to the Gateway.

→ See the full guide of [ACL](https://docs.zama.ai/protocol/solidity-guides/smart-contract/acl).

### Security role

Although the FHE computation happens off-chain, host contracts play a critical role in protocol security by:

- Enforcing ACL-based gating
- Ensuring only authorized contracts and users can decrypt or use a handle
- Preventing misuse of encrypted data (e.g., computation without access)

Access attempts without proper authorization are rejected at the smart contract level, protecting both the integrity of confidential operations and user privacy.
