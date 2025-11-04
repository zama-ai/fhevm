// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {FHE, ebool} from "../lib/FHE.sol";
import {EthereumConfig} from "../config/ZamaConfig.sol";

// This contract implements a simple example for public decryption using a HeadsOrTails game,
// instead of using an oracle workflow to reveal the result.
contract HeadsOrTails is EthereumConfig {
    constructor() {}

    uint256 private counter = 0;

    struct Game {
        address headsPlayer;
        address tailsPlayer;
        ebool encryptedHasHeadWon;
        address winner;
    }

    mapping(uint256 gameId => Game game) public games;

    event GameCreated(
        uint256 indexed gameId,
        address indexed headsPlayer,
        address indexed tailsPlayer,
        ebool encryptedHasHeadWon
    );

    // Start a new Heads or Tails game between two players. And put the result encrypted publicly decryptable.
    function headsOrTails(address headsPlayer, address tailsPlayer) external {
        require(headsPlayer != address(0), "Heads player is address zero");
        require(tailsPlayer != address(0), "Tails player is address zero");

        // true: Heads
        // false: Tails
        ebool headsOrTailsResult = FHE.randEbool();

        uint256 gameId = counter++;
        games[gameId] = Game({
            headsPlayer: headsPlayer,
            tailsPlayer: tailsPlayer,
            encryptedHasHeadWon: headsOrTailsResult,
            winner: address(0)
        });

        // Instead of calling the function FHE.requestDecryption, we make the result publicly decryptable directly.
        FHE.makePubliclyDecryptable(headsOrTailsResult);

        // You can catch the event to get the gameId and the encryptedHasHeadWon handle
        // for further decryption requests, or create a view function.
        emit GameCreated(gameId, headsPlayer, tailsPlayer, games[gameId].encryptedHasHeadWon);
    }

    // Getting the handle we need to pass to relayer sdk or relayer to http public decrypt.
    function hasHeadWon(uint256 gameId) public view returns (ebool) {
        return games[gameId].encryptedHasHeadWon;
    }

    // This logic was before called through an oracle worklfow after requestDecryption
    // now used by the enduser directly.
    function checkWinner(
        // pass handles in the right order, or it will fail.
        uint256 gameId,
        // Thoses two next arguments are provided by calling the endpoint of the relayer or the relayer sdk
        // using http public decrypt. There is some changes needed on the relayer sdk returns to fits this
        // new workflow.
        bytes memory clearGameResult,
        // Signatures + extradata relayer endpoint or relayer sdk function.
        bytes memory decryptionProof
    ) public returns (address) {
        require(games[gameId].winner != address(0), "Game winner already revealed");

        // 1. Decode the clear result and determine the winner's address.
        bool decodedClearGameResult = abi.decode(clearGameResult, (bool));
        address winner = decodedClearGameResult ? games[gameId].headsPlayer : games[gameId].tailsPlayer;

        // 2. Store the winner immediately to prevent re-entrancy issues in subsequent logic (if any).
        games[gameId].winner = winner;

        // 3. Verify that 'clearGameResult' is the legitimate decryption of the 'encryptedHasHeadWon'
        //    using the provided 'decryptionProof'.
        //    This implicitly checks that 'encryptedHasHeadWon' is a valid decryption handle.

        // Creating the list of handles.
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(games[gameId].encryptedHasHeadWon);

        FHE.verifySignatures(cts, clearGameResult, decryptionProof);

        return winner;
    }
}
