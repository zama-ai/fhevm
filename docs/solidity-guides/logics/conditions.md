# Branching in FHE

This document explains how to implement conditional logic (if/else branching) when working with encrypted values in FHEVM. Unlike typical Solidity programming, working with Fully Homomorphic Encryption (FHE) requires specialized methods to handle conditions on encrypted data.

This document covers encrypted branching and how to move from an encrypted condition to a non-encrypted business logic in your smart contract.

## What is confidential branching?

In FHEVM, when you perform [comparison operations](../references/functions.md#comparison-operation-eq-ne-ge-gt-le-lt), the result is an encrypted boolean (`ebool`). Since encrypted booleans do not support standard boolean operations like `if` statements or logical operators, conditional logic must be implemented using specialized methods.

To facilitate conditional assignments, FHEVM provides the `FHE.select` function, which acts as a ternary operator for encrypted values.

## **Using `FHE.select` for conditional logic**

The `FHE.select` function enables branching logic by selecting one of two encrypted values based on an encrypted condition (`ebool`). It works as follows:

```solidity
FHE.select(condition, valueIfTrue, valueIfFalse);
```

- **`condition`**: An encrypted boolean (`ebool`) resulting from a comparison.
- **`valueIfTrue`**: The encrypted value to return if the condition is true.
- **`valueIfFalse`**: The encrypted value to return if the condition is false.

## **Example: Auction Bidding Logic**

Here's an example of using conditional logic to update the highest winning number in a guessing game:

```solidity
function bid(externalEuint64 encryptedValue, bytes calldata inputProof) external onlyBeforeEnd {
  // Convert the encrypted input to an encrypted 64-bit integer
  euint64 bid = FHE.asEuint64(encryptedValue, inputProof);

  // Compare the current highest bid with the new bid
  ebool isAbove = FHE.lt(highestBid, bid);

  // Update the highest bid if the new bid is greater
  highestBid = FHE.select(isAbove, bid, highestBid);

  // Allow the contract to use the updated highest bid ciphertext
  FHE.allowThis(highestBid);
}
```

{% hint style="info" %} 
This is a simplified example to demonstrate the functionality. 
{% endhint %}

### How Does It Work?

- **Comparison**:
  - The `FHE.lt` function compares `highestBid` and `bid`, returning an `ebool` (`isAbove`) that indicates whether the new bid is higher.
- **Selection**:
  - The `FHE.select` function updates `highestBid` to either the new bid or the previous highest bid, based on the encrypted condition `isAbove`.
- **Permission Handling**:
  - After updating `highestBid`, the contract reauthorizes itself to manipulate the updated ciphertext using `FHE.allowThis`.

## Key Considerations

- **Value change behavior:** Each time `FHE.select` assigns a value, a new ciphertext is created, even if the underlying plaintext value remains unchanged. This behavior is inherent to FHE and ensures data confidentiality, but developers should account for it when designing their smart contracts.
- &#x20;**Gas consumption:** Using `FHE.select` and other encrypted operations incurs additional gas costs compared to traditional Solidity logic. Optimize your code to minimize unnecessary operations.
- **Access control:** Always use appropriate ACL functions (e.g., `FHE.allowThis`, `FHE.allow`) to ensure the updated ciphertexts are authorized for use in future computations or transactions.

---

## How to branch to a non-confidential path?

So far, this section only covered how to do branching using encrypted variables. However, there may be many cases where the "public" contract logic will depend on the outcome from a encrypted path.

To do so, there are only one way to branch from an encrypted path to a non-encrypted path: it requires a public decryption using the oracle. Hence, any contract logic that requires moving from an encrypted input to a non-encrypted path always requires an async contract logic.

## **Example: Auction Bidding Logic: Item Release**

Going back to our previous example with the auction bidding logic. Let's assume that the winner of the auction can receive some prize, which is not confidential.

```solidity
bool public isPrizeDistributed;
eaddress internal highestBidder;
euint64 internal highestBid;

function bid(externalEuint64 encryptedValue, bytes calldata inputProof) external onlyBeforeEnd {
  // Convert the encrypted input to an encrypted 64-bit integer
  euint64 bid = FHE.asEuint64(encryptedValue, inputProof);

  // Compare the current highest bid with the new bid
  ebool isAbove = FHE.lt(highestBid, bid);

  // Update the highest bid if the new bid is greater
  highestBid = FHE.select(isAbove, bid, highestBid);

  // Update the highest bidder address if the new bid is greater
  highestBidder = FHE.select(isAbove, FHE.asEaddress(msg.sender), currentBidder));

  // Allow the contract to use the highest bidder address
  FHE.allowThis(highestBidder);

  // Allow the contract to use the updated highest bid ciphertext
  FHE.allowThis(highestBid);
}

function revealWinner() external onlyAfterEnd {
  uint256 requestId = FHE.requestDecryption(highestBidder, this.transferPrize.selector);
}

function transferPrize(uint256 requestId, address auctionWinner, bytes memory signatures) external {
  require(!isPrizeDistributed, "Prize has already been distributed");
  FHE.verifySignatures(requestId, signatures)

  isPrizeDistributed = true;
  // Business logic to transfer the prize to the auction winner
}
```

{% hint style="info" %} 
This is a simplified example to demonstrate the functionality. 
{% endhint %}

As you can see the in the above example, the path to move from an encrypted condition to a decrypted business logic must be async and requires calling the decryption oracle contract to reveal the result of the logic using encrypted variables.

## Summary

- **`FHE.select`** is a powerful tool for conditional logic on encrypted values.
- Encrypted booleans (`ebool`) and values maintain confidentiality, enabling privacy-preserving logic.
- Developers should account for gas costs and ciphertext behavior when designing conditional operations.

For more information on the supported operations, see the [FHEVM API documentation](../functions.md).
