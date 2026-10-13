// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {fhevmExecutorAdd, aclAdd, confidentialBridgeAdd} from "../addresses/FHEVMHostAddresses.sol";
import {ACL} from "./ACL.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";

import {FheType} from "./shared/FheType.sol";

/**
 * @title HCULimit
 * @notice This contract manages the total allowed complexity for FHE operations at the
 * transaction level, including the maximum number of homomorphic complexity units (HCU) per transaction.
 * @dev The contract is designed to be used with the FHEVMExecutor contract.
 */
/// @dev This contract was migrated from Ownable2StepUpgradeable to ACLOwnable.
/// Deployed proxies retain residual `_owner` and `_pendingOwner` values in the
/// Ownable2StepUpgradeable EIP-7201 storage namespace. These slots are unused
/// by ACLOwnable and have no effect on contract behavior.
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract HCULimit is UUPSUpgradeableEmptyProxy, ACLOwnable {
    /// @notice Returned if the sender is not the FHEVMExecutor.
    error CallerMustBeFHEVMExecutorContract();

    /// @notice Returned if the block exceeds the maximum allowed homomorphic complexity units.
    error HCUBlockLimitExceeded();

    /// @notice Returned if the address is already block HCU whitelisted.
    error AlreadyBlockHCUWhitelisted(address account);

    /// @notice Returned if the address is not block HCU whitelisted.
    error NotBlockHCUWhitelisted(address account);

    /// @notice Returned if the transaction exceeds the maximum allowed homomorphic complexity units.
    error HCUTransactionLimitExceeded();

    /// @notice Returned if the transaction exceeds the maximum allowed depth of homomorphic complexity units.
    error HCUTransactionDepthLimitExceeded();

    /// @notice Returned if hcuPerBlock < maxHCUPerTx.
    error HCUPerBlockBelowMaxPerTx();

    /// @notice Returned if maxHCUPerTx < maxHCUDepthPerTx.
    error MaxHCUPerTxBelowDepth();

    /// @notice Returned if the operation is not supported.
    error UnsupportedOperation();

    /// @notice Returned if the operation is not scalar.
    error OnlyScalarOperationsAreSupported();

    /// @notice Returned if a handle is the zero handle.
    /// @dev Handle slot zero is reserved for the transaction HCU accumulator.
    error InvalidZeroHandle();

    /// @notice Returned when a computed handle's depth is neither cached in this transaction nor stored for its holder.
    error UnknownHandleDepth(bytes32 handle);

    /// @notice Returned when a depth hook is called by a contract other than the ACL or the bridge.
    error CallerMustBeACLOrBridge();

    /// @notice Returned when bridged depths are set by a contract other than the bridge.
    error CallerMustBeConfidentialBridge();

    /// @notice Returned when only one of the cross-transaction depth limit and its decay is zero.
    error InvalidCrossTxDepthLimit();

    /// @notice Returned when an operation's depth carried across transactions exceeds the limit.
    error HCUCrossTxDepthLimitExceeded();

    /// @notice Emitted when the cross-transaction depth limit is updated.
    event CrossTxDepthLimitSet(uint48 maxDepth, uint48 decayPerBlock);

    /// @notice Emitted when the global block HCU cap is updated.
    /// @param hcuPerBlock New global block HCU cap.
    event HCUPerBlockSet(uint48 hcuPerBlock);

    /// @notice Emitted when the per-transaction HCU depth limit is updated.
    /// @param maxHCUDepthPerTx New depth limit.
    event MaxHCUDepthPerTxSet(uint48 maxHCUDepthPerTx);

    /// @notice Emitted when the per-transaction HCU limit is updated.
    /// @param maxHCUPerTx New transaction limit.
    event MaxHCUPerTxSet(uint48 maxHCUPerTx);

    /// @notice Emitted when a caller is added to the block-cap whitelist.
    /// @param account Caller address that was whitelisted.
    event BlockHCUWhitelistAdded(address indexed account);

    /// @notice Emitted when a caller is removed from the block-cap whitelist.
    /// @param account Caller address that was removed from the whitelist.
    event BlockHCUWhitelistRemoved(address indexed account);

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "HCULimit";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 5;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @notice FHEVMExecutor address.
    address private constant FHEVM_EXECUTOR_ADDRESS = fhevmExecutorAdd;

    /// @dev Per-handle transient word: bit 255 marks the depths as known, bits 128-254 hold the
    ///      depth carried across transactions and bits 0-127 the depth within this transaction.
    uint256 private constant DEPTH_KNOWN = 1 << 255;

    /// `fheMulDiv` `scalarByte` bitmask: bit 0 (divisor) is always set; bit 1 marks `factor2` as scalar.
    bytes1 private constant FHE_MUL_DIV_FACTOR2_ENCRYPTED = 0x01;
    bytes1 private constant FHE_MUL_DIV_FACTOR2_SCALAR = 0x03;

    /// @custom:storage-location erc7201:fhevm.storage.HCULimit
    /// @dev All five uint48 fields pack into a single 256-bit slot (5 × 48 = 240 bits).
    struct HCULimitStorage {
        /// @notice Maximum homomorphic complexity units per block for non-whitelisted callers.
        uint48 globalHCUCapPerBlock;
        /// @notice Used HCU in the current block for non-whitelisted callers.
        uint48 usedBlockHCU;
        /// @notice Last seen block number for the block meter.
        uint48 lastSeenBlockNumber;
        /// @notice Maximum sequential HCU depth per transaction.
        uint48 maxHCUDepthPerTx;
        /// @notice Maximum total HCU per transaction.
        uint48 maxHCUPerTx;
        /// @notice Whitelisted callers bypass block-level cap.
        mapping(address account => bool isWhitelisted) blockHCUWhitelist;
        /// @notice Maximum depth carried across transactions; 0 disables the check.
        uint48 maxCrossTxDepth;
        /// @notice Carried depth the coprocessors are assumed to drain per block.
        uint48 crossTxDepthDecayPerBlock;
    }

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 6;

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.HCULimit")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant HCU_LIMIT_STORAGE_LOCATION =
        0xc13af6c514bff8997f30c90003baa82bd02aad978179d1ce58d85c4319ad6500;

    function _getHCULimitStorage() internal pure virtual returns (HCULimitStorage storage $) {
        assembly {
            $.slot := HCU_LIMIT_STORAGE_LOCATION
        }
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract.
     * @param hcuCapPerBlock Initial global HCU cap per block.
     * @param maxHCUDepthPerTx Maximum sequential HCU depth per transaction.
     * @param maxHCUPerTx Maximum total HCU per transaction.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        uint48 hcuCapPerBlock,
        uint48 maxHCUDepthPerTx,
        uint48 maxHCUPerTx
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        _setHCUPerBlock(hcuCapPerBlock);
        _setMaxHCUPerTx(maxHCUPerTx);
        _setMaxHCUDepthPerTx(maxHCUDepthPerTx);
    }

    /**
     * @notice Re-initializes the contract from V4.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV5() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice Check the homomorphic complexity units limit for FheAdd.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheAdd(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 84000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 93000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 95000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 133000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 172000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 88000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 93000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 125000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 162000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 259000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheSub.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheSub(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 84000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 93000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 95000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 133000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 172000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 91000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 93000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 125000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 162000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 260000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheMul.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheMul(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 122000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 193000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 265000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 365000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 696000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 222000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 328000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 596000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 1686000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheDiv.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheDiv(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 /*rhs*/,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();
        if (resultType == FheType.Uint8) {
            opHCU = 210000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 302000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 438000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 715000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 1225000;
        } else {
            revert UnsupportedOperation();
        }

        _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheRem.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheRem(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 /*rhs*/,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();
        if (resultType == FheType.Uint8) {
            opHCU = 440000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 580000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 792000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 1153000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 1943000;
        } else {
            revert UnsupportedOperation();
        }

        _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheBitAnd.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheBitAnd(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 22000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 37000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 38000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 25000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 37000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 38000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheBitOr.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheBitOr(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 22000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 30000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 30000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 37000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 38000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 24000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 30000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 37000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 38000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheBitXor.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheBitXor(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 22000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 37000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 39000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 22000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 37000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 39000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheShl.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheShl(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 37000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 39000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 92000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 125000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 162000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 208000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 272000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 378000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheShr.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheShr(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 37000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 38000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 91000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 123000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 163000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 209000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 272000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 369000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheRotl.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheRotl(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 37000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 38000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 91000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 125000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 163000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 209000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 278000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 378000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheRotr.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheRotr(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 31000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 32000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 34000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 37000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 40000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 93000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 125000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 160000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 209000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 283000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 375000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheEq.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheEq(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 25000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 55000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 55000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 82000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 83000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 117000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 117000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 118000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 26000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 55000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 83000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 86000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 120000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 122000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 137000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 152000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheNe.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheNe(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Bool) {
                opHCU = 23000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 55000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 55000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 83000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 84000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 117000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 117000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 117000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Bool) {
                opHCU = 23000;
            } else if (resultType == FheType.Uint8) {
                opHCU = 55000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 83000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 85000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 118000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 122000;
            } else if (resultType == FheType.Uint160) {
                opHCU = 136000;
            } else if (resultType == FheType.Uint256) {
                opHCU = 150000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheGe.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheGe(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 52000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 55000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 84000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 116000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 149000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 63000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 84000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 118000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 152000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 210000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheGt.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheGt(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 52000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 55000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 84000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 117000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 150000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 59000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 84000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 118000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 152000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 218000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheLe.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheLe(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 58000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 58000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 84000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 119000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 150000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 58000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 83000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 117000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 149000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 218000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheLt.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheLt(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 52000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 58000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 83000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 118000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 149000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 59000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 84000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 117000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 146000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 215000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheMin.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheMin(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 84000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 88000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 117000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 150000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 186000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 119000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 146000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 182000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 219000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 289000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheMax.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param lhs The left-hand side operand.
     * @param rhs The right-hand side operand.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheMax(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 lhs,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == 0x01) {
            if (resultType == FheType.Uint8) {
                opHCU = 89000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 89000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 117000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 149000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 180000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);
        } else {
            if (resultType == FheType.Uint8) {
                opHCU = 121000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 145000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 180000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 218000;
            } else if (resultType == FheType.Uint128) {
                opHCU = 290000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);
        }
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheNeg.
     * @param ct The only operand.
     * @param result Result.
     * @param caller Original caller address from FHEVMExecutor.
     */
    function checkHCUForFheNeg(FheType resultType, bytes32 ct, bytes32 result, address caller) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Uint8) {
            opHCU = 79000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 93000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 95000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 131000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 168000;
        } else if (resultType == FheType.Uint256) {
            opHCU = 269000;
        } else {
            revert UnsupportedOperation();
        }
        _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, ct, result);
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheNot.
     * @param ct The only operand.
     * @param result Result.
     * @param caller Original caller address from FHEVMExecutor.
     */
    function checkHCUForFheNot(FheType resultType, bytes32 ct, bytes32 result, address caller) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Bool) {
            opHCU = 2;
        } else if (resultType == FheType.Uint8) {
            opHCU = 9;
        } else if (resultType == FheType.Uint16) {
            opHCU = 16;
        } else if (resultType == FheType.Uint32) {
            opHCU = 32;
        } else if (resultType == FheType.Uint64) {
            opHCU = 63;
        } else if (resultType == FheType.Uint128) {
            opHCU = 130;
        } else if (resultType == FheType.Uint256) {
            opHCU = 130;
        } else {
            revert UnsupportedOperation();
        }
        _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, ct, result);
    }

    /**
     * @notice Check the homomorphic complexity units limit for Cast.
     * @param ct The only operand.
     * @param result Result.
     * @param caller Original caller address from FHEVMExecutor.
     */
    function checkHCUForCast(FheType resultType, bytes32 ct, bytes32 result, address caller) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Bool) {
            opHCU = 32;
        } else if (resultType == FheType.Uint8) {
            opHCU = 32;
        } else if (resultType == FheType.Uint16) {
            opHCU = 32;
        } else if (resultType == FheType.Uint32) {
            opHCU = 32;
        } else if (resultType == FheType.Uint64) {
            opHCU = 32;
        } else if (resultType == FheType.Uint128) {
            opHCU = 32;
        } else if (resultType == FheType.Uint256) {
            opHCU = 32;
        } else {
            revert UnsupportedOperation();
        }
        _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, ct, result);
    }

    /**
     * @notice Check the homomorphic complexity units limit for TrivialEncrypt.
     * @param resultType Result type.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForTrivialEncrypt(FheType resultType, bytes32 result, address caller) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Bool) {
            opHCU = 32;
        } else if (resultType == FheType.Uint8) {
            opHCU = 32;
        } else if (resultType == FheType.Uint16) {
            opHCU = 32;
        } else if (resultType == FheType.Uint32) {
            opHCU = 32;
        } else if (resultType == FheType.Uint64) {
            opHCU = 32;
        } else if (resultType == FheType.Uint128) {
            opHCU = 32;
        } else if (resultType == FheType.Uint160) {
            opHCU = 32;
        } else if (resultType == FheType.Uint256) {
            opHCU = 32;
        } else {
            revert UnsupportedOperation();
        }
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);
        _setHCUForHandle(result, _opDepths(opHCU));
    }

    /**
     * @notice Check the homomorphic complexity units limit for IfThenElse.
     * @param resultType Result type.
     * @param lhs The left-hand side operand.
     * @param middle The middle operand.
     * @param rhs The right-hand side operand.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForIfThenElse(
        FheType resultType,
        bytes32 lhs,
        bytes32 middle,
        bytes32 rhs,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Bool) {
            opHCU = 55000;
        } else if (resultType == FheType.Uint8) {
            opHCU = 55000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 55000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 55000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 55000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 57000;
        } else if (resultType == FheType.Uint160) {
            opHCU = 83000;
        } else if (resultType == FheType.Uint256) {
            opHCU = 108000;
        } else {
            revert UnsupportedOperation();
        }
        _adjustAndCheckFheTransactionLimitThreeOps(opHCU, caller, lhs, middle, rhs, result);
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheRand.
     * @param resultType Result type.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheRand(FheType resultType, bytes32 result, address caller) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Bool) {
            opHCU = 19000;
        } else if (resultType == FheType.Uint8) {
            opHCU = 23000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 23000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 24000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 24000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 25000;
        } else if (resultType == FheType.Uint256) {
            opHCU = 30000;
        } else {
            revert UnsupportedOperation();
        }
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);
        _setHCUForHandle(result, _opDepths(opHCU));
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheRandBounded.
     * @param resultType Result type.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheRandBounded(FheType resultType, bytes32 result, address caller) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (resultType == FheType.Uint8) {
            opHCU = 23000;
        } else if (resultType == FheType.Uint16) {
            opHCU = 23000;
        } else if (resultType == FheType.Uint32) {
            opHCU = 24000;
        } else if (resultType == FheType.Uint64) {
            opHCU = 24000;
        } else if (resultType == FheType.Uint128) {
            opHCU = 25000;
        } else if (resultType == FheType.Uint256) {
            opHCU = 30000;
        } else {
            revert UnsupportedOperation();
        }
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);
        _setHCUForHandle(result, _opDepths(opHCU));
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheSum.
     * @param resultType Result type.
     * @param values Input ciphertext handles.
     * @param result Result handle.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheSum(
        FheType resultType,
        bytes32[] calldata values,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 n = values.length;
        uint256 opHCU;
        if (resultType == FheType.Uint8) {
            if (n <= 10) opHCU = 90900;
            else if (n <= 30) opHCU = 127000;
            else if (n <= 60) opHCU = 148000;
            else opHCU = 159000;
        } else if (resultType == FheType.Uint16) {
            if (n <= 10) opHCU = 95000;
            else if (n <= 30) opHCU = 136000;
            else if (n <= 60) opHCU = 162000;
            else opHCU = 184000;
        } else if (resultType == FheType.Uint32) {
            if (n <= 10) opHCU = 116000;
            else if (n <= 30) opHCU = 164000;
            else if (n <= 60) opHCU = 205000;
            else opHCU = 281000;
        } else if (resultType == FheType.Uint64) {
            if (n <= 10) opHCU = 139000;
            else if (n <= 30) opHCU = 216000;
            else opHCU = 306000;
        } else if (resultType == FheType.Uint128) {
            if (n <= 10) opHCU = 219000;
            else if (n <= 30) opHCU = 355000;
            else opHCU = 552000;
        } else {
            revert UnsupportedOperation();
        }
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);

        uint256 inputDepths = 0;
        for (uint256 i = 0; i < values.length; i++) {
            inputDepths = _maxDepths(inputDepths, _handleDepths(values[i], caller));
        }

        uint256 depths = _opDepths(opHCU) + inputDepths;
        if (uint128(depths) > uint256(_getHCULimitStorage().maxHCUDepthPerTx)) {
            revert HCUTransactionDepthLimitExceeded();
        }
        _setHCUForHandle(result, depths);
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheIsIn.
     * @param valueType Value type.
     * @param value Input ciphertext handle.
     * @param values Encrypted set ciphertext handles.
     * @param result Result handle.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheIsIn(
        FheType valueType,
        bytes32 value,
        bytes32[] calldata values,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 n = values.length;
        uint256 opHCU;
        if (valueType == FheType.Uint8) {
            if (n <= 10) opHCU = 71300;
            else if (n <= 30) opHCU = 148000;
            else if (n <= 60) opHCU = 247000;
            else opHCU = 374000;
        } else if (valueType == FheType.Uint16) {
            if (n <= 10) opHCU = 103000;
            else if (n <= 30) opHCU = 218000;
            else if (n <= 60) opHCU = 378000;
            else opHCU = 605000;
        } else if (valueType == FheType.Uint32) {
            if (n <= 10) opHCU = 137000;
            else if (n <= 30) opHCU = 300000;
            else if (n <= 60) opHCU = 531000;
            else opHCU = 827000;
        } else if (valueType == FheType.Uint64) {
            if (n <= 10) opHCU = 218000;
            else if (n <= 30) opHCU = 492000;
            else opHCU = 879000;
        } else if (valueType == FheType.Uint128) {
            if (n <= 10) opHCU = 256000;
            else if (n <= 30) opHCU = 535000;
            else opHCU = 921000;
        } else if (valueType == FheType.Uint160) {
            if (n <= 10) opHCU = 286000;
            else if (n <= 30) opHCU = 584000;
            else opHCU = 885000;
        } else if (valueType == FheType.Uint256) {
            if (n <= 10) opHCU = 321000;
            else if (n <= 30) opHCU = 586000;
            else opHCU = 943000;
        } else {
            revert UnsupportedOperation();
        }
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);

        uint256 inputDepths = _handleDepths(value, caller);
        for (uint256 i = 0; i < values.length; i++) {
            inputDepths = _maxDepths(inputDepths, _handleDepths(values[i], caller));
        }

        uint256 depths = _opDepths(opHCU) + inputDepths;
        if (uint128(depths) > uint256(_getHCULimitStorage().maxHCUDepthPerTx)) {
            revert HCUTransactionDepthLimitExceeded();
        }
        _setHCUForHandle(result, depths);
    }

    /**
     * @notice Check the homomorphic complexity units limit for FheMulDiv.
     * @param resultType Result type.
     * @param scalarByte Scalar byte.
     * @param factor1 The first multiplication factor.
     * @param factor2 The second multiplication factor.
     * @param result Result.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function checkHCUForFheMulDiv(
        FheType resultType,
        bytes1 scalarByte,
        bytes32 factor1,
        bytes32 factor2,
        bytes32 result,
        address caller
    ) external virtual {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
        if (scalarByte == FHE_MUL_DIV_FACTOR2_SCALAR) {
            if (resultType == FheType.Uint8) {
                opHCU = 495000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 703000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 1080000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 1921000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, factor1, result);
        } else if (scalarByte == FHE_MUL_DIV_FACTOR2_ENCRYPTED) {
            if (resultType == FheType.Uint8) {
                opHCU = 524000;
            } else if (resultType == FheType.Uint16) {
                opHCU = 766000;
            } else if (resultType == FheType.Uint32) {
                opHCU = 1311000;
            } else if (resultType == FheType.Uint64) {
                opHCU = 2911000;
            } else {
                revert UnsupportedOperation();
            }

            _adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, factor1, factor2, result);
        } else {
            revert OnlyScalarOperationsAreSupported();
        }
    }

    /**
     * @notice Sets the block-level HCU limit for non-whitelisted callers.
     * @param hcuPerBlock New block-level cap.
     */
    function setHCUPerBlock(uint48 hcuPerBlock) external onlyACLOwner {
        _setHCUPerBlock(hcuPerBlock);
    }

    /**
     * @notice Sets the per-transaction HCU depth limit.
     * @param maxHCUDepthPerTx New depth limit.
     */
    function setMaxHCUDepthPerTx(uint48 maxHCUDepthPerTx) external onlyACLOwner {
        _setMaxHCUDepthPerTx(maxHCUDepthPerTx);
    }

    /**
     * @notice Sets the per-transaction HCU limit.
     * @param maxHCUPerTx New transaction limit.
     */
    function setMaxHCUPerTx(uint48 maxHCUPerTx) external onlyACLOwner {
        _setMaxHCUPerTx(maxHCUPerTx);
    }

    /**
     * @notice Adds one caller to the block-cap whitelist.
     * @param account Caller to whitelist.
     */
    function addToBlockHCUWhitelist(address account) external onlyACLOwner {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.blockHCUWhitelist[account]) revert AlreadyBlockHCUWhitelisted(account);
        $.blockHCUWhitelist[account] = true;
        emit BlockHCUWhitelistAdded(account);
    }

    /**
     * @notice Removes one caller from the block-cap whitelist.
     * @param account Caller to remove from whitelist.
     */
    function removeFromBlockHCUWhitelist(address account) external onlyACLOwner {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if (!$.blockHCUWhitelist[account]) revert NotBlockHCUWhitelisted(account);
        $.blockHCUWhitelist[account] = false;
        emit BlockHCUWhitelistRemoved(account);
    }

    /**
     * @notice Sets the cross-transaction depth limit and how much carried depth decays per block.
     * @dev (0, 0) disables it. Decay must be nonzero when enabled, or a chain that reaches the limit stays
     *      blocked forever. Permissions granted while disabled carry no depth.
     */
    function setCrossTxDepthLimit(uint48 maxDepth, uint48 decayPerBlock) external onlyACLOwner {
        if ((maxDepth == 0) != (decayPerBlock == 0)) revert InvalidCrossTxDepthLimit();
        HCULimitStorage storage $ = _getHCULimitStorage();
        $.maxCrossTxDepth = maxDepth;
        $.crossTxDepthDecayPerBlock = decayPerBlock;
        emit CrossTxDepthLimitSet(maxDepth, decayPerBlock);
    }

    function getCrossTxDepthLimit() external view returns (uint48 maxDepth, uint48 decayPerBlock) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        return ($.maxCrossTxDepth, $.crossTxDepthDecayPerBlock);
    }

    /**
     * @notice Returns the depth that `handle` carries into later transactions, loading it from
     *         `holder`'s permission if this transaction has not seen it yet. Returns 0 when disabled.
     */
    function loadDepth(bytes32 handle, address holder) external returns (uint48) {
        // Checked before the caller: when disabled this is a no-op for any bridge deployment.
        if (_getHCULimitStorage().maxCrossTxDepth == 0) return 0;
        if (msg.sender != aclAdd && msg.sender != confidentialBridgeAdd) revert CallerMustBeACLOrBridge();
        return uint48(_handleDepths(handle, holder) >> 128);
    }

    /**
     * @notice Sets the depths that bridged handles carry from their source chain.
     * @dev Each depth decays from `receivedBlock`, the block where the message arrived.
     */
    function setBridgedDepths(bytes32[] calldata handles, uint48[] calldata depths, uint48 receivedBlock) external {
        // Checked before the caller: when disabled this is a no-op for any bridge deployment.
        if (_getHCULimitStorage().maxCrossTxDepth == 0) return;
        if (msg.sender != confidentialBridgeAdd) revert CallerMustBeConfidentialBridge();
        for (uint256 i = 0; i < handles.length; i++) {
            bytes32 handle = handles[i];
            uint256 word = DEPTH_KNOWN | (_decayed(depths[i], receivedBlock) << 128);
            assembly {
                tstore(handle, word)
            }
        }
    }

    /// @dev An operation adds its HCU to both depths.
    function _opDepths(uint256 opHCU) private pure returns (uint256) {
        return opHCU | (opHCU << 128);
    }

    /// @dev Returns `depth` minus the decay accrued since `sinceBlock`, floored at 0.
    function _decayed(uint48 depth, uint48 sinceBlock) private view returns (uint256) {
        uint256 decay = (block.number - sinceBlock) * _getHCULimitStorage().crossTxDepthDecayPerBlock;
        return decay >= depth ? 0 : depth - decay;
    }

    /**
     * @dev Returns both depths of `handle`. A handle first seen in this transaction has no depth within it;
     *      its carried depth comes from `holder`'s permission, except for verified inputs, which start new chains.
     */
    function _handleDepths(bytes32 handle, address holder) private returns (uint256 depths) {
        // Handle slot zero is reserved for _getHCUForTransaction.
        if (handle == bytes32(0)) revert InvalidZeroHandle();
        assembly {
            depths := tload(handle)
        }
        if (depths & DEPTH_KNOWN != 0) return depths & ~DEPTH_KNOWN;
        // Byte 21 is 0xff only for computed handles; verified inputs carry their index there.
        if (uint8(handle[21]) != 0xff || _getHCULimitStorage().maxCrossTxDepth == 0) return 0;
        (bool allowed, uint48 storedDepth, uint48 storedBlock) = ACL(aclAdd).getStoredDepth(handle, holder);
        if (!allowed) revert UnknownHandleDepth(handle);
        depths = _decayed(storedDepth, storedBlock) << 128;
        uint256 word = depths | DEPTH_KNOWN;
        assembly {
            tstore(handle, word)
        }
    }

    /**
     * @notice Adjusts the sequential HCU for the transaction.
     */
    function _adjustAndCheckFheTransactionLimitOneOp(
        uint256 opHCU,
        address caller,
        bytes32 op1,
        bytes32 result
    ) internal virtual {
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);

        uint256 depths = _opDepths(opHCU) + _handleDepths(op1, caller);
        if (uint128(depths) > uint256(_getHCULimitStorage().maxHCUDepthPerTx)) {
            revert HCUTransactionDepthLimitExceeded();
        }

        _setHCUForHandle(result, depths);
    }

    /**
     * @notice Adjusts the current HCU for the transaction.
     */
    function _adjustAndCheckFheTransactionLimitTwoOps(
        uint256 opHCU,
        address caller,
        bytes32 op1,
        bytes32 op2,
        bytes32 result
    ) internal virtual {
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);

        uint256 depths = _opDepths(opHCU) + _maxDepths(_handleDepths(op1, caller), _handleDepths(op2, caller));
        if (uint128(depths) > uint256(_getHCULimitStorage().maxHCUDepthPerTx)) {
            revert HCUTransactionDepthLimitExceeded();
        }

        _setHCUForHandle(result, depths);
    }

    /**
     * @notice Adjusts the current HCU for the transaction.
     */
    function _adjustAndCheckFheTransactionLimitThreeOps(
        uint256 opHCU,
        address caller,
        bytes32 op1,
        bytes32 op2,
        bytes32 op3,
        bytes32 result
    ) internal virtual {
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);

        uint256 depths = _opDepths(opHCU) +
            _maxDepths(_handleDepths(op1, caller), _maxDepths(_handleDepths(op2, caller), _handleDepths(op3, caller)));

        if (uint128(depths) > uint256(_getHCULimitStorage().maxHCUDepthPerTx)) {
            revert HCUTransactionDepthLimitExceeded();
        }

        _setHCUForHandle(result, depths);
    }

    /**
     * @notice Updates and verifies the HCU transaction limit.
     * @param opHCU The HCU for the operation.
     * @param caller Original caller address for block-level checks.
     */
    function _updateAndVerifyHCUTransactionLimit(uint256 opHCU, address caller) internal virtual {
        _updateAndVerifyHCUBlockLimit(opHCU, caller);

        uint256 transactionHCU = opHCU + _getHCUForTransaction();
        if (transactionHCU > uint256(_getHCULimitStorage().maxHCUPerTx)) {
            revert HCUTransactionLimitExceeded();
        }
        _setHCUForTransaction(transactionHCU);
    }

    /**
     * @notice Updates and enforces the public block HCU cap for one operation.
     * @dev No-op if caller is whitelisted.
     * @param opHCU HCU cost of the current operation.
     * @param caller Original dapp caller address from FHEVMExecutor.
     */
    function _updateAndVerifyHCUBlockLimit(uint256 opHCU, address caller) internal virtual {
        HCULimitStorage storage $ = _getHCULimitStorage();

        if ($.blockHCUWhitelist[caller]) {
            return;
        }

        uint48 currentBlock = uint48(block.number);
        uint48 storedHCU = $.usedBlockHCU;
        if ($.lastSeenBlockNumber != currentBlock) {
            storedHCU = 0;
        }

        uint256 nextHCU = uint256(storedHCU) + opHCU;
        if (nextHCU > uint256($.globalHCUCapPerBlock)) {
            revert HCUBlockLimitExceeded();
        }
        $.usedBlockHCU = uint48(nextHCU);
        $.lastSeenBlockNumber = currentBlock;
    }

    /**
     * @notice Gets the total HCU for the transaction.
     * @return transactionHCU The HCU for the transaction.
     * @dev This function uses inline assembly to store the HCU in a specific storage location.
     */
    function _getHCUForTransaction() internal view virtual returns (uint256 transactionHCU) {
        assembly {
            transactionHCU := tload(0)
        }
    }

    /**
     * @notice Sets the HCU for a handle in the transient storage.
     * @param handle The handle for which to set the HCU.
     * @param handleHCU The HCU to set for the handle.
     * @dev This function uses inline assembly to store the HCU in a specific transient storage slot.
     */
    function _setHCUForHandle(bytes32 handle, uint256 handleHCU) internal virtual {
        // Handle slot zero is reserved for _setHCUForTransaction.
        if (handle == bytes32(0)) revert InvalidZeroHandle();
        uint48 maxDepth = _getHCULimitStorage().maxCrossTxDepth;
        if (maxDepth != 0 && handleHCU >> 128 > maxDepth) revert HCUCrossTxDepthLimitExceeded();
        handleHCU |= DEPTH_KNOWN;
        assembly {
            tstore(handle, handleHCU)
        }
    }

    /**
     * @notice Updates the current HCU consumption for the transaction and stores it in the transient storage.
     * @param transactionHCU The total HCU for the transaction.
     * @dev This function uses inline assembly to store the HCU in a specific transient storage slot.
     */
    function _setHCUForTransaction(uint256 transactionHCU) internal virtual {
        assembly {
            tstore(0, transactionHCU) // to avoid collisions with handles (see _setHCUForHandle)
        }
    }

    /**
     * @notice Sets the global HCU cap per block.
     * @param hcuPerBlock New cap value.
     * @dev Enforces hcuPerBlock >= maxHCUPerTx.
     */
    function _setHCUPerBlock(uint48 hcuPerBlock) private {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if (hcuPerBlock < $.maxHCUPerTx) revert HCUPerBlockBelowMaxPerTx();
        $.globalHCUCapPerBlock = hcuPerBlock;
        emit HCUPerBlockSet(hcuPerBlock);
    }

    /**
     * @notice Sets the per-transaction HCU depth limit.
     * @param maxHCUDepthPerTx New depth limit.
     * @dev Enforces maxHCUPerTx >= maxHCUDepthPerTx.
     */
    function _setMaxHCUDepthPerTx(uint48 maxHCUDepthPerTx) private {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.maxHCUPerTx < maxHCUDepthPerTx) revert MaxHCUPerTxBelowDepth();
        $.maxHCUDepthPerTx = maxHCUDepthPerTx;
        emit MaxHCUDepthPerTxSet(maxHCUDepthPerTx);
    }

    /**
     * @notice Sets the per-transaction HCU limit.
     * @param maxHCUPerTx New transaction limit.
     * @dev Enforces hcuPerBlock >= maxHCUPerTx and maxHCUPerTx >= maxHCUDepthPerTx.
     */
    function _setMaxHCUPerTx(uint48 maxHCUPerTx) private {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.globalHCUCapPerBlock < maxHCUPerTx) revert HCUPerBlockBelowMaxPerTx();
        if (maxHCUPerTx < $.maxHCUDepthPerTx) revert MaxHCUPerTxBelowDepth();
        $.maxHCUPerTx = maxHCUPerTx;
        emit MaxHCUPerTxSet(maxHCUPerTx);
    }

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}

    /// @dev Takes the maximum of each depth separately: the deepest branch in this transaction may carry less.
    function _maxDepths(uint256 a, uint256 b) private pure returns (uint256) {
        uint256 txDepth = uint128(a) >= uint128(b) ? uint128(a) : uint128(b);
        uint256 carried = a >> 128 >= b >> 128 ? a >> 128 : b >> 128;
        return txDepth | (carried << 128);
    }

    /**
     * @notice Getter function for the FHEVMExecutor contract address.
     * @return fhevmExecutorAddress Address of the FHEVMExecutor.
     */
    function getFHEVMExecutorAddress() public view virtual returns (address) {
        return FHEVM_EXECUTOR_ADDRESS;
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
     * @notice Returns the global block HCU cap.
     */
    function getGlobalHCUCapPerBlock() public view virtual returns (uint48) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        return $.globalHCUCapPerBlock;
    }

    /**
     * @notice Returns the per-transaction HCU depth limit.
     */
    function getMaxHCUDepthPerTx() public view virtual returns (uint48) {
        return _getHCULimitStorage().maxHCUDepthPerTx;
    }

    /**
     * @notice Returns the per-transaction HCU limit.
     */
    function getMaxHCUPerTx() public view virtual returns (uint48) {
        return _getHCULimitStorage().maxHCUPerTx;
    }

    /**
     * @notice Returns whether a caller bypasses the global block HCU cap.
     * @param account Caller address.
     */
    function isBlockHCUWhitelisted(address account) public view virtual returns (bool) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        return $.blockHCUWhitelist[account];
    }

    /**
     * @notice Returns the effective public block HCU meter for the current block.
     * @dev If storage still contains a previous block meter, returns `(block.number, 0)`.
     */
    function getBlockMeter() external view returns (uint48 blockNumber, uint48 usedHCU) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint48 currentBlock = uint48(block.number);
        if ($.lastSeenBlockNumber != currentBlock) {
            return (currentBlock, 0);
        }
        return (currentBlock, $.usedBlockHCU);
    }
}
