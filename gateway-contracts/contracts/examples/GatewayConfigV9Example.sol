// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import '../GatewayConfig.sol';

/**
 * @notice Current GatewayConfig plus a reader for the deprecated priority coprocessor slot.
 * @dev The production contract deliberately exposes no getter for that slot. This example is the
 *      upgrade target in the tests that assert `reinitializeV9` zeroes it.
 */
contract GatewayConfigV9Example is GatewayConfig {
    function getDeprecatedPriorityCoprocessorTxSenderForTest() external view virtual returns (address) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.deprecatedPriorityCoprocessorTxSender;
    }
}
