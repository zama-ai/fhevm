import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { expect } from 'chai';
import { network } from 'hardhat';

import type { HighestDieRoll, HighestDieRoll__factory } from '../../types/ethers-contracts/index.ts';
import { requireReceipt } from '../utils/receipts.ts';
import { getSigners } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;
// The doc example's cast (accounts #0, #1, #2).
type Signers = { owner: HardhatEthersSigner; alice: HardhatEthersSigner; bob: HardhatEthersSigner };
// ethers' receipt type, named through the contract so `ethers` stays an indirect dependency of the suite.
type Receipt = NonNullable<Awaited<ReturnType<Awaited<ReturnType<HighestDieRoll['highestDieRoll']>>['wait']>>>;

async function deployFixture(): Promise<{
  readonly highestDiceRoll: HighestDieRoll;
  readonly highestDiceRollAddress: Hex;
}> {
  // Contracts are deployed using the first signer/account by default
  const factory: HighestDieRoll__factory = await ethers.getContractFactory('HighestDieRoll');
  const highestDiceRoll = await factory.deploy();
  const highestDiceRollAddress = (await highestDiceRoll.getAddress()) as Hex;

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
  let contractAddress: Hex;
  let signers: Signers;
  let playerA: HardhatEthersSigner;
  let playerB: HardhatEthersSigner;

  before(async function () {
    // Check whether the tests are running against an FHEVM mock environment
    if (!fhevm.isCleartext) {
      throw new Error(`This hardhat test suite can only run on a cleartext node`);
    }

    const suiteSigners = await getSigners(connection);
    signers = { owner: suiteSigners.alice, alice: suiteSigners.bob, bob: suiteSigners.carol };

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
  function parseGameCreatedEvent(txReceipt: Receipt): {
    txHash: Hex;
    gameId: number;
    playerA: Hex;
    playerB: Hex;
    playerAEncryptedDiceRoll: Hex;
    playerBEncryptedDiceRoll: Hex;
  } {
    const gameCreatedEvents: Array<{
      txHash: Hex;
      gameId: number;
      playerA: Hex;
      playerB: Hex;
      playerAEncryptedDiceRoll: Hex;
      playerBEncryptedDiceRoll: Hex;
    }> = [];

    for (const log of txReceipt.logs) {
      const parsedLog = contract.interface.parseLog(log);
      if (parsedLog?.name !== 'GameCreated') {
        continue;
      }
      // `parsedLog.args` is a Result: every element is `any`, so each one is asserted to the
      // field type declared above rather than assigned blind.
      gameCreatedEvents.push({
        txHash: txReceipt.hash as Hex,
        gameId: Number(parsedLog.args[0]),
        playerA: parsedLog.args[1] as Hex,
        playerB: parsedLog.args[2] as Hex,
        playerAEncryptedDiceRoll: parsedLog.args[3] as Hex,
        playerBEncryptedDiceRoll: parsedLog.args[4] as Hex,
      });
    }

    // In this example, we expect on one single GameCreated event
    expect(gameCreatedEvents.length).to.eq(1);
    const [event] = gameCreatedEvents;
    if (event === undefined) throw new Error('no GameCreated event');
    return event;
  }

  // ✅ Test should succeed
  it('decryption should succeed', async function () {
    console.log(``);
    console.log(`🎲 HighestDieRoll Game contract address: ${contractAddress}`);
    console.log(`   🤖 playerA.address: ${playerA.address}`);
    console.log(`   🎃 playerB.address: ${playerB.address}`);
    console.log(``);

    // Starts a new game. This will emit a `GameCreated` event
    const tx = await contract.connect(signers.owner).highestDieRoll(playerA, playerB);

    const receipt = requireReceipt(await tx.wait());

    // Parse the `GameCreated` event
    const gameCreatedEvent = parseGameCreatedEvent(receipt);

    // GameId is 1 since we are playing the first game
    expect(gameCreatedEvent.gameId).to.eq(1);
    expect(gameCreatedEvent.playerA).to.eq(playerA.address);
    expect(gameCreatedEvent.playerB).to.eq(playerB.address);
    expect(await contract.getGamesCount()).to.eq(1n);

    console.log(`✅ New game #${String(gameCreatedEvent.gameId)} created!`);
    console.log(JSON.stringify(gameCreatedEvent, null, 2));

    const gameId = gameCreatedEvent.gameId;
    const playerADiceRoll = gameCreatedEvent.playerAEncryptedDiceRoll;
    const playerBDiceRoll = gameCreatedEvent.playerBEncryptedDiceRoll;

    // Call the Zama Relayer to compute the decryption
    const publicDecryptResults = await fhevm.publicDecrypt([playerADiceRoll, playerBDiceRoll]);

    // The Relayer returns a `PublicDecryptResults` object containing:
    // - the ORDERED clear values
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
    console.log(`🎲 playerA's 8-sided die roll is ${String(a)}`);
    console.log(`🎲 playerB's 8-sided die roll is ${String(b)}`);

    // Let's forward the `PublicDecryptResults` content to the on-chain contract whose job
    // will simply be to verify the proof and store the final winner of the game
    await contract.recordAndVerifyWinner(gameId, abiEncodedClearGameResult, decryptionProof);

    const isRevealed = await contract.isGameRevealed(gameId);
    const winner = await contract.getWinner(gameId);

    expect(isRevealed).to.eq(true);
    expect(winner === playerA.address || winner === playerB.address || winner === ethers.ZeroAddress).to.eq(true);

    expect(isDraw).to.eq(winner === ethers.ZeroAddress);
    expect(playerAWon).to.eq(winner === playerA.address);
    expect(playerBWon).to.eq(winner === playerB.address);

    console.log(``);
    if (winner === playerA.address) {
      console.log(`🤖 playerA is the winner 🥇🥇`);
    } else if (winner === playerB.address) {
      console.log(`🎃 playerB is the winner 🥇🥇`);
    } else if (winner === ethers.ZeroAddress) {
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
    ).to.be.revertedWithCustomError(...fhevm.revertedWithCustomErrorArgs('KMSVerifier', 'KMSInvalidSigner'));
  });
});
