// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import '../GatewayConfig.sol';
import '../shared/Structs.sol';

/**
 * @notice Reproduces the deployed v0.7.0 state, where the priority coprocessor feature was live and
 *         its transaction sender could be recorded in what is now a deprecated storage slot.
 * @dev Used to verify that upgrading to the current GatewayConfig clears that slot.
 */
contract GatewayConfigV8Example is GatewayConfig {
    string private constant CONTRACT_NAME = 'GatewayConfig';

    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 7;
    uint256 private constant PATCH_VERSION = 0;

    function initializeV8ForTest(
        address initialOwner,
        Coprocessor[] calldata initialCoprocessors,
        uint256 initialCoprocessorThreshold,
        address priorityCoprocessorTxSender
    ) public virtual reinitializer(9) {
        __Ownable_init(initialOwner);
        _setCoprocessors(initialCoprocessors, initialCoprocessorThreshold);

        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        $.deprecatedPriorityCoprocessorTxSender = priorityCoprocessorTxSender;
    }

    function getDeprecatedPriorityCoprocessorTxSenderForTest() external view virtual returns (address) {
        GatewayConfigStorage storage $ = _getGatewayConfigStorage();
        return $.deprecatedPriorityCoprocessorTxSender;
    }

    function getVersion() external pure virtual override returns (string memory) {
        return
            string(
                abi.encodePacked(
                    CONTRACT_NAME,
                    ' v',
                    Strings.toString(MAJOR_VERSION),
                    '.',
                    Strings.toString(MINOR_VERSION),
                    '.',
                    Strings.toString(PATCH_VERSION)
                )
            );
    }
}
