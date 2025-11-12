// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {FHE, ebool} from "../lib/FHE.sol";
import {ZamaEthereumConfig} from "../config/ZamaConfig.sol";

/**
 * @title HeadsOrTails
 * @notice Implements a simple Heads or Tails game demonstrating public, permissionless decryption
 *         using the FHE.makePubliclyDecryptable feature.
 * @dev Inherits from ZamaEthereumConfig to access FHE functions.
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

        // Instead of calling the function FHE.requestDecryption, we make the result publicly decryptable directly.
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

        // 1. Decode the clear result and determine the winner's address.
        //    In this very specific case, the function argument `abiEncodedClearGameResult` could have been a simple
        //    `bool` instead of an abi-encoded bool. In this case, we should have compute abi.encode on-chain
        bool decodedClearGameResult = abi.decode(abiEncodedClearGameResult, (bool));
        address winner = decodedClearGameResult ? games[gameId].headsPlayer : games[gameId].tailsPlayer;

        // 2. Store the winner immediately to prevent re-entrancy issues
        games[gameId].winner = winner;

        // 3. FHE Verification: Build the list of ciphertexts (handles) and verify the proof.
        //    The verification checks that 'abiEncodedClearGameResult' is the true decryption
        //    of the 'encryptedHasHeadsWon' handle using the provided 'decryptionProof'.

        // Creating the list of handles in the right order! In this case the order does not matter since the proof
        // only involves 1 single handle.
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(games[gameId].encryptedHasHeadsWon);

        // This FHE call reverts the transaction if the decryption proof is invalid.
        FHE.checkSignatures(cts, abiEncodedClearGameResult, decryptionProof);
    }
}
