interface PriceData {
  [key: string]: {
    binary: boolean;
    scalar?: { [key: string]: number };
    nonScalar?: { [key: string]: number };
    types?: { [key: string]: number };
  };
}

export function generateSolidityFHEGasLimit(priceData: PriceData): string {
  let output = `// SPDX-License-Identifier: BSD-3-Clause-Clear
  pragma solidity ^0.8.24;
  
  import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
  import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
  import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
  import {tfheExecutorAdd} from "../addresses/TFHEExecutorAddress.sol";

  import {FheType} from "./FheType.sol"; 

  /**
   * @title  FHEGasLimit
   * @notice This contract manages the amount of gas to be paid for FHE operations.
  */
contract FHEGasLimit is UUPSUpgradeable, Ownable2StepUpgradeable {
    /// @notice Returned if the sender is not the TFHEExecutor.
    error CallerMustBeTFHEExecutorContract();

    /// @notice Returned if the block limit is higher than limit for FHE operation.
    error FHEGasBlockLimitExceeded();

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

    /// @notice TFHEExecutor address.
    address private constant tfheExecutorAddress = tfheExecutorAdd;

    /// @notice Gas block limit for FHEGas operation.
    uint256 private constant FHE_GAS_BLOCKLIMIT = 10_000_000;

    /// @custom:storage-location erc7201:fhevm.storage.FHEGasLimit
    struct FHEGasLimitStorage {
        uint256 lastBlock;
        uint256 currentBlockConsumption;
    }

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.FHEGasLimit")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant FHEGasLimitStorageLocation =
        0xb5c80b3bbe0bcbcea690f6dbe62b32a45bd1ad263b78db2f25ef8414efe9bc00;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

\n\n`;

  for (const [operation, data] of Object.entries(priceData)) {
    const functionName = `payFor${operation.charAt(0).toUpperCase() + operation.slice(1)}`;
    if (data.binary) {
      output += `    /**
     * @notice              Computes the gas required for ${operation.charAt(0).toUpperCase() + operation.slice(1)}.
     * @param resultType    Result type.
     * @param scalarByte    Scalar byte.
     */
     function ${functionName}(FheType resultType, bytes1 scalarByte) external virtual {
        if(msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        _checkIfNewBlock();
`;
    } else {
      output += `    /**
     * @notice              Computes the gas required for ${operation.charAt(0).toUpperCase() + operation.slice(1)}.
     * @param resultType    Result type.
     */
    function ${functionName}(FheType resultType) external virtual {
        if(msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        _checkIfNewBlock();
`;
    }

    if (data.scalar && data.nonScalar) {
      output += `        if (scalarByte == 0x01) {
${generatePriceChecks(data.scalar)}
        } else {
${generatePriceChecks(data.nonScalar)}
        }`;
    } else if (data.scalar) {
      output += `        if(scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();`;
      output += `${generatePriceChecks(data.scalar)}`;
    } else if (data.nonScalar) {
      output += `        if(scalarByte != 0x00) revert OnlyNonScalarOperationsAreSupported();`;
      output += `${generatePriceChecks(data.nonScalar)}`;
    } else {
      if (data.types) output += `${generatePriceChecks(data.types)}`;
    }

    output += `_checkFHEGasBlockLimit();
    }\n\n`;
  }

  return (
    output +
    `    /**
     * @notice                     Getter function for the TFHEExecutor contract address.
     * @return tfheExecutorAddress Address of the TFHEExecutor.
     */
    function getTFHEExecutorAddress() public view virtual returns (address) {
        return tfheExecutorAddress;
    }

    /**
     * @notice        Getter for the name and version of the contract.
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
     * @dev Checks the accumulated FHE gas used and checks if it is inferior to the limit.
     *      If so, it reverts.
     */
    function _checkFHEGasBlockLimit() internal view virtual {
        FHEGasLimitStorage storage $ = _getFHEGasLimitStorage();
        if ($.currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    /**
     * @dev Checks if it is a new block. If so, it resets information for new block.
     */
    function _checkIfNewBlock() internal virtual {
        FHEGasLimitStorage storage $ = _getFHEGasLimitStorage();
        uint256 lastBlock_ = block.number;
        if (lastBlock_ > $.lastBlock) {
            $.lastBlock = lastBlock_;
            $.currentBlockConsumption = 0;
        }
    }

    /**
     * @dev                 Updates the funding.
     * @param paidAmountGas Paid amount gas.
     */
    function _updateFunding(uint256 paidAmountGas) internal virtual {
        FHEGasLimitStorage storage $ = _getFHEGasLimitStorage();
        $.currentBlockConsumption += paidAmountGas;
    }

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /**
     * @dev  Returns the FHEGasLimit storage location.
     */
    function _getFHEGasLimitStorage() internal pure returns (FHEGasLimitStorage storage $) {
        assembly {
            $.slot := FHEGasLimitStorageLocation
        }
    }
  }
  `
  );
}

function generatePriceChecks(prices: { [key: string]: number }): string {
  return (
    Object.entries(prices)
      .map(
        ([resultType, price]) => `        if (resultType == FheType.${resultType}) {
        _updateFunding(${price});
        }`,
      )
      .join(' else ') + 'else { revert UnsupportedOperation();}'
  );
}
