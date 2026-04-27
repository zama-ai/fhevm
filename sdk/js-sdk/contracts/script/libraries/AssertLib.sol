// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {VmSafe} from "forge-std/Vm.sol";

library AssertLib {
    function boxMessage(string memory message) internal pure returns (string memory) {
        return string.concat(
            "\n",
            "================================================================================\n",
            message,
            "\n",
            "================================================================================\n",
            "\n"
        );
    }

    function assertEnvExists(VmSafe vm, string memory envName) internal {
        require(vm.envExists(envName), boxMessage(string.concat("Env var must be set:\n", "  - ", envName)));
    }
}
