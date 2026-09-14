import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { expect } from 'chai';
import { network } from 'hardhat';

import type { HeadsOrTails, HeadsOrTails__factory } from '../../types/ethers-contracts/index.ts';
import { requireReceipt } from '../utils/receipts.ts';
import { getSigners } from '../utils/signers.ts';

const connection = await network.getOrCreate();
const { ethers, fhevm } = connection;

type Hex = `0x${string}`;
// The doc example's cast (accounts #0, #1, #2).
type Signers = { owner: HardhatEthersSigner; alice: HardhatEthersSigner; bob: HardhatEthersSigner };
// ethers' receipt type, named through the contract so `ethers` stays an indirect dependency of the suite.
type Receipt = NonNullable<Awaited<ReturnType<Awaited<ReturnType<HeadsOrTails['headsOrTails']>>['wait']>>>;

async function deployFixture(): Promise<{
  readonly headsOrTails: HeadsOrTails;
  readonly headsOrTailsAddress: Hex;
}> {
  // Contracts are deployed using the first signer/account by default
  const factory: HeadsOrTails__factory = await ethers.getContractFactory('HeadsOrTails');
  const headsOrTails = await factory.deploy();
  const headsOrTailsAddress = (await headsOrTails.getAddress()) as Hex;

  return { headsOrTails, headsOrTailsAddress };
}

/**
 * The `HeadsOrTails` example showcases the public decryption mechanism and
 * its corresponding on-chain verification in the case of a single value.
 * The core assertion is to guarantee that a single given cleartext is the
 * cryptographically verifiable result of the decryption of a single original
 * on-chain ciphertext.
 */
