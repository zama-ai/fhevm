import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import type { FhevmInstance } from '@zama-fhe/relayer-sdk/node';
import { expect } from 'chai';
import { ethers as EthersT } from 'ethers';
import { ethers } from 'hardhat';

import { HeadsOrTails, HeadsOrTails__factory } from '../../typechain-types';
import { createInstance } from '../instance';
import { Signers, getSigners, initSigners } from '../signers';

async function deployFixture() {
  // Contracts are deployed using the first signer/account by default
  const factory = (await ethers.getContractFactory('HeadsOrTails')) as unknown as HeadsOrTails__factory;
  const headsOrTails = (await factory.deploy()) as HeadsOrTails;
  const headsOrTails_address = await headsOrTails.getAddress();

  return { headsOrTails, headsOrTails_address };
}

describe('BBB HeadsOrTails', function () {
  let contract: HeadsOrTails;
  let contractAddress: string;
  let signers: Signers;
  let owner: HardhatEthersSigner;
  let playerA: HardhatEthersSigner;
  let playerB: HardhatEthersSigner;
  let instance: FhevmInstance;

  before(async function () {
    await initSigners(5);
    signers = await getSigners();
    instance = await createInstance();

    owner = signers.alice;
    playerA = signers.bob;
    playerB = signers.carol;
  });

  beforeEach(async function () {
    // Deploy a new contract each time we run a new test
    const deployment = await deployFixture();
    contractAddress = deployment.headsOrTails_address;
    contract = deployment.headsOrTails;
  });

  function parseGameCreatedEvent(txReceipt: EthersT.ContractTransactionReceipt | null):
    | {
        gameId: number;
        headsPlayer: string;
        tailsPlayer: string;
        encryptedHasHeadWon: string;
      }
    | undefined {
    let gameCreatedEvent:
      | { gameId: number; headsPlayer: string; tailsPlayer: string; encryptedHasHeadWon: string }
      | undefined = undefined;

    if (txReceipt) {
      console.log(`✅ New game created tx:${txReceipt.hash}`);
      const logs = Array.isArray(txReceipt.logs) ? txReceipt.logs : [txReceipt.logs];
      for (let i = 0; i < logs.length; ++i) {
        const parsedLog = contract.interface.parseLog(logs[i]);
        if (!parsedLog) {
          continue;
        }
        if (parsedLog.name !== 'GameCreated') {
          continue;
        }

        gameCreatedEvent = {
          gameId: Number(parsedLog.args[0]),
          headsPlayer: parsedLog.args[1],
          tailsPlayer: parsedLog.args[2],
          encryptedHasHeadWon: parsedLog.args[3],
        };
      }
    }

    return gameCreatedEvent;
  }

  // ✅ Test should succeed
  it('decryption should succeed', async function () {
    console.log(``);
    console.log(`🎲 HeadsOrTails Game contract address: ${contractAddress}`);
    console.log(`   🤖 playerA.address: ${playerA.address}`);
    console.log(`   🎃 playerB.address: ${playerB.address}`);
    console.log(``);

    // Starts a new Heads or Tails game. This will emit a `GameCreated` event
    const tx = await contract.connect(owner).headsOrTails(playerA, playerB);

    // Parse the `GameCreated` event
    const gameCreatedEvent = parseGameCreatedEvent(await tx.wait())!;

    // GameId is 1 since we are playing the first game
    expect(gameCreatedEvent !== undefined).to.eq(true);
    expect(gameCreatedEvent.gameId).to.eq(1);
    expect(gameCreatedEvent.headsPlayer).to.eq(playerA.address);
    expect(gameCreatedEvent.tailsPlayer).to.eq(playerB.address);
    expect(await contract.getGamesCount()).to.eq(1);

    console.log(JSON.stringify(gameCreatedEvent, null, 2));

    const gameId = gameCreatedEvent.gameId;
    const encryptedBool: string = gameCreatedEvent.encryptedHasHeadWon;

    // Call the Zama Relayer to compute the decryption
    const publicDecryptResults = await instance.publicDecrypt([encryptedBool]);

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

  // ❌ Test should fail if proof is invalid
  it('decryption should fail', async function () {
    const tx = await contract.connect(owner).headsOrTails(playerA, playerB);
    const gameCreatedEvent = parseGameCreatedEvent(await tx.wait())!;
    const publicDecryptResults = await instance.publicDecrypt([gameCreatedEvent.encryptedHasHeadWon]);
    // ❌ Call `contract.recordAndVerifyWinner` using order (B, A)
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
});
