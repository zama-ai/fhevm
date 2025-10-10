// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { ciphertextCommitsAddress } from "../../addresses/GatewayAddresses.sol";
import { ICiphertextCommits } from "../interfaces/ICiphertextCommits.sol";

/**
 * @title CiphertextCommits Checks
 * @dev Contract that provides checks on top of the CiphertextCommits contract
 */
abstract contract CiphertextCommitsChecks {
    /**
     * @notice The address of the CiphertextCommits contract.
     */
    ICiphertextCommits private constant CIPHERTEXT_COMMITS = ICiphertextCommits(ciphertextCommitsAddress);

    /**
     * @notice Error indicating that the ciphertext material has not been added to the contract.
     * @param ctHandle The handle representing the ciphertext material.
     */
    error CiphertextMaterialNotAdded(bytes32 ctHandle);

    /**
     * @notice Error indicating that the handles were generated with different keyIds.
     * @param ctHandles The list of ciphertext handles to check
     * @dev TODO: This can be removed once batched decryption requests with different keys is
     * supported by the KMS (see https://github.com/zama-ai/fhevm-internal/issues/376)
     */
    error DifferentKeyIdsNotAllowed(bytes32[] ctHandles);

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

    /**
     * @notice Checks if the handles were generated with the same keyId.
     * @param ctHandles The list of ciphertext handles to check.
     * @dev TODO: This can be removed once batched decryption requests with different keys is
     * supported by the KMS (see https://github.com/zama-ai/fhevm-internal/issues/376)
     */
    function _checkIsSameKeyId(bytes32[] memory ctHandles) internal view {
        if (!CIPHERTEXT_COMMITS.isSameKeyId(ctHandles)) {
            revert DifferentKeyIdsNotAllowed(ctHandles);
        }
    }
}
