import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import { expect } from 'chai';
import { ethers as EthersT } from 'ethers';
import { ethers, fhevm } from 'hardhat';
import * as hre from 'hardhat';

import type { HeadsOrTails, HeadsOrTails__factory } from '../../../typechain-types';
import type { Signers } from '../signers';

async function deployFixture(): Promise<{
  readonly headsOrTails: HeadsOrTails;
  readonly headsOrTailsAddress: string;
}> {
  // Contracts are deployed using the first signer/account by default
  const factory: HeadsOrTails__factory = await ethers.getContractFactory('HeadsOrTails');
  const headsOrTails = (await factory.deploy()) as HeadsOrTails;
  const headsOrTailsAddress = await headsOrTails.getAddress();

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
    contractAddress = deployment.headsOrTailsAddress;
    contract = deployment.headsOrTails;
  });

  /**
   * Helper: Parses the GameCreated event from a transaction receipt.
   * WARNING: This function is for illustrative purposes only and is not production-ready
   * (it does not handle several events in same tx).
   */
  function parseGameCreatedEvent(txReceipt: EthersT.ContractTransactionReceipt | null): {
    txHash: `0x${string}`;
    gameId: number;
    headsPlayer: `0x${string}`;
    tailsPlayer: `0x${string}`;
    encryptedHasHeadsWon: `0x${string}`;
  } {
    const gameCreatedEvents: Array<{
      txHash: `0x${string}`;
      gameId: number;
      headsPlayer: `0x${string}`;
      tailsPlayer: `0x${string}`;
      encryptedHasHeadsWon: `0x${string}`;
    }> = [];

    if (txReceipt) {
      const logs = Array.isArray(txReceipt.logs) ? txReceipt.logs : [txReceipt.logs];
      for (let i = 0; i < logs.length; ++i) {
        const parsedLog = contract.interface.parseLog(logs[i]);
        if (parsedLog?.name !== 'GameCreated') {
          continue;
        }
        const ge = {
          txHash: txReceipt.hash as `0x${string}`,
          gameId: Number(parsedLog.args[0]),
          headsPlayer: parsedLog.args[1] as `0x${string}`,
          tailsPlayer: parsedLog.args[2] as `0x${string}`,
          encryptedHasHeadsWon: parsedLog.args[3] as `0x${string}`,
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
    console.log(`🎲 HeadsOrTails Game contract address: ${contractAddress}`);
    console.log(`   🤖 playerA.address: ${playerA.address}`);
    console.log(`   🎃 playerB.address: ${playerB.address}`);
    console.log(``);

    // Starts a new Heads or Tails game. This will emit a `GameCreated` event
    const tx = await contract.connect(signers.owner).headsOrTails(playerA, playerB);

    // Parse the `GameCreated` event
    const gameCreatedEvent = parseGameCreatedEvent(await tx.wait());

    // GameId is 1 since we are playing the first game
    expect(gameCreatedEvent.gameId).to.eq(1);
    expect(gameCreatedEvent.headsPlayer).to.eq(playerA.address);
    expect(gameCreatedEvent.tailsPlayer).to.eq(playerB.address);
    expect(await contract.getGamesCount()).to.eq(1);

    console.log(`✅ New game #${gameCreatedEvent.gameId} created!`);
    console.log(JSON.stringify(gameCreatedEvent, null, 2));

    const gameId = gameCreatedEvent.gameId;
    const encryptedBool: string = gameCreatedEvent.encryptedHasHeadsWon;

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
    const tx = await contract.connect(signers.owner).headsOrTails(playerA, playerB);
    const gameCreatedEvent = parseGameCreatedEvent(await tx.wait());

    const publicDecryptResults = await fhevm.publicDecrypt([gameCreatedEvent.encryptedHasHeadsWon]);
    await expect(
      contract.recordAndVerifyWinner(
        gameCreatedEvent.gameId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof + 'dead',
      ),
    ).to.be.revertedWithCustomError(
      { interface: new EthersT.Interface(['error KMSInvalidSigner(address invalidSigner)']) },
      'KMSInvalidSigner',
    );
  });

  // ❌ The test must fail if a malicious operator attempts to use a decryption proof
  // with a forged game result.
  it('should fail when using a decryption proof with a forged game result', async function () {
    const tx = await contract.connect(signers.owner).headsOrTails(playerA, playerB);
    const gameCreatedEvent = parseGameCreatedEvent(await tx.wait());

    const publicDecryptResults = await fhevm.publicDecrypt([gameCreatedEvent.encryptedHasHeadsWon]);
    const clearHeadsHasWon = publicDecryptResults.clearValues[gameCreatedEvent.encryptedHasHeadsWon];

    // The clear value is also ABI-encoded
    const decodedHeadsHasWon: unknown = EthersT.AbiCoder.defaultAbiCoder().decode(
      ['bool'],
      publicDecryptResults.abiEncodedClearValues,
    )[0];
    expect(decodedHeadsHasWon).to.eq(clearHeadsHasWon);

    // Let's try to forge the game result
    const forgedABIEncodedClearValues = EthersT.AbiCoder.defaultAbiCoder().encode(
      ['bool'],
      [!(clearHeadsHasWon as boolean)],
    );

    await expect(
      contract.recordAndVerifyWinner(
        gameCreatedEvent.gameId,
        forgedABIEncodedClearValues,
        publicDecryptResults.decryptionProof,
      ),
    ).to.be.revertedWithCustomError(
      { interface: new EthersT.Interface(['error KMSInvalidSigner(address invalidSigner)']) },
      'KMSInvalidSigner',
    );
  });

  // ❌ Two games (Game1 and Game2) are played between playerA and playerB.
  // The test must fail if a malicious operator attempts to forge the result of Game1
  // with the result of Game2
  it('should fail when using the result of a different game', async function () {
    // Game 1
    const tx1 = await contract.connect(signers.owner).headsOrTails(playerA, playerB);
    const gameCreatedEvent1 = parseGameCreatedEvent(await tx1.wait());

    // Game 2
    const tx2 = await contract.connect(signers.owner).headsOrTails(playerA, playerB);
    const gameCreatedEvent2 = parseGameCreatedEvent(await tx2.wait());

    // Let's try to forge the Game1's winner using the result of Game2
    const publicDecryptResults2 = await fhevm.publicDecrypt([gameCreatedEvent2.encryptedHasHeadsWon]);

    await expect(
      contract.recordAndVerifyWinner(
        gameCreatedEvent1.gameId,
        publicDecryptResults2.abiEncodedClearValues,
        publicDecryptResults2.decryptionProof,
      ),
    ).to.be.revertedWithCustomError(
      { interface: new EthersT.Interface(['error KMSInvalidSigner(address invalidSigner)']) },
      'KMSInvalidSigner',
    );
  });
});
