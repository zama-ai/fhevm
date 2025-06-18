# Coprocessor

The coprocessor is the compute engine of FHEVM, designed to handle resource-intensive homomorphic operations.

### **Key functions**:

1. **Execution**: Performs encrypted operations (e.g., _add_, _mul_) on ciphertexts using the evaluation key.
2. **Ciphertext management**: Stores and retrieves ciphertexts securely in an off-chain database. Only handles are
   returned on-chain.

## **Computation**

Encrypted computations are performed using the **evaluation key** on the coprocessor.

- **How it works**:
  1. The smart contract emits FHE operation events as symbolic instructions.
  2. These events are picked up by the coprocessor, which evaluates each operation individually using the evaluation
     key, without ever decrypting the data.
  3. The resulting ciphertext is persisted in the coprocessor database, while only a handle is returned on-chain.
- **Data flow**:
  - **Source**: Blockchain smart contracts (via symbolic execution).
  - **Processing**: Coprocessor (using the evaluation key).
  - **Destination**: Blockchain (updated ciphertexts).

<figure><img src="../.gitbook/assets/computation.png" alt="computation"><figcaption></figcaption></figure>
