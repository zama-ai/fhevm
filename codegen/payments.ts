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
  import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
  import "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
  
  error FHEGasBlockLimitExceeded();
  error UnsupportedOperation();
  error CallerMustBeTFHEExecutorContract();
  error OnlyScalarOperationsAreSupported();
  error RecoveryFailed();
  error WithdrawalFailed();
  error AccountNotEnoughFunded();
  error AlreadyAuthorizedAllContracts();
  error AlreadyWhitelistedContract();
  error AllContractsNotAuthorized();
  error ContractNotWhitelisted();
  
  contract FHEPayment is UUPSUpgradeable, Ownable2StepUpgradeable {
      /// @notice Name of the contract
      string private constant CONTRACT_NAME = "FHEPayment";
  
      /// @notice Version of the contract
      uint256 private constant MAJOR_VERSION = 0;
      uint256 private constant MINOR_VERSION = 1;
      uint256 private constant PATCH_VERSION = 0;
      address private constant tfheExecutorAddress = tfheExecutorAdd;
  
      uint256 private constant FHE_GAS_BLOCKLIMIT = 10_000_000;
      uint256 private constant MIN_FHE_GASPRICE = 0; // eg: 10_000_000 means a minimum of 0.01 Gwei
      uint256 private constant FHE_GASPRICE_NATIVE_RATIO = 0; // eg: 1000 means fhe gas price is set to 0.1% of native gas price (if above minimum)
  
      /// @custom:storage-location erc7201:fhevm.storage.FHEPayment
      struct FHEPaymentStorage {
          uint256 lastBlock;
          uint256 currentBlockConsumption;
          uint256 claimableUsedFHEGas;
          mapping(address payer => uint256 depositedAmount) depositsETH;
          mapping(address user => bool allowedAllContracts) allowedAll;
          mapping(address user => mapping(address dappContract => bool isWhitelisted)) whitelistedDapps;
      }
  
      // keccak256(abi.encode(uint256(keccak256("fhevm.storage.FHEPayment")) - 1)) & ~bytes32(uint256(0xff))
      bytes32 private constant FHEPaymentStorageLocation =
          0x4c5af501c90907b9fb888b6dd79405547def38a1dc3110f42d77f5dbc3222e00;
  
      function _getFHEPaymentStorage() internal pure returns (FHEPaymentStorage storage $) {
          assembly {
              $.slot := FHEPaymentStorageLocation
          }
      }
      
      /// @notice Getter function for the TFHEExecutor contract address
      function getTFHEExecutorAddress() public view virtual returns (address) {
          return tfheExecutorAddress;
      }
  
      function getClaimableUsedFHEGas() public view virtual returns (uint256) {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          return $.claimableUsedFHEGas;
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
  
      function recoverBurntFunds(address receiver) external virtual onlyOwner {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          uint256 claimableUsedFHEGas_ = $.claimableUsedFHEGas;
          $.claimableUsedFHEGas = 0;
          (bool success, ) = receiver.call{value: claimableUsedFHEGas_}("");
          if (!success) revert RecoveryFailed();
      }
  
      function depositETH(address account) external payable virtual {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          $.depositsETH[account] += msg.value;
      }
  
      function withdrawETH(uint256 amount, address receiver) external virtual {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          $.depositsETH[msg.sender] -= amount;
          (bool success, ) = receiver.call{value: amount}("");
          if (!success) revert WithdrawalFailed();
      }
  
      function getAvailableDepositsETH(address account) external view virtual returns (uint256) {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          return $.depositsETH[account];
      }
  
      function didAuthorizeAllContracts(address account) external view virtual returns (bool) {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          return $.allowedAll[account];
      }
  
      function didWhitelistContract(address user, address dappContract) external view virtual returns (bool) {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          return $.whitelistedDapps[user][dappContract];
      }
  
      function authorizeAllContracts() external virtual {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          if ($.allowedAll[msg.sender]) revert AlreadyAuthorizedAllContracts();
          $.allowedAll[msg.sender] = true;
      }
  
      function whitelistContract(address dappContract) external virtual {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          if ($.whitelistedDapps[msg.sender][dappContract]) revert AlreadyWhitelistedContract();
          $.whitelistedDapps[msg.sender][dappContract] = true;
      }
  
      function removeAuthorizationAllContracts() external virtual {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          if (!$.allowedAll[msg.sender]) revert AllContractsNotAuthorized();
          $.allowedAll[msg.sender] = false;
      }
  
      function removeWhitelistedContract(address dappContract) external virtual {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          if (!$.whitelistedDapps[msg.sender][dappContract]) revert ContractNotWhitelisted();
          $.whitelistedDapps[msg.sender][dappContract] = false;
      }
  
      // @notice: to be used in the context of account abstraction, before an FHE tx, to make the contract address replace tx.origin as a spender
      function becomeTransientSpender() external virtual {
          assembly {
              tstore(0, caller())
          }
      }
  
      // @notice: to be used in the context of account abstraction, after an FHE tx, to avoid issues if batched with other userOps
      function stopBeingTransientSpender() external virtual {
          assembly {
              tstore(0, 0)
          }
      }
  
      function updateFunding(address payer, uint256 paidAmountGas) internal virtual {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          uint256 ratio_gas = (tx.gasprice * FHE_GASPRICE_NATIVE_RATIO) / 1_000_000;
          uint256 effective_fhe_gasPrice = ratio_gas > MIN_FHE_GASPRICE ? ratio_gas : MIN_FHE_GASPRICE;
          uint256 paidAmountWei = effective_fhe_gasPrice * paidAmountGas;
          uint256 depositedAmount = $.depositsETH[payer];
          if (paidAmountWei > depositedAmount) {
              // if dApp is not enough funded, fallbacks to user (tx.origin by default, in case of an EOA,
              // otherwise a smart contract account should call \`becomeTransientSpender\` before, in the same tx
              address spender;
              assembly {
                  spender := tload(0)
              }
              spender = spender == address(0) ? tx.origin : spender;
              if ($.allowedAll[spender] || $.whitelistedDapps[spender][payer]) {
                  uint256 depositedAmountUser = $.depositsETH[spender];
                  if (paidAmountWei > depositedAmountUser) revert AccountNotEnoughFunded();
                  unchecked {
                      $.depositsETH[spender] = depositedAmountUser - paidAmountWei;
                  }
                  $.currentBlockConsumption += paidAmountGas;
                  $.claimableUsedFHEGas += paidAmountWei;
              } else {
                  revert AccountNotEnoughFunded();
              }
          } else {
              unchecked {
                  $.depositsETH[payer] = depositedAmount - paidAmountWei;
              }
              $.currentBlockConsumption += paidAmountGas;
              $.claimableUsedFHEGas += paidAmountWei;
          }
      }
  
      function checkIfNewBlock() internal virtual {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          uint256 lastBlock_ = block.number;
          if (lastBlock_ > $.lastBlock) {
              $.lastBlock = lastBlock_;
              $.currentBlockConsumption = 0;
          }
      }
  
      function checkFHEGasBlockLimit() internal view virtual {
          FHEPaymentStorage storage $ = _getFHEPaymentStorage();
          if ($.currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
      }\n\n`;

  for (const [operation, data] of Object.entries(priceData)) {
    const functionName = `payFor${operation.charAt(0).toUpperCase() + operation.slice(1)}`;
    if (data.binary) {
      output += `    function ${functionName}(address payer, uint8 resultType, bytes1 scalarByte) external virtual {
        if(msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
`;
    } else {
      output += `        function ${functionName}(address payer, uint8 resultType) external virtual {
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
        updateFunding(payer, ${price});
        }`,
      )
      .join(' else ') + 'else { revert UnsupportedOperation();}'
  );
}