describe('HeadsOrTails', function () {
  let contract: HeadsOrTails;
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
    contractAddress = deployment.headsOrTailsAddress;
    contract = deployment.headsOrTails;
  });

  /**
   * Helper: Parses the GameCreated event from a transaction receipt.
   * WARNING: This function is for illustrative purposes only and is not production-ready
   * (it does not handle several events in same tx).
   */
  function parseGameCreatedEvent(txReceipt: Receipt): {
    txHash: Hex;
    gameId: number;
    headsPlayer: Hex;
    tailsPlayer: Hex;
    encryptedHasHeadsWon: Hex;
  } {
    const gameCreatedEvents: Array<{
      txHash: Hex;
      gameId: number;
      headsPlayer: Hex;
      tailsPlayer: Hex;
      encryptedHasHeadsWon: Hex;
    }> = [];

    for (const log of txReceipt.logs) {
      const parsedLog = contract.interface.parseLog(log);
      if (parsedLog?.name !== 'GameCreated') {
        continue;
      }
      gameCreatedEvents.push({
        txHash: txReceipt.hash as Hex,
        gameId: Number(parsedLog.args[0]),
        headsPlayer: parsedLog.args[1] as Hex,
        tailsPlayer: parsedLog.args[2] as Hex,
        encryptedHasHeadsWon: parsedLog.args[3] as Hex,
      });
    }

    // In this example, we expect on one single GameCreated event
    expect(gameCreatedEvents.length).to.eq(1);
    const [event] = gameCreatedEvents;
    if (event === undefined) throw new Error('no GameCreated event');
    return event;
  }

  async function play(): Promise<ReturnType<typeof parseGameCreatedEvent>> {
    const tx = await contract.connect(signers.owner).headsOrTails(playerA, playerB);
    return parseGameCreatedEvent(requireReceipt(await tx.wait()));
  }

  // ✅ Test should succeed
  it('decryption should succeed', async function () {
    console.log(``);
    console.log(`🎲 HeadsOrTails Game contract address: ${contractAddress}`);
    console.log(`   🤖 playerA.address: ${playerA.address}`);
    console.log(`   🎃 playerB.address: ${playerB.address}`);
    console.log(``);

    // Starts a new Heads or Tails game. This will emit a `GameCreated` event
    const gameCreatedEvent = await play();

    // GameId is 1 since we are playing the first game
    expect(gameCreatedEvent.gameId).to.eq(1);
    expect(gameCreatedEvent.headsPlayer).to.eq(playerA.address);
    expect(gameCreatedEvent.tailsPlayer).to.eq(playerB.address);
    expect(await contract.getGamesCount()).to.eq(1n);

    console.log(`✅ New game #${String(gameCreatedEvent.gameId)} created!`);
    console.log(JSON.stringify(gameCreatedEvent, null, 2));

    const gameId = gameCreatedEvent.gameId;
    const encryptedBool = gameCreatedEvent.encryptedHasHeadsWon;

    // Call the Zama Relayer to compute the decryption
    const publicDecryptResults = await fhevm.publicDecrypt([encryptedBool]);

    // The Relayer returns a `PublicDecryptResults` object containing:
    // - the ORDERED clear values (here we have only one single value)
    // - the ORDERED clear values in ABI-encoded form
    // - the KMS decryption proof associated with the ORDERED clear values in ABI-encoded form
    const abiEncodedClearGameResult = publicDecryptResults.abiEncodedClearValues;
    const decryptionProof = publicDecryptResults.decryptionProof;

    // Let's forward the `PublicDecryptResults` content to the on-chain contract whose job
    // will simply be to verify the proof and declare the final winner of the game
    await contract.recordAndVerifyWinner(gameId, abiEncodedClearGameResult, decryptionProof);

    const winner = await contract.getWinner(gameId);

    expect(winner === playerA.address || winner === playerB.address).to.eq(true);

    console.log(``);
    if (winner === playerA.address) {
      console.log(`🤖 playerA is the winner 🥇🥇`);
    } else if (winner === playerB.address) {
      console.log(`🎃 playerB is the winner 🥇🥇`);
    }
  });

  // ❌ The test must fail if the decryption proof is invalid
  it('should fail when the decryption proof is invalid', async function () {
    const gameCreatedEvent = await play();

    const publicDecryptResults = await fhevm.publicDecrypt([gameCreatedEvent.encryptedHasHeadsWon]);
    await expect(
      contract.recordAndVerifyWinner(
        gameCreatedEvent.gameId,
        publicDecryptResults.abiEncodedClearValues,
        `${publicDecryptResults.decryptionProof}dead`,
      ),
    ).to.be.revertedWithCustomError(...fhevm.revertedWithCustomErrorArgs('KMSVerifier', 'KMSInvalidSigner'));
  });

  // ❌ The test must fail if a malicious operator attempts to use a decryption proof
  // with a forged game result.
  it('should fail when using a decryption proof with a forged game result', async function () {
    const gameCreatedEvent = await play();

    const publicDecryptResults = await fhevm.publicDecrypt([gameCreatedEvent.encryptedHasHeadsWon]);
    const clearHeadsHasWon = publicDecryptResults.clearValues[gameCreatedEvent.encryptedHasHeadsWon];

    // The clear value is also ABI-encoded
    const decodedHeadsHasWon: unknown = ethers.AbiCoder.defaultAbiCoder().decode(
      ['bool'],
      publicDecryptResults.abiEncodedClearValues,
    )[0];
    expect(decodedHeadsHasWon).to.eq(clearHeadsHasWon);

    // Let's try to forge the game result
    const forgedABIEncodedClearValues = ethers.AbiCoder.defaultAbiCoder().encode(
      ['bool'],
      [!(clearHeadsHasWon as boolean)],
    );

    await expect(
      contract.recordAndVerifyWinner(
        gameCreatedEvent.gameId,
        forgedABIEncodedClearValues,
        publicDecryptResults.decryptionProof,
      ),
    ).to.be.revertedWithCustomError(...fhevm.revertedWithCustomErrorArgs('KMSVerifier', 'KMSInvalidSigner'));
  });

  // ❌ Two games (Game1 and Game2) are played between playerA and playerB.
  // The test must fail if a malicious operator attempts to forge the result of Game1
  // with the result of Game2
  it('should fail when using the result of a different game', async function () {
    // Game 1
    const gameCreatedEvent1 = await play();

    // Game 2
    const gameCreatedEvent2 = await play();

    // Let's try to forge the Game1's winner using the result of Game2
    const publicDecryptResults2 = await fhevm.publicDecrypt([gameCreatedEvent2.encryptedHasHeadsWon]);

    await expect(
      contract.recordAndVerifyWinner(
        gameCreatedEvent1.gameId,
        publicDecryptResults2.abiEncodedClearValues,
        publicDecryptResults2.decryptionProof,
      ),
    ).to.be.revertedWithCustomError(...fhevm.revertedWithCustomErrorArgs('KMSVerifier', 'KMSInvalidSigner'));
  });
});
