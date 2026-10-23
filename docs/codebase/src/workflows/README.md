# Key Workflows

This section documents the critical operational flows in FHEVM, showing how components interact to process encrypted data from end to end.

## Three Core Workflows

### 1. [Symbolic Execution Pattern](symbolic-execution.md)
How FHE operations execute on-chain symbolically while computation happens asynchronously off-chain.

**When it happens:** Every time a smart contract performs an FHE operation like `FHE.add(a, b)`

**Key insight:** On-chain transactions complete immediately with handle generation; actual FHE computation happens seconds/minutes later.

### 2. [Decryption Pipeline](decryption-pipeline.md)
How encrypted data is decrypted through threshold cryptography and delivered back to smart contracts via callbacks.

**When it happens:** When a contract requests decryption via `Gateway.requestDecryption()`

**Key insight:** Decryption requires MPC across multiple KMS nodes; no single party can decrypt alone.

### 3. [Input Verification](input-verification.md)
How users submit encrypted inputs with zero-knowledge proofs to ensure data validity without revealing content.

**When it happens:** When users submit encrypted data to smart contracts (e.g., encrypted token amounts)

**Key insight:** ZK proofs verify that ciphertext is well-formed and meets constraints without revealing plaintext.

## Workflow Interactions

These three workflows compose to enable confidential smart contracts:

```
User prepares encrypted input
         ↓
   Input Verification (ZK proof validation)
         ↓
Contract receives validated ciphertext handle
         ↓
   Symbolic Execution (FHE operations)
         ↓
   [Async FHE computation off-chain]
         ↓
Contract requests decryption (optional)
         ↓
   Decryption Pipeline (threshold MPC)
         ↓
Contract receives plaintext via callback
```

## Understanding the Workflows

Each workflow document includes:
- **Step-by-step flow** with component interactions
- **Example scenarios** showing real-world usage
- **Sequence diagrams** visualizing the flow
- **Error handling** and edge cases
- **Performance characteristics** (timing, gas costs)

---

Choose a workflow to explore its detailed documentation.
