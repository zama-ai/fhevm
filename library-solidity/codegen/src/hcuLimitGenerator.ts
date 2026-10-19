import type { NBucketedCost, PriceData } from './common';

export function generateSolidityHCULimit(priceData: PriceData): string {
  if (!priceData) {
    throw new Error('undefined or null priceData');
  }

  let output = `// SPDX-License-Identifier: BSD-3-Clause-Clear
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
/// Deployed proxies retain residual \`_owner\` and \`_pendingOwner\` values in the
/// Ownable2StepUpgradeable EIP-7201 storage namespace. These slots are unused
/// by ACLOwnable and have no effect on contract behavior.
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract HCULimit is UUPSUpgradeableEmptyProxy, ACLOwnable {
    /// @notice Returned if the sender is not the FHEVMExecutor.
    error CallerMustBeFHEVMExecutorContract();

    /// @notice Returned when an operation would take a quota over its cap for this block.
    /// @param account Contract that ran the operation.
    /// @param quotaId Quota the contract is assigned to.
    error HCUQuotaExceeded(address account, uint16 quotaId);

    /// @notice Returned when the sender is neither the ACL owner nor a pauser.
    error NotPauser(address sender);

    /// @notice Returned when a pauser tries to raise a quota's cap or assign a contract to a quota other than the paused one.
    error QuotaNotTighter(uint16 quotaId);

    /// @notice Returned when setting the paused quota's cap, which stays 0, or when assigning a contract to
    ///         a quota whose cap is 0, which would pause it without saying so.
    error InvalidQuota(uint16 quotaId);

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

    /// @notice Emitted when a quota's cap per block is set.
    event QuotaCapSet(uint16 indexed quotaId, uint48 capPerBlock, address indexed setBy);

    /// @notice Emitted when a contract is assigned to a quota.
    event ContractQuotaAssigned(address indexed account, uint16 indexed quotaId, address indexed setBy);

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

    /// @notice Quota with cap 0: contracts assigned to it cannot run FHE operations.
    uint16 public constant PAUSED_QUOTA = type(uint16).max;

    /// @notice A quota with this cap is not metered.
    uint48 private constant NO_CAP = type(uint48).max;

    /// @dev Per-handle transient word: bit 255 marks the depths as known, bits 128-254 hold the
    ///      depth carried across transactions and bits 0-127 the depth within this transaction.
    uint256 private constant DEPTH_KNOWN = 1 << 255;

    /// \`fheMulDiv\` \`scalarByte\` bitmask: bit 0 (divisor) is always set; bit 1 marks \`factor2\` as scalar.
    bytes1 private constant FHE_MUL_DIV_FACTOR2_ENCRYPTED = 0x01;
    bytes1 private constant FHE_MUL_DIV_FACTOR2_SCALAR = 0x03;

    /// @dev Cap and meter of one quota, in one slot. All contracts assigned to the quota share it.
    struct Quota {
        uint48 capPerBlock;
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
    }

    /// Constant used for making sure the version number used in the \`reinitializer\` modifier is
    /// identical between \`initializeFromEmptyProxy\` and the \`reinitializeVX\` method
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
     * @param defaultQuotaCapPerBlock Cap per block of the default quota; type(uint48).max leaves it unmetered.
     * @param maxHCUDepthPerTx Maximum sequential HCU depth per transaction.
     * @param maxHCUPerTx Maximum total HCU per transaction.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(uint48 defaultQuotaCapPerBlock, uint48 maxHCUDepthPerTx, uint48 maxHCUPerTx) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        _setQuotaCap(DEFAULT_QUOTA, defaultQuotaCapPerBlock);
        _setMaxHCUPerTx(maxHCUPerTx);
        _setMaxHCUDepthPerTx(maxHCUDepthPerTx);
    }

    /**
     * @notice Re-initializes the contract from V4.
     * @dev Quotas replace the global block cap, so its value becomes the default quota's cap.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV5() public virtual reinitializer(REINITIALIZER_VERSION) {
        _setQuotaCap(DEFAULT_QUOTA, _getHCULimitStorage().globalHCUCapPerBlock);
    }

\n\n`;

  for (const [operation, data] of Object.entries(priceData)) {
    const functionName = `checkHCUFor${operation.charAt(0).toUpperCase() + operation.slice(1)}`;
    const lhsName = operation === 'fheMulDiv' ? 'factor1' : 'lhs';
    const rhsName = operation === 'fheMulDiv' ? 'factor2' : 'rhs';
    const lhsDoc = operation === 'fheMulDiv' ? 'The first multiplication factor.' : 'The left-hand side operand.';
    const rhsDoc = operation === 'fheMulDiv' ? 'The second multiplication factor.' : 'The right-hand side operand.';

    if (data.supportScalar && data.scalar && data.nonScalar) {
      switch (data.numberInputs) {
        case 1:
          output += `
        /**
        * @notice Check the homomorphic complexity units limit required for ${operation.charAt(0).toUpperCase() + operation.slice(1)}.
        * @param resultType Result type.
        * @param scalarByte Scalar byte.
        * @param ct The only operand.
        * @param result Result.
        * @param caller Original caller address from FHEVMExecutor.
         */
         function ${functionName}(FheType resultType, bytes1 scalarByte, bytes32 ct, bytes32 result, address caller) external virtual {
        if(msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
    `;
          break;
        case 2:
          output += `
        /**
         * @notice Check the homomorphic complexity units limit for ${operation.charAt(0).toUpperCase() + operation.slice(1)}.
         * @param resultType Result type.
         * @param scalarByte Scalar byte.
         * @param ${lhsName} ${lhsDoc}
         * @param ${rhsName} ${rhsDoc}
         * @param result Result.
         * @param caller Original dapp caller address from FHEVMExecutor.
         */
         function ${functionName}(FheType resultType, bytes1 scalarByte, bytes32 ${lhsName}, bytes32 ${rhsName}, bytes32 result, address caller) external virtual {
        if(msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
    `;
          break;
        default:
          throw new Error('Number of inputs for scalar and non-scalar must be less than 3');
      }
    } else if (data.supportScalar && data.scalar && !data.nonScalar) {
      switch (data.numberInputs) {
        case 2:
          output += `
        /**
         * @notice Check the homomorphic complexity units limit for ${operation.charAt(0).toUpperCase() + operation.slice(1)}.
         * @param resultType Result type.
         * @param scalarByte Scalar byte.
         * @param lhs The left-hand side operand.
         * @param result Result.
         * @param caller Original dapp caller address from FHEVMExecutor.
         */
         function ${functionName}(FheType resultType, bytes1 scalarByte, bytes32 lhs, bytes32 /*rhs*/, bytes32 result, address caller) external virtual {
        if(msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
           `;
          break;
        default:
          throw new Error('Number of inputs for scalar with only scalar must be 2');
      }
    } else {
      switch (data.numberInputs) {
        case 0:
          output += `    /**
         * @notice Check the homomorphic complexity units limit for ${operation.charAt(0).toUpperCase() + operation.slice(1)}.
         * @param resultType Result type.
         * @param result Result.
         * @param caller Original dapp caller address from FHEVMExecutor.
         */
        function ${functionName}(FheType resultType, bytes32 result, address caller) external virtual {
        if(msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
    `;
          break;
        case 1:
          output += `
      /**
        * @notice Check the homomorphic complexity units limit for ${operation.charAt(0).toUpperCase() + operation.slice(1)}.
        * @param ct The only operand.
        * @param result Result.
        * @param caller Original caller address from FHEVMExecutor.
        */
        function ${functionName}(FheType resultType, bytes32 ct, bytes32 result, address caller) external virtual {
        if(msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
    `;
          break;
        case 2:
          output += `
        /**
         * @notice Check the homomorphic complexity units limit for ${operation.charAt(0).toUpperCase() + operation.slice(1)}.
         * @param resultType Result type.
         * @param lhs The left-hand side operand.
         * @param rhs The right-hand side operand.
         * @param result Result.
         * @param caller Original dapp caller address from FHEVMExecutor.
         */
        function ${functionName}(FheType resultType, bytes32 lhs, bytes32 rhs, bytes32 result, address caller) external virtual {
        if(msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
    `;
          break;
        case 3:
          output += `
        /**
         * @notice Check the homomorphic complexity units limit for ${operation.charAt(0).toUpperCase() + operation.slice(1)}.
         * @param resultType Result type.
         * @param lhs The left-hand side operand.
         * @param middle The middle operand.
         * @param rhs The right-hand side operand.
         * @param caller Original dapp caller address from FHEVMExecutor.
         */
        function ${functionName}(FheType resultType, bytes32 lhs, bytes32 middle, bytes32 rhs, bytes32 result, address caller) external virtual {
        if(msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 opHCU;
    `;
          break;
        case -1:
          output += `
        /**
         * @notice Check the homomorphic complexity units limit for ${operation.charAt(0).toUpperCase() + operation.slice(1)}.
         * @param resultType Result type.
         * @param values Input ciphertext handles.
         * @param result Result handle.
         * @param caller Original dapp caller address from FHEVMExecutor.
         */
        function ${functionName}(FheType resultType, bytes32[] calldata values, bytes32 result, address caller) external virtual {
        if(msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 n = values.length;
        uint256 opHCU;
    `;
          break;
        case -2:
          output += `
        /**
         * @notice Check the homomorphic complexity units limit for ${operation.charAt(0).toUpperCase() + operation.slice(1)}.
         * @param valueType Value type.
         * @param value Input ciphertext handle.
         * @param values Encrypted set ciphertext handles.
         * @param result Result handle.
         * @param caller Original dapp caller address from FHEVMExecutor.
         */
        function ${functionName}(FheType valueType, bytes32 value, bytes32[] calldata values, bytes32 result, address caller) external virtual {
        if(msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();
        uint256 n = values.length;
        uint256 opHCU;
    `;
          break;
        default:
          throw new Error('Number of inputs must be less than 4');
      }
    }

    if (data.scalar && data.nonScalar) {
      if (operation === 'fheMulDiv') {
        output += `if (scalarByte == FHE_MUL_DIV_FACTOR2_SCALAR) {
          ${generatePriceChecks(data.scalar)}

          ${generateCheckTransactionLimit(data.numberInputs, true, lhsName, rhsName)}
        } else if (scalarByte == FHE_MUL_DIV_FACTOR2_ENCRYPTED) {
          ${generatePriceChecks(data.nonScalar)}

        ${generateCheckTransactionLimit(data.numberInputs, false, lhsName, rhsName)}
        } else {
          revert OnlyScalarOperationsAreSupported();
    }`;
      } else {
        output += `if (scalarByte == 0x01) {
          ${generatePriceChecks(data.scalar)}

          ${generateCheckTransactionLimit(data.numberInputs, true, lhsName, rhsName)}
        } else {
          ${generatePriceChecks(data.nonScalar)}

        ${generateCheckTransactionLimit(data.numberInputs, false, lhsName, rhsName)}
    }`;
      }
    } else if (data.scalar) {
      output += `if(scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();`;
      output += `${generatePriceChecks(data.scalar)}

                ${generateCheckTransactionLimit(data.numberInputs, true, lhsName, rhsName)} `;
    } else if (data.nonScalar) {
      output += `        if(scalarByte != 0x00) revert OnlyNonScalarOperationsAreSupported();`;
      output += `${generatePriceChecks(data.nonScalar)}
      `;
      output += `${generateCheckTransactionLimit(data.numberInputs, false, lhsName, rhsName)}`;
    } else if (data.types) {
      output += `${generatePriceChecks(data.types)}
      `;
      output += `${generateCheckTransactionLimit(data.numberInputs, false, lhsName, rhsName)}`;
    } else if (data.nBucketed) {
      output += generateNBucketedPriceChecks(data.nBucketed, data.numberInputs === -2 ? 'valueType' : 'resultType');
      output += data.numberInputs === -2 ? generateIsInTransactionLimit() : generateVariadicTransactionLimit();
    } else {
      throw new Error('No prices provided for the operation');
    }
    output += `
        }

    `;
  }

  return (
    output +
    `    /**
     * @notice Sets a quota's cap per block; type(uint48).max leaves the quota unmetered.
     * @dev The ACL owner sets any cap; a pauser can only lower one. The paused quota stays at 0.
     */
    function setQuotaCap(uint16 quotaId, uint48 capPerBlock) external {
        if (quotaId == PAUSED_QUOTA) revert InvalidQuota(quotaId);
        if (!_isOwnerElseRequirePauser() && capPerBlock > _getHCULimitStorage().quotas[quotaId].capPerBlock) {
            revert QuotaNotTighter(quotaId);
        }
        _setQuotaCap(quotaId, capPerBlock);
    }

    /**
     * @notice Assigns a contract to a quota: its FHE operations then spend that quota's budget.
     * @dev The ACL owner assigns to any quota with a cap; a pauser can only assign to the paused quota.
     */
    function assignQuota(address account, uint16 quotaId) external {
        if (!_isOwnerElseRequirePauser() && quotaId != PAUSED_QUOTA) revert QuotaNotTighter(quotaId);
        if (quotaId != PAUSED_QUOTA && _getHCULimitStorage().quotas[quotaId].capPerBlock == 0) {
            revert InvalidQuota(quotaId);
        }
        _getHCULimitStorage().contractQuota[account] = quotaId;
        emit ContractQuotaAssigned(account, quotaId, msg.sender);
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
        if (maxDepth != 0 && maxDepth < $.maxHCUDepthPerTx) revert CrossTxDepthBelowMaxDepthPerTx();
        $.maxCrossTxDepth = maxDepth;
        $.crossTxDepthDecayPerBlock = decayPerBlock;
        emit CrossTxDepthLimitSet(maxDepth, decayPerBlock);
    }

    function getCrossTxDepthLimit() external view returns (uint48 maxDepth, uint48 decayPerBlock) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        return ($.maxCrossTxDepth, $.crossTxDepthDecayPerBlock);
    }

    /**
     * @notice Caches \`handle\`'s carried depth for the rest of this transaction and returns it. If this
     *         transaction has not seen the handle yet, the depth comes from \`holder\`'s permission.
     *         Returns 0 when disabled.
     */
    function cacheDepth(bytes32 handle, address holder) external returns (uint48) {
        if (msg.sender != aclAdd) revert CallerMustBeACLContract();
        if (_getHCULimitStorage().maxCrossTxDepth == 0) return 0;
        return uint48(_handleDepths(handle, holder) >> 128);
    }

    /// @dev Records \`result\`'s depths: \`opHCU\` added to both depths of its deepest input, checked against both limits.
    function _setResultDepths(bytes32 result, uint256 opHCU, uint256 inputDepths) private {
        uint256 depths = (opHCU | (opHCU << 128)) + inputDepths;
        HCULimitStorage storage $ = _getHCULimitStorage();
        if (uint128(depths) > $.maxHCUDepthPerTx) revert HCUTransactionDepthLimitExceeded();
        uint256 maxDepth = $.maxCrossTxDepth;
        uint256 carried = depths >> 128;
        if (maxDepth != 0 && carried > maxDepth) {
            uint256 decay = $.crossTxDepthDecayPerBlock;
            revert HCUCrossTxDepthLimitExceeded((carried - maxDepth + decay - 1) / decay);
        }
        _setHCUForHandle(result, depths | DEPTH_KNOWN);
    }

    /// @dev Returns \`depth\` minus the decay accrued since \`sinceBlock\`, floored at 0.
    function _decayed(uint48 depth, uint48 sinceBlock) private view returns (uint256) {
        uint256 decay = (block.number - sinceBlock) * _getHCULimitStorage().crossTxDepthDecayPerBlock;
        return decay >= depth ? 0 : depth - decay;
    }

    /**
     * @dev Returns both depths of \`handle\`. A handle first seen in this transaction has no depth within it;
     *      its carried depth is the one stored with \`holder\`'s permission, or 0 without one: verified inputs,
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
        depths = _decayed(storedDepth, storedBlock) << 128;
        _setHCUForHandle(handle, depths | DEPTH_KNOWN);
    }

    /**
     * @notice Adjusts the sequential HCU for the transaction.
     */
    function _adjustAndCheckFheTransactionLimitOneOp(uint256 opHCU, address caller, bytes32 op1, bytes32 result) internal virtual {
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
     * @notice Charges one operation to the quota of the contract that runs it.
     * @param opHCU HCU cost of the current operation.
     * @param caller Contract that calls FHEVMExecutor.
     */
    function _updateAndVerifyHCUQuota(uint256 opHCU, address caller) internal virtual {
        HCULimitStorage storage $ = _getHCULimitStorage();
        uint16 quotaId = $.contractQuota[caller];
        Quota storage quota = $.quotas[quotaId];
        uint48 cap = quota.capPerBlock;
        if (cap == NO_CAP) return;
        uint256 used = (quota.lastBlock == block.number ? quota.usedHCU : 0) + opHCU;
        if (used > cap) revert HCUQuotaExceeded(caller, quotaId);
        quota.usedHCU = uint48(used);
        quota.lastBlock = uint48(block.number);
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

    function _setQuotaCap(uint16 quotaId, uint48 capPerBlock) private {
        _getHCULimitStorage().quotas[quotaId].capPerBlock = capPerBlock;
        emit QuotaCapSet(quotaId, capPerBlock, msg.sender);
    }

    /// @dev Returns whether the sender is the ACL owner; otherwise reverts unless it is a pauser.
    function _isOwnerElseRequirePauser() private view returns (bool) {
        if (msg.sender == ACL(aclAdd).owner()) return true;
        if (!ACL(aclAdd).isPauser(msg.sender)) revert NotPauser(msg.sender);
        return false;
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
     * @notice Returns a quota's cap per block and the HCU it has used in the current block.
     */
    function getQuota(uint16 quotaId) public view virtual returns (uint48 capPerBlock, uint48 usedThisBlock) {
        Quota storage quota = _getHCULimitStorage().quotas[quotaId];
        return (quota.capPerBlock, quota.lastBlock == block.number ? quota.usedHCU : 0);
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

  `
  );
}

