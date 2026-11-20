# Decryption Pipeline

## Overview

The decryption pipeline enables smart contracts to request decryption of encrypted values and receive plaintext results via callbacks. Decryption uses threshold cryptography (MPC) across multiple KMS nodes, ensuring no single party can decrypt alone.

## The Pipeline

When a contract requests decryption:

### Step 1: Request Decryption (On-Chain)

```solidity
// Contract requests decryption
Gateway.requestDecryption(
    ciphertextHandle,
    this.onDecrypt.selector
);
```

**What happens:**
- Contract calls `Decryption.sol` on Gateway
- Request logged with callback selector
- Transaction completes immediately

### Step 2: ACL Verification (Gateway)

**MultichainACL checks:**
- Does the requesting contract have permission to decrypt this handle?
- Is the request properly authorized?

**If denied:** Request is rejected, callback never called

**If approved:** Proceed to KMS

### Step 3: KMS Notification (Off-Chain)

**Gateway emits event:**
```
DecryptionRequested(requestId, handle, callbackSelector, contractAddress)
```

**KMS Connector (gw-listener) detects event:**
- Parses decryption request
- Creates job for kms-worker
- Forwards to external KMS Core

### Step 4: Threshold Decryption (KMS Core - External)

**KMS Core orchestrates MPC protocol:**
1. Broadcast decryption request to all KMS nodes (threshold: t-of-n)
2. Each node performs partial decryption using its key share
3. Combine t partial decryptions to recover plaintext
4. Generate EIP712 signature over result (threshold signature)

**Security properties:**
- No single KMS node sees the plaintext key
- Requires threshold (e.g., 3-of-5) nodes to cooperate
- Malicious minority cannot decrypt or corrupt result

### Step 5: Submit Signed Result (Off-Chain → On-Chain)

**KMS Connector (transaction-sender):**
- Receives signed plaintext from KMS Core
- Submits transaction to `KMSVerifier` contract
- Includes: plaintext value, EIP712 signature, requestId

### Step 6: Verify Signature (Host Chain)

**KMSVerifier contract:**
- Verifies EIP712 threshold signature
- Checks signature matches registered KMS nodes
- Validates requestId matches pending request

**If valid:** Approve callback
**If invalid:** Reject and log error

### Step 7: Callback Delivery (On-Chain)

**KMSVerifier calls back to original contract:**

```solidity
// Contract receives decrypted value
function onDecrypt(uint256 plaintext) external onlyGateway {
    // Use decrypted value
    require(plaintext > minimumBid, "Bid too low");
    winningBid = plaintext;
    winningBidder = msg.sender;
}
```

## Example: Blind Auction Reveal

```solidity
contract BlindAuction {
    mapping(address => euint32) private bids;
    uint256 public winningBid;
    address public winningBidder;

    // User submits encrypted bid
    function submitBid(euint32 encryptedBid) external {
        bids[msg.sender] = encryptedBid;
    }

    // Auction ends, reveal winning bid
    function revealWinner(address bidder) external {
        Gateway.requestDecryption(
            bids[bidder],
            this.onBidRevealed.selector
        );
    }

    // Callback receives plaintext
    function onBidRevealed(uint256 bidAmount) external onlyGateway {
        if (bidAmount > winningBid) {
            winningBid = bidAmount;
            winningBidder = msg.sender;
        }
    }
}
```

**Flow:**
1. T=0s: Contract calls `revealWinner(alice)`
2. T=0s: Request logged, transaction completes
3. T=2s: KMS Connector detects request
4. T=5s: KMS nodes perform threshold decryption
5. T=8s: Signed result submitted to KMSVerifier
6. T=9s: `onBidRevealed(1500)` called on contract
7. T=9s: Contract updates state with plaintext bid

## Security Considerations

### Threshold Security

**Trust model:**
- Requires threshold (t) of total (n) KMS nodes to cooperate
- Adversary controlling < t nodes cannot decrypt
- Example: 3-of-5 means adversary must compromise 3+ nodes

### ACL Enforcement

**Permission checks:**
- Only contracts with ACL permission can request decryption
- ACL permissions set by data owner
- Prevents unauthorized decryption of sensitive data

### Callback Safety

**Reentrancy protection:**
- Callbacks should use `nonReentrant` modifier
- Validate caller is KMSVerifier
- Handle partial failures gracefully

## Areas for Deeper Documentation

**[TODO: EIP712 signature format]** - Complete specification of the signed data structure for decryption results and verification process.

**[TODO: Threshold MPC protocol]** - Detailed explanation of the multi-party computation protocol used for threshold decryption.

**[TODO: ACL permission model]** - How permissions are granted, revoked, and checked across multiple chains.

**[TODO: Error handling patterns]** - Best practices for handling decryption failures, timeouts, and partial results in smart contracts.

**[TODO: Gas optimization]** - Strategies for minimizing gas costs in callback functions and batch decryption requests.

---

**Related:**
- [Gateway Contracts](../components/gateway-contracts.md) - Decryption.sol and MultichainACL
- [KMS Connector](../components/kms-connector.md) - Off-chain KMS coordination
- [Host Contracts](../components/host-contracts.md) - KMSVerifier implementation
