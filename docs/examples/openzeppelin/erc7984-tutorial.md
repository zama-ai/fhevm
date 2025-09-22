# ERC7984 Tutorial

This tutorial explains how to create a confidential fungible token using Fully Homomorphic Encryption (FHE) and the OpenZeppelin smart contract library. By following this guide, you will learn how to build a token where balances and transactions remain encrypted while maintaining full functionality.

## Why FHE for Confidential Tokens?

Confidential tokens make sense in many real-world scenarios:

- **Privacy**: Users can transact without revealing their exact balances or transaction amounts
- **Regulatory Compliance**: Maintains privacy while allowing for selective disclosure when needed
- **Business Intelligence**: Companies can keep their token holdings private from competitors
- **Personal Privacy**: Individuals can participate in DeFi without exposing their financial position
- **Audit Trail**: All transactions are still recorded on-chain, just in encrypted form

FHE enables these benefits by allowing computations on encrypted data without decryption, ensuring privacy while maintaining the security and transparency of blockchain.
## Understanding the Architecture

Our confidential token will inherit from several key contracts:

1. **`ERC7984`** - OpenZeppelin's base for confidential tokens
2. **`Ownable2Step`** - Access control for minting and administrative functions
3. **`SepoliaConfig`** - FHE configuration for the Sepolia testnet

## The base smart contract

Create a new file `contracts/ERC7984Example.sol`:

Contract written like this assumes minimal privacy assumptions. Here you are minting a clear amount directly when instantiating.

This example assumes that the mint has been done only once, with a clear amount.

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {FHE, externalEuint64, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {SepoliaConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {ERC7984} from "@openzeppelin/confidential-contracts/token/ERC7984.sol";

contract ERC7984Example is SepoliaConfig, ERC7984, Ownable2Step {
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


## Test Workflow

Now let's create comprehensive tests to verify our confidential token works correctly. Create a new file `test/ERC7984Example.test.ts`:

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

  describe('Contract Deployment', function () {
    it('should deploy successfully', async function () {
      expect(await token.getAddress()).to.be.properAddress;
    });

    it('should set the correct name', async function () {
      expect(await token.name()).to.equal('Confidential Token');
    });

    it('should set the correct symbol', async function () {
      expect(await token.symbol()).to.equal('CTKN');
    });

    it('should set the correct token URI', async function () {
      expect(await token.tokenURI()).to.equal('https://example.com/token');
    });

    it('should mint initial amount to owner', async function () {
      // Verify that the owner has a balance (without decryption for now)
      const balanceHandle = await token.confidentialBalanceOf(owner.address);
      expect(balanceHandle).to.not.be.undefined;
    });
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

    it('should allow chained transfers', async function () {
      // First transfer from owner to recipient
      const encryptedInput1 = await fhevm
        .createEncryptedInput(await token.getAddress(), owner.address)
        .add64(TRANSFER_AMOUNT)
        .encrypt();

      await expect(token
        .connect(owner)
        ['confidentialTransfer(address,bytes32,bytes)'](
          recipient.address,
          encryptedInput1.handles[0],
          encryptedInput1.inputProof
        )).to.not.be.reverted;

      // Second transfer from recipient to other
      const encryptedInput2 = await fhevm
        .createEncryptedInput(await token.getAddress(), recipient.address)
        .add64(50) // Transfer half of what recipient received
        .encrypt();

      await expect(token
        .connect(recipient)
        ['confidentialTransfer(address,bytes32,bytes)'](
          other.address,
          encryptedInput2.handles[0],
          encryptedInput2.inputProof
        )).to.not.be.reverted;

      // Check that all addresses have balance handles (without decryption for now)
      const otherBalanceHandle = await token.confidentialBalanceOf(other.address);
      const recipientBalanceHandle = await token.confidentialBalanceOf(recipient.address);
      expect(otherBalanceHandle).to.not.be.undefined;
      expect(recipientBalanceHandle).to.not.be.undefined;
    });

    it('should maintain privacy - balances are encrypted', async function () {
      // Transfer some tokens
      const encryptedInput = await fhevm
        .createEncryptedInput(await token.getAddress(), owner.address)
        .add64(TRANSFER_AMOUNT)
        .encrypt();

      await expect(token
        .connect(owner)
        ['confidentialTransfer(address,bytes32,bytes)'](
          recipient.address,
          encryptedInput.handles[0],
          encryptedInput.inputProof
        )).to.not.be.reverted;

      // The balance handles should be encrypted (different values)
      const ownerBalanceHandle = await token.confidentialBalanceOf(owner.address);
      const recipientBalanceHandle = await token.confidentialBalanceOf(recipient.address);
      
      // These should be different encrypted values
      expect(ownerBalanceHandle).to.not.equal(recipientBalanceHandle);
      
      // Both handles should exist (without decryption for now)
      expect(ownerBalanceHandle).to.not.be.undefined;
      expect(recipientBalanceHandle).to.not.be.undefined;
    });

    it('should revert when trying to transfer more than balance', async function () {
      const excessiveAmount = INITIAL_AMOUNT + 100;
      const encryptedInput = await fhevm
        .createEncryptedInput(await token.getAddress(), recipient.address)
        .add64(excessiveAmount)
        .encrypt();

      await expect(
        token
          .connect(recipient)
          ['confidentialTransfer(address,bytes32,bytes)'](
            other.address,
            encryptedInput.handles[0],
            encryptedInput.inputProof
          )
      ).to.be.revertedWithCustomError(token, 'ERC7984ZeroBalance')
        .withArgs(recipient.address);
    });

    it('should revert when transferring to zero address', async function () {
      const encryptedInput = await fhevm
        .createEncryptedInput(await token.getAddress(), owner.address)
        .add64(TRANSFER_AMOUNT)
        .encrypt();

      await expect(
        token
          .connect(owner)
          ['confidentialTransfer(address,bytes32,bytes)'](
            ethers.ZeroAddress,
            encryptedInput.handles[0],
            encryptedInput.inputProof
          )
      ).to.be.revertedWithCustomError(token, 'ERC7984InvalidReceiver')
        .withArgs(ethers.ZeroAddress);
    });
  });
});
```

To run the tests, use:

```bash
npx hardhat test test/ERC7984Example.test.ts
```


## Advanced Features and Extensions

The basic ERC7984Example contract provides core functionality, but you can extend it with additional features:

### Minting Functions

**Visible Mint** - Allows the owner to mint tokens with a clear amount:
```solidity
    function mint(address to, uint64 amount) external onlyOwner {
        _mint(to, FHE.asEuint64(amount));
    }
```

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

### Burning Functions

**Visible Burn** - Allows the owner to burn tokens with a clear amount:
```solidity
    function burn(address from, uint64 amount) external onlyOwner {
        _burn(from, FHE.asEuint64(amount));
    }

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

### Total Supply Visibility

If you want the owner to be able to view the total supply (useful for administrative purposes):
```solidity
    function _update(address from, address to, euint64 amount) internal virtual override returns (euint64 transferred) {
        transferred = super._update(from, to, amount);
        FHE.allow(confidentialTotalSupply(), owner());
    }
```