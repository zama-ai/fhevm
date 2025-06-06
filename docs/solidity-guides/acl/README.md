# Access Control List

This document describes the Access Control List (ACL) system in fhevm, a core feature that governs access to encrypted data. The ACL ensures that only authorized accounts or contracts can interact with specific ciphertexts, preserving confidentiality while enabling composable smart contracts. This overview provides a high-level understanding of what the ACL is, why it's essential, and how it works.

## What is the ACL?

The ACL is a permission management system designed to control who can access, compute on, or decrypt encrypted values in fhevm. By defining and enforcing these permissions, the ACL ensures that encrypted data remains secure while still being usable within authorized contexts.

## Why is the ACL important?

Encrypted data in fhevm is entirely confidential, meaning that without proper access control, even the contract holding the ciphertext cannot interact with it. The ACL enables:

- **Granular permissions**: Define specific access rules for individual accounts or contracts.
- **Secure computations**: Ensure that only authorized entities can manipulate or decrypt encrypted data.
- **Gas efficiency**: Optimize permissions using transient access for temporary needs, reducing storage and gas costs.

## How does the ACL work?

### Types of access

- **Permanent allowance**:
  - Configured using `FHE.allow(ciphertext, address)`.
  - Grants long-term access to the ciphertext for a specific address.
  - Stored in a dedicated contract for persistent storage.
- **Transient allowance**:
  - Configured using `FHE.allowTransient(ciphertext, address)`.
  - Grants access to the ciphertext only for the duration of the current transaction.
  - Stored in transient storage, reducing gas costs.
  - Ideal for temporary operations like passing ciphertexts to external functions.

**Syntactic sugar**:

- `FHE.allowThis(ciphertext)` is shorthand for `FHE.allow(ciphertext, address(this))`. It authorizes the current contract to reuse a ciphertext handle in future transactions.

### Transient vs. permanent allowance

| Allowance type | Purpose                                        | Storage type                                                            | Use case                                                                                            |
| -------------- | ---------------------------------------------- | ----------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| **Transient**  | Temporary access during a transaction.         | [Transient storage](https://eips.ethereum.org/EIPS/eip-1153) (EIP-1153) | Calling external functions or computations with ciphertexts. Use when wanting to save on gas costs. |
| **Permanent**  | Long-term access across multiple transactions. | Dedicated contract storage                                              | Persistent ciphertexts for contracts or users requiring ongoing access.                             |

## Granting and verifying access

### Granting access

Developers can use functions like `allow`, `allowThis`, and `allowTransient` to grant permissions:

- **`allow`**: Grants permanent access to an address.
- **`allowThis`**: Grants the current contract access to manipulate the ciphertext.
- **`allowTransient`**: Grants temporary access to an address for the current transaction.

### Verifying access

To check if an entity has permission to access a ciphertext, use functions like `isAllowed` or `isSenderAllowed`:

- **`isAllowed`**: Verifies if a specific address has permission.
- **`isSenderAllowed`**: Simplifies checks for the current transaction sender.

## Practical uses of the ACL

- **Confidential parameters**: Pass encrypted values securely between contracts, ensuring only authorized entities can access them.
- **Secure state management**: Store encrypted state variables while controlling who can modify or read them.
- **Privacy-preserving computations**: Enable computations on encrypted data with confidence that permissions are enforced.

---

For a detailed explanation of the ACL's functionality, including code examples and advanced configurations, see [ACL examples](acl_examples.md).
