// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, ebool, euint8} from "@fhevm/solidity/lib/FHE.sol";

/**
 * @title       EncryptedErrors.
 * @notice      This abstract contract is used for error handling in the fhEVM.
 *              Error codes are encrypted in the constructor inside the `_errorCodeDefinitions` mapping.
 * @dev         `_errorCodeDefinitions[0]` should always refer to the `NO_ERROR` code, by default.
 */
abstract contract EncryptedErrors {
    /// @notice Returned if the error index is invalid.
    error ErrorIndexInvalid();

    /// @notice Returned if the error index is null.
    error ErrorIndexIsNull();

    /// @notice Returned if the total number of errors is equal to zero.
    error TotalNumberErrorCodesEqualToZero();

    /// @notice Total number of error codes.
    /// @dev    Should hold the constant size of the `_errorCodeDefinitions` mapping.
    uint8 private immutable _TOTAL_NUMBER_ERROR_CODES;

    /// @notice Used to keep track of number of emitted errors.
    /// @dev Should hold the size of the _errorCodesEmitted mapping.
    uint256 private _errorCounter;

    /// @notice Mapping of trivially encrypted error codes definitions.
    /// @dev In storage because solc does not support immutable mapping, neither immutable arrays, yet.
    mapping(uint8 errorCode => euint8 encryptedErrorCode) private _errorCodeDefinitions;

    /// @notice Mapping of encrypted error codes emitted.
    mapping(uint256 errorIndex => euint8 encryptedErrorCode) private _errorCodesEmitted;

    /**
     * @notice                       Sets the non-null value for `_TOTAL_NUMBER_ERROR_CODES`
     *                               corresponding to the total number of errors.
     * @param totalNumberErrorCodes_ Total number of different errors.
     * @dev                          `totalNumberErrorCodes_` must be non-null
     *                               (`_errorCodeDefinitions[0]` corresponds to the `NO_ERROR` code).
     */
    constructor(uint8 totalNumberErrorCodes_) {
        if (totalNumberErrorCodes_ == 0) {
            revert TotalNumberErrorCodesEqualToZero();
        }

        for (uint8 i; i <= totalNumberErrorCodes_; i++) {
            euint8 errorCode = FHE.asEuint8(i);
            _errorCodeDefinitions[i] = errorCode;
            FHE.allowThis(errorCode);
        }

        _TOTAL_NUMBER_ERROR_CODES = totalNumberErrorCodes_;
    }

    /**
     * @notice                  Computes an encrypted error code, result will be either a reencryption of
     *                          `_errorCodeDefinitions[indexCode]` if `condition` is an encrypted `true`
     *                          or of `errorCode` otherwise.
     * @param condition         Encrypted boolean used in the select operator.
     * @param errorCode         Selected error code if `condition` encrypts `true`.
     * @return newErrorCode     New reencrypted error code depending on `condition` value.
     * @dev                 `   indexCode` must be below the total number of error codes.
     */
    function _errorChangeIf(ebool condition, uint8 indexCode, euint8 errorCode)
        internal
        virtual
        returns (euint8 newErrorCode)
    {
        if (indexCode > _TOTAL_NUMBER_ERROR_CODES) {
            revert ErrorIndexInvalid();
        }

        newErrorCode = FHE.select(condition, _errorCodeDefinitions[indexCode], errorCode);
    }

    /**
     * @notice                  Does the opposite of `changeErrorIf`, i.e result will be either a reencryption of
     *                          `_errorCodeDefinitions[indexCode]` if `condition` is an encrypted `false`
     *                          or of `errorCode` otherwise.
     * @param condition         The encrypted boolean used in the `FHE.select`.
     * @param errorCode         The selected error code if `condition` encrypts `false`.
     * @return newErrorCode     New error code depending on `condition` value.
     * @dev                     `indexCode` must be below the total number of error codes.
     */
    function _errorChangeIfNot(ebool condition, uint8 indexCode, euint8 errorCode)
        internal
        virtual
        returns (euint8 newErrorCode)
    {
        if (indexCode > _TOTAL_NUMBER_ERROR_CODES) {
            revert ErrorIndexInvalid();
        }

        newErrorCode = FHE.select(condition, errorCode, _errorCodeDefinitions[indexCode]);
    }

    /**
     * @notice              Computes an encrypted error code, result will be either a reencryption of
     *                      `_errorCodeDefinitions[indexCode]` if `condition` is an encrypted `true`
     *                      or of `NO_ERROR` otherwise.
     * @param condition     Encrypted boolean used in the select operator.
     * @param indexCode     Index of the selected error code if `condition` encrypts `true`.
     * @return errorCode    Reencrypted error code depending on `condition` value.
     * @dev                 `indexCode` must be non-null and below the total number of defined error codes.
     */
    function _errorDefineIf(ebool condition, uint8 indexCode) internal virtual returns (euint8 errorCode) {
        if (indexCode == 0) {
            revert ErrorIndexIsNull();
        }

        if (indexCode > _TOTAL_NUMBER_ERROR_CODES) {
            revert ErrorIndexInvalid();
        }

        errorCode = FHE.select(condition, _errorCodeDefinitions[indexCode], _errorCodeDefinitions[0]);
    }

    /**
     * @notice                Does the opposite of `defineErrorIf`, i.e result will be either a reencryption of
     *                        `_errorCodeDefinitions[indexCode]` if `condition` is an encrypted `false` or
     *                        of `NO_ERROR` otherwise.
     * @param condition       Encrypted boolean used in the select operator.
     * @param indexCode       Index of the selected error code if `condition` encrypts `false`.
     * @return errorCode      Reencrypted error code depending on `condition` value.
     * @dev                   `indexCode` must be non-null and below the total number of defined error codes.
     */
    function _errorDefineIfNot(ebool condition, uint8 indexCode) internal virtual returns (euint8 errorCode) {
        if (indexCode == 0) {
            revert ErrorIndexIsNull();
        }

        if (indexCode > _TOTAL_NUMBER_ERROR_CODES) {
            revert ErrorIndexInvalid();
        }

        errorCode = FHE.select(condition, _errorCodeDefinitions[0], _errorCodeDefinitions[indexCode]);
    }

    /**
     * @notice                  Saves `errorCode` in storage, in the `_errorCodesEmitted` mapping.
     * @param errorCode         Encrypted error code to be saved in storage.
     * @return errorId          The `errorId` key in `_errorCodesEmitted` where `errorCode` is stored.
     */
    function _errorSave(euint8 errorCode) internal virtual returns (uint256 errorId) {
        errorId = _errorCounter;
        _errorCounter++;
        _errorCodesEmitted[errorId] = errorCode;

        FHE.allowThis(errorCode);
    }

    /**
     * @notice                     Returns the trivially encrypted error code at index `indexCodeDefinition`.
     * @param indexCodeDefinition  Index of the requested error code definition.
     * @return errorCode           Encrypted error code located at `indexCodeDefinition` in `_errorCodeDefinitions`.
     */
    function _errorGetCodeDefinition(uint8 indexCodeDefinition) internal view virtual returns (euint8 errorCode) {
        if (indexCodeDefinition >= _TOTAL_NUMBER_ERROR_CODES) {
            revert ErrorIndexInvalid();
        }

        errorCode = _errorCodeDefinitions[indexCodeDefinition];
    }

    /**
     * @notice                  Returns the encrypted error code which was stored in `_errorCodesEmitted`
     *                          at key `errorId`.
     * @param errorId           Requested key stored in the `_errorCodesEmitted` mapping.
     * @return errorCode        Encrypted error code located at the `errorId` key.
     * @dev                     `errorId` must be a valid id, i.e below the error counter.
     */
    function _errorGetCodeEmitted(uint256 errorId) internal view virtual returns (euint8 errorCode) {
        if (errorId >= _errorCounter) {
            revert ErrorIndexInvalid();
        }

        errorCode = _errorCodesEmitted[errorId];
    }

    /**
     * @notice             Returns the total counter of emitted of error codes.
     * @return countErrors Number of errors emitted.
     */
    function _errorGetCounter() internal view virtual returns (uint256 countErrors) {
        countErrors = _errorCounter;
    }

    /**
     * @notice                       Returns the total number of the possible error codes defined.
     * @return totalNumberErrorCodes Total number of the different possible error codes.
     */
    function _errorGetNumCodesDefined() internal view virtual returns (uint8 totalNumberErrorCodes) {
        totalNumberErrorCodes = _TOTAL_NUMBER_ERROR_CODES;
    }
}
