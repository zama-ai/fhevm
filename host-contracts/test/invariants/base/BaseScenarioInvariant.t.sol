// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {StdInvariant} from "forge-std/StdInvariant.sol";
import {HostContractsDeployerTestUtils} from "../../../fhevm-foundry/HostContractsDeployerTestUtils.sol";

abstract contract BaseScenarioInvariant is StdInvariant, HostContractsDeployerTestUtils {
    function _targetInvariant(address handler, bytes4[] memory selectors, address[] memory senders) internal {
        targetContract(handler);
        for (uint256 i = 0; i < senders.length; i++) {
            targetSender(senders[i]);
        }
        targetSelector(FuzzSelector({addr: handler, selectors: selectors}));
    }

    function _contains(address[] memory values, address candidate) internal pure returns (bool) {
        for (uint256 i = 0; i < values.length; i++) {
            if (values[i] == candidate) {
                return true;
            }
        }
        return false;
    }
}
