# Host chain

FHEVM smart contracts are Solidity contracts that interact with encrypted values through symbolic execution.

### **Symbolic execution in Solidity**

- **Handles**: Smart contract operations return handles (references to ciphertexts), rather than directly manipulating
  encrypted data.
- **Lazy Execution**: Actual computation is done off-chain by the coprocessor after the contract emits symbolic
  instructions.

This allows efficient, gas-minimized interaction with encrypted data, while preserving EVM compatibility.

### **Zero-Knowledge proofs of knowledge (ZKPoKs)**

FHEVM incorporates ZKPoKs to verify the correctness of encrypted inputs and outputs:

- **Validation**: ZKPoKs ensure that inputs are correctly formed and correspond to known plaintexts without revealing
  sensitive data.
- **Integrity**: They prevent misuse of ciphertexts and ensure the correctness of computations.

By combining symbolic execution and ZKPoKs, FHEVM smart contracts maintain both privacy and verifiability.
