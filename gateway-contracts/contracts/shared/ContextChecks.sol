// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { coprocessorContextsAddress } from "../../addresses/GatewayAddresses.sol";
import { ICoprocessorContexts } from "../interfaces/ICoprocessorContexts.sol";

/**
 * @title Context Checks
 * @notice Base contract that provides modifiers associated with context checks
 */
abstract contract ContextChecks {
    /**
     * @notice The address of the CoprocessorContexts contract
     */
    ICoprocessorContexts private constant COPROCESSOR_CONTEXTS = ICoprocessorContexts(coprocessorContextsAddress);

    /**
     * @notice Error indicating that an address is not a coprocessor transaction sender from a context.
     * @param contextId The coprocessor context ID.
     * @param txSenderAddress The address to check.
     */
    error NotCoprocessorTxSenderFromContext(uint256 contextId, address txSenderAddress);

    /**
     * @notice Error indicating that an address is not a coprocessor signer from a context.
     * @param contextId The coprocessor context ID.
     * @param signerAddress The address to check.
     */
    error NotCoprocessorSignerFromContext(uint256 contextId, address signerAddress);

    /**
     * @notice Refresh the coprocessor context statuses.
     */
    modifier refreshCoprocessorContextStatuses() {
        COPROCESSOR_CONTEXTS.refreshCoprocessorContextStatuses();
        _;
    }

    /**
     * @notice Checks if the address is a coprocessor transaction sender.
     * @param txSenderAddress The address to check.
     */
    function _checkIsCoprocessorTxSender(uint256 contextId, address txSenderAddress) internal view {
        if (!COPROCESSOR_CONTEXTS.isCoprocessorTxSender(contextId, txSenderAddress)) {
            revert NotCoprocessorTxSenderFromContext(contextId, txSenderAddress);
        }
    }

    /**
     * @notice Checks if the address is a coprocessor signer.
     * @param signerAddress The address to check.
     */
    function _checkIsCoprocessorSigner(uint256 contextId, address signerAddress) internal view {
        if (!COPROCESSOR_CONTEXTS.isCoprocessorSigner(contextId, signerAddress)) {
            revert NotCoprocessorSignerFromContext(contextId, signerAddress);
        }
    }
}
