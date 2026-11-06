// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "../contracts/ProtocolConfig.sol";

/// @notice Leaves a proxy with KMS state and no coprocessor context, as ProtocolConfig v0.14 does.
/// @dev Used to test the upgrade to the version that defines the first coprocessor context.
contract ProtocolConfigV014Example is ProtocolConfig {
    function _initializeCoprocessorContext(Coprocessor[] calldata, uint256) internal virtual override {}
}
