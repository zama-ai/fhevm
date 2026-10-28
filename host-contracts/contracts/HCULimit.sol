// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {fhevmExecutorAdd, aclAdd} from "../addresses/FHEVMHostAddresses.sol";
import {ACL} from "./ACL.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";

import {FheType} from "./shared/FheType.sol";

/**
 * @title HCULimit
 * @notice This contract manages the total allowed complexity for FHE operations at the
 * transaction level, the HCU depth carried across transactions, and the HCU quotas per block.
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

    /// @notice Returned when an operation would take a quota's usage over its maximum.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    /// @param retryAfterBlocks Blocks of decay after which this operation fits, if nothing else spends the quota;
    ///        type(uint256).max if the operation alone exceeds the maximum. When the chain's load is also above the
    ///        tier's threshold, the longer of the two waits. Later operations of the same transaction can need more,
    ///        so clients simulate again after waiting.
    error HCUQuotaExceeded(address account, uint16 quotaId, uint256 retryAfterBlocks);

    /// @notice Returned when the operation fits the contract's quota, but would take the chain's load over the threshold
    ///         of the quota's tier.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    /// @param retryAfterBlocks Blocks of decay after which this operation fits, if no contract adds load meanwhile;
    ///        type(uint256).max if the operation alone exceeds the threshold. Other contracts keep adding load, so
    ///        the wait can be longer.
    error HCUChainLoadTooHigh(address account, uint16 quotaId, uint256 retryAfterBlocks);

    /// @notice Returned when the contract's quota has a maximum of 0. Retrying fails until the contract is moved
    ///         or the owner resizes the quota.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    error HCUQuotaStopped(address account, uint16 quotaId);

    /// @notice Returned when the contract's quota is paused. Retrying fails until the owner unpauses it.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    error HCUQuotaPaused(address account, uint16 quotaId);

    /// @notice Returned when the sender is neither the ACL owner nor a pauser.
    error NotPauser(address sender);

    /// @notice Returned when the sender is not the admin of the quota.
    error NotQuotaAdmin(uint16 quotaId, address sender);

    /// @notice Returned when a quota admin moves a contract into or out of a paused quota.
    error QuotaMembershipPaused(uint16 quotaId);

    /// @notice Returned when the contract is not in the quota it is moved out of.
    error ContractNotInQuota(address account, uint16 quotaId);

    /// @notice Returned when a quota admin adds a contract while the default quota's maximum is above 0: the
    ///         contract may then have more room in the default quota than in the admin's quota.
    error DefaultQuotaNotStopped();

    /// @notice Returned when:
    ///         - only one of a quota's maximum and decay is 0;
    ///         - a contract is assigned or granted to a quota whose maximum is 0, other than the default quota;
    ///         - an admin is set for the default quota.
    error InvalidQuota(uint16 quotaId);

    /// @notice Returned when only one of the capacity's maximum and decay is 0.
    error InvalidCapacity();

    /// @notice Returned when a tier's threshold is above 10000, or a quota is moved to a tier whose threshold is 0.
    error InvalidTier(uint8 tier);

    /// @notice Returned if the transaction exceeds the maximum allowed homomorphic complexity units.
    error HCUTransactionLimitExceeded();

    /// @notice Returned if the transaction exceeds the maximum allowed depth of homomorphic complexity units.
    error HCUTransactionDepthLimitExceeded();

    /// @notice Returned if maxHCUPerTx < maxHCUDepthPerTx.
    error MaxHCUPerTxBelowDepth();

    /// @notice Returned if the operation is not supported.
    error UnsupportedOperation();

    /// @notice Returned if the operation is not scalar.
    error OnlyScalarOperationsAreSupported();

    /// @notice Returned if a handle is the zero handle.
    /// @dev Handle slot zero is reserved for the transaction HCU accumulator.
    error InvalidZeroHandle();

    /// @notice Returned when the caller is not the ACL contract.
    error CallerMustBeACLContract();

    /// @notice Returned when only one of the cross-transaction depth limit and its decay is zero.
    error InvalidCrossTxDepthLimit();

    /// @notice Returned if a nonzero cross-transaction depth limit is below maxHCUDepthPerTx.
    error CrossTxDepthBelowMaxDepthPerTx();

    /// @notice Returned when an operation's depth carried across transactions exceeds the limit.
    /// @param retryAfterBlocks Blocks of decay after which this operation fits, if nothing else extends its chain.
    ///        Later operations of the same transaction can need more, so clients simulate again after waiting.
    error HCUCrossTxDepthLimitExceeded(uint256 retryAfterBlocks);

    /// @notice Emitted when the cross-transaction depth limit is updated.
    event CrossTxDepthLimitSet(uint48 maxDepth, uint48 decayPerBlock);

    /// @notice Emitted when a quota's maximum and decay are set.
    event QuotaSet(uint16 indexed quotaId, uint48 maxHCU, uint48 decayPerBlock, address indexed setBy);

    /// @notice Emitted when the owner names a quota's admin; address(0) removes it.
    event QuotaAdminSet(uint16 indexed quotaId, address indexed admin);

    /// @notice Emitted when a quota is paused or unpaused.
    event QuotaPauseSet(uint16 indexed quotaId, bool paused, address indexed setBy);

    /// @notice Emitted when the chain's capacity is set.
    event CapacitySet(uint48 maxHCU, uint48 decayPerBlock, address indexed setBy);

    /// @notice Emitted when a tier's load threshold is set.
    event TierSet(uint8 indexed tier, uint16 maxLoadBps, address indexed setBy);

    /// @notice Emitted when a quota is moved to a tier.
    event QuotaTierSet(uint16 indexed quotaId, uint8 indexed tier, address indexed setBy);

    /// @notice Emitted when a contract is assigned to a quota, by the owner or the quota's admin.
    event ContractQuotaAssigned(uint16 indexed quotaId, address indexed account, address indexed setBy);

    /// @notice Emitted when the per-transaction HCU depth limit is updated.
    /// @param maxHCUDepthPerTx New depth limit.
    event MaxHCUDepthPerTxSet(uint48 maxHCUDepthPerTx);

    /// @notice Emitted when the per-transaction HCU limit is updated.
    /// @param maxHCUPerTx New transaction limit.
    event MaxHCUPerTxSet(uint48 maxHCUPerTx);

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

    /// @notice Quota of every contract that was never assigned.
    uint16 private constant DEFAULT_QUOTA = 0;

    /// @notice A quota or capacity with this maximum is not metered.
    uint48 private constant NO_CAP = type(uint48).max;

    /// @notice Tier of every quota never moved to another tier.
    uint8 private constant DEFAULT_TIER = 0;

    /// @notice A tier threshold of this many basis points admits operations until the capacity is full.
    uint16 private constant MAX_BPS = 10_000;

    /// @dev Per-handle transient word: bit 255 marks the depths as known, bits 128-254 hold the
    ///      depth carried across transactions and bits 0-127 the depth within this transaction.
    uint256 private constant DEPTH_KNOWN = 1 << 255;

    /// `fheMulDiv` `scalarByte` bitmask: bit 0 (divisor) is always set; bit 1 marks `factor2` as scalar.
    bytes1 private constant FHE_MUL_DIV_FACTOR2_ENCRYPTED = 0x01;
    bytes1 private constant FHE_MUL_DIV_FACTOR2_SCALAR = 0x03;

    /// @dev Limit, meter, pause flag and tier of one quota, in one slot. All contracts assigned to the quota share it.
    ///      Usage decays by decayPerBlock every block, as carried depth does, and an operation fits while usage stays at
    ///      most maxHCU.
    struct Quota {
        uint48 maxHCU;
        uint48 decayPerBlock;
        uint48 usedHCU;
        uint48 lastBlock;
        bool paused;
        uint8 tier;
    }

    /// @dev Limit and meter of all FHE work on this chain, in one slot. Every operation charges it, whatever its quota,
    ///      and the load decays as a quota's usage does.
    struct Capacity {
        uint48 maxHCU;
        uint48 decayPerBlock;
        uint48 usedHCU;
        uint48 lastBlock;
    }

    /// @custom:storage-location erc7201:fhevm.storage.HCULimit
    /// @dev The first five uint48 fields pack into a single 256-bit slot (5 × 48 = 240 bits). The
    ///      cross-transaction fields and the quotas are appended after the mapping to keep the existing layout.
    struct HCULimitStorage {
        /// @notice Unused since quotas replaced the global block cap; reinitializeV5 copies it into the default quota.
        uint48 globalHCUCapPerBlock;
        /// @notice Unused since quotas replaced the global block cap.
        uint48 usedBlockHCU;
        /// @notice Unused since quotas replaced the global block cap.
        uint48 lastSeenBlockNumber;
        /// @notice Maximum sequential HCU depth per transaction.
        uint48 maxHCUDepthPerTx;
        /// @notice Maximum total HCU per transaction.
        uint48 maxHCUPerTx;
        /// @notice Unused since quotas replaced the block cap whitelist, which had no entries.
        mapping(address account => bool isWhitelisted) blockHCUWhitelist;
        /// @notice Maximum depth carried across transactions; 0 disables the check.
        uint48 maxCrossTxDepth;
        /// @notice Carried depth the coprocessors are assumed to drain per block.
        uint48 crossTxDepthDecayPerBlock;
        /// @notice Cap and meter of each quota.
        mapping(uint16 quotaId => Quota) quotas;
        /// @notice Quota of each contract; the default quota when never assigned.
        mapping(address account => uint16 quotaId) contractQuota;
        /// @notice Address that moves contracts between the default quota and each quota; address(0) when none.
        mapping(uint16 quotaId => address admin) quotaAdmins;
        /// @notice HCU load the coprocessors absorb on this chain, metered across all quotas.
        Capacity capacity;
        /// @notice Load, in basis points of the capacity's maximum, above which each tier's operations are refused.
        mapping(uint8 tier => uint16 maxLoadBps) tierMaxLoadBps;
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

    /// @dev Pausers are the ACL's pausers (PauserSet).
    modifier onlyACLOwnerOrPauser() {
        if (msg.sender != ACL(aclAdd).owner() && !ACL(aclAdd).isPauser(msg.sender)) revert NotPauser(msg.sender);
        _;
    }

    /// @dev A paused quota's membership is frozen for its admin; the owner can still move contracts with assignQuota.
    modifier onlyQuotaAdmin(uint16 quotaId) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if (msg.sender != $.quotaAdmins[quotaId]) revert NotQuotaAdmin(quotaId, msg.sender);
        if ($.quotas[quotaId].paused) revert QuotaMembershipPaused(quotaId);
        _;
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract.
     * @param defaultQuotaHCUPerBlock Maximum and decay per block of the default quota; type(uint48).max leaves it unmetered.
     * @param maxHCUDepthPerTx Maximum sequential HCU depth per transaction.
     * @param maxHCUPerTx Maximum total HCU per transaction.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        uint48 defaultQuotaHCUPerBlock,
        uint48 maxHCUDepthPerTx,
        uint48 maxHCUPerTx
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        _setQuota(DEFAULT_QUOTA, defaultQuotaHCUPerBlock, defaultQuotaHCUPerBlock);
        _setCapacity(NO_CAP, NO_CAP);
        _setTier(DEFAULT_TIER, MAX_BPS);
        _setMaxHCUPerTx(maxHCUPerTx);
        _setMaxHCUDepthPerTx(maxHCUDepthPerTx);
    }

    /**
     * @notice Re-initializes the contract from V4.
     * @dev Quotas replace the global block cap. The default quota gets it as maximum and decay per block,
     *      which allows the same HCU per block. The capacity starts unmetered and the default tier admits up to full
     *      capacity, so tiers change nothing until the owner sets the capacity.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV5() public virtual reinitializer(REINITIALIZER_VERSION) {
        uint48 globalCap = _getHCULimitStorage().globalHCUCapPerBlock;
        _setQuota(DEFAULT_QUOTA, globalCap, globalCap);
        _setCapacity(NO_CAP, NO_CAP);
        _setTier(DEFAULT_TIER, MAX_BPS);
    }

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
        _setResultDepths(result, opHCU, 0);
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
        _setResultDepths(result, opHCU, 0);
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
        _setResultDepths(result, opHCU, 0);
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

        _setResultDepths(result, opHCU, inputDepths);
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

        _setResultDepths(result, opHCU, inputDepths);
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
     * @notice Sets a quota's maximum usage and how much usage decays per block, like setCrossTxDepthLimit.
     *         A maximum of type(uint48).max leaves the quota unmetered; (0, 0) stops it.
     */
    function setQuota(uint16 quotaId, uint48 maxHCU, uint48 decayPerBlock) external onlyACLOwner {
        _setQuota(quotaId, maxHCU, decayPerBlock);
    }

    /**
     * @notice Sets the HCU load the coprocessors absorb on this chain and how much they drain per block. Every
     *         operation charges this load, whatever its quota. A maximum of type(uint48).max leaves it unmetered;
     *         (0, 0) stops every operation.
     */
    function setCapacity(uint48 maxHCU, uint48 decayPerBlock) external onlyACLOwner {
        _setCapacity(maxHCU, decayPerBlock);
    }

    /**
     * @notice Sets the load above which a tier's operations are refused, in basis points of the capacity's maximum.
     *         10000 admits operations until the capacity is full; 0 stops the tier.
     */
    function setTier(uint8 tier, uint16 maxLoadBps) external onlyACLOwner {
        _setTier(tier, maxLoadBps);
    }

    /**
     * @notice Moves a quota to a tier whose threshold is above 0.
     */
    function setQuotaTier(uint16 quotaId, uint8 tier) external onlyACLOwner {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.tierMaxLoadBps[tier] == 0) revert InvalidTier(tier);
        $.quotas[quotaId].tier = tier;
        emit QuotaTierSet(quotaId, tier, msg.sender);
    }

    /**
     * @notice Names the address that moves contracts between the default quota and this quota.
     *         address(0) removes the admin.
     */
    function setQuotaAdmin(uint16 quotaId, address admin) external onlyACLOwner {
        if (quotaId == DEFAULT_QUOTA) revert InvalidQuota(quotaId);
        _getHCULimitStorage().quotaAdmins[quotaId] = admin;
        emit QuotaAdminSet(quotaId, admin);
    }

    /**
     * @notice Moves a contract to the default quota or to a quota whose maximum is above 0, from any quota,
     *         including a paused one.
     */
    function assignQuota(uint16 quotaId, address account) external onlyACLOwner {
        if (quotaId != DEFAULT_QUOTA && _getHCULimitStorage().quotas[quotaId].maxHCU == 0) revert InvalidQuota(quotaId);
        _assignQuota(quotaId, account);
    }

    /**
     * @notice Moves a contract from the default quota into the admin's quota.
     * @dev Only while the default quota is stopped: a contract then gains room by joining any quota, so an admin
     *      cannot starve a contract it does not own.
     */
    function grantQuota(uint16 quotaId, address account) external onlyQuotaAdmin(quotaId) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.quotas[DEFAULT_QUOTA].maxHCU != 0) revert DefaultQuotaNotStopped();
        if ($.quotas[quotaId].maxHCU == 0) revert InvalidQuota(quotaId);
        if ($.contractQuota[account] != DEFAULT_QUOTA) revert ContractNotInQuota(account, DEFAULT_QUOTA);
        _assignQuota(quotaId, account);
    }

    /**
     * @notice Moves a contract from the admin's quota back to the default quota.
     */
    function revokeQuota(uint16 quotaId, address account) external onlyQuotaAdmin(quotaId) {
        if (_getHCULimitStorage().contractQuota[account] != quotaId) revert ContractNotInQuota(account, quotaId);
        _assignQuota(DEFAULT_QUOTA, account);
    }

    /**
     * @notice Stops every FHE operation from the quota's contracts and stops its admin from moving contracts,
     *         keeping its maximum, decay and contracts. Only the owner unpauses.
     */
    function pauseQuota(uint16 quotaId) external onlyACLOwnerOrPauser {
        _setQuotaPaused(quotaId, true);
    }

    /**
     * @notice Lifts a quota's pause.
     */
    function unpauseQuota(uint16 quotaId) external onlyACLOwner {
        _setQuotaPaused(quotaId, false);
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
     * @notice Sets the cross-transaction depth limit and how much carried depth decays per block.
     * @dev (0, 0) disables it. Decay must be nonzero when enabled, or a chain that reaches the limit stays
     *      blocked forever. The limit must be at least maxHCUDepthPerTx, so a transaction whose inputs carry no
     *      depth is never refused. Permissions granted while disabled carry no depth.
     */
    function setCrossTxDepthLimit(uint48 maxDepth, uint48 decayPerBlock) external onlyACLOwner {
        if ((maxDepth == 0) != (decayPerBlock == 0)) revert InvalidCrossTxDepthLimit();
        HCULimitStorage storage $ = _getHCULimitStorage();
        if (maxDepth != 0) {
            if (maxDepth < $.maxHCUDepthPerTx) revert CrossTxDepthBelowMaxDepthPerTx();
            // Every operation on a stored handle reads ACL.getStoredDepth: refuse to enable before ACL has it.
            ACL(aclAdd).getStoredDepth(bytes32(0), address(0));
        }
        $.maxCrossTxDepth = maxDepth;
        $.crossTxDepthDecayPerBlock = decayPerBlock;
        emit CrossTxDepthLimitSet(maxDepth, decayPerBlock);
    }

    function getCrossTxDepthLimit() external view returns (uint48 maxDepth, uint48 decayPerBlock) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        return ($.maxCrossTxDepth, $.crossTxDepthDecayPerBlock);
    }

    /**
     * @notice Caches `handle`'s carried depth for the rest of this transaction and returns it. If this
     *         transaction has not seen the handle yet, the depth comes from `holder`'s permission.
     *         Returns 0 when disabled.
     */
    function cacheDepth(bytes32 handle, address holder) external returns (uint48) {
        if (msg.sender != aclAdd) revert CallerMustBeACLContract();
        if (_getHCULimitStorage().maxCrossTxDepth == 0) return 0;
        return uint48(_handleDepths(handle, holder) >> 128);
    }

    /// @dev Records `result`'s depths: `opHCU` added to both depths of its deepest input, checked against both limits.
    function _setResultDepths(bytes32 result, uint256 opHCU, uint256 inputDepths) private {
        uint256 depths = (opHCU | (opHCU << 128)) + inputDepths;
        HCULimitStorage storage $ = _getHCULimitStorage();
        if (uint128(depths) > $.maxHCUDepthPerTx) revert HCUTransactionDepthLimitExceeded();
        uint256 maxDepth = $.maxCrossTxDepth;
        uint256 carried = depths >> 128;
        if (maxDepth != 0 && carried > maxDepth) {
            revert HCUCrossTxDepthLimitExceeded(_blocksToDecay(carried - maxDepth, $.crossTxDepthDecayPerBlock));
        }
        _setHCUForHandle(result, depths | DEPTH_KNOWN);
    }

    /// @dev Returns `value` minus `decayPerBlock` for each block since `sinceBlock`, floored at 0.
    ///      Carried depth and quota usage both decay this way.
    function _decayed(uint48 value, uint48 sinceBlock, uint256 decayPerBlock) private view returns (uint256) {
        uint256 decay = (block.number - sinceBlock) * decayPerBlock;
        return decay >= value ? 0 : value - decay;
    }

    /// @dev Returns the blocks of decay needed to absorb `excess`, rounded up.
    function _blocksToDecay(uint256 excess, uint256 decayPerBlock) private pure returns (uint256) {
        return (excess + decayPerBlock - 1) / decayPerBlock;
    }

    /// @dev Returns a meter's usage after decay plus `opHCU`, and the blocks of decay before that fits `limit`:
    ///      0 when it fits now, type(uint256).max when `opHCU` alone exceeds `limit`.
    function _meter(
        uint48 usedHCU,
        uint48 lastBlock,
        uint48 decayPerBlock,
        uint256 limit,
        uint256 opHCU
    ) private view returns (uint256 used, uint256 retryAfterBlocks) {
        used = _decayed(usedHCU, lastBlock, decayPerBlock) + opHCU;
        if (used > limit) {
            retryAfterBlocks = opHCU > limit ? type(uint256).max : _blocksToDecay(used - limit, decayPerBlock);
        }
    }

    /**
     * @dev Returns both depths of `handle`. A handle first seen in this transaction has no depth within it;
     *      its carried depth is the one stored with `holder`'s permission, or 0 without one: verified inputs,
     *      bridged handles and permissions granted while disabled start new chains.
     */
    function _handleDepths(bytes32 handle, address holder) private returns (uint256 depths) {
        // Handle slot zero is reserved for _getHCUForTransaction.
        if (handle == bytes32(0)) revert InvalidZeroHandle();
        assembly {
            depths := tload(handle)
        }
        if (depths & DEPTH_KNOWN != 0) return depths & ~DEPTH_KNOWN;
        if (_getHCULimitStorage().maxCrossTxDepth == 0) return 0;
        (uint48 storedDepth, uint48 storedBlock) = ACL(aclAdd).getStoredDepth(handle, holder);
        depths = _decayed(storedDepth, storedBlock, _getHCULimitStorage().crossTxDepthDecayPerBlock) << 128;
        _setHCUForHandle(handle, depths | DEPTH_KNOWN);
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
        _setResultDepths(result, opHCU, _handleDepths(op1, caller));
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
        _setResultDepths(result, opHCU, _maxDepths(_handleDepths(op1, caller), _handleDepths(op2, caller)));
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
        _setResultDepths(
            result,
            opHCU,
            _maxDepths(_handleDepths(op1, caller), _maxDepths(_handleDepths(op2, caller), _handleDepths(op3, caller)))
        );
    }

    /**
     * @notice Updates and verifies the HCU transaction limit.
     * @param opHCU The HCU for the operation.
     * @param caller Contract that runs the operation, charged to its quota.
     */
    function _updateAndVerifyHCUTransactionLimit(uint256 opHCU, address caller) internal virtual {
        _updateAndVerifyHCUQuota(opHCU, caller);

        uint256 transactionHCU = opHCU + _getHCUForTransaction();
        if (transactionHCU > uint256(_getHCULimitStorage().maxHCUPerTx)) {
            revert HCUTransactionLimitExceeded();
        }
        _setHCUForTransaction(transactionHCU);
    }

    /**
     * @notice Charges one operation to the quota of the contract that runs it and to the chain's capacity. The
     *         operation must fit the quota's maximum and keep the chain's load within the threshold of the quota's tier.
     * @dev Checks both meters before writing either, so a refused operation spends neither.
     * @param opHCU HCU cost of the current operation.
     * @param caller Contract that calls FHEVMExecutor.
     */
    function _updateAndVerifyHCUQuota(uint256 opHCU, address caller) internal virtual {
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint16 quotaId = $.contractQuota[caller];
        Quota storage quota = $.quotas[quotaId];
        if (quota.paused) revert HCUQuotaPaused(caller, quotaId);
        if (quota.maxHCU == 0) revert HCUQuotaStopped(caller, quotaId);
        uint256 used;
        uint256 quotaWait;
        if (quota.maxHCU != NO_CAP) {
            (used, quotaWait) = _meter(quota.usedHCU, quota.lastBlock, quota.decayPerBlock, quota.maxHCU, opHCU);
        }
        // An unmetered quota still charges the capacity, so the capacity sees all load.
        Capacity storage capacity = $.capacity;
        uint256 load;
        uint256 loadWait;
        if (capacity.maxHCU != NO_CAP) {
            uint256 threshold = (uint256(capacity.maxHCU) * $.tierMaxLoadBps[quota.tier]) / MAX_BPS;
            (load, loadWait) = _meter(capacity.usedHCU, capacity.lastBlock, capacity.decayPerBlock, threshold, opHCU);
        }
        if (quotaWait != 0) revert HCUQuotaExceeded(caller, quotaId, quotaWait > loadWait ? quotaWait : loadWait);
        if (loadWait != 0) revert HCUChainLoadTooHigh(caller, quotaId, loadWait);
        if (quota.maxHCU != NO_CAP) {
            quota.usedHCU = uint48(used);
            quota.lastBlock = uint48(block.number);
        }
        if (capacity.maxHCU != NO_CAP) {
            capacity.usedHCU = uint48(load);
            capacity.lastBlock = uint48(block.number);
        }
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

    function _setQuota(uint16 quotaId, uint48 maxHCU, uint48 decayPerBlock) private {
        // Both 0 stops the quota. A maximum without decay would never free up, and a decay without maximum means nothing.
        bool stopped = maxHCU == 0;
        if (stopped != (decayPerBlock == 0)) revert InvalidQuota(quotaId);
        Quota storage quota = _getHCULimitStorage().quotas[quotaId];
        quota.maxHCU = maxHCU;
        quota.decayPerBlock = decayPerBlock;
        emit QuotaSet(quotaId, maxHCU, decayPerBlock, msg.sender);
    }

    function _setCapacity(uint48 maxHCU, uint48 decayPerBlock) private {
        // Same rule as a quota: both 0 stops every operation.
        if ((maxHCU == 0) != (decayPerBlock == 0)) revert InvalidCapacity();
        Capacity storage capacity = _getHCULimitStorage().capacity;
        capacity.maxHCU = maxHCU;
        capacity.decayPerBlock = decayPerBlock;
        emit CapacitySet(maxHCU, decayPerBlock, msg.sender);
    }

    function _setTier(uint8 tier, uint16 maxLoadBps) private {
        if (maxLoadBps > MAX_BPS) revert InvalidTier(tier);
        _getHCULimitStorage().tierMaxLoadBps[tier] = maxLoadBps;
        emit TierSet(tier, maxLoadBps, msg.sender);
    }

    function _assignQuota(uint16 quotaId, address account) private {
        _getHCULimitStorage().contractQuota[account] = quotaId;
        emit ContractQuotaAssigned(quotaId, account, msg.sender);
    }

    function _setQuotaPaused(uint16 quotaId, bool paused) private {
        _getHCULimitStorage().quotas[quotaId].paused = paused;
        emit QuotaPauseSet(quotaId, paused, msg.sender);
    }

    /**
     * @notice Sets the per-transaction HCU depth limit.
     * @param maxHCUDepthPerTx New depth limit.
     * @dev Enforces maxHCUPerTx >= maxHCUDepthPerTx, and a nonzero maxCrossTxDepth >= maxHCUDepthPerTx.
     */
    function _setMaxHCUDepthPerTx(uint48 maxHCUDepthPerTx) private {
        HCULimitStorage storage $ = _getHCULimitStorage();
        if ($.maxHCUPerTx < maxHCUDepthPerTx) revert MaxHCUPerTxBelowDepth();
        if ($.maxCrossTxDepth != 0 && $.maxCrossTxDepth < maxHCUDepthPerTx) revert CrossTxDepthBelowMaxDepthPerTx();
        $.maxHCUDepthPerTx = maxHCUDepthPerTx;
        emit MaxHCUDepthPerTxSet(maxHCUDepthPerTx);
    }

    /**
     * @notice Sets the per-transaction HCU limit.
     * @param maxHCUPerTx New transaction limit.
     * @dev Enforces maxHCUPerTx >= maxHCUDepthPerTx.
     */
    function _setMaxHCUPerTx(uint48 maxHCUPerTx) private {
        HCULimitStorage storage $ = _getHCULimitStorage();
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
     * @notice Returns a quota's maximum usage, its decay per block and its current usage after decay.
     */
    function getQuota(
        uint16 quotaId
    ) public view virtual returns (uint48 maxHCU, uint48 decayPerBlock, uint48 usedHCU) {
        Quota storage quota = _getHCULimitStorage().quotas[quotaId];
        return (
            quota.maxHCU,
            quota.decayPerBlock,
            uint48(_decayed(quota.usedHCU, quota.lastBlock, quota.decayPerBlock))
        );
    }

    /**
     * @notice Returns the chain's capacity, its decay per block and its current load after decay.
     */
    function getCapacity() public view virtual returns (uint48 maxHCU, uint48 decayPerBlock, uint48 usedHCU) {
        Capacity storage capacity = _getHCULimitStorage().capacity;
        return (
            capacity.maxHCU,
            capacity.decayPerBlock,
            uint48(_decayed(capacity.usedHCU, capacity.lastBlock, capacity.decayPerBlock))
        );
    }

    /**
     * @notice Returns a tier's load threshold, in basis points of the capacity's maximum; 0 when stopped or never set.
     */
    function getTier(uint8 tier) public view virtual returns (uint16) {
        return _getHCULimitStorage().tierMaxLoadBps[tier];
    }

    /**
     * @notice Returns the tier of a quota.
     */
    function getQuotaTier(uint16 quotaId) public view virtual returns (uint8) {
        return _getHCULimitStorage().quotas[quotaId].tier;
    }

    /**
     * @notice Returns whether a quota is paused.
     */
    function isQuotaPaused(uint16 quotaId) public view virtual returns (bool) {
        return _getHCULimitStorage().quotas[quotaId].paused;
    }

    /**
     * @notice Returns a quota's admin; address(0) when it has none.
     */
    function getQuotaAdmin(uint16 quotaId) public view virtual returns (address) {
        return _getHCULimitStorage().quotaAdmins[quotaId];
    }

    /**
     * @notice Returns the quota a contract is assigned to; 0 is the default quota.
     */
    function getContractQuota(address account) public view virtual returns (uint16) {
        return _getHCULimitStorage().contractQuota[account];
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
}
