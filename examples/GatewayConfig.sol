// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "fhevm-core-contracts/addresses/GatewayContractAddress.sol";

/**
 * @title   FHEVMConfig
 * @notice  This library returns the GatewayContract address
 */
library GatewayConfig {
    /**
     * @notice This function returns a the gateway contract address.
     */
    function defaultGatewayContract() internal pure returns (address) {
        return GATEWAY_CONTRACT_PREDEPLOY_ADDRESS;
    }
}
