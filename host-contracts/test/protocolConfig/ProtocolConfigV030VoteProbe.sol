// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";

/// @dev Test-only implementation that reads the v0.3.0 epoch activation vote from base storage.
contract ProtocolConfigV030VoteProbe is ProtocolConfig {
    function v030Vote(uint256 epochId, address signer) external view returns (bool) {
        return _getProtocolConfigStorage().epochActivationConfirmedBySigner[epochId][signer];
    }
}
