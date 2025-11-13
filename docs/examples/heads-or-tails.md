This example showcases the public decryption mechanism and its corresponding on-chain verification in the case of a single value.
The core assertion is to guarantee that a single given cleartext is the cryptographically verifiable result of the decryption of a single original on-chain ciphertext.

{% hint style="info" %}
To run this example correctly, make sure the files are placed in the following directories:

- `.sol` file → `<your-project-root-dir>/contracts/`
- `.ts` file → `<your-project-root-dir>/test/`

This ensures Hardhat can compile and test your contracts as expected.
{% endhint %}

{% tabs %}

{% tab title="HeadsOrTails.sol" %}

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import { FHE, ebool } from "@fhevm/solidity/lib/FHE.sol";
import { ZamaEthereumConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

/**
 * @title HeadsOrTails
 * @notice Implements a simple Heads or Tails game demonstrating public, permissionless decryption
 *         using the FHE.makePubliclyDecryptable feature.
 * @dev Inherits from ZamaEthereumConfig to access FHE functions like FHE.randEbool() and FHE.verifySignatures().
 */
contract HeadsOrTails is ZamaEthereumConfig {
    constructor() {}

    /**
     * @notice Simple counter to assign a unique ID to each new game.
     */
    uint256 private counter = 0;

    /**
     * @notice Defines the entire state for a single Heads or Tails game instance.
     */
    struct Game {
        /// @notice The address of the player who chose Heads.
        address headsPlayer;
        /// @notice The address of the player who chose Tails.
        address tailsPlayer;
        /// @notice The core encrypted result. This is a publicly decryptable ebool handle.
        //          true means Heads won; false means Tails won.
        ebool encryptedHasHeadsWon;
        /// @notice The clear address of the final winner, set after decryption and verification.
        address winner;
    }

    /**
     * @notice Mapping to store all game states, accessible by a unique game ID.
     */
    mapping(uint256 gameId => Game game) public games;

    /**
     * @notice Emitted when a new game is started, providing the encrypted handle required for decryption.
     * @param gameId The unique identifier for the game.
     * @param headsPlayer The address choosing Heads.
     * @param tailsPlayer The address choosing Tails.
     * @param encryptedHasHeadsWon The encrypted handle (ciphertext) storing the result.
     */
    event GameCreated(
        uint256 indexed gameId,
        address indexed headsPlayer,
        address indexed tailsPlayer,
        ebool encryptedHasHeadsWon
    );

    /**
     * @notice Initiates a new Heads or Tails game, generates the result using FHE,
     *         and makes the result publicly available for decryption.
     * @param headsPlayer The player address choosing Heads.
     * @param tailsPlayer The player address choosing Tails.
     */
    function headsOrTails(address headsPlayer, address tailsPlayer) external {
        require(headsPlayer != address(0), "Heads player is address zero");
        require(tailsPlayer != address(0), "Tails player is address zero");
        require(headsPlayer != tailsPlayer, "Heads player and Tails player should be different");

        // true: Heads
        // false: Tails
        ebool headsOrTailsResult = FHE.randEbool();

        counter++;

        // gameId > 0
        uint256 gameId = counter;
        games[gameId] = Game({
            headsPlayer: headsPlayer,
            tailsPlayer: tailsPlayer,
            encryptedHasHeadsWon: headsOrTailsResult,
            winner: address(0)
        });

        // We make the result publicly decryptable.
        FHE.makePubliclyDecryptable(headsOrTailsResult);

        // You can catch the event to get the gameId and the encryptedHasHeadsWon handle
        // for further decryption requests, or create a view function.
        emit GameCreated(gameId, headsPlayer, tailsPlayer, games[gameId].encryptedHasHeadsWon);
    }

    /**
     * @notice Returns the number of games created so far.
     * @return The number of games created.
     */
    function getGamesCount() public view returns (uint256) {
        return counter;
    }

    /**
     * @notice Returns the encrypted ebool handle that stores the game result.
     * @param gameId The ID of the game.
     * @return The encrypted result (ebool handle).
     */
    function hasHeadsWon(uint256 gameId) public view returns (ebool) {
        return games[gameId].encryptedHasHeadsWon;
    }

    /**
     * @notice Returns the address of the game winner.
     * @param gameId The ID of the game.
     * @return The winner's address (address(0) if not yet revealed).
     */
    function getWinner(uint256 gameId) public view returns (address) {
        require(games[gameId].winner != address(0), "Game winner not yet revealed");
        return games[gameId].winner;
    }

    /**
     * @notice Verifies the provided (decryption proof, ABI-encoded clear value) pair against the stored ciphertext,
     *         and then stores the winner of the game.
     * @param gameId The ID of the game to settle.
     * @param abiEncodedClearGameResult The ABI-encoded clear value (bool) associated to the `decryptionProof`.
     * @param decryptionProof The proof that validates the decryption.
     */
    function recordAndVerifyWinner(
        uint256 gameId,
        bytes memory abiEncodedClearGameResult,
        bytes memory decryptionProof
    ) public {
        require(games[gameId].winner == address(0), "Game winner already revealed");

        // 1. FHE Verification: Build the list of ciphertexts (handles) and verify the proof.
        //    The verification checks that 'abiEncodedClearGameResult' is the true decryption
        //    of the 'encryptedHasHeadsWon' handle using the provided 'decryptionProof'.

        // Creating the list of handles in the right order! In this case the order does not matter since the proof
        // only involves 1 single handle.
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(games[gameId].encryptedHasHeadsWon);

        // This FHE call reverts the transaction if the decryption proof is invalid.
        FHE.checkSignatures(cts, abiEncodedClearGameResult, decryptionProof);

        // 2. Decode the clear result and determine the winner's address.
        //    In this very specific case, the function argument `abiEncodedClearGameResult` could have been a simple
        //    `bool` instead of an abi-encoded bool. In this case, we should have compute abi.encode on-chain
        bool decodedClearGameResult = abi.decode(abiEncodedClearGameResult, (bool));
        address winner = decodedClearGameResult ? games[gameId].headsPlayer : games[gameId].tailsPlayer;

        // 3. Store the winner
        games[gameId].winner = winner;
    }
}
```

{% endtab %}

{% tab title="HeadsOrTails.ts" %}

```ts
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { expect } from "chai";
import { ethers as EthersT } from "ethers";
import { ethers, fhevm } from "hardhat";
import * as hre from "hardhat";

import { HeadsOrTails, HeadsOrTails__factory } from "../../../typechain-types";
import { Signers } from "../signers";

async function deployFixture() {
  // Contracts are deployed using the first signer/account by default
  const factory = (await ethers.getContractFactory("HeadsOrTails")) as HeadsOrTails__factory;
  const headsOrTails = (await factory.deploy()) as HeadsOrTails;
  const headsOrTails_address = await headsOrTails.getAddress();

  return { headsOrTails, headsOrTails_address };
}

describe("HeadsOrTails", function () {
  let contract: HeadsOrTails;
  let contractAddress: string;
  let signers: Signers;
  let playerA: HardhatEthersSigner;
  let playerB: HardhatEthersSigner;

  before(async function () {
    // Check whether the tests are running against an FHEVM mock environment
    if (!hre.fhevm.isMock) {
      throw new Error(`This hardhat test suite cannot run on Sepolia Testnet`);
    }

    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { owner: ethSigners[0], alice: ethSigners[1], bob: ethSigners[2] };

    playerA = signers.alice;
    playerB = signers.bob;
  });

  beforeEach(async function () {
    // Deploy a new contract each time we run a new test
    const deployment = await deployFixture();
    contractAddress = deployment.headsOrTails_address;
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
        if (!parsedLog || parsedLog.name !== "GameCreated") {
          continue;
        }
        const ge = {
          txHash: txReceipt.hash as `0x${string}`,
          gameId: Number(parsedLog.args[0]),
          headsPlayer: parsedLog.args[1],
          tailsPlayer: parsedLog.args[2],
          encryptedHasHeadsWon: parsedLog.args[3],
        };
        gameCreatedEvents.push(ge);
      }
    }

    // In this example, we expect on one single GameCreated event
    expect(gameCreatedEvents.length).to.eq(1);

    return gameCreatedEvents[0];
  }

  // ✅ Test should succeed
  it("decryption should succeed", async function () {
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
  it("should fail when the decryption proof is invalid", async function () {
    const tx = await contract.connect(signers.owner).headsOrTails(playerA, playerB);
    const gameCreatedEvent = parseGameCreatedEvent(await tx.wait());

    const publicDecryptResults = await fhevm.publicDecrypt([gameCreatedEvent.encryptedHasHeadsWon]);
    await expect(
      contract.recordAndVerifyWinner(
        gameCreatedEvent.gameId,
        publicDecryptResults.abiEncodedClearValues,
        publicDecryptResults.decryptionProof + "dead",
      ),
    ).to.be.revertedWithCustomError(
      { interface: new EthersT.Interface(["error KMSInvalidSigner(address invalidSigner)"]) },
      "KMSInvalidSigner",
    );
  });

  // ❌ The test must fail if a malicious operator attempts to use a decryption proof
  // with a forged game result.
  it("should fail when using a decryption proof with a forged game result", async function () {
    const tx = await contract.connect(signers.owner).headsOrTails(playerA, playerB);
    const gameCreatedEvent = parseGameCreatedEvent(await tx.wait());

    const publicDecryptResults = await fhevm.publicDecrypt([gameCreatedEvent.encryptedHasHeadsWon]);
    const clearHeadsHasWon = publicDecryptResults.clearValues[gameCreatedEvent.encryptedHasHeadsWon];

    // The clear value is also ABI-encoded
    const decodedHeadsHasWon = EthersT.AbiCoder.defaultAbiCoder().decode(
      ["bool"],
      publicDecryptResults.abiEncodedClearValues,
    )[0];
    expect(decodedHeadsHasWon).to.eq(clearHeadsHasWon);

    // Let's try to forge the game result
    const forgedABIEncodedClearValues = EthersT.AbiCoder.defaultAbiCoder().encode(["bool"], [!clearHeadsHasWon]);

    await expect(
      contract.recordAndVerifyWinner(
        gameCreatedEvent.gameId,
        forgedABIEncodedClearValues,
        publicDecryptResults.decryptionProof,
      ),
    ).to.be.revertedWithCustomError(
      { interface: new EthersT.Interface(["error KMSInvalidSigner(address invalidSigner)"]) },
      "KMSInvalidSigner",
    );
  });

  // ❌ Two games (Game1 and Game2) are played between playerA and playerB.
  // The test must fail if a malicious operator attempts to forge the result of Game1
  // with the result of Game2
  it("should fail when using the result of a different game", async function () {
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
      { interface: new EthersT.Interface(["error KMSInvalidSigner(address invalidSigner)"]) },
      "KMSInvalidSigner",
    );
  });
});
```

{% endtab %}

{% endtabs %}
