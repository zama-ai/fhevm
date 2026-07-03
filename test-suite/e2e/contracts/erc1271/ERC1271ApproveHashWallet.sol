// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {IERC1271} from "@openzeppelin/contracts/interfaces/IERC1271.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @dev Decryption-signature-invalidation surface on the host-chain ACL.
interface IACLDecryptionSignatureInvalidation {
    function invalidateDecryptionSignaturesBefore(uint256 timestamp) external;
}

/// @notice ERC-1271 smart-account mock following Safe's `approveHash` /
///         `signedMessages` pattern: a request carries an EMPTY signature and is
///         considered valid iff the message hash was previously approved on-chain.
///         Exercises the ERC-1271 empty-signature path (the Connector calls
///         `isValidSignature(digest, "")`).
contract ERC1271ApproveHashWallet is IERC1271, E2ECoprocessorConfig {
    bytes4 private constant MAGIC_VALUE = 0x1626ba7e;
    bytes4 private constant INVALID = 0xffffffff;

    address public owner;
    euint64 public value;
    mapping(bytes32 => bool) public approvedHashes;

    constructor(address _owner) {
        require(_owner != address(0), "owner cannot be zero address");
        owner = _owner;
    }

    /// @notice Store a trivially-encrypted value and authorize THIS wallet (the
    ///         `userAddress` in the request) to decrypt it.
    function initValue(uint64 v) external {
        value = FHE.asEuint64(v);
        FHE.allowThis(value);
    }

    /// @notice Pre-approve a message hash (the EIP-712 digest of the request).
    function approveHash(bytes32 hash) external {
        require(msg.sender == owner, "only owner");
        approvedHashes[hash] = true;
    }

    /// @notice Signer-rotation hygiene: invalidate ALL decryption signatures
    ///         issued for this wallet before now (`0` resolves to
    ///         `block.timestamp` inside the ACL). The invalidation mapping is
    ///         keyed by `msg.sender`, so the wallet itself must send the call —
    ///         mirroring the recommended practice of calling
    ///         `invalidateDecryptionSignaturesBefore(0)` on every signer rotation.
    function invalidateDecryptionSignatures(address acl) external {
        require(msg.sender == owner, "only owner");
        IACLDecryptionSignatureInvalidation(acl).invalidateDecryptionSignaturesBefore(0);
    }

    /// @inheritdoc IERC1271
    /// @dev Safe-style: an empty signature is valid iff the hash was pre-approved.
    function isValidSignature(bytes32 hash, bytes calldata signature) external view override returns (bytes4) {
        if (signature.length == 0 && approvedHashes[hash]) {
            return MAGIC_VALUE;
        }
        return INVALID;
    }
}
