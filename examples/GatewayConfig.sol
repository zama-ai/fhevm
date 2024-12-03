// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "fhevm-core-contracts/addresses/GatewayContractAddress.sol";

/**
 * @title   FHEVMConfig
 * @notice  This library returns all addresses for the ACL, TFHEExecutor, FHEPayment,
 *          and KMSVerifier contracts.
 */
library GatewayConfig {
    /**
     * @notice This function returns a struct containing all contract addresses.
     * @dev    It returns an immutable struct.
     */
    function defaultGatewayContract() internal pure returns (address) {
        return GATEWAY_CONTRACT_PREDEPLOY_ADDRESS;
    }
}
