// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {IERC1271} from "@openzeppelin/contracts/interfaces/IERC1271.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @notice M-of-N multisig ERC-1271 mock mirroring Safe's signature encoding:
///         `signature` starts with `threshold` concatenated 65-byte {r,s,v}
///         parts sorted STRICTLY ascending by owner address (the ordering
///         doubles as dedup — repeating one owner cannot inflate the count).
///         Like Safe, the length rule is only `length >= threshold * 65`:
///         trailing bytes of ANY length beyond the static parts are ignored,
///         so valid blobs need not be a multiple of 65 bytes.
///         Supported part types, keyed on Safe's overloaded `v` byte:
///           - v 27/28: ECDSA signature over the raw `hash`;
///           - v > 30:  eth_sign signature — verified over the
///                      "\x19Ethereum Signed Message" wrap of `hash`, v - 4;
///           - v == 1:  pre-approved hash — `r` carries the approving owner's
///                      address; consumed against `approvedHashes` (Safe's
///                      `approveHash` mechanism inside checkSignatures).
///         NOT modeled (a real-Safe interop test is the tracked follow-up):
///           - v == 0 contract-signature parts (nested ERC-1271 with dynamic
///             data appended past the static section);
///           - the SafeMessage EIP-712 re-hash: real Safe owners sign
///             getMessageHash(hash), not `hash` itself. That wrapping is
///             internal to the wallet — the SDK/relayer/KMS pass the digest
///             and blob through opaquely either way — so owners here sign the
///             raw digest to keep the mock self-contained.
///         Exercises the >65-byte opaque-blob branch of the ERC-1271
///         fallback: `ecrecover` on the whole blob is impossible, so every
///         layer must forward the FULL blob to this contract.
contract ERC1271MultisigWallet is IERC1271, E2ECoprocessorConfig {
    bytes4 private constant MAGIC_VALUE = 0x1626ba7e;
    bytes4 private constant INVALID = 0xffffffff;
    uint256 private constant SIG_PART_LENGTH = 65;

    uint256 public immutable threshold;
    mapping(address => bool) public isOwner;
    mapping(address => mapping(bytes32 => bool)) public approvedHashes;
    euint64 public value;

    constructor(address[] memory _owners, uint256 _threshold) {
        require(_threshold >= 1 && _threshold <= _owners.length, "invalid threshold");
        for (uint256 i = 0; i < _owners.length; i++) {
            require(_owners[i] != address(0), "owner cannot be zero address");
            require(!isOwner[_owners[i]], "duplicate owner");
            isOwner[_owners[i]] = true;
        }
        threshold = _threshold;
    }

    /// @notice Store a trivially-encrypted value and authorize THIS wallet (the
    ///         `userAddress` in a multisig ERC-1271 request) to decrypt it.
    function initValue(uint64 v) external {
        value = FHE.asEuint64(v);
        FHE.allowThis(value); // isAllowed(value, address(this)) == true
    }

    /// @notice Safe's `approveHash` analogue: an owner pre-approves a hash
    ///         on-chain, later consumed by a v=1 part inside `isValidSignature`.
    function approveHash(bytes32 hash) external {
        require(isOwner[msg.sender], "only owner");
        approvedHashes[msg.sender][hash] = true;
    }

    /// @inheritdoc IERC1271
    /// @dev Validates the FIRST `threshold` 65-byte parts; anything after them
    ///      is ignored (Safe semantics — real Safe appends dynamic data there
    ///      for v=0 parts). Every failure returns a non-magic value rather
    ///      than reverting — the revert rejection branch is owned by
    ///      `ERC1271RejectWallet`.
    function isValidSignature(bytes32 hash, bytes calldata signature) external view override returns (bytes4) {
        // Safe's length rule (GS020 analogue): enough static parts must exist.
        // Covers the empty blob and any blob shorter than `threshold` parts.
        if (signature.length < threshold * SIG_PART_LENGTH) {
            return INVALID;
        }
        address lastOwner = address(0);
        for (uint256 i = 0; i < threshold; i++) {
            uint256 offset = i * SIG_PART_LENGTH;
            bytes32 r = bytes32(signature[offset:offset + 32]);
            bytes32 s = bytes32(signature[offset + 32:offset + 64]);
            uint8 v = uint8(signature[offset + 64]);
            address currentOwner;
            if (v == 1) {
                // Pre-approved hash: r carries the approving owner's address.
                currentOwner = address(uint160(uint256(r)));
                if (!approvedHashes[currentOwner][hash]) {
                    return INVALID;
                }
            } else if (v > 30) {
                // eth_sign part: v is shifted by 4, signature is over the
                // eth_sign wrap of the hash.
                (address recovered, ECDSA.RecoverError err, ) = ECDSA.tryRecover(
                    MessageHashUtils.toEthSignedMessageHash(hash),
                    v - 4,
                    r,
                    s
                );
                if (err != ECDSA.RecoverError.NoError) {
                    return INVALID;
                }
                currentOwner = recovered;
            } else {
                // Plain ECDSA part over the raw hash.
                (address recovered, ECDSA.RecoverError err, ) = ECDSA.tryRecover(hash, v, r, s);
                if (err != ECDSA.RecoverError.NoError) {
                    return INVALID;
                }
                currentOwner = recovered;
            }
            // Strictly ascending: `==` rejects a duplicated owner part, `<`
            // rejects out-of-order parts (Safe's canonical-ordering rule).
            if (currentOwner <= lastOwner || !isOwner[currentOwner]) {
                return INVALID;
            }
            lastOwner = currentOwner;
        }
        return MAGIC_VALUE;
    }
}
