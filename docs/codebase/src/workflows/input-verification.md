# Input Verification

## Overview

Input verification enables users to submit encrypted data to smart contracts with zero-knowledge proofs that validate the data's properties without revealing its content. This ensures ciphertexts are well-formed and meet constraints before processing.

## The Workflow

When a user submits encrypted input to a smart contract:

### Step 1: Client-Side Preparation

**User's client application:**

1. **Encrypt plaintext data**
   ```javascript
   const plaintext = 1000; // e.g., token amount
   const { ciphertext, publicKey } = await fhevmClient.encrypt(plaintext);
   ```

2. **Generate zero-knowledge proof**
   ```javascript
   const proof = await fhevmClient.generateProof(ciphertext, plaintext);
   ```

   **Proof certifies:**
   - Ciphertext encrypts a valid value (not malformed)
   - Value meets constraints (e.g., within range, non-negative)
   - User knows the plaintext (prevents replay of intercepted ciphertexts)

3. **Submit to blockchain**
   ```javascript
   await contract.transfer(recipient, ciphertext, proof);
   ```

### Step 2: On-Chain Proof Verification

**Contract calls InputVerifier:**

```solidity
function transfer(address to, bytes calldata ciphertext, bytes calldata proof) external {
    // Verify proof and get handle
    euint32 amount = FHE.asEuint32(ciphertext, proof);

    // Now safe to use in operations
    balances[msg.sender] = FHE.sub(balances[msg.sender], amount);
    balances[to] = FHE.add(balances[to], amount);
}
```

**InputVerifier.sol checks:**
- ZK proof is valid (cryptographic verification)
- Ciphertext matches proof
- Public inputs are correct
- Proof hasn't been used before (replay protection)

**If invalid:** Transaction reverts
**If valid:** Ciphertext registered, handle returned

### Step 3: Ciphertext Registration

**On successful verification:**

1. **Generate handle**: `handle = hash(ciphertext, nonce)`
2. **Register in CiphertextCommits**: Store commitment to ciphertext
3. **Return handle to contract**: Contract receives `euint32` handle
4. **Contract proceeds**: Can now use handle in FHE operations

## Example: Confidential Token Transfer

```solidity
contract ConfidentialToken {
    mapping(address => euint32) private balances;

    function transfer(
        address to,
        bytes calldata encryptedAmount,
        bytes calldata proof
    ) external {
        // Verify input and get handle
        euint32 amount = FHE.asEuint32(encryptedAmount, proof);

        // Perform transfer using verified input
        euint32 senderBalance = balances[msg.sender];
        euint32 recipientBalance = balances[to];

        balances[msg.sender] = FHE.sub(senderBalance, amount);
        balances[to] = FHE.add(recipientBalance, amount);

        // Note: No need to check senderBalance >= amount on-chain
        // The FHE operations handle this confidentially
    }
}
```

**Security guarantee:**
- User cannot submit arbitrary/malformed ciphertexts
- Value is proven to be valid without revealing it
- Prevents attacks like "ciphertext of negative value"

## Input Types & Constraints

Different encrypted types support different constraints:

### Range Proofs

**euint32 with range constraint:**
```javascript
// Prove value is in range [0, 10000]
const proof = await fhevmClient.generateProof(
    ciphertext,
    plaintext,
    { min: 0, max: 10000 }
);
```

### Boolean Proofs

**ebool (true/false):**
```javascript
// Prove ciphertext encrypts exactly 0 or 1
const proof = await fhevmClient.generateBoolProof(ciphertext, plaintext);
```

### Multiple Inputs

**Batch verification:**
```javascript
// Submit multiple encrypted values with single proof
const proof = await fhevmClient.generateBatchProof([ct1, ct2, ct3], [pt1, pt2, pt3]);
```

## Verification Locations

FHEVM has two input verification points:

### Host Chain (InputVerifier.sol)

- **Single-chain deployments**: Primary verification point
- **Local verification**: Fast, low-latency
- **Per-chain state**: Each chain maintains its own verified inputs

### Gateway Chain (InputVerification.sol)

- **Multi-chain deployments**: Central verification point
- **Cross-chain verification**: Verified inputs usable on multiple hosts
- **Coordinated state**: Gateway tracks verified inputs across all hosts

> **Note**: Most deployments use Host's InputVerifier for simplicity. Gateway's InputVerification is for advanced multi-chain scenarios.

## Security Properties

### Soundness
- Invalid ciphertexts cannot pass verification
- User cannot cheat constraints (e.g., claim negative value is positive)

### Zero-Knowledge
- Proof reveals nothing about plaintext
- Observer learns only that constraints are satisfied

### Replay Protection
- Each proof can only be used once
- Prevents replaying valid proofs from other users

## Performance Characteristics

**Client-side (user's device):**
- Proof generation: ~500ms - 2s (depends on constraint complexity)
- Uses WASM/WebAssembly in browser

**On-chain verification:**
- Gas cost: ~100k - 300k gas per proof
- Verification time: <1s (within block time)

## Areas for Deeper Documentation

**[TODO: ZK proof system details]** - Explain the specific zero-knowledge proof scheme used (Groth16, PLONK, etc.), trusted setup requirements, and security parameters.

**[TODO: Constraint types and composition]** - Document all available constraint types, how to compose multiple constraints, and performance trade-offs.

**[TODO: Client SDK usage]** - Complete guide to fhevm-client SDK for generating proofs, encryption parameters, and error handling.

**[TODO: Proof batching optimization]** - Strategies for batching multiple input proofs to reduce gas costs.

**[TODO: Replay protection mechanism]** - Detailed implementation of nonce tracking and proof uniqueness enforcement.

---

**Related:**
- [Host Contracts](../components/host-contracts.md) - InputVerifier.sol implementation
- [Gateway Contracts](../components/gateway-contracts.md) - InputVerification.sol for multi-chain
- [Solidity Library](../components/library-solidity.md) - FHE.asEuint32() and type conversion
