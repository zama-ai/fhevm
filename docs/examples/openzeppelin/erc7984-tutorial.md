This tutorial explains how to create a confidential fungible token using Fully Homomorphic Encryption (FHE) and the OpenZeppelin smart contract library. By following this guide, you will learn how to build a token where balances and transactions remain encrypted while maintaining full functionality.

## Why FHE for confidential tokens?

Confidential tokens make sense in many real-world scenarios:

- **Privacy**: Users can transact without revealing their exact balances or transaction amounts
- **Regulatory Compliance**: Maintains privacy while allowing for selective disclosure when needed
- **Business Intelligence**: Companies can keep their token holdings private from competitors
- **Personal Privacy**: Individuals can participate in DeFi without exposing their financial position
- **Audit Trail**: All transactions are still recorded on-chain, just in encrypted form

FHE enables these benefits by allowing computations on encrypted data without decryption, ensuring privacy while maintaining the security and transparency of blockchain.

# Project Setup

Before starting this tutorial, ensure you have:

1. Installed the FHEVM hardhat template
2. Set up the OpenZeppelin confidential contracts library 

For help with these steps, refer to the following tutorial:
- [Setting up OpenZeppelin confidential contracts](./openzeppelin/README.md)

## Understanding the architecture

Our confidential token will inherit from several key contracts:

1. **`ERC7984`** - OpenZeppelin's base for confidential tokens
2. **`Ownable2Step`** - Access control for minting and administrative functions
3. **`ZamaEthereumConfig`** - FHE configuration for the Ethereum mainnet or Ethereum Sepolia testnet networks

## The base smart contract

Let's create our confidential token contract in `contracts/ERC7984Example.sol`. This contract will demonstrate the core functionality of ERC7984 tokens.

A few key points about this implementation:

- The contract mints an initial supply with a clear (non-encrypted) amount during deployment
- The initial mint is done once during construction, establishing the token's total supply
- All subsequent transfers will be fully encrypted, preserving privacy
- The contract inherits from ERC7984 for confidential token functionality and Ownable2Step for secure access control

