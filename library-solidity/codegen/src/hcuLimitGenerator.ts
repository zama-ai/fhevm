import type { PriceData } from './common';

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

    /// @notice Returned if the operation is not supported.
    error UnsupportedOperation();

    /// @notice Returned if the operation is not scalar.
    error OnlyScalarOperationsAreSupported();

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
    uint256 private constant MINOR_VERSION = 2;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @notice FHEVMExecutor address.
    address private constant fhevmExecutorAddress = fhevmExecutorAdd;

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
        mapping(address => bool) blockHCUWhitelist;
    }

    /// Constant used for making sure the version number used in the \`reinitializer\` modifier is
    /// identical between \`initializeFromEmptyProxy\` and the \`reinitializeVX\` method
    uint64 private constant REINITIALIZER_VERSION = 3;

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.HCULimit")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant HCULimitStorageLocation =
      0xc13af6c514bff8997f30c90003baa82bd02aad978179d1ce58d85c4319ad6500;

    function _getHCULimitStorage() internal pure virtual returns (HCULimitStorage storage $) {
        assembly {
            $.slot := HCULimitStorageLocation
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
        _setMaxHCUDepthPerTx(maxHCUDepthPerTx);
        _setMaxHCUPerTx(maxHCUPerTx);
    }

    /**
     * @notice Re-initializes the contract from V1.
     * @param hcuCapPerBlock New global HCU cap per block.
     * @param maxHCUDepthPerTx Maximum sequential HCU depth per transaction.
     * @param maxHCUPerTx Maximum total HCU per transaction.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    function reinitializeV2(uint48 hcuCapPerBlock, uint48 maxHCUDepthPerTx, uint48 maxHCUPerTx) public virtual reinitializer(REINITIALIZER_VERSION) {
        _setHCUPerBlock(hcuCapPerBlock);
        _setMaxHCUDepthPerTx(maxHCUDepthPerTx);
        _setMaxHCUPerTx(maxHCUPerTx);
    }

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
        if(msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
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
        if(msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
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
        if(msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
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
        if(msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
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
        if(msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
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
        if(msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
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
        if(msg.sender != fhevmExecutorAddress) revert CallerMustBeFHEVMExecutorContract();
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
     */
    function _setHCUPerBlock(uint48 hcuPerBlock) internal {
        _getHCULimitStorage().globalHCUCapPerBlock = hcuPerBlock;
        emit HCUPerBlockSet(hcuPerBlock);
    }

    /**
     * @notice Sets the per-transaction HCU depth limit.
     * @param maxHCUDepthPerTx New depth limit.
     */
    function _setMaxHCUDepthPerTx(uint48 maxHCUDepthPerTx) internal {
        _getHCULimitStorage().maxHCUDepthPerTx = maxHCUDepthPerTx;
        emit MaxHCUDepthPerTxSet(maxHCUDepthPerTx);
    }

    /**
     * @notice Sets the per-transaction HCU limit.
     * @param maxHCUPerTx New transaction limit.
     */
    function _setMaxHCUPerTx(uint48 maxHCUPerTx) internal {
        _getHCULimitStorage().maxHCUPerTx = maxHCUPerTx;
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
