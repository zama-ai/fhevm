// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { ciphertextCommitsAddress } from "../../addresses/GatewayAddresses.sol";
import { ICiphertextCommits } from "../interfaces/ICiphertextCommits.sol";
import { SnsCiphertextMaterial } from "./Structs.sol";

/**
 * @title CiphertextCommits utils
 * @dev Contract that provides utilities on top of the CiphertextCommits contract
 */
abstract contract CiphertextCommitsUtils {
    /**
     * @notice The address of the CiphertextCommits contract.
     */
    ICiphertextCommits internal constant CIPHERTEXT_COMMITS = ICiphertextCommits(ciphertextCommitsAddress);

    /**
     * @notice Error indicating that the ciphertext material has not been added to the contract.
     * @param ctHandle The handle representing the ciphertext material.
     */
    error CiphertextMaterialNotAdded(bytes32 ctHandle);

    function _isCiphertextMaterialAdded(bytes32 ctHandle) internal view returns (bool) {
        return CIPHERTEXT_COMMITS.isCiphertextMaterialAdded(ctHandle);
    }

    /**
     * @notice Checks if the ciphertext material has been added to the contract.
     * @param ctHandle The ciphertext handle to check.
     */
    function _checkIsCiphertextMaterialAdded(bytes32 ctHandle) internal view {
        if (!CIPHERTEXT_COMMITS.isCiphertextMaterialAdded(ctHandle)) {
            revert CiphertextMaterialNotAdded(ctHandle);
        }
    }
}