function generatePriceChecks(prices: { [key: string]: number }): string {
  return (
    Object.entries(prices)
      .map(
        ([resultType, price]) => `if (resultType == FheType.${resultType}) {
        opHCU = ${price};
        }`,
      )
      .join(' else ') + 'else { revert UnsupportedOperation();}'
  );
}

function generateNBucketedPriceChecks(
  nBucketed: Partial<Record<string, NBucketedCost>>,
  typeParam: string = 'resultType',
): string {
  const typeChecks = Object.entries(nBucketed)
    .map(([typeName, buckets]) => {
      if (!buckets) return '';
      const entries: Array<[number, number]> = [];
      entries.push([10, buckets.le10]);
      if (buckets.le30 !== undefined) entries.push([30, buckets.le30]);
      if (buckets.le60 !== undefined) entries.push([60, buckets.le60]);
      if (buckets.le100 !== undefined) entries.push([100, buckets.le100]);
      if (buckets.le128 !== undefined) entries.push([128, buckets.le128]);

      const lines = entries.map(([threshold, cost], i) => {
        if (i === entries.length - 1) return `else opHCU = ${cost};`;
        if (i === 0) return `if (n <= ${threshold}) opHCU = ${cost};`;
        return `else if (n <= ${threshold}) opHCU = ${cost};`;
      });
      return `if (${typeParam} == FheType.${typeName}) {
        ${lines.join('\n        ')}
      }`;
    })
    .join(' else ');
  return typeChecks + ' else { revert UnsupportedOperation(); }';
}

