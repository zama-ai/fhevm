interface PriceData {
  [key: string]: {
    binary: boolean;
    scalar?: { [key: string]: number };
    nonScalar?: { [key: string]: number };
    types?: { [key: string]: number };
  };
}

export function generateFHEGasLimit(priceData: PriceData): string {
  let output = `// SPDX-License-Identifier: BSD-3-Clause-Clear

  pragma solidity ^0.8.24;
  
  import "../addresses/TFHEExecutorAddress.sol";
  import "@openzeppelin/contracts/utils/Strings.sol";
  import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
  import "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
  
  error FHEGasBlockLimitExceeded();
  error UnsupportedOperation();
  error CallerMustBeTFHEExecutorContract();
  error OnlyScalarOperationsAreSupported();
  
  contract FHEGasLimit is UUPSUpgradeable, Ownable2StepUpgradeable {
      /// @notice Name of the contract
      string private constant CONTRACT_NAME = "FHEGasLimit";
  
      /// @notice Version of the contract
      uint256 private constant MAJOR_VERSION = 0;
      uint256 private constant MINOR_VERSION = 1;
      uint256 private constant PATCH_VERSION = 0;
      
      address private constant tfheExecutorAddress = tfheExecutorAdd;
      uint256 private constant FHE_GAS_BLOCKLIMIT = 10_000_000;
  
      /// @custom:storage-location erc7201:fhevm.storage.FHEGasLimit
      struct FHEGasLimitStorage {
          uint256 lastBlock;
          uint256 currentBlockConsumption;
      }
  
      // keccak256(abi.encode(uint256(keccak256("fhevm.storage.FHEGasLimit")) - 1)) & ~bytes32(uint256(0xff))
      bytes32 private constant FHEGasLimitStorageLocation =
        0xb5c80b3bbe0bcbcea690f6dbe62b32a45bd1ad263b78db2f25ef8414efe9bc00;
  
      function _getFHEGasLimitStorage() internal pure returns (FHEGasLimitStorage storage $) {
          assembly {
              $.slot := FHEGasLimitStorageLocation
          }
      }
      
      /// @notice Getter function for the TFHEExecutor contract address
      function getTFHEExecutorAddress() public view virtual returns (address) {
          return tfheExecutorAddress;
      }
  
      function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}
  
      /// @custom:oz-upgrades-unsafe-allow constructor
      constructor() {
          _disableInitializers();
      }
  
      /// @notice Initializes the contract setting \`initialOwner\` as the initial owner
      function initialize(address initialOwner) external initializer {
          __Ownable_init(initialOwner);
      }

      function updateFunding(uint256 paidAmountGas) internal virtual {
          FHEGasLimitStorage storage $ = _getFHEGasLimitStorage();
          $.currentBlockConsumption += paidAmountGas;
      }
  
      function checkIfNewBlock() internal virtual {
          FHEGasLimitStorage storage $ = _getFHEGasLimitStorage();
          uint256 lastBlock_ = block.number;
          if (lastBlock_ > $.lastBlock) {
              $.lastBlock = lastBlock_;
              $.currentBlockConsumption = 0;
          }
      }
  
      function checkFHEGasBlockLimit() internal view virtual {
          FHEGasLimitStorage storage $ = _getFHEGasLimitStorage();
          if ($.currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
      }\n\n`;

  for (const [operation, data] of Object.entries(priceData)) {
    const functionName = `payFor${operation.charAt(0).toUpperCase() + operation.slice(1)}`;
    if (data.binary) {
      output += `    function ${functionName}(uint8 resultType, bytes1 scalarByte) external virtual {
        if(msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
`;
    } else {
      output += `        function ${functionName}(uint8 resultType) external virtual {
        if(msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
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

    output += `checkFHEGasBlockLimit();
    }\n\n`;
  }

  return (
    output +
    `    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
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
}`
  );
}

function generatePriceChecks(prices: { [key: string]: number }): string {
  return (
    Object.entries(prices)
      .map(
        ([resultType, price]) => `        if (resultType == ${resultType}) {
        updateFunding(${price});
        }`,
      )
      .join(' else ') + 'else { revert UnsupportedOperation();}'
  );
}
