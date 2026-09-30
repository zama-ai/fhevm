// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, ebool, euint8, externalEbool, externalEuint8} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {EncryptedErrors} from "../../utils/EncryptedErrors.sol";

contract TestEncryptedErrors is ZamaEthereumConfig, EncryptedErrors {
    constructor(uint8 totalNumberErrorCodes_) EncryptedErrors(totalNumberErrorCodes_) {
        for (uint8 i; i <= totalNumberErrorCodes_; i++) {
            /// @dev It is not possible to access the _errorCodeDefinitions since it is private.
            FHE.allow(FHE.asEuint8(i), msg.sender);
        }
    }

    function errorChangeIf(
        externalEbool encryptedCondition,
        externalEuint8 encryptedErrorCode,
        bytes calldata inputProof,
        uint8 indexCode
    ) external returns (euint8 newErrorCode) {
        ebool condition = FHE.fromExternal(encryptedCondition, inputProof);
        euint8 errorCode = FHE.fromExternal(encryptedErrorCode, inputProof);
        newErrorCode = _errorChangeIf(condition, indexCode, errorCode);
        _errorSave(newErrorCode);
        FHE.allow(newErrorCode, msg.sender);
    }

    function errorChangeIfNot(
        externalEbool encryptedCondition,
        externalEuint8 encryptedErrorCode,
        bytes calldata inputProof,
        uint8 indexCode
    ) external returns (euint8 newErrorCode) {
        ebool condition = FHE.fromExternal(encryptedCondition, inputProof);
        euint8 errorCode = FHE.fromExternal(encryptedErrorCode, inputProof);
        newErrorCode = _errorChangeIfNot(condition, indexCode, errorCode);
        _errorSave(newErrorCode);
        FHE.allow(newErrorCode, msg.sender);
    }

    function errorDefineIf(externalEbool encryptedCondition, bytes calldata inputProof, uint8 indexCode)
        external
        returns (euint8 errorCode)
    {
        ebool condition = FHE.fromExternal(encryptedCondition, inputProof);
        errorCode = _errorDefineIf(condition, indexCode);
        _errorSave(errorCode);
        FHE.allow(errorCode, msg.sender);
    }

    function errorDefineIfNot(externalEbool encryptedCondition, bytes calldata inputProof, uint8 indexCode)
        external
        returns (euint8 errorCode)
    {
        ebool condition = FHE.fromExternal(encryptedCondition, inputProof);
        errorCode = _errorDefineIfNot(condition, indexCode);
        _errorSave(errorCode);
        FHE.allow(errorCode, msg.sender);
    }

    function errorGetCodeDefinition(uint8 indexCodeDefinition) external view returns (euint8 errorCode) {
        errorCode = _errorGetCodeDefinition(indexCodeDefinition);
    }

    function errorGetCodeEmitted(uint256 errorId) external view returns (euint8 errorCode) {
        errorCode = _errorGetCodeEmitted(errorId);
    }

    function errorGetCounter() external view returns (uint256 countErrors) {
        countErrors = _errorGetCounter();
    }

    function errorGetNumCodesDefined() external view returns (uint8 totalNumberErrorCodes) {
        totalNumberErrorCodes = _errorGetNumCodesDefined();
    }
}
