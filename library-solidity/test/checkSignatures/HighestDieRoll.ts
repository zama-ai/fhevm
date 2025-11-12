import { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/signers';
import type { ClearValueType, FhevmInstance } from '@zama-fhe/relayer-sdk/node';
import { expect } from 'chai';
import { ethers as EthersT } from 'ethers';
import { ethers } from 'hardhat';

import { HighestDieRoll, HighestDieRoll__factory } from '../../typechain-types';
import { createInstance } from '../instance';
import { Signers, getSigners, initSigners } from '../signers';

async function deployFixture() {
  // Contracts are deployed using the first signer/account by default
  const factory = (await ethers.getContractFactory('HighestDieRoll')) as unknown as HighestDieRoll__factory;
  const highestDiceRoll = (await factory.deploy()) as HighestDieRoll;
  const highestDiceRoll_address = await highestDiceRoll.getAddress();

  return { highestDiceRoll, highestDiceRoll_address };
}

describe('HighestDieRoll', function () {
  let contract: HighestDieRoll;
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
    contractAddress = deployment.highestDiceRoll_address;
    contract = deployment.highestDiceRoll;
  });

  function parseGameCreatedEvent(txReceipt: EthersT.ContractTransactionReceipt | null):
    | {
        gameId: number;
        playerA: string;
        playerB: string;
        playerAEncryptedDiceRoll: `0x${string}`;
        playerBEncryptedDiceRoll: `0x${string}`;
      }
    | undefined {
    let gameCreatedEvent:
      | {
          gameId: number;
          playerA: string;
          playerB: string;
          playerAEncryptedDiceRoll: `0x${string}`;
          playerBEncryptedDiceRoll: `0x${string}`;
        }
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
          playerA: parsedLog.args[1],
          playerB: parsedLog.args[2],
          playerAEncryptedDiceRoll: parsedLog.args[3],
          playerBEncryptedDiceRoll: parsedLog.args[4],
        };
      }
    }

    return gameCreatedEvent;
  }

  // ✅ Test should succeed
  it('decryption should succeed', async function () {
    console.log(``);
    console.log(`🎲 HighestDieRoll Game contract address: ${contractAddress}`);
    console.log(`   🤖 playerA.address: ${playerA.address}`);
    console.log(`   🎃 playerB.address: ${playerB.address}`);
    console.log(``);

    // Starts a new Heads or Tails game. This will emit a `GameCreated` event
    const tx = await contract.connect(owner).highestDieRoll(playerA, playerB);

    // Parse the `GameCreated` event
    const gameCreatedEvent = parseGameCreatedEvent(await tx.wait())!;

    // GameId is 1 since we are playing the first game
    expect(gameCreatedEvent !== undefined).to.eq(true);
    expect(gameCreatedEvent.gameId).to.eq(1);
    expect(gameCreatedEvent.playerA).to.eq(playerA.address);
    expect(gameCreatedEvent.playerB).to.eq(playerB.address);
    expect(await contract.getGamesCount()).to.eq(1);

    console.log(JSON.stringify(gameCreatedEvent, null, 2));

    const gameId = gameCreatedEvent.gameId;
    const playerADiceRoll = gameCreatedEvent.playerAEncryptedDiceRoll;
    const playerBDiceRoll = gameCreatedEvent.playerBEncryptedDiceRoll;

    // Call the Zama Relayer to compute the decryption
    const publicDecryptResults = await instance.publicDecrypt([playerADiceRoll, playerBDiceRoll]);

    // The Relayer returns a `PublicDecryptResults` object containing:
    // - the ORDERED clear values (here we have only one single value)
    // - the ORDERED clear values in ABI-encoded form
    // - the KMS decryption proof associated with the ORDERED clear values in ABI-encoded form
    const abiEncodedClearGameResult = publicDecryptResults.abiEncodedClearValues;
    const decryptionProof = publicDecryptResults.decryptionProof;

    const clearValueA: ClearValueType = publicDecryptResults.clearValues[playerADiceRoll];
    const clearValueB: ClearValueType = publicDecryptResults.clearValues[playerBDiceRoll];

    expect(typeof clearValueA).to.eq('bigint');
    expect(typeof clearValueB).to.eq('bigint');

    const a = (Number(clearValueA) % 8) + 1;
    const b = (Number(clearValueB) % 8) + 1;

    const isDraw = a === b;
    const playerAWon = a > b;
    const playerBWon = a < b;

    console.log('🎲 playerA die roll is ' + a);
    console.log('🎲 playerB die roll is ' + b);

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
  it('decryption should fail', async function () {
    // Test Case: Verify strict ordering is enforced for cryptographic proof generation.
    // The `decryptionProof` is generated based on the expected order (A, B). By ABI-encoding
    // the clear values in the **reverse order** (B, A), we create a mismatch when the contract
    // internally verifies the proof (e.g., checks a signature against a newly computed hash).
    // This intentional failure is expected to revert with the `KMSInvalidSigner` error,
    // confirming the proof's order dependency.
    const tx = await contract.connect(owner).highestDieRoll(playerA, playerB);
    const gameCreatedEvent = parseGameCreatedEvent(await tx.wait())!;
    const gameId = gameCreatedEvent.gameId;
    const playerADiceRoll = gameCreatedEvent.playerAEncryptedDiceRoll;
    const playerBDiceRoll = gameCreatedEvent.playerBEncryptedDiceRoll;
    // Call `fhevm.publicDecrypt` using order (A, B)
    const publicDecryptResults = await instance.publicDecrypt([playerADiceRoll, playerBDiceRoll]);
    const clearValueA: ClearValueType = publicDecryptResults.clearValues[playerADiceRoll];
    const clearValueB: ClearValueType = publicDecryptResults.clearValues[playerBDiceRoll];
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
