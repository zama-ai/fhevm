import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers as EthersT } from 'ethers';
import { ethers, fhevm } from 'hardhat';
import * as hre from 'hardhat';

import type { HighestDieRoll, HighestDieRoll__factory } from '../../../typechain-types';
import type { Signers } from '../signers';
import { requireReceipt } from '../utils';

async function deployFixture(): Promise<{
  readonly highestDiceRoll: HighestDieRoll;
  readonly highestDiceRollAddress: string;
}> {
  // Contracts are deployed using the first signer/account by default
  const factory: HighestDieRoll__factory = await ethers.getContractFactory('HighestDieRoll');
  const highestDiceRoll = (await factory.deploy()) as HighestDieRoll;
  const highestDiceRollAddress = await highestDiceRoll.getAddress();

  return { highestDiceRoll, highestDiceRollAddress };
}

/**
 * The `HighestDieRoll` example showcases the public decryption mechanism and
 * its corresponding on-chain verification in the case of multiple values.
 * The core assertion is to guarantee that multiple given cleartexts are the
 * cryptographically verifiable results of the decryption of multiple original
 * on-chain ciphertexts.
 */
describe('HighestDieRoll', function () {
  let contract: HighestDieRoll;
  let contractAddress: string;
  let signers: Signers;
  let playerA: HardhatEthersSigner;
  let playerB: HardhatEthersSigner;

  before(async function () {
    // Check whether the tests are running against an FHEVM mock environment
    if (!hre.fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }

    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { owner: ethSigners[0], alice: ethSigners[1], bob: ethSigners[2] };

    playerA = signers.alice;
    playerB = signers.bob;
  });

  beforeEach(async function () {
    // Deploy a new contract each time we run a new test
    const deployment = await deployFixture();
    contractAddress = deployment.highestDiceRollAddress;
    contract = deployment.highestDiceRoll;
  });

  /**
   * Helper: Parses the GameCreated event from a transaction receipt.
   * WARNING: This function is for illustrative purposes only and is not production-ready
   * (it does not handle several events in same tx).
   */
  function parseGameCreatedEvent(txReceipt: EthersT.ContractTransactionReceipt | null): {
    txHash: `0x${string}`;
    gameId: number;
    playerA: `0x${string}`;
    playerB: `0x${string}`;
    playerAEncryptedDiceRoll: `0x${string}`;
    playerBEncryptedDiceRoll: `0x${string}`;
  } {
    const gameCreatedEvents: Array<{
      txHash: `0x${string}`;
      gameId: number;
      playerA: `0x${string}`;
      playerB: `0x${string}`;
      playerAEncryptedDiceRoll: `0x${string}`;
      playerBEncryptedDiceRoll: `0x${string}`;
    }> = [];

    if (txReceipt) {
      const logs = Array.isArray(txReceipt.logs) ? txReceipt.logs : [txReceipt.logs];
      for (let i = 0; i < logs.length; ++i) {
        const parsedLog = contract.interface.parseLog(logs[i]);
        if (parsedLog?.name !== 'GameCreated') {
          continue;
        }
        // `parsedLog.args` is a Result: every element is `any`, so each one is asserted to the
        // field type declared above rather than assigned blind.
        const ge = {
          txHash: txReceipt.hash as `0x${string}`,
          gameId: Number(parsedLog.args[0]),
          playerA: parsedLog.args[1] as `0x${string}`,
          playerB: parsedLog.args[2] as `0x${string}`,
          playerAEncryptedDiceRoll: parsedLog.args[3] as `0x${string}`,
          playerBEncryptedDiceRoll: parsedLog.args[4] as `0x${string}`,
        };
        gameCreatedEvents.push(ge);
      }
    }

    // In this example, we expect on one single GameCreated event
    expect(gameCreatedEvents.length).to.eq(1);

    return gameCreatedEvents[0];
  }

  // ✅ Test should succeed
  it('decryption should succeed', async function () {
    console.log(``);
    console.log(`🎲 HighestDieRoll Game contract address: ${contractAddress}`);
    console.log(`   🤖 playerA.address: ${playerA.address}`);
    console.log(`   🎃 playerB.address: ${playerB.address}`);
    console.log(``);

    // Starts a new Heads or Tails game. This will emit a `GameCreated` event
    const tx = await contract.connect(signers.owner).highestDieRoll(playerA, playerB);

    const receipt = requireReceipt(await tx.wait());

    // Parse the `GameCreated` event
    const gameCreatedEvent = parseGameCreatedEvent(receipt);

    // GameId is 1 since we are playing the first game
    expect(gameCreatedEvent.gameId).to.eq(1);
    expect(gameCreatedEvent.playerA).to.eq(playerA.address);
    expect(gameCreatedEvent.playerB).to.eq(playerB.address);
    expect(await contract.getGamesCount()).to.eq(1);

    console.log(`✅ New game #${gameCreatedEvent.gameId} created!`);
    console.log(JSON.stringify(gameCreatedEvent, null, 2));

    const gameId = gameCreatedEvent.gameId;
    const playerADiceRoll = gameCreatedEvent.playerAEncryptedDiceRoll;
    const playerBDiceRoll = gameCreatedEvent.playerBEncryptedDiceRoll;

    // Call the Zama Relayer to compute the decryption
    const publicDecryptResults = await fhevm.publicDecrypt([playerADiceRoll, playerBDiceRoll]);

    // The Relayer returns a `PublicDecryptResults` object containing:
    // - the ORDERED clear values (here we have only one single value)
    // - the ORDERED clear values in ABI-encoded form
    // - the KMS decryption proof associated with the ORDERED clear values in ABI-encoded form
    const abiEncodedClearGameResult = publicDecryptResults.abiEncodedClearValues;
    const decryptionProof = publicDecryptResults.decryptionProof;

    const clearValueA = publicDecryptResults.clearValues[playerADiceRoll];
    const clearValueB = publicDecryptResults.clearValues[playerBDiceRoll];

    expect(typeof clearValueA).to.eq('bigint');
    expect(typeof clearValueB).to.eq('bigint');

    // playerA's 8-sided die roll result (between 1 and 8)
    const a = (Number(clearValueA) % 8) + 1;
    // playerB's 8-sided die roll result (between 1 and 8)
    const b = (Number(clearValueB) % 8) + 1;

    const isDraw = a === b;
    const playerAWon = a > b;
    const playerBWon = a < b;

    console.log(``);
    console.log(`🎲 playerA's 8-sided die roll is ${a}`);
    console.log(`🎲 playerB's 8-sided die roll is ${b}`);

    // Let's forward the `PublicDecryptResults` content to the on-chain contract whose job
    // will simply be to verify the proof and store the final winner of the game
    await contract.recordAndVerifyWinner(gameId, abiEncodedClearGameResult, decryptionProof);

    const isRevealed = await contract.isGameRevealed(gameId);
    const winner = await contract.getWinner(gameId);

    expect(isRevealed).to.eq(true);
    expect(winner === playerA.address || winner === playerB.address || winner === EthersT.ZeroAddress).to.eq(true);

    expect(isDraw).to.eq(winner === EthersT.ZeroAddress);
    expect(playerAWon).to.eq(winner === playerA.address);
    expect(playerBWon).to.eq(winner === playerB.address);

    console.log(``);
    if (winner === playerA.address) {
      console.log(`🤖 playerA is the winner 🥇🥇`);
    } else if (winner === playerB.address) {
      console.log(`🎃 playerB is the winner 🥇🥇`);
    } else if (winner === EthersT.ZeroAddress) {
      console.log(`Game is a draw!`);
    }
  });

  // ❌ Test should fail because clear values are ABI-encoded in the wrong order.
  it('decryption should fail when ABI-encoding is wrongly ordered', async function () {
    // Test Case: Verify strict ordering is enforced for cryptographic proof generation.
    // The `decryptionProof` is generated based on the expected order (A, B). By ABI-encoding
    // the clear values in the **reverse order** (B, A), we create a mismatch when the contract
    // internally verifies the proof (e.g., checks a signature against a newly computed hash).
    // This intentional failure is expected to revert with the `KMSInvalidSigner` error,
    // confirming the proof's order dependency.
    const tx = await contract.connect(signers.owner).highestDieRoll(playerA, playerB);
    const receipt = requireReceipt(await tx.wait());
    const gameCreatedEvent = parseGameCreatedEvent(receipt);
    const gameId = gameCreatedEvent.gameId;
    const playerADiceRoll = gameCreatedEvent.playerAEncryptedDiceRoll;
    const playerBDiceRoll = gameCreatedEvent.playerBEncryptedDiceRoll;
    // Call `fhevm.publicDecrypt` using order (A, B)
    const publicDecryptResults = await fhevm.publicDecrypt([playerADiceRoll, playerBDiceRoll]);
    const clearValueA = publicDecryptResults.clearValues[playerADiceRoll];
    const clearValueB = publicDecryptResults.clearValues[playerBDiceRoll];
    const decryptionProof = publicDecryptResults.decryptionProof;
    expect(typeof clearValueA).to.eq('bigint');
    expect(typeof clearValueB).to.eq('bigint');
    expect(ethers.AbiCoder.defaultAbiCoder().encode(['uint256', 'uint256'], [clearValueA, clearValueB])).to.eq(
      publicDecryptResults.abiEncodedClearValues,
    );
    const wrongOrderBAInsteadOfABAbiEncodedValues = ethers.AbiCoder.defaultAbiCoder().encode(
      ['uint256', 'uint256'],
      [clearValueB, clearValueA],
    );
    // ❌ Call `contract.recordAndVerifyWinner` using order (B, A)
    await expect(
      contract.recordAndVerifyWinner(gameId, wrongOrderBAInsteadOfABAbiEncodedValues, decryptionProof),
    ).to.be.revertedWithCustomError(
      { interface: new EthersT.Interface(['error KMSInvalidSigner(address invalidSigner)']) },
      'KMSInvalidSigner',
    );
  });
});