While this example uses a clear initial mint for simplicity, in production you may want to consider:
- Using encrypted minting for complete privacy from genesis
- Implementing a more sophisticated minting schedule
- Overriding some privacy assumptions

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {FHE, externalEuint64, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {ERC7984} from "@openzeppelin/confidential-contracts/token/ERC7984.sol";

contract ERC7984Example is ZamaEthereumConfig, ERC7984, Ownable2Step {
    constructor(
        address owner,
        uint64 amount,
        string memory name_,
        string memory symbol_,
        string memory tokenURI_
    ) ERC7984(name_, symbol_, tokenURI_) Ownable(owner) {
        euint64 encryptedAmount = FHE.asEuint64(amount);
        _mint(owner, encryptedAmount);
    }
}
```


## Test workflow

Now let's test the token transfer process. We'll create a test that:
1. Encrypts a transfer amount
2. Sends tokens from owner to recipient 
3. Verifies the transfer was successful by checking balance handles

Create a new file `test/ERC7984Example.test.ts` with the following test:

```ts
import { expect } from 'chai';
import { ethers, fhevm } from 'hardhat';

describe('ERC7984Example', function () {
  let token: any;
  let owner: any;
  let recipient: any;
  let other: any;

  const INITIAL_AMOUNT = 1000;
  const TRANSFER_AMOUNT = 100;

  beforeEach(async function () {
    [owner, recipient, other] = await ethers.getSigners();

    // Deploy ERC7984Example contract
    token = await ethers.deployContract('ERC7984Example', [
      owner.address,
      INITIAL_AMOUNT,
      'Confidential Token',
      'CTKN',
      'https://example.com/token'
    ]);
  });

  describe('Confidential Transfer Process', function () {
    it('should transfer tokens from owner to recipient', async function () {
      // Create encrypted input for transfer amount
      const encryptedInput = await fhevm
        .createEncryptedInput(await token.getAddress(), owner.address)
        .add64(TRANSFER_AMOUNT)
        .encrypt();

      // Perform the confidential transfer
      await expect(token
        .connect(owner)
        ['confidentialTransfer(address,bytes32,bytes)'](
          recipient.address,
          encryptedInput.handles[0],
          encryptedInput.inputProof
        )).to.not.be.reverted;

      // Check that both addresses have balance handles (without decryption for now)
      const recipientBalanceHandle = await token.confidentialBalanceOf(recipient.address);
      const ownerBalanceHandle = await token.confidentialBalanceOf(owner.address);
      expect(recipientBalanceHandle).to.not.be.undefined;
      expect(ownerBalanceHandle).to.not.be.undefined;
    });
  });
});
```

To run the tests, use:

```bash
npx hardhat test test/ERC7984Example.test.ts
```


## Advanced features and extensions

The basic ERC7984Example contract provides core functionality, but you can extend it with additional features. For example:

### Minting functions

**Visible Mint** - Allows the owner to mint tokens with a clear amount:
```solidity
    function mint(address to, uint64 amount) external onlyOwner {
        _mint(to, FHE.asEuint64(amount));
    }
```

- **When to use**: Prefer this for public/tokenomics-driven mints where transparency is desired (e.g., scheduled emissions).
- **Privacy caveat**: The minted amount is visible in calldata and events; use `confidentialMint` for privacy.
- **Access control**: Consider replacing `onlyOwner` with role-based access via `AccessControl` (e.g., `MINTER_ROLE`) for multi-signer workflows.
- **Supply caps**: If you need a hard cap, add a check before `_mint` and enforce it consistently for both visible and confidential flows.

**Confidential Mint** - Allows minting with encrypted amounts for enhanced privacy:
```solidity
    function confidentialMint(
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof
    ) external onlyOwner returns (euint64 transferred) {
        return _mint(to, FHE.fromExternal(encryptedAmount, inputProof));
    }
```

- **Inputs**: `encryptedAmount` and `inputProof` are produced off-chain with the SDK. Always validate and revert on malformed inputs.
- **Gas considerations**: Confidential operations cost more gas; batch mints sparingly and prefer fewer larger mints to reduce overhead.
- **Auditing**: While amounts stay private, you still get a verifiable audit trail of mints (timestamps, sender, recipient).
- **Example (Hardhat SDK)**:
```ts
const enc = await fhevm
  .createEncryptedInput(await token.getAddress(), owner.address)
  .add64(1_000)
  .encrypt();

await token.confidentialMint(recipient.address, enc.handles[0], enc.inputProof);
```

### Burning functions

**Visible Burn** - Allows the owner to burn tokens with a clear amount:
```solidity
    function burn(address from, uint64 amount) external onlyOwner {
        _burn(from, FHE.asEuint64(amount));
    }
```
**Confidential Burn** - Allows burning with encrypted amounts:
```solidity
    function confidentialBurn(
        address from,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof
    ) external onlyOwner returns (euint64 transferred) {
        return _burn(from, FHE.fromExternal(encryptedAmount, inputProof));
    }
```

- **Authorization**: Burning from arbitrary accounts is powerful; consider stronger controls (roles, multisig, timelocks) or user-consented burns.
- **Event strategy**: Decide whether to emit custom events revealing intent (not amounts) for better observability and offchain indexing.
- **Error surfaces**: Expect balance/allowance-like failures if encrypted amount exceeds balance; test both success and revert paths.
- **Example (Hardhat SDK)**:
```ts
const enc = await fhevm
  .createEncryptedInput(await token.getAddress(), owner.address)
  .add64(250)
  .encrypt();

await token.confidentialBurn(holder.address, enc.handles[0], enc.inputProof);
```

### Total supply visibility

If you want the owner to be able to view the total supply (useful for administrative purposes):
```solidity
    function _update(address from, address to, euint64 amount) internal virtual override returns (euint64 transferred) {
        transferred = super._update(from, to, amount);
        FHE.allow(confidentialTotalSupply(), owner());
    }
```

- **What this does**: Grants the `owner` permission to decrypt the latest total supply handle after every state-changing update.
- **Operational model**: The owner can call `confidentialTotalSupply()` and use their off-chain key material to decrypt the returned handle.
- **Security considerations**:
  - If ownership changes, ensure only the new owner can decrypt going forward. With `Ownable2Step`, this function will automatically allow the current `owner()`.
  - Be mindful of compliance: granting supply visibility may be considered privileged access; document who holds the key and why.
- **Alternatives**: If you want organization-wide access, grant via a dedicated admin contract that holds decryption authority instead of a single EOA.