function generateVariadicTransactionLimit(): string {
  return `
    _updateAndVerifyHCUTransactionLimit(opHCU, caller);

    uint256 inputDepths = 0;
    for (uint256 i = 0; i < values.length; i++) {
        inputDepths = _maxDepths(inputDepths, _handleDepths(values[i], caller));
    }

    _setResultDepths(result, opHCU, inputDepths);
  `;
}

function generateIsInTransactionLimit(): string {
  return `
    _updateAndVerifyHCUTransactionLimit(opHCU, caller);

    uint256 inputDepths = _handleDepths(value, caller);
    for (uint256 i = 0; i < values.length; i++) {
        inputDepths = _maxDepths(inputDepths, _handleDepths(values[i], caller));
    }

    _setResultDepths(result, opHCU, inputDepths);
  `;
}

function generateCheckTransactionLimit(
  numberInputs: number,
  isScalar: boolean,
  lhsName: string = 'lhs',
  rhsName: string = 'rhs',
): string {
  if (!isScalar) {
    switch (numberInputs) {
      case 0:
        return `_updateAndVerifyHCUTransactionLimit(opHCU, caller);
                _setResultDepths(result, opHCU, 0);`;
      case 1:
        return `_adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, ct, result);`;
      case 2:
        return `_adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, ${lhsName}, ${rhsName}, result);`;
      case 3:
        return `_adjustAndCheckFheTransactionLimitThreeOps(opHCU, caller, ${lhsName}, middle, ${rhsName}, result);`;
      default:
        throw new Error('Number of inputs for non-scalar must be less than 4');
    }
  } else {
    switch (numberInputs) {
      case 0:
        throw new Error('Number of inputs must be greater than 0 if scalar');
      case 1:
        return `_updateAndVerifyHCUTransactionLimit(opHCU, caller);
                _setResultDepths(result, opHCU, 0);`;
      case 2:
        return `_adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, ${lhsName}, result);`;
      default:
        throw new Error('Number of inputs for scalar must be less than 3');
    }
  }
}
