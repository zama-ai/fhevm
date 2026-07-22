// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {IERC1271} from "@openzeppelin/contracts/interfaces/IERC1271.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @notice M-of-N multisig ERC-1271 mock mirroring Safe's STATIC (ECDSA-only)
///         signature encoding: `signature` is a concatenation of 65-byte {r,s,v}
///         owner signatures over the raw hash, sorted STRICTLY ascending by
///         recovered signer address — the ordering doubles as deduplication, so
///         repeating one owner's signature cannot inflate the approval count.
///         Exercises the >65-byte opaque-blob branch of the ERC-1271 fallback:
///         `ecrecover` on the whole blob is impossible (length != 65), so the
///         relayer/KMS Connector must forward the FULL blob to this contract.
///         Unlike a real Safe there is no SafeMessage EIP-712 re-hash (owners
///         sign the raw digest) and no proxy/fallback-handler deployment — a
///         real-Safe interop test would add those plus the SafeMessage wrapping
///         in the signing helper, but would exercise the identical verifier path.
contract ERC1271MultisigWallet is IERC1271, E2ECoprocessorConfig {
    bytes4 private constant MAGIC_VALUE = 0x1626ba7e;
    bytes4 private constant INVALID = 0xffffffff;
    uint256 private constant SIG_PART_LENGTH = 65;

    uint256 public immutable threshold;
    mapping(address => bool) public isOwner;
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

    /// @inheritdoc IERC1271
    /// @dev At least `threshold` parts, and the blob must be an exact multiple of
    ///      65 bytes — the alignment requirement is a MOCK-ONLY simplification of
    ///      the ECDSA-static case (real Safe only requires length >= threshold*65
    ///      and ignores trailing bytes of any length). As in Safe, only the FIRST
    ///      `threshold` parts are validated; further parts are ignored. Every
    ///      failure returns a non-magic value rather than reverting — the revert
    ///      rejection branch is owned by `ERC1271RejectWallet`.
    function isValidSignature(bytes32 hash, bytes calldata signature) external view override returns (bytes4) {
        // Covers the empty blob (0 parts < threshold) and misaligned garbage.
        if (signature.length % SIG_PART_LENGTH != 0 || signature.length / SIG_PART_LENGTH < threshold) {
            return INVALID;
        }
        address lastOwner = address(0);
        for (uint256 i = 0; i < threshold; i++) {
            uint256 offset = i * SIG_PART_LENGTH;
            bytes32 r = bytes32(signature[offset:offset + 32]);
            bytes32 s = bytes32(signature[offset + 32:offset + 64]);
            uint8 v = uint8(signature[offset + 64]);
            (address recovered, ECDSA.RecoverError err, ) = ECDSA.tryRecover(hash, v, r, s);
            // Strictly ascending: `==` rejects a duplicated owner part, `<`
            // rejects mis-ordered parts (Safe's canonical-ordering rule).
            if (err != ECDSA.RecoverError.NoError || recovered <= lastOwner || !isOwner[recovered]) {
                return INVALID;
            }
            lastOwner = recovered;
        }
        return MAGIC_VALUE;
    }
}
