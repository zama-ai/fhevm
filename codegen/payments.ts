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
  
  contract FHEPayment {
      /// @notice Name of the contract
      string private constant CONTRACT_NAME = "FHEPayment";
  
      /// @notice Version of the contract
      uint256 private constant MAJOR_VERSION = 0;
      uint256 private constant MINOR_VERSION = 1;
      uint256 private constant PATCH_VERSION = 0;
      address public immutable tfheExecutorAddress = tfheExecutorAdd;
  
      uint256 private constant FHE_GAS_BLOCKLIMIT = 10_000_000;
  
      uint128 private lastBlock;
      uint128 private currentBlockConsumption;
  
      mapping(address payer => uint256 depositedAmount) private depositsETH;
  
      function depositETH(address account) external payable {
          depositsETH[account] += msg.value;
      }
  
      function withdrawETH(uint256 amount, address receiver) external {
          depositsETH[msg.sender] -= amount;
          (bool success, ) = receiver.call{value: amount}("");
          require(success, "Withdrawal failed");
      }
  
      function getAvailableDepositsETH(address account) external view returns (uint256) {
          return depositsETH[account];
      }
  
      function checkIfNewBlock() private {
          uint128 lastBlock_ = uint128(block.number);
          if (block.number > lastBlock) {
              lastBlock = lastBlock_;
              currentBlockConsumption = 0;
          }
      }\n\n`;

  for (const [operation, data] of Object.entries(priceData)) {
    const functionName = `payFor${operation.charAt(0).toUpperCase() + operation.slice(1)}`;
    if (data.binary) {
      output += `    function ${functionName}(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
`;
    } else {
      output += `    function ${functionName}(address payer, uint8 resultType) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
`;
    }

    if (data.scalar && data.nonScalar) {
      output += `        if (scalarByte == 0x01) {
${generatePriceChecks(data.scalar)}
        } else {
${generatePriceChecks(data.nonScalar)}
        }`;
    } else if (data.scalar) {
      output += `        require(scalarByte == 0x01, "Only scalar operations are supported");`;
      output += `${generatePriceChecks(data.scalar)}`;
    } else if (data.nonScalar) {
      output += `        require(scalarByte == 0x00, "Only non-scalar operations are supported");`;
      output += `${generatePriceChecks(data.nonScalar)}`;
    } else {
      if (data.types) output += `${generatePriceChecks(data.types)}`;
    }

    output += `require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
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
            depositsETH[payer] -= ${price};
            currentBlockConsumption += ${price};
        }`,
    )
    .join(' else ');
}
