# Branching in FHE

This document explains how to implement conditional logic (if/else branching) when working with encrypted values in fhEVM. Unlike typical Solidity programming, working with Fully Homomorphic Encryption (FHE) requires specialized methods to handle conditions on encrypted data.

## **Overview**

In fhEVM, when you perform [comparison operations](../references/functions.md#comparison-operation-eq-ne-ge-gt-le-lt), the result is an encrypted boolean (`ebool`). Since encrypted booleans do not support standard boolean operations like `if` statements or logical operators, conditional logic must be implemented using specialized methods.

To facilitate conditional assignments, fhEVM provides the `TFHE.select` function, which acts as a ternary operator for encrypted values.

## **Using `TFHE.select` for conditional logic**

The `TFHE.select` function enables branching logic by selecting one of two encrypted values based on an encrypted condition (`ebool`). It works as follows:

```solidity
TFHE.select(condition, valueIfTrue, valueIfFalse);
```

- **`condition`**: An encrypted boolean (`ebool`) resulting from a comparison.
- **`valueIfTrue`**: The encrypted value to return if the condition is true.
- **`valueIfFalse`**: The encrypted value to return if the condition is false.

## **Example: Auction Bidding Logic**

Here's an example of using conditional logic to update the highest winning number in a guessing game:

```solidity
function bid(einput encryptedValue, bytes calldata inputProof) external onlyBeforeEnd {
  // Convert the encrypted input to an encrypted 64-bit integer
  euint64 bid = TFHE.asEuint64(encryptedValue, inputProof);

  // Compare the current highest bid with the new bid
  ebool isAbove = TFHE.lt(highestBid, bid);

  // Update the highest bid if the new bid is greater
  highestBid = TFHE.select(isAbove, bid, highestBid);

  // Allow the contract to use the updated highest bid ciphertext
  TFHE.allowThis(highestBid);
}
```

{% hint style="info" %}
This is a simplified example to demonstrate the functionality. For a complete implementation with proper error handling and additional features, see the [Blind Auction contract example](https://github.com/zama-ai/fhevm/blob/29fe1f12236010737d86df156dc22eb6dedd0caa/examples/BlindAuction.sol#L92-L143).
{% endhint %}

### **How It Works**

- **Comparison**:
  - The `TFHE.lt` function compares `highestBid` and `bid`, returning an `ebool` (`isAbove`) that indicates whether the new bid is higher.
- **Selection**:
  - The `TFHE.select` function updates `highestBid` to either the new bid or the previous highest bid, based on the encrypted condition `isAbove`.
- **Permission Handling**:
  - After updating `highestBid`, the contract reauthorizes itself to manipulate the updated ciphertext using `TFHE.allowThis`.

## **Key Considerations**

- **Value change behavior:** Each time `TFHE.select` assigns a value, a new ciphertext is created, even if the underlying plaintext value remains unchanged. This behavior is inherent to FHE and ensures data confidentiality, but developers should account for it when designing their smart contracts.
- &#x20;**Gas consumption:** Using `TFHE.select` and other encrypted operations incurs additional gas costs compared to traditional Solidity logic. Optimize your code to minimize unnecessary operations.
- **Access control:** Always use appropriate ACL functions (e.g., `TFHE.allowThis`, `TFHE.allow`) to ensure the updated ciphertexts are authorized for use in future computations or transactions.

---

## **Summary**

- **`TFHE.select`** is a powerful tool for conditional logic on encrypted values.
- Encrypted booleans (`ebool`) and values maintain confidentiality, enabling privacy-preserving logic.
- Developers should account for gas costs and ciphertext behavior when designing conditional operations.

For more information on the supported operations, see the [fhEVM API documentation](../references/functions.md).
