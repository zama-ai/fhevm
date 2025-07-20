// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {fhevmExecutorAdd} from "../addresses/FHEVMHostAddresses.sol";

import {FheType} from "./shared/FheType.sol";

/**
 * @title HCULimit
 * @notice This contract manages the total allowed complexity for FHE operations at the
 * transaction level, including the maximum number of homomorphic complexity units (HCU) per transaction.
 * @dev The contract is designed to be used with the FHEVMExecutor contract.
 */
contract HCULimit is UUPSUpgradeableEmptyProxy, Ownable2StepUpgradeable {
    /// @custom:storage-location erc7201:fhevm.storage.HCULimit
    struct HCULimitStorage {
        mapping(bytes32 => uint256) hcuValues;
    }

    /// @notice Returned if the sender is not the FHEVMExecutor.
    error CallerMustBeFHEVMExecutorContract();

    /// @notice Returned if the transaction exceeds the maximum allowed homomorphic complexity units.
    error HCUTransactionLimitExceeded();

    /// @notice Returned if the transaction exceeds the maximum allowed depth of homomorphic complexity units.
    error HCUTransactionDepthLimitExceeded();

    /// @notice Returned if the operation is not supported.
    error UnsupportedOperation();

    /// @notice Returned if the operation is not scalar.
    error OnlyScalarOperationsAreSupported();

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "HCULimit";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 2;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @notice FHEVMExecutor address.
    address private constant fhevmExecutorAddress = fhevmExecutorAdd;

    /// @notice Maximum homomorphic complexity units depth per block.
    /// @dev This is the maximum number of homomorphic complexity units that can be sequential.
    uint256 private constant MAX_HOMOMORPHIC_COMPUTE_UNITS_DEPTH_PER_TX = 5_000_000;

    /// @notice Maximum homomorphic complexity units per transaction.
    /// @dev This is the maximum number of homomorphic complexity units that can be used in a single transaction.
    uint256 private constant MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX = 20_000_000;

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 3;

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.HCULimit")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant HCULimitStorageLocation =
        0xc13af6c514bff8997f30c90003baa82bd02aad978179d1ce58d85c4319ad6500;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __Ownable_init(owner());
    }

    /**
     * @notice Re-initializes the contract from V1.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice Check the homomorphic complexity units limit for FheAdd.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheAdd(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheAdd_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheAdd", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheSub.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheSub(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheSub_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheSub", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheMul.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheMul(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheMul_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheMul", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheDiv.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheDiv(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 /*rhs*/,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        if (scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU = $.hcuValues[_getHcuKey("fheDiv_scalar", resultType)];
        _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheRem.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheRem(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 /*rhs*/,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        if (scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU = $.hcuValues[_getHcuKey("fheRem_scalar", resultType)];
        _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheBitAnd.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheBitAnd(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheBitAnd_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheBitAnd", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheBitOr.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheBitOr(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheBitOr_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheBitOr", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheBitXor.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheBitXor(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheBitXor_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheBitXor", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheShl.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheShl(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheShl_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheShl", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheShr.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheShr(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheShr_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheShr", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheRotl.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheRotl(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheRotl_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheRotl", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheRotr.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheRotr(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheRotr_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheRotr", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheEq.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheEq(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheEq_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheEq", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheNe.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheNe(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheNe_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheNe", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheGe.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheGe(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheGe_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheGe", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheGt.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheGt(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheGt_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheGt", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheLe.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheLe(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheLe_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheLe", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheLt.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheLt(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheLt_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheLt", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheMin.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheMin(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheMin_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheMin", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheMax.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheMax(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            opHCU = $.hcuValues[_getHcuKey("fheMax_scalar", resultType)];
            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            opHCU = $.hcuValues[_getHcuKey("fheMax", resultType)];
            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheNeg.
     * @param ct The only operand.
     * @param result Result.
     */
    function checkHCUForFheNeg(FheType resultType, bytes32 ct, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU = $.hcuValues[_getHcuKey("fheNeg", resultType)];
        _adjustAndCheckFheTransactionLimitOneOp(opHCU, ct, result);
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheNot.
     * @param ct The only operand.
     * @param result Result.
     */
    function checkHCUForFheNot(FheType resultType, bytes32 ct, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU = $.hcuValues[_getHcuKey("fheNot", resultType)];
        _adjustAndCheckFheTransactionLimitOneOp(opHCU, ct, result);
    }
    /**
     * @notice Check the homomorphic complexity units limit for Cast.
     * @param ct The only operand.
     * @param result Result.
     */
    function checkHCUForCast(FheType resultType, bytes32 ct, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU = $.hcuValues[_getHcuKey("cast", resultType)];
        _adjustAndCheckFheTransactionLimitOneOp(opHCU, ct, result);
    }
    /**
     * @notice Check the homomorphic complexity units limit for TrivialEncrypt.
     * @param resultType Result type.
     * @param result Result.
     */
    function checkHCUForTrivialEncrypt(FheType resultType, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU = $.hcuValues[_getHcuKey("trivialEncrypt", resultType)];
        _updateAndVerifyHCUTransactionLimit(opHCU);
        _setHCUForHandle(result, opHCU);
    }
    /**
     * @notice Check the homomorphic complexity units limit for IfThenElse.
     * @param resultType Result type.
     * @param lhs The left-hand side operand.
     * @param middle The middle operand.
     * @param rhs The right-hand side operand.
     */
    function checkHCUForIfThenElse(
        FheType resultType,
        bytes32 lhs,
        bytes32 middle,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU = $.hcuValues[_getHcuKey("ifThenElse", resultType)];
        _adjustAndCheckFheTransactionLimitThreeOps(opHCU, lhs, middle, rhs, result);
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheRand.
     * @param resultType Result type.
     * @param result Result.
     */
    function checkHCUForFheRand(FheType resultType, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU = $.hcuValues[_getHcuKey("fheRand", resultType)];
        _updateAndVerifyHCUTransactionLimit(opHCU);
        _setHCUForHandle(result, opHCU);
    }
    /**
     * @notice Check the homomorphic complexity units limit for FheRandBounded.
     * @param resultType Result type.
     * @param result Result.
     */
    function checkHCUForFheRandBounded(FheType resultType, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint256 opHCU = $.hcuValues[_getHcuKey("fheRandBounded", resultType)];
        _updateAndVerifyHCUTransactionLimit(opHCU);
        _setHCUForHandle(result, opHCU);
    }
    /**
     * @notice Getter function for the FHEVMExecutor contract address.
     * @return fhevmExecutorAddress Address of the FHEVMExecutor.
     */
    function getFHEVMExecutorAddress() public view virtual returns (address) {
        return fhevmExecutorAddress;
    }

    /**
     * @notice Getter for the name and version of the contract.
     * @return string Name and the version of the contract.
     */
    function getVersion() external pure virtual returns (string memory) {
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

    function setHcuValue(string memory operation, FheType fheType, uint256 value) external onlyOwner {
        HCULimitStorage storage $ = _getHCULimitStorage();
        $.hcuValues[_getHcuKey(operation, fheType)] = value;
    }

    function _getHcuKey(string memory operation, FheType fheType) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(operation, fheType));
    }

    function _getHCULimitStorage() internal pure returns (HCULimitStorage storage $) {
        assembly {
            $.slot := HCULimitStorageLocation
        }
    }

    /**
     * @notice Adjusts the sequential HCU for the transaction.
     */
    function _adjustAndCheckFheTransactionLimitOneOp(uint256 opHCU, bytes32 op1, bytes32 result) internal virtual {
        _updateAndVerifyHCUTransactionLimit(opHCU);

        uint256 totalHCU = opHCU + _getHCUForHandle(op1);
        if (totalHCU >= MAX_HOMOMORPHIC_COMPUTE_UNITS_DEPTH_PER_TX) {
            revert HCUTransactionDepthLimitExceeded();
        }

        _setHCUForHandle(result, totalHCU);
    }

    /**
     * @notice Adjusts the current HCU for the transaction.
     */
    function _adjustAndCheckFheTransactionLimitTwoOps(
        uint256 opHCU,
        bytes32 op1,
        bytes32 op2,
        bytes32 result
    ) internal virtual {
        _updateAndVerifyHCUTransactionLimit(opHCU);

        uint256 totalHCU = opHCU + _max(_getHCUForHandle(op1), _getHCUForHandle(op2));
        if (totalHCU >= MAX_HOMOMORPHIC_COMPUTE_UNITS_DEPTH_PER_TX) {
            revert HCUTransactionDepthLimitExceeded();
        }

        _setHCUForHandle(result, totalHCU);
    }

    /**
     * @notice Adjusts the current HCU for the transaction.
     */
    function _adjustAndCheckFheTransactionLimitThreeOps(
        uint256 opHCU,
        bytes32 op1,
        bytes32 op2,
        bytes32 op3,
        bytes32 result
    ) internal virtual {
        _updateAndVerifyHCUTransactionLimit(opHCU);

        uint256 totalHCU = opHCU + _max(_getHCUForHandle(op1), _max(_getHCUForHandle(op2), _getHCUForHandle(op3)));

        if (totalHCU >= MAX_HOMOMORPHIC_COMPUTE_UNITS_DEPTH_PER_TX) {
            revert HCUTransactionDepthLimitExceeded();
        }

        _setHCUForHandle(result, totalHCU);
    }

    /**
     * @notice Updates and verifies the HCU transaction limit.
     * @param opHCU The HCU for the operation.
     */
    function _updateAndVerifyHCUTransactionLimit(uint256 opHCU) internal virtual {
        uint256 transactionHCU = opHCU + _getHCUForTransaction();
        if (transactionHCU >= MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX) {
            revert HCUTransactionLimitExceeded();
        }
        _setHCUForTransaction(transactionHCU);
    }

    /**
     * @notice Gets the current HCU for the handle.
     * @param handle The handle for which to get the HCU.
     * @return handleHCU The current HCU for the handle.
     * @dev This function uses inline assembly to load the HCU from a specific storage location.
     */
    function _getHCUForHandle(bytes32 handle) internal view virtual returns (uint256 handleHCU) {
        bytes32 slot = keccak256(abi.encodePacked(HCULimitStorageLocation, handle));
        assembly {
            // Ensure the slot is properly aligned and validated before using tload.
            // This assumes the slot is derived from a secure and deterministic process.
            handleHCU := tload(slot)
        }
    }

    /**
     * @notice Gets the total HCU for the transaction.
     * @return transactionHCU The HCU for the transaction.
     * @dev This function uses inline assembly to store the HCU in a specific storage location.
     */
    function _getHCUForTransaction() internal view virtual returns (uint256 transactionHCU) {
        /// @dev keccak256(abi.encodePacked(HCULimitStorageLocation, "HCU"))
        bytes32 slot = 0x9fe02aa19e370f46d43dc2b6620733ba9c3b193659e9699f55eefe911af8a4b4;
        assembly {
            transactionHCU := tload(slot)
        }
    }

    /**
     * @notice Sets the HCU for a handle in the transient storage.
     * @param handle The handle for which to set the HCU.
     * @param handleHCU The HCU to set for the handle.
     * @dev This function uses inline assembly to store the HCU in a specific storage location.
     */
    function _setHCUForHandle(bytes32 handle, uint256 handleHCU) internal virtual {
        bytes32 slot = keccak256(abi.encodePacked(HCULimitStorageLocation, handle));
        assembly {
            tstore(slot, handleHCU)
        }
    }

    /**
     * @notice Updates the current HCU consumption for the transaction and stores it in the transient storage.
     * @param transactionHCU The total HCU for the transaction.
     * @dev This function uses inline assembly to store the HCU in a specific storage location.
     */
    function _setHCUForTransaction(uint256 transactionHCU) internal virtual {
        /// @dev keccak256(abi.encodePacked(HCULimitStorageLocation, "HCU"))
        bytes32 slot = 0x9fe02aa19e370f46d43dc2b6620733ba9c3b193659e9699f55eefe911af8a4b4;
        assembly {
            tstore(slot, transactionHCU)
        }
    }

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /**
     * @dev Returns the maximum of two numbers.
     * @param a The first number.
     * @param b The second number.
     * @return The maximum of a and b.
     */
    function _max(uint256 a, uint256 b) private pure returns (uint256) {
        return a >= b ? a : b;
    }
}
