// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {IERC1271} from "@openzeppelin/contracts/interfaces/IERC1271.sol";
import {E2ECoprocessorConfig} from "../E2ECoprocessorConfigLocal.sol";

/// @notice ERC-1271 mock that NEVER validates a signature, with a configurable
///         rejection mode covering each ERC-1271 verification rejection branch:
///         - `WrongMagic`: returns a non-magic `bytes4` (well-formed reject);
///         - `Revert`: `isValidSignature` reverts;
///         - `ShortReturndata`: returns fewer than 32 bytes, like a
///           non-compliant fallback function or a proxy without
///           `isValidSignature` (`bytes4` is ABI-encoded as a full word, so the
///           verifier must treat short returns as rejection).
///         In every mode the relayer's synchronous pre-check must reject the
///         request (`400 invalid_signature`).
contract ERC1271RejectWallet is IERC1271, E2ECoprocessorConfig {
    bytes4 private constant INVALID = 0xffffffff;

    enum RejectMode {
        WrongMagic,
        Revert,
        ShortReturndata
    }

    RejectMode public mode; // defaults to WrongMagic
    euint64 public value;

    /// @notice Store a trivially-encrypted value and authorize THIS wallet so the
    ///         request is well-formed and the ONLY failing check is the signature.
    function initValue(uint64 v) external {
        value = FHE.asEuint64(v);
        FHE.allowThis(value);
    }

    /// @notice Select which ERC-1271 rejection branch to exercise.
    function setMode(RejectMode m) external {
        mode = m;
    }

    /// @inheritdoc IERC1271
    /// @dev Rejects according to the configured mode; never returns the magic value.
    function isValidSignature(bytes32, bytes calldata) external view override returns (bytes4) {
        if (mode == RejectMode.Revert) {
            revert("ERC1271RejectWallet: rejected");
        }
        if (mode == RejectMode.ShortReturndata) {
            // Return 4 raw bytes instead of an ABI-encoded 32-byte word.
            assembly {
                mstore(0, 0)
                return(0, 4)
            }
        }
        return INVALID;
    }
}
