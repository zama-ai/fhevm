interface PriceData {
  [key: string]: {
    binary: boolean;
    scalar?: { [key: string]: number };
    nonScalar?: { [key: string]: number };
    types?: { [key: string]: number };
  };
}

export function generateFHEPayment(priceData: PriceData): string {
  let output = `// SPDX-License-Identifier: BSD-3-Clause-Clear

  pragma solidity ^0.8.24;
  
  import "./TFHEExecutorAddress.sol";
  import "@openzeppelin/contracts/utils/Strings.sol";
  import "@openzeppelin/contracts/access/Ownable2Step.sol";

  error FHEGasBlockLimitExceeded();
  error CallerMustBeTFHEExecutorContract();
  error OnlyScalarOperationsAreSupported();
  error OnlyNonScalarOperationsAreSupported();
  error RecoveryFailed();
  error WithdrawalFailed();
  error AccountNotEnoughFunded();
  
  contract FHEPayment is Ownable2Step {
      /// @notice Name of the contract
      string private constant CONTRACT_NAME = "FHEPayment";
  
      /// @notice Version of the contract
      uint256 private constant MAJOR_VERSION = 0;
      uint256 private constant MINOR_VERSION = 1;
      uint256 private constant PATCH_VERSION = 0;
      address public immutable tfheExecutorAddress = tfheExecutorAdd;
  
      uint256 private constant FHE_GAS_BLOCKLIMIT = 10_000_000;
  
      uint64 private lastBlock;
      uint64 private currentBlockConsumption;
      uint64 public claimableUsedFHEGas;
  
      mapping(address payer => uint256 depositedAmount) private depositsETH;

      constructor() Ownable(msg.sender) {}

      function recoverBurntFunds(address receiver) external onlyOwner {
        uint64 claimableUsedFHEGas_ = claimableUsedFHEGas;
        claimableUsedFHEGas = 0;
        (bool success, ) = receiver.call{value: claimableUsedFHEGas_}("");
        if(!success) revert RecoveryFailed();
      }
  
      function depositETH(address account) external payable {
          depositsETH[account] += msg.value;
      }
  
      function withdrawETH(uint256 amount, address receiver) external {
          depositsETH[msg.sender] -= amount;
          (bool success, ) = receiver.call{value: amount}("");
          if(!success) revert WithdrawalFailed();
      }
  
      function getAvailableDepositsETH(address account) external view returns (uint256) {
          return depositsETH[account];
      }

      function updateFunding(address payer, uint256 paidAmount) private {
        uint256 depositedAmount = depositsETH[payer];
        if(paidAmount>depositedAmount) revert AccountNotEnoughFunded();
        unchecked{
          depositsETH[payer] = depositedAmount - paidAmount;
        }
        currentBlockConsumption += uint64(paidAmount);
        claimableUsedFHEGas += uint64(paidAmount);
      }
  
      function checkIfNewBlock() private {
          uint64 lastBlock_ = uint64(block.number);
          if (block.number > lastBlock) {
              lastBlock = lastBlock_;
              currentBlockConsumption = 0;
          }
      }\n\n`;

  for (const [operation, data] of Object.entries(priceData)) {
    const functionName = `payFor${operation.charAt(0).toUpperCase() + operation.slice(1)}`;
    if (data.binary) {
      output += `    function ${functionName}(address payer, uint8 resultType, bytes1 scalarByte) external {
        if(msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
`;
    } else {
      output += `    function ${functionName}(address payer, uint8 resultType) external {
        if(msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
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

    output += `if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }\n\n`;
  }

  return (
    output +
    `    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
    function getVersion() external pure returns (string memory) {
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
  return Object.entries(prices)
    .map(
      ([resultType, price]) => `        if (resultType == ${resultType}) {
        updateFunding(payer, ${price});
        }`,
    )
    .join(' else ');
}
