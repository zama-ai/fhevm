This tutorial explains how to build a confidential payment system for AI agents using FHE and the x402 HTTP payment standard. In this system, agents make encrypted payments for API access where nobody can see the payment amounts on-chain.

By following this guide, you will learn how to:

- Wrap ERC-20 tokens into FHE-encrypted ERC-7984 confidential tokens
- Build a server that requires encrypted payments for API access
- Build a client that automatically handles x402 payment flows with FHE
- Record payment nonces on-chain for verification
- Keep all payment amounts fully confidential

# Project Setup

Before starting this tutorial, ensure you have:

1. Installed the fhEVM hardhat template
2. Set up the OpenZeppelin confidential contracts library
3. A basic understanding of the x402 payment standard (HTTP 402 responses)

For help with these steps, refer to these tutorials:

- [Setting up OpenZeppelin confidential contracts](./openzeppelin/README.md)
- [Deploying a Confidential Token](./openzeppelin/erc7984-tutorial.md)

# Architecture Overview

The x402 FHE payment flow works as follows:

```
Agent (Client)                    API Server                     Blockchain
     |                               |                               |
     |--- GET /api/data ------------>|                               |
     |<-- 402 Payment Required ------|                               |
     |                               |                               |
     |   [Encrypt amount with FHE]   |                               |
     |                               |                               |
     |--- confidentialTransfer() ----|------------------------------>|
     |--- recordPayment() ----------|------------------------------>|
     |                               |                               |
     |--- GET /api/data ------------>|                               |
     |    + Payment header           |                               |
     |                               |--- verify events on-chain --->|
     |<-- 200 OK + data ------------|                               |
```

The key difference from standard x402: the transfer amount is FHE-encrypted. The server verifies that a `ConfidentialTransfer` event was emitted and a payment nonce was recorded, but it never sees the actual amount.

# Step 1: Deploy the Confidential Token

First, deploy a confidential ERC-7984 token that wraps a standard ERC-20 (like USDC). This token encrypts all balances and transfer amounts using FHE.

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import { FHE, euint64, externalEuint64 } from "@fhevm/solidity/lib/FHE.sol";
import { ZamaEthereumConfig } from "@fhevm/solidity/config/ZamaConfig.sol";
import { ERC7984 } from "@openzeppelin/confidential-contracts/token/ERC7984/ERC7984.sol";
import { ERC7984ERC20Wrapper } from "@openzeppelin/confidential-contracts/token/ERC7984/extensions/ERC7984ERC20Wrapper.sol";
import { Ownable2Step } from "@openzeppelin/contracts/access/Ownable2Step.sol";
import { Pausable } from "@openzeppelin/contracts/utils/Pausable.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/// @title ConfidentialUSDC
/// @notice Wraps USDC into FHE-encrypted cUSDC. Balances and transfer amounts
///         are always encrypted. Fee-free peer-to-peer transfers incentivize
///         agents to stay in the encrypted layer.
contract ConfidentialUSDC is
    ZamaEthereumConfig,
    ERC7984ERC20Wrapper,
    Ownable2Step,
    Pausable
{
    uint64 public constant FEE_BPS = 10;       // 0.1%
    uint64 public constant BPS = 10_000;
    uint64 public constant MIN_FEE = 10_000;   // 0.01 USDC (6 decimals)

    address public treasury;
    uint256 public accumulatedFees;

    constructor(
        IERC20 _underlying,
        address _treasury
    ) ERC7984ERC20Wrapper(_underlying) Ownable(msg.sender) {
        treasury = _treasury;
    }

    /// @notice Wrap USDC into encrypted cUSDC
    /// @param to Recipient of the encrypted tokens
    /// @param amount Amount of USDC to wrap (fee deducted before encryption)
    function wrap(address to, uint256 amount) public override whenNotPaused {
        uint64 fee = _calculateFee(uint64(amount));
        uint64 netAmount = uint64(amount) - fee;

        SafeERC20.safeTransferFrom(underlying(), msg.sender, address(this), amount);
        _mint(to, FHE.asEuint64(netAmount));

        accumulatedFees += uint256(fee);
    }

    function _calculateFee(uint64 amount) internal pure returns (uint64) {
        uint64 percentFee = (amount * FEE_BPS) / BPS;
        return percentFee > MIN_FEE ? percentFee : MIN_FEE;
    }
}
```

Key points:

- `wrap()` converts plain USDC into FHE-encrypted cUSDC
- A small fee (0.1%, minimum 0.01 USDC) is charged on wrap and unwrap
- Peer-to-peer `confidentialTransfer()` is inherited from ERC-7984 and is **fee-free**
- All balances are stored as `euint64` (encrypted 64-bit integers)

# Step 2: Deploy the Payment Verifier

The payment verifier is a simple nonce registry. When an agent makes a payment, it records a unique nonce on-chain. The server later checks this nonce to confirm payment.

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import { Ownable2Step } from "@openzeppelin/contracts/access/Ownable2Step.sol";
import { Pausable } from "@openzeppelin/contracts/utils/Pausable.sol";

/// @title X402PaymentVerifier
/// @notice Records payment nonces for x402 verification.
///         The server checks for both a ConfidentialTransfer event
///         and a PaymentVerified event to confirm payment.
contract X402PaymentVerifier is Ownable2Step, Pausable {
    constructor() Ownable(msg.sender) {}

    mapping(bytes32 => bool) public usedNonces;

    event PaymentVerified(
        address indexed payer,
        address indexed server,
        bytes32 indexed nonce,
        uint64 minPrice
    );

    error NonceAlreadyUsed();
    error ZeroMinPrice();

    /// @notice Record a payment nonce after making a confidential transfer
    /// @param server The API server address being paid
    /// @param nonce Unique payment identifier (prevents replay)
    /// @param minPrice Minimum price commitment (for server verification)
    function recordPayment(
        address server,
        bytes32 nonce,
        uint64 minPrice
    ) external whenNotPaused {
        if (minPrice == 0) revert ZeroMinPrice();
        if (usedNonces[nonce]) revert NonceAlreadyUsed();

        usedNonces[nonce] = true;

        emit PaymentVerified(msg.sender, server, nonce, minPrice);
    }
}
```

