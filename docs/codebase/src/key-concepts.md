# Key Concepts

Before diving into the architecture, these concepts are essential to understanding FHEVM:

## Ciphertext Handles

A **ciphertext handle** is a 32-byte identifier (`bytes32`) that references encrypted data. Think of it like a pointer or database key:

- The **handle** is stored on-chain (small, cheap)
- The actual **ciphertext** (encrypted data) is stored off-chain in the coprocessor
- Smart contracts operate on handles; the coprocessor operates on ciphertexts

## Symbolic Execution

**Symbolic execution** means the on-chain contracts don't perform actual FHE computation. Instead, they:

1. **Generate deterministic handles**: Given inputs `handle_a` and `handle_b` for an `add` operation, compute `handle_result = hash(handle_a, handle_b, "add", counter)`
2. **Emit events**: Log the operation details for the off-chain coprocessor
3. **Return immediately**: The transaction completes without waiting for FHE computation

### Example: FHE.add(a, b)

When a contract calls `FHE.add(a, b)`:

```
On-chain (FHEVMExecutor):              Off-chain (Coprocessor):
1. Validate inputs
2. Generate handle_c = hash(...)
3. Emit event(ADD, a, b, c)      -->   4. Listen for event
4. Return handle_c                     5. Load ciphertexts a, b
                                       6. Compute c = TFHE.add(a, b)
                                       7. Store ciphertext c
                                       8. Commit to CiphertextCommits
```

The on-chain transaction completes in step 4. Steps 5-8 happen asynchronously seconds/minutes later.

## Asynchronous Computation Model

Because FHE operations are slow (seconds to minutes), FHEVM uses an **eventual consistency** model:

- **Writes are immediate**: `balances[user] = FHE.add(balances[user], amount)` completes immediately (new handle stored)
- **Results arrive later**: The actual encrypted value is computed and stored asynchronously
- **Reads use latest state**: Subsequent operations on the same handle will use the computed ciphertext once available

### Decryption Requires Callbacks

Contracts must request decryption explicitly and receive results via callback:

```solidity
// Request decryption (async)
Gateway.requestDecryption(handle, callbackSelector);

// Receive result later via callback
function onDecrypt(uint256 plaintext) external onlyGateway {
    // Use decrypted value
}
```

## Host Chain vs Gateway Chain

FHEVM supports **multiple host chains** (any EVM-compatible chain) coordinated by a single **gateway chain**:

- **Host Chain**: Where your dApp runs. Each host chain has its own FHEVMExecutor, ACL, etc.
- **Gateway Chain**: Central coordination point. Stores ciphertext commitments, manages cross-chain ACLs, coordinates with coprocessors and KMS.

In simple deployments, these may be the same chain. In multichain deployments, the gateway is a separate chain that coordinates FHE operations across all hosts.

## Key Terminology Quick Reference

| Term | Meaning |
|------|---------|
| **Handle** | On-chain identifier (bytes32) referencing encrypted data |
| **Ciphertext** | The actual encrypted data (stored off-chain) |
| **Symbolic Execution** | On-chain execution that generates handles without computing |
| **Coprocessor** | Off-chain service performing actual FHE computation |
| **Gateway** | Coordination layer between chains and off-chain services |
| **KMS** | Key Management System using threshold cryptography |
| **HCU** | Homomorphic Complexity Unit - measures FHE operation cost |

---

**Next:** Explore the [Architecture Overview](architecture.md) →
