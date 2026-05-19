import type { NBucketedCost, PriceData } from './common';

export function generateSolidityHCULimit(priceData: PriceData): string {
  if (!priceData) {
    throw new Error('undefined or null priceData');
  }

  let output = `// SPDX-License-Identifier: BSD-3-Clause-Clear
  pragma solidity ^0.8.24;

  import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
  import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
  import {fhevmExecutorAdd} from "../addresses/FHEVMHostAddresses.sol";
  import {ACLOwnable} from "./shared/ACLOwnable.sol";

  import {FheType} from "./shared/FheType.sol";

  /**
   * @title HCULimit
   * @notice This contract manages the total allowed complexity for FHE operations at the
   * transaction level, including the maximum number of homomorphic complexity units (HCU) per transaction.
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
    uint256 private constant MINOR_VERSION = 3;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @notice FHEVMExecutor address.
    address private constant FHEVM_EXECUTOR_ADDRESS = fhevmExecutorAdd;

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
    }

    /// Constant used for making sure the version number used in the \`reinitializer\` modifier is
    /// identical between \`initializeFromEmptyProxy\` and the \`reinitializeVX\` method
    uint64 private constant REINITIALIZER_VERSION = 4;

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
    function initializeFromEmptyProxy(uint48 hcuCapPerBlock, uint48 maxHCUDepthPerTx, uint48 maxHCUPerTx) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        _setHCUPerBlock(hcuCapPerBlock);
        _setMaxHCUPerTx(maxHCUPerTx);
        _setMaxHCUDepthPerTx(maxHCUDepthPerTx);
    }

    /**
     * @notice Re-initializes the contract from V2.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV3() public virtual reinitializer(REINITIALIZER_VERSION) {}

\n\n`;

  for (const [operation, data] of Object.entries(priceData)) {
    const functionName = `checkHCUFor${operation.charAt(0).toUpperCase() + operation.slice(1)}`;

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
         * @param lhs The left-hand side operand.
         * @param rhs The right-hand side operand.
         * @param result Result.
         * @param caller Original dapp caller address from FHEVMExecutor.
         */
         function ${functionName}(FheType resultType, bytes1 scalarByte, bytes32 lhs, bytes32 rhs, bytes32 result, address caller) external virtual {
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
      output += `if (scalarByte == 0x01) {
          ${generatePriceChecks(data.scalar)}

          ${generateCheckTransactionLimit(data.numberInputs, true)}
        } else {
          ${generatePriceChecks(data.nonScalar)}

        ${generateCheckTransactionLimit(data.numberInputs, false)}
    }`;
    } else if (data.scalar) {
      output += `if(scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();`;
      output += `${generatePriceChecks(data.scalar)}

                ${generateCheckTransactionLimit(data.numberInputs, true)} `;
    } else if (data.nonScalar) {
      output += `        if(scalarByte != 0x00) revert OnlyNonScalarOperationsAreSupported();`;
      output += `${generatePriceChecks(data.nonScalar)}
      `;
      output += `${generateCheckTransactionLimit(data.numberInputs, false)}`;
    } else if (data.types) {
      output += `${generatePriceChecks(data.types)}
      `;
      output += `${generateCheckTransactionLimit(data.numberInputs, false)}`;
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
     * @notice Adjusts the sequential HCU for the transaction.
     */
    function _adjustAndCheckFheTransactionLimitOneOp(uint256 opHCU, address caller, bytes32 op1, bytes32 result) internal virtual {
        _updateAndVerifyHCUTransactionLimit(opHCU, caller);

        uint256 totalHCU = opHCU + _getHCUForHandle(op1);
        if (totalHCU > uint256(_getHCULimitStorage().maxHCUDepthPerTx)) {
            revert HCUTransactionDepthLimitExceeded();
        }

        _setHCUForHandle(result, totalHCU);
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

        uint256 totalHCU = opHCU + _max(_getHCUForHandle(op1), _getHCUForHandle(op2));
        if (totalHCU > uint256(_getHCULimitStorage().maxHCUDepthPerTx)) {
            revert HCUTransactionDepthLimitExceeded();
        }

        _setHCUForHandle(result, totalHCU);
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

        uint256 totalHCU = opHCU +
            _max(_getHCUForHandle(op1), _max(_getHCUForHandle(op2), _getHCUForHandle(op3)));

        if (totalHCU > uint256(_getHCULimitStorage().maxHCUDepthPerTx)) {
            revert HCUTransactionDepthLimitExceeded();
        }

        _setHCUForHandle(result, totalHCU);

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
     * @notice Gets the current HCU for the handle.
     * @param handle The handle for which to get the HCU.
     * @return handleHCU The current HCU for the handle.
     * @dev This function uses inline assembly to load the HCU from a specific storage location.
     */
    function _getHCUForHandle(bytes32 handle) internal view virtual returns (uint256 handleHCU) {
        // Handle slot zero is reserved for _getHCUForTransaction.
        if (handle == bytes32(0)) revert InvalidZeroHandle();
        assembly {
            handleHCU := tload(handle)
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

    /**
     * @dev Returns the maximum of two numbers.
     * @param a The first number.
     * @param b The second number.
     * @return The maximum of a and b.
     */
    function _max(uint256 a, uint256 b) private pure returns (uint256) {
        return a >= b ? a : b;
    }

    /**
     * @notice Getter function for the FHEVMExecutor contract address.
     * @return FHEVM_EXECUTOR_ADDRESS Address of the FHEVMExecutor.
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
     * @dev If storage still contains a previous block meter, returns \`(block.number, 0)\`.
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

    uint256 maxInputDepth = 0;
    for (uint256 i = 0; i < values.length; i++) {
        uint256 inputDepth = _getHCUForHandle(values[i]);
        if (inputDepth > maxInputDepth) {
            maxInputDepth = inputDepth;
        }
    }

    uint256 totalHCU = opHCU + maxInputDepth;
    if (totalHCU > uint256(_getHCULimitStorage().maxHCUDepthPerTx)) {
        revert HCUTransactionDepthLimitExceeded();
    }
    _setHCUForHandle(result, totalHCU);
  `;
}

function generateIsInTransactionLimit(): string {
  return `
    _updateAndVerifyHCUTransactionLimit(opHCU, caller);

    uint256 maxInputDepth = _getHCUForHandle(value);
    for (uint256 i = 0; i < values.length; i++) {
        uint256 inputDepth = _getHCUForHandle(values[i]);
        if (inputDepth > maxInputDepth) {
            maxInputDepth = inputDepth;
        }
    }

    uint256 totalHCU = opHCU + maxInputDepth;
    if (totalHCU > uint256(_getHCULimitStorage().maxHCUDepthPerTx)) {
        revert HCUTransactionDepthLimitExceeded();
    }
    _setHCUForHandle(result, totalHCU);
  `;
}

function generateCheckTransactionLimit(numberInputs: number, isScalar: boolean): string {
  if (!isScalar) {
    switch (numberInputs) {
      case 0:
        return `_updateAndVerifyHCUTransactionLimit(opHCU, caller);
                _setHCUForHandle(result, opHCU);`;
      case 1:
        return `_adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, ct, result);`;
      case 2:
        return `_adjustAndCheckFheTransactionLimitTwoOps(opHCU, caller, lhs, rhs, result);`;
      case 3:
        return `_adjustAndCheckFheTransactionLimitThreeOps(opHCU, caller, lhs, middle, rhs, result);`;
      default:
        throw new Error('Number of inputs for non-scalar must be less than 4');
    }
  } else {
    switch (numberInputs) {
      case 0:
        throw new Error('Number of inputs must be greater than 0 if scalar');
      case 1:
        return `_updateAndVerifyHCUTransactionLimit(opHCU, caller);
                _setHCUForHandle(result, opHCU);`;
      case 2:
        return `_adjustAndCheckFheTransactionLimitOneOp(opHCU, caller, lhs, result);`;
      default:
        throw new Error('Number of inputs for scalar must be less than 3');
    }
  }
}