The two-transaction payment flow:

1. **TX1:** Agent calls `confidentialTransfer()` on the token contract (encrypted amount)
2. **TX2:** Agent calls `recordPayment()` on the verifier (nonce + price commitment)

The server verifies both events exist on-chain before granting access.

# Step 3: Build the Server (Express Middleware)

The server uses Express middleware that returns HTTP 402 when payment is required, then verifies the encrypted payment on-chain.

```typescript
import express from "express";
import { ethers } from "ethers";

const TOKEN_ABI = [
  "event ConfidentialTransfer(address indexed from, address indexed to, bytes32 indexed amount)",
];

const VERIFIER_ABI = [
  "event PaymentVerified(address indexed payer, address indexed server, bytes32 indexed nonce, uint64 minPrice)",
];

// Configuration
const config = {
  tokenAddress: "0xYourConfidentialUSDCAddress",
  verifierAddress: "0xYourVerifierAddress",
  recipientAddress: "0xYourServerWallet",
  price: "1000000", // 1 USDC in 6 decimals
  chainId: 11155111, // Sepolia
  rpcUrl: "https://ethereum-sepolia-rpc.publicnode.com",
};

const provider = new ethers.JsonRpcProvider(config.rpcUrl);
const tokenIface = new ethers.Interface(TOKEN_ABI);
const verifierIface = new ethers.Interface(VERIFIER_ABI);

// Track used nonces to prevent replay
const usedNonces = new Set<string>();

async function fhePaywall(
  req: express.Request,
  res: express.Response,
  next: express.NextFunction
) {
  const authHeader = req.headers["authorization"];

  // No payment header: return 402 challenge
  if (!authHeader || !authHeader.startsWith("Payment ")) {
    return res.status(402).json({
      scheme: "fhe-confidential-v1",
      network: `eip155:${config.chainId}`,
      token: config.tokenAddress,
      verifier: config.verifierAddress,
      recipient: config.recipientAddress,
      amount: config.price,
    });
  }

  // Parse and verify payment
  const credential = authHeader.slice(8); // Remove "Payment "
  const decoded = JSON.parse(Buffer.from(credential, "base64url").toString());

  // Check nonce replay
  if (usedNonces.has(decoded.nonce)) {
    return res.status(401).json({ error: "Nonce already used" });
  }

  // Verify on-chain events
  try {
    const valid = await verifyPayment(decoded);
    if (!valid) {
      return res.status(401).json({ error: "Payment verification failed" });
    }
    usedNonces.add(decoded.nonce);
    next();
  } catch {
    return res.status(500).json({ error: "Verification error" });
  }
}

async function verifyPayment(credential: {
  txHash: string;
  verifierTxHash: string;
  nonce: string;
  from: string;
}): Promise<boolean> {
  // Verify ConfidentialTransfer event
  const transferReceipt = await provider.getTransactionReceipt(credential.txHash);
  if (!transferReceipt) return false;

  const hasTransfer = transferReceipt.logs.some((log) => {
    try {
      const parsed = tokenIface.parseLog({ topics: [...log.topics], data: log.data });
      return (
        parsed?.name === "ConfidentialTransfer" &&
        parsed.args.to.toLowerCase() === config.recipientAddress.toLowerCase()
      );
    } catch {
      return false;
    }
  });

  if (!hasTransfer) return false;

  // Verify PaymentVerified event
  const verifierReceipt = await provider.getTransactionReceipt(credential.verifierTxHash);
  if (!verifierReceipt) return false;

  const hasVerified = verifierReceipt.logs.some((log) => {
    try {
      const parsed = verifierIface.parseLog({ topics: [...log.topics], data: log.data });
      return (
        parsed?.name === "PaymentVerified" &&
        parsed.args.nonce === credential.nonce
      );
    } catch {
      return false;
    }
  });

  return hasVerified;
}

// Usage
const app = express();

app.use("/api/premium", fhePaywall);

app.get("/api/premium", (req, res) => {
  res.json({ data: "This is premium content paid with encrypted cUSDC" });
});

app.listen(3000, () => console.log("Server running on port 3000"));
```

