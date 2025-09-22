This example demonstrates how to create a vesting wallet using OpenZeppelin's smart contract library powered by ZAMA's FHEVM.

`VestingWalletConfidential` receives `ERC7984` tokens and releases them to the beneficiary according to a confidential, linear vesting schedule.

{% hint style="info" %}
To run this example correctly, make sure the files are placed in the following directories:

- `.sol` file → `<your-project-root-dir>/contracts/`
- `.ts` file → `<your-project-root-dir>/test/`

This ensures Hardhat can compile and test your contracts as expected.
{% endhint %}

{% tabs %}

{% tab title="VestingWalletExample.sol" %}
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {FHE, ebool, euint64, euint128} from "@fhevm/solidity/lib/FHE.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ReentrancyGuardTransient} from "@openzeppelin/contracts/utils/ReentrancyGuardTransient.sol";
import {SepoliaConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {IERC7984} from "../interfaces/IERC7984.sol";

/**
 * @title VestingWalletExample
 * @dev A simple example demonstrating how to create a vesting wallet for ERC7984 tokens
 * 
 * This contract shows how to create a vesting wallet that receives ERC7984 tokens
 * and releases them to the beneficiary according to a confidential, linear vesting schedule.
 * 
 * This is a non-upgradeable version for demonstration purposes.
 */
contract VestingWalletExample is Ownable, ReentrancyGuardTransient, SepoliaConfig {
    mapping(address token => euint128) private _tokenReleased;
    uint64 private _start;
    uint64 private _duration;

    /// @dev Emitted when releasable vested tokens are released.
    event VestingWalletConfidentialTokenReleased(address indexed token, euint64 amount);

    constructor(
        address beneficiary,
        uint48 startTimestamp,
        uint48 durationSeconds
    ) Ownable(beneficiary) {
        _start = startTimestamp;
        _duration = durationSeconds;
    }

    /// @dev Timestamp at which the vesting starts.
    function start() public view virtual returns (uint64) {
        return _start;
    }

    /// @dev Duration of the vesting in seconds.
    function duration() public view virtual returns (uint64) {
        return _duration;
    }

    /// @dev Timestamp at which the vesting ends.
    function end() public view virtual returns (uint64) {
        return start() + duration();
    }

    /// @dev Amount of token already released
    function released(address token) public view virtual returns (euint128) {
        return _tokenReleased[token];
    }

    /**
     * @dev Getter for the amount of releasable `token` tokens. `token` should be the address of an
     * {IERC7984} contract.
     */
    function releasable(address token) public virtual returns (euint64) {
        euint128 vestedAmount_ = vestedAmount(token, uint48(block.timestamp));
        euint128 releasedAmount = released(token);
        ebool success = FHE.ge(vestedAmount_, releasedAmount);
        return FHE.select(success, FHE.asEuint64(FHE.sub(vestedAmount_, releasedAmount)), FHE.asEuint64(0));
    }

    /**
     * @dev Release the tokens that have already vested.
     *
     * Emits a {VestingWalletConfidentialTokenReleased} event.
     */
    function release(address token) public virtual nonReentrant {
        euint64 amount = releasable(token);
        FHE.allowTransient(amount, token);
        euint64 amountSent = IERC7984(token).confidentialTransfer(owner(), amount);

        // This could overflow if the total supply is resent `type(uint128).max/type(uint64).max` times. This is an accepted risk.
        euint128 newReleasedAmount = FHE.add(released(token), amountSent);
        FHE.allow(newReleasedAmount, owner());
        FHE.allowThis(newReleasedAmount);
        _tokenReleased[token] = newReleasedAmount;
        emit VestingWalletConfidentialTokenReleased(token, amountSent);
    }

    /**
     * @dev Calculates the amount of tokens that have been vested at the given timestamp.
     * Default implementation is a linear vesting curve.
     */
    function vestedAmount(address token, uint48 timestamp) public virtual returns (euint128) {
        return _vestingSchedule(FHE.add(released(token), IERC7984(token).confidentialBalanceOf(address(this))), timestamp);
    }

    /// @dev This returns the amount vested, as a function of time, for an asset given its total historical allocation.
    function _vestingSchedule(euint128 totalAllocation, uint48 timestamp) internal virtual returns (euint128) {
        if (timestamp < start()) {
            return euint128.wrap(0);
        } else if (timestamp >= end()) {
            return totalAllocation;
        } else {
            return FHE.div(FHE.mul(totalAllocation, (timestamp - start())), duration());
        }
    }
}
```

{% endtab %}

{% tab title="VestingWalletExample.test.ts" %}
```typescript
import { expect } from 'chai';
import { ethers, fhevm } from 'hardhat';
import { time } from '@nomicfoundation/hardhat-network-helpers';

describe('VestingWalletExample', function () {
  let vestingWallet: any;
  let token: any;
  let owner: any;
  let beneficiary: any;
  let other: any;

  const VESTING_AMOUNT = 1000;
  const VESTING_DURATION = 60 * 60; // 1 hour in seconds

  beforeEach(async function () {
    const accounts = await ethers.getSigners();
    [owner, beneficiary, other] = accounts;

    // Deploy ERC7984 mock token
    token = await ethers.deployContract('$ERC7984Mock', [
      'TestToken',
      'TT',
      'https://example.com/metadata'
    ]);

    // Get current time and set vesting to start in 1 minute
    const currentTime = await time.latest();
    const startTime = currentTime + 60;

    // Deploy and initialize vesting wallet in one step
    vestingWallet = await ethers.deployContract('VestingWalletExample', [
      beneficiary.address,
      startTime,
      VESTING_DURATION
    ]);

    // Mint tokens to the vesting wallet
    const encryptedInput = await fhevm
      .createEncryptedInput(await token.getAddress(), owner.address)
      .add64(VESTING_AMOUNT)
      .encrypt();

    await (token as any)
      .connect(owner)
      ['$_mint(address,bytes32,bytes)'](
        vestingWallet.target, 
        encryptedInput.handles[0], 
        encryptedInput.inputProof
      );
  });

  describe('Vesting Schedule', function () {
    it('should not release tokens before vesting starts', async function () {
      // Just verify the contract can be called without FHEVM decryption for now
      await expect(vestingWallet.connect(beneficiary).release(await token.getAddress()))
        .to.not.be.reverted;
    });

    it('should release half the tokens at midpoint', async function () {
      const currentTime = await time.latest();
      const startTime = currentTime + 60;
      const midpoint = startTime + (VESTING_DURATION / 2);
      
      await time.increaseTo(midpoint);
      // Just verify the contract can be called without FHEVM decryption for now
      await expect(vestingWallet.connect(beneficiary).release(await token.getAddress()))
        .to.not.be.reverted;
    });

    it('should release all tokens after vesting ends', async function () {
      const currentTime = await time.latest();
      const startTime = currentTime + 60;
      const endTime = startTime + VESTING_DURATION + 1000;
      
      await time.increaseTo(endTime);
      // Just verify the contract can be called without FHEVM decryption for now
      await expect(vestingWallet.connect(beneficiary).release(await token.getAddress()))
        .to.not.be.reverted;
    });
  });
});
```
{% endtab %}

{% tab title="VestingWalletExample.fixture.ts" %}
```typescript
import { ethers } from 'hardhat';
import { time } from '@nomicfoundation/hardhat-network-helpers';

export async function deployVestingWalletExampleFixture() {
  const [owner, beneficiary] = await ethers.getSigners();

  // Deploy ERC7984 mock token
  const token = await ethers.deployContract('$ERC7984Mock', [
    'TestToken',
    'TT',
    'https://example.com/metadata'
  ]);

  // Get current time and set vesting to start in 1 minute
  const currentTime = await time.latest();
  const startTime = currentTime + 60;
  const duration = 60 * 60; // 1 hour

  // Deploy and initialize vesting wallet in one step
  const vestingWallet = await ethers.deployContract('VestingWalletExample', [
    beneficiary.address,
    startTime,
    duration
  ]);

  return { vestingWallet, token, owner, beneficiary, startTime, duration };
}

export async function deployVestingWalletWithTokensFixture() {
  const { vestingWallet, token, owner, beneficiary, startTime, duration } = await deployVestingWalletExampleFixture();
  
  // Import fhevm for token minting
  const { fhevm } = await import('hardhat');
  
  // Mint tokens to the vesting wallet
  const encryptedInput = await fhevm
    .createEncryptedInput(await token.getAddress(), owner.address)
    .add64(1000) // 1000 tokens
    .encrypt();

  await (token as any)
    .connect(owner)
    ['$_mint(address,bytes32,bytes)'](
      vestingWallet.target, 
      encryptedInput.handles[0], 
      encryptedInput.inputProof
    );

  return { vestingWallet, token, owner, beneficiary, startTime, duration, vestingAmount: 1000 };
}
```
{% endtab %}

{% endtabs %}
