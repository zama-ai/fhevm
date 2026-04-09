// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { IERC7913SignatureVerifier } from "@openzeppelin/contracts/interfaces/IERC7913.sol";
import { Ed25519 } from "./Ed25519.sol";

/**
 * @title SolanaEd25519Verifier
 * @notice ERC-7913 verifier for native Solana Ed25519 public keys.
 * @dev The signer format is `verifier || pubkey`, where `pubkey` is 32 bytes and `signature` is 64 bytes (`R || S`).
 *      The signed message is the 32-byte gateway decryption request digest.
 */
contract SolanaEd25519Verifier is IERC7913SignatureVerifier {
    bytes4 internal constant INVALID_SIGNATURE = 0xffffffff;

    /// @inheritdoc IERC7913SignatureVerifier
    function verify(bytes calldata key, bytes32 hash, bytes calldata signature) external pure returns (bytes4) {
        if (key.length != 32 || signature.length != 64) {
            return INVALID_SIGNATURE;
        }

        bytes32 publicKey = bytes32(key);
        bytes32 r = bytes32(signature[:32]);
        bytes32 s = bytes32(signature[32:64]);

        if (!Ed25519.verify(publicKey, r, s, abi.encodePacked(hash))) {
            return INVALID_SIGNATURE;
        }

        return IERC7913SignatureVerifier.verify.selector;
    }
}
