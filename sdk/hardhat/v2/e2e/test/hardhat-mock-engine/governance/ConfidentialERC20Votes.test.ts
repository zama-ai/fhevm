/* eslint-disable no-unexpected-multiline */
import { FhevmType } from '@fhevm/hardhat-plugin';
import { expect } from 'chai';
import { parseUnits } from 'ethers';
import * as hre from 'hardhat';
import { ethers, network } from 'hardhat';

import type { TestConfidentialERC20Votes } from '../../../typechain-types';
import { userDecryptBalance } from '../confidentialERC20/ConfidentialERC20.fixture';
import { type Signers, getSigners, initSigners } from '../signers';
import { waitNBlocks } from '../utils';
import {
  deployConfidentialERC20Votes,
  userDecryptCurrentVotes,
  userDecryptPriorVotes,
} from './ConfidentialERC20Votes.fixture';
import { delegateBySig } from './DelegateBySig';

describe('ConfidentialERC20Votes', function () {
  // @dev The placeholder is type(uint256).max --> 2**256 - 1.
  const PLACEHOLDER = 2n ** 256n - 1n;
  const NULL_ADDRESS = '0x0000000000000000000000000000000000000000';

  let signers: Signers;
  let confidentialERC20Votes: TestConfidentialERC20Votes;
  let confidentialERC20VotesAddress: string;

  before(async function () {
    await initSigners();
    signers = await getSigners();
  });

  beforeEach(async function () {
    const contract = await deployConfidentialERC20Votes(signers.alice);
    confidentialERC20VotesAddress = await contract.getAddress();
    confidentialERC20Votes = contract;
  });

  it('should transfer tokens', async function () {
    const transferAmount = parseUnits(String(2_000_000), 6);

    const input = hre.fhevm.createEncryptedInput(confidentialERC20VotesAddress, signers.alice.address);

    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    const tx = await confidentialERC20Votes['transfer(address,bytes32,bytes)'](
      signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );
    await tx.wait();

    await expect(tx).to.emit(confidentialERC20Votes, 'Transfer').withArgs(signers.alice, signers.bob, PLACEHOLDER);

    // Decrypt Alice's balance
    expect(await userDecryptBalance(signers.alice, confidentialERC20Votes, confidentialERC20VotesAddress)).to.equal(
      parseUnits(String(8_000_000), 6),
    );

    // Decrypt Bob's balance
    expect(await userDecryptBalance(signers.bob, confidentialERC20Votes, confidentialERC20VotesAddress)).to.equal(
      parseUnits(String(2_000_000), 6),
    );
  });

  it('can delegate tokens on-chain', async function () {
    const tx = await confidentialERC20Votes.connect(signers.alice).delegate(signers.bob.address);
    await tx.wait();
    await expect(tx)
      .to.emit(confidentialERC20Votes, 'DelegateChanged')
      .withArgs(signers.alice, NULL_ADDRESS, signers.bob);

    const latestBlockNumber = await ethers.provider.getBlockNumber();
    await waitNBlocks(hre, 1);

    expect(
      await userDecryptPriorVotes(
        signers.bob,
        latestBlockNumber,
        confidentialERC20Votes,
        confidentialERC20VotesAddress,
      ),
    ).to.equal(parseUnits(String(10_000_000), 6));

    // Verify the two functions return the same.
    expect(
      await userDecryptPriorVotes(
        signers.bob,
        latestBlockNumber,
        confidentialERC20Votes,
        confidentialERC20VotesAddress,
      ),
    ).to.equal(await userDecryptCurrentVotes(signers.bob, confidentialERC20Votes, confidentialERC20VotesAddress));
  });

  it('can delegate votes via delegateBySig if signature is valid', async function () {
    const delegator = signers.alice;
    const delegatee = signers.bob;
    const nonce = 0;
    let latestBlockNumber = await ethers.provider.getBlockNumber();
    const block = await ethers.provider.getBlock(latestBlockNumber);
    if (block === null) {
      throw new Error(`Block ${latestBlockNumber} not found`);
    }
    const expiry = block.timestamp + 100;
    const signature = await delegateBySig(delegator, delegatee.address, confidentialERC20Votes, nonce, expiry);

    const tx = await confidentialERC20Votes
      .connect(signers.alice)
      .delegateBySig(delegator, delegatee, nonce, expiry, signature);
    await tx.wait();

    await expect(tx)
      .to.emit(confidentialERC20Votes, 'DelegateChanged')
      .withArgs(signers.alice, NULL_ADDRESS, signers.bob);

    latestBlockNumber = await ethers.provider.getBlockNumber();
    await waitNBlocks(hre, 1);

    expect(
      await userDecryptPriorVotes(
        signers.bob,
        latestBlockNumber,
        confidentialERC20Votes,
        confidentialERC20VotesAddress,
      ),
    ).to.equal(parseUnits(String(10_000_000), 6));

    // Verify the two functions return the same.
    expect(
      await userDecryptPriorVotes(
        signers.bob,
        latestBlockNumber,
        confidentialERC20Votes,
        confidentialERC20VotesAddress,
      ),
    ).to.equal(await userDecryptCurrentVotes(signers.bob, confidentialERC20Votes, confidentialERC20VotesAddress));
  });

  it('cannot delegate votes to self but it gets removed once the tokens are transferred', async function () {
    let tx = await confidentialERC20Votes.connect(signers.alice).delegate(signers.alice.address);
    await tx.wait();

    let latestBlockNumber = await ethers.provider.getBlockNumber();
    await waitNBlocks(hre, 1);

    expect(
      await userDecryptPriorVotes(
        signers.alice,
        latestBlockNumber,
        confidentialERC20Votes,
        confidentialERC20VotesAddress,
      ),
    ).to.equal(parseUnits(String(10_000_000), 6));

    const transferAmount = parseUnits(String(10_000_000), 6);
    const input = hre.fhevm.createEncryptedInput(confidentialERC20VotesAddress, signers.alice.address);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    tx = await confidentialERC20Votes
      .connect(signers.alice)
      ['transfer(address,bytes32,bytes)'](
        signers.bob.address,
        encryptedTransferAmount.handles[0],
        encryptedTransferAmount.inputProof,
      );

    await tx.wait();

    latestBlockNumber = await ethers.provider.getBlockNumber();
    await waitNBlocks(hre, 1);

    expect(
      await userDecryptPriorVotes(
        signers.alice,
        latestBlockNumber,
        confidentialERC20Votes,
        confidentialERC20VotesAddress,
      ),
    ).to.equal(0);
  });

  it('cannot delegate votes if nonce is invalid', async function () {
    const delegator = signers.alice;
    const delegatee = signers.bob;
    const nonce = 0;
    const latestBlockNumber = await ethers.provider.getBlockNumber();
    const block = await ethers.provider.getBlock(latestBlockNumber);
    if (block === null) {
      throw new Error(`Block ${latestBlockNumber} not found`);
    }
    const expiry = block.timestamp + 100;
    const signature = await delegateBySig(delegator, delegatee.address, confidentialERC20Votes, nonce, expiry);

    const tx = await confidentialERC20Votes
      .connect(signers.alice)
      .delegateBySig(delegator, delegatee, nonce, expiry, signature);
    await tx.wait();

    // Cannot reuse same nonce when delegating by sig
    await expect(
      confidentialERC20Votes.delegateBySig(delegator, delegatee, nonce, expiry, signature),
    ).to.be.revertedWithCustomError(confidentialERC20Votes, 'SignatureNonceInvalid');
  });

  it('cannot delegate votes if nonce is invalid due to the delegator incrementing her nonce', async function () {
    const delegator = signers.alice;
    const delegatee = signers.bob;
    const nonce = 0;
    const latestBlockNumber = await ethers.provider.getBlockNumber();
    const block = await ethers.provider.getBlock(latestBlockNumber);
    if (block === null) {
      throw new Error(`Block ${latestBlockNumber} not found`);
    }
    const expiry = block.timestamp + 100;
    const signature = await delegateBySig(delegator, delegatee.address, confidentialERC20Votes, nonce, expiry);

    const tx = await confidentialERC20Votes.connect(delegator).incrementNonce();
    await tx.wait();

    // @dev the newNonce is 1
    await expect(tx).to.emit(confidentialERC20Votes, 'NonceIncremented').withArgs(delegator, BigInt('1'));

    // Cannot reuse same nonce when delegating by sig
    await expect(
      confidentialERC20Votes.delegateBySig(delegator, delegatee, nonce, expiry, signature),
    ).to.be.revertedWithCustomError(confidentialERC20Votes, 'SignatureNonceInvalid');
  });

  it('cannot delegate votes if signer is invalid', async function () {
    const delegator = signers.alice;
    const delegatee = signers.bob;
    const nonce = 0;
    const latestBlockNumber = await ethers.provider.getBlockNumber();
    const block = await ethers.provider.getBlock(latestBlockNumber);
    if (block === null) {
      throw new Error(`Block ${latestBlockNumber} not found`);
    }
    const expiry = block.timestamp + 100;

    // Signer is not the delegator
    const signature = await delegateBySig(signers.carol, delegatee.address, confidentialERC20Votes, nonce, expiry);
    await expect(
      confidentialERC20Votes.delegateBySig(delegator, delegatee, nonce, expiry, signature),
    ).to.be.revertedWithCustomError(confidentialERC20Votes, 'SignatureVerificationFail');
  });

  it('cannot delegate votes if signature has expired', async function () {
    const delegator = signers.alice;
    const delegatee = signers.bob;
    const nonce = 0;
    const latestBlockNumber = await ethers.provider.getBlockNumber();
    const block = await ethers.provider.getBlock(latestBlockNumber);
    if (block === null) {
      throw new Error(`Block ${latestBlockNumber} not found`);
    }
    const expiry = block.timestamp + 100;
    const signature = await delegateBySig(delegator, delegatee.address, confidentialERC20Votes, nonce, expiry);

    await ethers.provider.send('evm_increaseTime', ['0xffff']);

    await expect(
      confidentialERC20Votes.connect(delegatee).delegateBySig(delegator, delegatee, nonce, expiry, signature),
    ).to.be.revertedWithCustomError(confidentialERC20Votes, 'SignatureExpired');
  });

  it('cannot request votes if blocktime is equal to current blocktime', async function () {
    let blockNumber = await ethers.provider.getBlockNumber();

    await expect(confidentialERC20Votes.getPriorVotes(signers.alice, blockNumber + 1)).to.be.revertedWithCustomError(
      confidentialERC20Votes,
      'BlockNumberEqualOrHigherThanCurrentBlock',
    );

    const tx = await confidentialERC20Votes.connect(signers.alice).setGovernor(signers.bob);
    await tx.wait();

    await expect(tx).to.emit(confidentialERC20Votes, 'NewGovernor').withArgs(signers.bob);

    blockNumber = await ethers.provider.getBlockNumber();

    await expect(
      confidentialERC20Votes.connect(signers.bob).getPriorVotesForGovernor(signers.alice, blockNumber + 1),
    ).to.be.revertedWithCustomError(confidentialERC20Votes, 'BlockNumberEqualOrHigherThanCurrentBlock');
  });

  it('users can request past votes getPriorVotes', async function () {
    // Alice transfers 1M tokens to Bob, 1M tokens to Carol, 1M tokens to Dave
    const transferAmount = parseUnits(String(1_000_000), 6);

    const input = hre.fhevm.createEncryptedInput(confidentialERC20VotesAddress, signers.alice.address);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    let tx = await confidentialERC20Votes['transfer(address,bytes32,bytes)'](
      signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );

    await tx.wait();

    tx = await confidentialERC20Votes['transfer(address,bytes32,bytes)'](
      signers.carol.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );

    await tx.wait();

    tx = await confidentialERC20Votes['transfer(address,bytes32,bytes)'](
      signers.dave.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );

    await tx.wait();

    tx = await confidentialERC20Votes.connect(signers.bob).delegate(signers.dave.address);
    await tx.wait();

    const firstCheckPointBlockNumber = await ethers.provider.getBlockNumber();
    await waitNBlocks(hre, 1);

    tx = await confidentialERC20Votes.connect(signers.carol).delegate(signers.dave.address);
    await tx.wait();

    const secondCheckPointBlockNumber = await ethers.provider.getBlockNumber();
    await waitNBlocks(hre, 1);

    expect(
      await userDecryptPriorVotes(
        signers.dave,
        firstCheckPointBlockNumber,
        confidentialERC20Votes,
        confidentialERC20VotesAddress,
      ),
    ).to.be.equal(parseUnits(String(1_000_000), 6));

    expect(
      await userDecryptPriorVotes(
        signers.dave,
        secondCheckPointBlockNumber,
        confidentialERC20Votes,
        confidentialERC20VotesAddress,
      ),
    ).to.be.equal(parseUnits(String(2_000_000), 6));
  });

  it('only governor contract can call getPriorVotes', async function () {
    await expect(
      confidentialERC20Votes.getPriorVotesForGovernor('0xE359a77c3bFE58792FB167D05720e37032A1e520', 0),
    ).to.be.revertedWithCustomError(confidentialERC20Votes, 'GovernorInvalid');
  });

  it('only owner can set governor contract', async function () {
    const newAllowedContract = '0x9d3e06a2952dc49EDCc73e41C76645797fC53967';
    await expect(confidentialERC20Votes.connect(signers.bob).setGovernor(newAllowedContract))
      .to.be.revertedWithCustomError(confidentialERC20Votes, 'OwnableUnauthorizedAccount')
      .withArgs(signers.bob.address);
  });

  it('getCurrentVote/getPriorVotes without any vote cannot be decrypted', async function () {
    // 1. If no checkpoint exists using getCurrentVotes
    let currentVoteHandle = await confidentialERC20Votes.connect(signers.bob).getCurrentVotes(signers.bob.address);
    expect(currentVoteHandle).to.be.eq(0n);

    await expect(
      hre.fhevm.userDecryptEuint(FhevmType.euint64, currentVoteHandle, confidentialERC20VotesAddress, signers.bob),
    ).to.be.rejectedWith('Handle is not initialized');

    // 2. If no checkpoint exists using getPriorVotes
    let latestBlockNumber = await ethers.provider.getBlockNumber();
    await waitNBlocks(hre, 1);

    currentVoteHandle = await confidentialERC20Votes
      .connect(signers.bob)
      .getPriorVotes(signers.bob.address, latestBlockNumber);

    // The handle is not set.
    expect(currentVoteHandle).to.be.eq(0n);

    await expect(
      hre.fhevm.userDecryptEuint(FhevmType.euint64, currentVoteHandle, confidentialERC20VotesAddress, signers.bob),
    ).to.be.rejectedWith('Handle is not initialized');

    // 3. If a checkpoint exists using getPriorVotes but block.number < block of first checkpoint
    latestBlockNumber = await ethers.provider.getBlockNumber();
    await waitNBlocks(hre, 1);

    const tx = await confidentialERC20Votes.connect(signers.alice).delegate(signers.bob.address);
    await tx.wait();

    currentVoteHandle = await confidentialERC20Votes
      .connect(signers.bob)
      .getPriorVotes(signers.bob.address, latestBlockNumber);

    // It is an encrypted constant that is not reencryptable by Bob.
    expect(currentVoteHandle).to.eq(0n);

    await expect(
      hre.fhevm.userDecryptEuint(FhevmType.euint64, currentVoteHandle, confidentialERC20VotesAddress, signers.bob),
    ).to.be.rejectedWith('Handle is not initialized');
  });

  it('can do multiple checkpoints and access the values when needed', async function () {
    let i = 0;

    const blockNumbers = [];

    const thisBlockNumber = await ethers.provider.getBlockNumber();

    while (i < 20) {
      let tx = await confidentialERC20Votes.connect(signers.alice).delegate(signers.alice.address);
      await tx.wait();
      blockNumbers.push(await ethers.provider.getBlockNumber());

      tx = await confidentialERC20Votes.connect(signers.alice).delegate(signers.carol.address);
      await tx.wait();
      blockNumbers.push(await ethers.provider.getBlockNumber());
      i++;
    }

    await waitNBlocks(hre, 1);

    // There are 40 checkpoints for Alice and 39 checkpoints for Carol
    expect(await confidentialERC20Votes.numCheckpoints(signers.alice.address)).to.eq(BigInt(40));
    expect(await confidentialERC20Votes.numCheckpoints(signers.carol.address)).to.eq(BigInt(39));

    i = 0;

    const startWithAlice = thisBlockNumber % 2 === 1;

    while (i < 40) {
      if (blockNumbers[i] % 2 === 0) {
        expect(
          await userDecryptPriorVotes(
            startWithAlice ? signers.alice : signers.carol,
            blockNumbers[i],
            confidentialERC20Votes,
            confidentialERC20VotesAddress,
          ),
        ).to.be.eq(parseUnits(String(10_000_000), 6));
      } else {
        expect(
          await userDecryptPriorVotes(
            startWithAlice ? signers.carol : signers.alice,
            blockNumbers[i],
            confidentialERC20Votes,
            confidentialERC20VotesAddress,
          ),
        ).to.be.eq(parseUnits(String(10_000_000), 6));
      }
      i++;
    }
  });

  it('governor address can access votes for any account', async function () {
    // Bob becomes the governor address.
    let tx = await confidentialERC20Votes.connect(signers.alice).setGovernor(signers.bob.address);
    await tx.wait();

    await expect(tx).to.emit(confidentialERC20Votes, 'NewGovernor').withArgs(signers.bob);

    // Alice delegates her votes to Carol.
    tx = await confidentialERC20Votes.connect(signers.alice).delegate(signers.carol.address);
    await tx.wait();

    const latestBlockNumber = await ethers.provider.getBlockNumber();
    await waitNBlocks(hre, 1);
    await waitNBlocks(hre, 1);

    // Bob, the governor address, gets the prior votes of Carol.
    // @dev It is not possible to catch the return value since it is not a view function.
    // ConfidentialGovernorAlpha.test.ts contains tests that use this function.
    await confidentialERC20Votes
      .connect(signers.bob)
      .getPriorVotesForGovernor(signers.carol.address, latestBlockNumber + 1);
  });

  it('different voters can delegate to same delegatee', async function () {
    const transferAmount = parseUnits(String(2_000_000), 6);

    const input = hre.fhevm.createEncryptedInput(confidentialERC20VotesAddress, signers.alice.address);
    input.add64(transferAmount);
    const encryptedTransferAmount = await input.encrypt();

    let tx = await confidentialERC20Votes['transfer(address,bytes32,bytes)'](
      signers.bob.address,
      encryptedTransferAmount.handles[0],
      encryptedTransferAmount.inputProof,
    );

    await tx.wait();

    tx = await confidentialERC20Votes.connect(signers.alice).delegate(signers.carol);
    await tx.wait();

    tx = await confidentialERC20Votes.connect(signers.bob).delegate(signers.carol);
    await tx.wait();

    const latestBlockNumber = await ethers.provider.getBlockNumber();
    await waitNBlocks(hre, 1);

    expect(
      await userDecryptCurrentVotes(signers.carol, confidentialERC20Votes, confidentialERC20VotesAddress),
    ).to.equal(parseUnits(String(10_000_000), 6));

    expect(
      await userDecryptPriorVotes(
        signers.carol,
        latestBlockNumber,
        confidentialERC20Votes,
        confidentialERC20VotesAddress,
      ),
    ).to.equal(await userDecryptCurrentVotes(signers.carol, confidentialERC20Votes, confidentialERC20VotesAddress));
  });

  // @dev To run this test, it is required to add gas = "auto" in the hardhat config.
  it.skip('number of checkpoints is incremented once per block, even when written multiple times in same block', async function () {
    await network.provider.send('evm_setAutomine', [false]);
    await network.provider.send('evm_setIntervalMining', [0]);

    // @dev There are two checkpoints in the same block.
    await confidentialERC20Votes.connect(signers.alice).delegate(signers.bob);
    await confidentialERC20Votes.connect(signers.alice).delegate(signers.carol);

    await network.provider.send('evm_mine');
    await network.provider.send('evm_setAutomine', [true]);

    expect(await confidentialERC20Votes.numCheckpoints(signers.alice.address)).to.be.equal(0n);
    expect(await confidentialERC20Votes.numCheckpoints(signers.bob.address)).to.be.equal(1n);
    expect(await confidentialERC20Votes.numCheckpoints(signers.carol.address)).to.be.equal(1n);

    expect(await userDecryptCurrentVotes(signers.bob, confidentialERC20Votes, confidentialERC20VotesAddress)).to.equal(
      0,
    );

    expect(
      await userDecryptCurrentVotes(signers.carol, confidentialERC20Votes, confidentialERC20VotesAddress),
    ).to.equal(parseUnits(String(10_000_000), 6));
  });
});
