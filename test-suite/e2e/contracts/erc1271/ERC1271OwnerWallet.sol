// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {IERC1271} from "@openzeppelin/contracts/interfaces/IERC1271.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @notice Minimal ERC-1271 smart-account mock that validates a single owner's
///         ECDSA signature — the canonical single-signer wallet (Safe/Argent-style
///         single owner). Exercises the `ecrecover`-inside-`isValidSignature`
///         branch of the ERC-1271 fallback: the KMS Connector / relayer recover the
///         signer inside this contract and compare it to the configured owner.
contract ERC1271OwnerWallet is IERC1271, E2ECoprocessorConfig {
    bytes4 private constant MAGIC_VALUE = 0x1626ba7e;
    bytes4 private constant INVALID = 0xffffffff;

    address public owner;
    euint64 public value;

    constructor(address _owner) {
        require(_owner != address(0), "owner cannot be zero address");
        owner = _owner;
    }

    /// @notice Store a trivially-encrypted value and authorize THIS wallet (the
    ///         `userAddress` in an ERC-1271 request) to decrypt it.
    function initValue(uint64 v) external {
        value = FHE.asEuint64(v);
        FHE.allowThis(value); // isAllowed(value, address(this)) == true
    }

    /// @inheritdoc IERC1271
    /// @dev Returns the ERC-1271 magic value iff `signature` is a valid ECDSA
    ///      signature over `hash` produced by `owner`; otherwise a non-magic value.
    function isValidSignature(bytes32 hash, bytes calldata signature) external view override returns (bytes4) {
        (address recovered, ECDSA.RecoverError err, ) = ECDSA.tryRecover(hash, signature);
        if (err == ECDSA.RecoverError.NoError && recovered == owner) {
            return MAGIC_VALUE;
        }
        return INVALID;
    }
}
