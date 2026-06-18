// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../GatewayConfig.sol";
import "../shared/Structs.sol";

contract GatewayConfigV7Example is GatewayConfig {
    string private constant CONTRACT_NAME = "GatewayConfig";

    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 6;
    uint256 private constant PATCH_VERSION = 0;

    function initializeV7ForTest(
        address initialOwner,
        Coprocessor[] calldata initialCoprocessors,
        uint256 initialCoprocessorThreshold
    ) public virtual reinitializer(8) {
        __Ownable_init(initialOwner);
        _setCoprocessors(initialCoprocessors, initialCoprocessorThreshold);
    }

    function getVersion() external pure virtual override returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    " v",
                    Strings.toString(MAJOR_VERSION),
                    ".",
                    Strings.toString(MINOR_VERSION),
                    ".",
                    Strings.toString(PATCH_VERSION)
                )
            );
    }
}