When an agent requests `/api/premium` without payment, the server returns:

```json
HTTP/1.1 402 Payment Required

{
  "scheme": "fhe-confidential-v1",
  "network": "eip155:11155111",
  "token": "0xYourConfidentialUSDCAddress",
  "verifier": "0xYourVerifierAddress",
  "recipient": "0xYourServerWallet",
  "amount": "1000000"
}
```

The agent then makes an encrypted payment and retries with the payment credential.

# Step 4: Build the Client (Agent SDK)

The client automatically handles the 402 flow: detect payment required, encrypt the amount with FHE, make the on-chain payment, and retry the request.

```typescript
import { ethers } from "ethers";
import { createInstance } from "@zama-fhe/relayer-sdk";

const TOKEN_ABI = [
  "function confidentialTransfer(address to, bytes32 encryptedAmount, bytes inputProof) returns (bytes32)",
];

const VERIFIER_ABI = [
  "function recordPayment(address server, bytes32 nonce, uint64 minPrice) external",
];

async function fheFetch(url: string, signer: ethers.Signer): Promise<Response> {
  // Step 1: Make initial request
  const response = await fetch(url);
  if (response.status !== 402) return response;

  // Step 2: Parse the 402 challenge
  const challenge = await response.json();
  const signerAddress = await signer.getAddress();

  // Step 3: Initialize FHE and encrypt the amount
  const fhevmInstance = await createInstance({
    chainId: challenge.network.split(":")[1],
    networkUrl: "https://ethereum-sepolia-rpc.publicnode.com",
  });

  const input = fhevmInstance.createEncryptedInput(challenge.token, signerAddress);
  input.add64(BigInt(challenge.amount));
  const { handles, inputProof } = input.encrypt();

  // Step 4: Make encrypted transfer (TX1)
  const token = new ethers.Contract(challenge.token, TOKEN_ABI, signer);
  const transferTx = await token.confidentialTransfer(
    challenge.recipient,
    handles[0],
    inputProof
  );
  await transferTx.wait();

  // Step 5: Record payment nonce (TX2)
  const nonce = ethers.hexlify(ethers.randomBytes(32));
  const verifier = new ethers.Contract(challenge.verifier, VERIFIER_ABI, signer);
  const verifierTx = await verifier.recordPayment(
    challenge.recipient,
    nonce,
    BigInt(challenge.amount)
  );
  await verifierTx.wait();

  // Step 6: Create payment credential
  const credential = Buffer.from(
    JSON.stringify({
      scheme: "fhe-confidential-v1",
      txHash: transferTx.hash,
      verifierTxHash: verifierTx.hash,
      nonce: nonce,
      from: signerAddress,
      chainId: Number(challenge.network.split(":")[1]),
    })
  ).toString("base64url");

  // Step 7: Retry with payment proof
  return fetch(url, {
    headers: { Authorization: `Payment ${credential}` },
  });
}

// Usage
const provider = new ethers.JsonRpcProvider("https://ethereum-sepolia-rpc.publicnode.com");
const signer = new ethers.Wallet("YOUR_PRIVATE_KEY", provider);

const response = await fheFetch("https://api.example.com/api/premium", signer);
const data = await response.json();
console.log(data); // { data: "This is premium content paid with encrypted cUSDC" }
```

# What Stays Private

| Data | On-chain visibility |
|------|-------------------|
| Transfer amount | **Encrypted** (FHE euint64) |
| Sender address | Public (blockchain requirement) |
| Recipient address | Public (blockchain requirement) |
| Payment nonce | Public (replay prevention) |
| API endpoint | Not on-chain |
| Response data | Not on-chain |

The critical difference: with standard x402, anyone can see "Agent A paid 1 USDC to Server B." With FHE x402, they can only see "Agent A transferred an encrypted amount to Server B." The actual payment value is never revealed.

# Silent Failure Pattern

A unique property of FHE transfers: if an agent does not have enough encrypted balance, the transfer does not revert. Instead, it silently transfers zero. This is a privacy feature, not a bug.

With standard ERC-20, a failed transfer reverts with "insufficient balance," which leaks information about the sender's balance. With FHE, the transaction always succeeds, but the amount transferred may be zero. The server detects this by checking if the `ConfidentialTransfer` event was emitted, but it cannot determine whether the transferred amount was the requested price or zero.

For production systems, additional verification through the payment verifier's `minPrice` commitment helps the server establish a minimum expected value without seeing the actual encrypted amount.

# Next Steps

- Add batch prepayment support (pay once for N API requests)
- Implement the MPP (Machine Payments Protocol) standard alongside x402
- Add agent identity (ERC-8004) for reputation-based pricing
- Add encrypted job escrow (ERC-8183) for multi-step agent workflows

For a production-ready implementation with all these features, see [MARC Protocol](https://github.com/marc-protocol/marc).
