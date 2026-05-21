// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, externalEuint64, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "./E2ECoprocessorConfigLocal.sol";

/// @notice Minimal app contract used by the wildcard-delegation e2e tests to
/// seed an encrypted handle whose ACL access belongs to a delegator. Lighter
/// than redeploying EncryptedERC20 to exercise a second app contract address.
contract WildcardDelegationTarget is E2ECoprocessorConfig {
    mapping(address => euint64) private _values;

    /// @notice Store an encrypted input under `delegator`, granting ACL access
    /// to this contract and to `delegator`.
    function deposit(address delegator, externalEuint64 inputHandle, bytes calldata inputProof) external {
        euint64 v = FHE.fromExternal(inputHandle, inputProof);
        FHE.allowThis(v);
        FHE.allow(v, delegator);
        _values[delegator] = v;
    }

    /// @notice Returns the handle stored for `delegator` (zero handle if unset).
    function euint64Of(address delegator) external view returns (bytes32) {
        return euint64.unwrap(_values[delegator]);
    }
}
