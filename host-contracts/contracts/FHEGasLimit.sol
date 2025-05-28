// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {fhevmExecutorAdd} from "../addresses/FHEVMExecutorAddress.sol";

import {FheType} from "./FheType.sol";

/**
 * @title  FHEGasLimit
 * @notice This contract manages the total allowed complexity for FHE operations at the
 * transaction level, including the maximum number of homomorphic compute units (HCU) per transaction.
 * @dev The contract is designed to be used with the FHEVMExecutor contract.
 */
contract FHEGasLimit is UUPSUpgradeable, Ownable2StepUpgradeable {
    /// @notice Returned if the sender is not the FHEVMExecutor.
    error CallerMustBeFHEVMExecutorContract();

    /// @notice Returned if the transaction exceeds the maximum allowed homomorphic compute units.
    error HCUTransactionLimitExceeded();

    /// @notice Returned if the transaction exceeds the maximum allowed depth of homomorphic compute units.
    error HCUTransactionDepthLimitExceeded();

    /// @notice Returned if the operation is not supported.
    error UnsupportedOperation();

    /// @notice Returned if the operation is not scalar.
    error OnlyScalarOperationsAreSupported();

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "FHEGasLimit";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 1;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @notice FHEVMExecutor address.
    address private constant fhevmExecutorAddress = fhevmExecutorAdd;

    /// @notice Maximum homomorphic compute units depth per block.
    /// @dev This is the maximum number of homomorphic compute units that can be sequential.
    uint256 private constant MAX_HOMOMORPHIC_COMPUTE_UNITS_DEPTH_PER_TX = 5_000_000;

    /// @notice Maximum homomorphic compute units per transaction.
    /// @dev This is the maximum number of homomorphic compute units that can be used in a single transaction.
    uint256 private constant MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX = 20_000_000;

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.FHEGasLimit")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant FHEGasLimitStorageLocation =
        0xb5c80b3bbe0bcbcea690f6dbe62b32a45bd1ad263b78db2f25ef8414efe9bc00;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Re-initializes the contract.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitialize() public virtual reinitializer(2) {
        __Ownable_init(owner());
    }

    /**
     * @notice Check the homomorphic computation units limit for FheAdd.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 94000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 133000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 162000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 188000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 218000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 94000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 133000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 162000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 188000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 218000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheSub.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 94000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 133000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 162000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 188000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 218000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 94000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 133000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 162000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 188000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 218000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheMul.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 159000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 208000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 264000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 356000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 480000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 197000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 262000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 359000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 641000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 1145000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheDiv.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheDiv(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();
        if (resultType == FheType.Uint8) {
            opHCU = 238000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 314000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 398000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 584000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 857000;
        } else {
            revert UnsupportedOperation();
        }

        _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
    }
    /**
     * @notice Check the homomorphic computation units limit for FheRem.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     */
    function checkHCUForFheRem(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result
    ) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();
        if (resultType == FheType.Uint8) {
            opHCU = 460000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 622000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 805000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 1095000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 1499000;
        } else {
            revert UnsupportedOperation();
        }

        _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
    }
    /**
     * @notice Check the homomorphic computation units limit for FheBitAnd.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 26000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 38000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 41000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 44000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 26000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 38000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 41000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 44000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheBitOr.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 26000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 38000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 41000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 44000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 26000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 38000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 41000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 44000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheBitXor.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 26000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 38000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 41000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 44000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 26000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 38000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 41000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 44000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheShl.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 38000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 41000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 44000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 133000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 153000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 183000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 227000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 282000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 350000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheShr.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 38000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 41000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 44000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 133000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 153000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 183000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 227000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 282000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 350000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheRotl.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 38000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 41000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 44000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 133000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 153000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 183000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 227000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 282000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 350000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheRotr.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 35000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 38000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 41000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 44000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 133000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 153000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 183000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 227000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 282000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 350000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheEq.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 49000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 53000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 54000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 86000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 88000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 90000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 100000;
            } else if (resultType == FheType.Uint512) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint1024) {
                opHCU = 200000;
            } else if (resultType == FheType.Uint2048) {
                opHCU = 300000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 49000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 53000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 54000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 86000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 88000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 90000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 100000;
            } else if (resultType == FheType.Uint512) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint1024) {
                opHCU = 200000;
            } else if (resultType == FheType.Uint2048) {
                opHCU = 300000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit required for FheEqBytes.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param ct The only operand.
     * @param result Result.
     */
    function checkHCUForFheEqBytes(FheType resultType, bytes1 scalarByte, bytes32 ct, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 49000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 53000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 54000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 86000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 88000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 90000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 100000;
            } else if (resultType == FheType.Uint512) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint1024) {
                opHCU = 200000;
            } else if (resultType == FheType.Uint2048) {
                opHCU = 300000;
            } else {
                revert UnsupportedOperation();
            }

            _updateAndVerifyHCUTransactionLimit(opHCU);
            _setHCUForHandle(result, opHCU);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 49000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 53000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 54000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 86000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 88000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 90000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 100000;
            } else if (resultType == FheType.Uint512) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint1024) {
                opHCU = 200000;
            } else if (resultType == FheType.Uint2048) {
                opHCU = 300000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, ct, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheNe.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 49000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 53000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 54000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 86000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 88000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 90000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 100000;
            } else if (resultType == FheType.Uint512) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint1024) {
                opHCU = 200000;
            } else if (resultType == FheType.Uint2048) {
                opHCU = 300000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 49000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 53000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 54000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 86000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 88000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 90000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 100000;
            } else if (resultType == FheType.Uint512) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint1024) {
                opHCU = 200000;
            } else if (resultType == FheType.Uint2048) {
                opHCU = 300000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit required for FheNeBytes.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param ct The only operand.
     * @param result Result.
     */
    function checkHCUForFheNeBytes(FheType resultType, bytes1 scalarByte, bytes32 ct, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 49000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 53000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 54000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 86000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 88000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 90000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 100000;
            } else if (resultType == FheType.Uint512) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint1024) {
                opHCU = 200000;
            } else if (resultType == FheType.Uint2048) {
                opHCU = 300000;
            } else {
                revert UnsupportedOperation();
            }

            _updateAndVerifyHCUTransactionLimit(opHCU);
            _setHCUForHandle(result, opHCU);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 49000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 53000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 54000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 86000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 88000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 90000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 100000;
            } else if (resultType == FheType.Uint512) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint1024) {
                opHCU = 200000;
            } else if (resultType == FheType.Uint2048) {
                opHCU = 300000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, ct, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheGe.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 105000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 156000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 190000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 105000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 156000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 190000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheGt.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 105000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 156000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 190000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 105000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 156000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 190000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheLe.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 105000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 156000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 190000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 105000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 156000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 190000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheLt.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 105000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 156000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 190000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 105000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 156000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 190000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheMin.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 164000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 192000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 225000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 153000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 183000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 210000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 241000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheMax.
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
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 164000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 192000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 225000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 128000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 153000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 183000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 210000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 241000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, lhs, rhs, result);
        }
    }
    /**
     * @notice Check the homomorphic computation units limit for FheNeg.
     * @param ct The only operand.
     * @param result Result.
     */
    function checkHCUForFheNeg(FheType resultType, bytes32 ct, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Uint8) {
            opHCU = 95000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 131000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 160000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 199000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 248000;
        } else if (resultType == FheType.Uint256) {
            opHCU = 309000;
        } else {
            revert UnsupportedOperation();
        }
        _adjustAndCheckFheTransactionLimitOneOp(opHCU, ct, result);
    }
    /**
     * @notice Check the homomorphic computation units limit for FheNot.
     * @param ct The only operand.
     * @param result Result.
     */
    function checkHCUForFheNot(FheType resultType, bytes32 ct, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Bool) {
            opHCU = 30000;
        } else if (resultType == FheType.Uint8) {
            opHCU = 34000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 35000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 36000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 37000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 38000;
        } else if (resultType == FheType.Uint256) {
            opHCU = 39000;
        } else {
            revert UnsupportedOperation();
        }
        _adjustAndCheckFheTransactionLimitOneOp(opHCU, ct, result);
    }
    /**
     * @notice Check the homomorphic computation units limit for Cast.
     * @param ct The only operand.
     * @param result Result.
     */
    function checkHCUForCast(FheType resultType, bytes32 ct, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Bool) {
            opHCU = 200;
        } else if (resultType == FheType.Uint8) {
            opHCU = 200;
        } else if (resultType == FheType.Uint16) {
            opHCU = 200;
        } else if (resultType == FheType.Uint32) {
            opHCU = 200;
        } else if (resultType == FheType.Uint64) {
            opHCU = 200;
        } else if (resultType == FheType.Uint128) {
            opHCU = 200;
        } else if (resultType == FheType.Uint256) {
            opHCU = 200;
        } else {
            revert UnsupportedOperation();
        }
        _adjustAndCheckFheTransactionLimitOneOp(opHCU, ct, result);
    }
    /**
     * @notice Check the homomorphic computation units limit for TrivialEncrypt.
     * @param resultType Result type.
     * @param result Result.
     */
    function checkHCUForTrivialEncrypt(FheType resultType, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Bool) {
            opHCU = 100;
        } else if (resultType == FheType.Uint8) {
            opHCU = 100;
        } else if (resultType == FheType.Uint16) {
            opHCU = 200;
        } else if (resultType == FheType.Uint32) {
            opHCU = 300;
        } else if (resultType == FheType.Uint64) {
            opHCU = 600;
        } else if (resultType == FheType.Uint128) {
            opHCU = 650;
        } else if (resultType == FheType.Uint160) {
            opHCU = 700;
        } else if (resultType == FheType.Uint256) {
            opHCU = 800;
        } else if (resultType == FheType.Uint512) {
            opHCU = 1600;
        } else if (resultType == FheType.Uint1024) {
            opHCU = 3200;
        } else if (resultType == FheType.Uint2048) {
            opHCU = 6400;
        } else {
            revert UnsupportedOperation();
        }
        _updateAndVerifyHCUTransactionLimit(opHCU);
        _setHCUForHandle(result, opHCU);
    }
    /**
     * @notice Check the homomorphic computation units limit for IfThenElse.
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
        uint256 opHCU;
        if (resultType == FheType.Bool) {
            opHCU = 43000;
        } else if (resultType == FheType.Uint8) {
            opHCU = 47000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 47000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 50000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 53000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 70000;
        } else if (resultType == FheType.Uint160) {
            opHCU = 80000;
        } else if (resultType == FheType.Uint256) {
            opHCU = 90000;
        } else if (resultType == FheType.Uint512) {
            opHCU = 150000;
        } else if (resultType == FheType.Uint1024) {
            opHCU = 200000;
        } else if (resultType == FheType.Uint2048) {
            opHCU = 300000;
        } else {
            revert UnsupportedOperation();
        }
        _adjustAndCheckFheTransactionLimitThreeOps(opHCU, lhs, middle, rhs, result);
    }
    /**
     * @notice Check the homomorphic computation units limit for FheRand.
     * @param resultType Result type.
     * @param result Result.
     */
    function checkHCUForFheRand(FheType resultType, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Bool) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint8) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint256) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint512) {
            opHCU = 200000;
        } else if (resultType == FheType.Uint1024) {
            opHCU = 300000;
        } else if (resultType == FheType.Uint2048) {
            opHCU = 400000;
        } else {
            revert UnsupportedOperation();
        }
        _updateAndVerifyHCUTransactionLimit(opHCU);
        _setHCUForHandle(result, opHCU);
    }
    /**
     * @notice Check the homomorphic computation units limit for FheRandBounded.
     * @param resultType Result type.
     * @param result Result.
     */
    function checkHCUForFheRandBounded(FheType resultType, bytes32 result) external virtual {
        if (msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Uint8) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 100000;
        } else if (resultType == FheType.Uint256) {
            opHCU = 100000;
        } else {
            revert UnsupportedOperation();
        }
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
        bytes32 slot = keccak256(abi.encodePacked(FHEGasLimitStorageLocation, handle));
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
        /// @dev keccak256(abi.encodePacked(FHEGasLimitStorageLocation, "HCU"))
        bytes32 slot = 0xf0a6f781dda4e666410a23516da0a5550b29ecafc3a35849ff9af2e2ec3b6123;
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
        bytes32 slot = keccak256(abi.encodePacked(FHEGasLimitStorageLocation, handle));
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
        /// @dev keccak256(abi.encodePacked(FHEGasLimitStorageLocation, "HCU"))
        bytes32 slot = 0xf0a6f781dda4e666410a23516da0a5550b29ecafc3a35849ff9af2e2ec3b6123;
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
