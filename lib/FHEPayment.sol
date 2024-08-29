// SPDX-License-Identifier: BSD-3-Clause-Clear

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
error AlreadyAuthorizedAllContracts();
error AlreadyWhitelistedContract();
error AllContractsNotAuthorized();
error ContractNotWhitelisted();

contract FHEPayment is Ownable2Step {
    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "FHEPayment";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;
    address public immutable tfheExecutorAddress = tfheExecutorAdd;

    uint256 private constant FHE_GAS_BLOCKLIMIT = 10_000_000;
    uint256 private constant MIN_FHE_GASPRICE = 10_000_000; // minimum of 0.01 Gwei
    uint256 private constant FHE_GASPRICE_NATIVE_RATIO = 1000; // fhe gas price is set to 0.1% of native gas price (if above minimum)

    uint256 private lastBlock;
    uint256 private currentBlockConsumption;
    uint256 public claimableUsedFHEGas;

    mapping(address payer => uint256 depositedAmount) private depositsETH;
    mapping(address user => bool allowedAllContracts) private allowedAll;
    mapping(address user => mapping(address dappContract => bool isWhitelisted)) private whitelistedDapps;

    constructor() Ownable(msg.sender) {}

    function recoverBurntFunds(address receiver) external onlyOwner {
        uint256 claimableUsedFHEGas_ = claimableUsedFHEGas;
        claimableUsedFHEGas = 0;
        (bool success, ) = receiver.call{value: claimableUsedFHEGas_}("");
        if (!success) revert RecoveryFailed();
    }

    function depositETH(address account) external payable {
        depositsETH[account] += msg.value;
    }

    function withdrawETH(uint256 amount, address receiver) external {
        depositsETH[msg.sender] -= amount;
        (bool success, ) = receiver.call{value: amount}("");
        if (!success) revert WithdrawalFailed();
    }

    function getAvailableDepositsETH(address account) external view returns (uint256) {
        return depositsETH[account];
    }

    function didAuthorizeAllContracts(address account) external view returns (bool) {
        return allowedAll[account];
    }

    function didWhitelistContract(address user, address dappContract) external view returns (bool) {
        return whitelistedDapps[user][dappContract];
    }

    function authorizeAllContracts() external {
        if (allowedAll[msg.sender]) revert AlreadyAuthorizedAllContracts();
        allowedAll[msg.sender] = true;
    }

    function whitelistContract(address dappContract) external {
        if (whitelistedDapps[msg.sender][dappContract]) revert AlreadyWhitelistedContract();
        whitelistedDapps[msg.sender][dappContract] = true;
    }

    function removeAuthorizationAllContracts() external {
        if (!allowedAll[msg.sender]) revert AllContractsNotAuthorized();
        allowedAll[msg.sender] = false;
    }

    function removeWhitelistedContract(address dappContract) external {
        if (!whitelistedDapps[msg.sender][dappContract]) revert ContractNotWhitelisted();
        whitelistedDapps[msg.sender][dappContract] = false;
    }

    // @notice: to be used in the context of account abstraction, before an FHE tx, to make the contract address replace tx.origin as a spender
    function becomeTransientSpender() external {
        assembly {
            tstore(0, caller())
        }
    }

    // @notice: to be used in the context of account abstraction, after an FHE tx, to avoid issues if batched with other userOps
    function stopBeingTransientSpender() external {
        assembly {
            tstore(0, 0)
        }
    }

    function updateFunding(address payer, uint256 paidAmountGas) private {
        uint256 ratio_gas = (tx.gasprice * FHE_GASPRICE_NATIVE_RATIO) / 1_000_000;
        uint256 effective_fhe_gasPrice = ratio_gas > MIN_FHE_GASPRICE ? ratio_gas : MIN_FHE_GASPRICE;
        uint256 paidAmountWei = effective_fhe_gasPrice * paidAmountGas;
        uint256 depositedAmount = depositsETH[payer];
        if (paidAmountWei > depositedAmount) {
            // if dApp is not enough funded, fallbacks to user (tx.origin by default, in case of an EOA,
            // otherwise a smart contract account should call `becomeTransientSpender` before, in the same tx
            address spender;
            assembly {
                spender := tload(0)
            }
            spender = spender == address(0) ? tx.origin : spender;
            if (allowedAll[spender] || whitelistedDapps[spender][payer]) {
                uint256 depositedAmountUser = depositsETH[spender];
                if (paidAmountWei > depositedAmountUser) revert AccountNotEnoughFunded();
                unchecked {
                    depositsETH[spender] = depositedAmountUser - paidAmountWei;
                }
                currentBlockConsumption += paidAmountGas;
                claimableUsedFHEGas += paidAmountWei;
            } else {
                revert AccountNotEnoughFunded();
            }
        } else {
            unchecked {
                depositsETH[payer] = depositedAmount - paidAmountWei;
            }
            currentBlockConsumption += paidAmountGas;
            claimableUsedFHEGas += paidAmountWei;
        }
    }

    function checkIfNewBlock() private {
        uint256 lastBlock_ = block.number;
        if (block.number > lastBlock) {
            lastBlock = lastBlock_;
            currentBlockConsumption = 0;
        }
    }

    function payForFheAdd(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 65000);
            } else if (resultType == 2) {
                updateFunding(payer, 94000);
            } else if (resultType == 3) {
                updateFunding(payer, 133000);
            } else if (resultType == 4) {
                updateFunding(payer, 162000);
            } else if (resultType == 5) {
                updateFunding(payer, 188000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 65000);
            } else if (resultType == 2) {
                updateFunding(payer, 94000);
            } else if (resultType == 3) {
                updateFunding(payer, 133000);
            } else if (resultType == 4) {
                updateFunding(payer, 162000);
            } else if (resultType == 5) {
                updateFunding(payer, 188000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheSub(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 65000);
            } else if (resultType == 2) {
                updateFunding(payer, 94000);
            } else if (resultType == 3) {
                updateFunding(payer, 133000);
            } else if (resultType == 4) {
                updateFunding(payer, 162000);
            } else if (resultType == 5) {
                updateFunding(payer, 188000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 65000);
            } else if (resultType == 2) {
                updateFunding(payer, 94000);
            } else if (resultType == 3) {
                updateFunding(payer, 133000);
            } else if (resultType == 4) {
                updateFunding(payer, 162000);
            } else if (resultType == 5) {
                updateFunding(payer, 188000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheMul(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 88000);
            } else if (resultType == 2) {
                updateFunding(payer, 159000);
            } else if (resultType == 3) {
                updateFunding(payer, 208000);
            } else if (resultType == 4) {
                updateFunding(payer, 264000);
            } else if (resultType == 5) {
                updateFunding(payer, 356000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 150000);
            } else if (resultType == 2) {
                updateFunding(payer, 197000);
            } else if (resultType == 3) {
                updateFunding(payer, 262000);
            } else if (resultType == 4) {
                updateFunding(payer, 359000);
            } else if (resultType == 5) {
                updateFunding(payer, 641000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheDiv(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();
        if (resultType == 1) {
            updateFunding(payer, 139000);
        } else if (resultType == 2) {
            updateFunding(payer, 238000);
        } else if (resultType == 3) {
            updateFunding(payer, 314000);
        } else if (resultType == 4) {
            updateFunding(payer, 398000);
        } else if (resultType == 5) {
            updateFunding(payer, 584000);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheRem(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();
        if (resultType == 1) {
            updateFunding(payer, 286000);
        } else if (resultType == 2) {
            updateFunding(payer, 460000);
        } else if (resultType == 3) {
            updateFunding(payer, 622000);
        } else if (resultType == 4) {
            updateFunding(payer, 805000);
        } else if (resultType == 5) {
            updateFunding(payer, 1095000);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheBitAnd(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte != 0x00) revert OnlyNonScalarOperationsAreSupported();
        if (resultType == 0) {
            updateFunding(payer, 26000);
        } else if (resultType == 1) {
            updateFunding(payer, 32000);
        } else if (resultType == 2) {
            updateFunding(payer, 34000);
        } else if (resultType == 3) {
            updateFunding(payer, 34000);
        } else if (resultType == 4) {
            updateFunding(payer, 35000);
        } else if (resultType == 5) {
            updateFunding(payer, 38000);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheBitOr(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte != 0x00) revert OnlyNonScalarOperationsAreSupported();
        if (resultType == 0) {
            updateFunding(payer, 26000);
        } else if (resultType == 1) {
            updateFunding(payer, 32000);
        } else if (resultType == 2) {
            updateFunding(payer, 34000);
        } else if (resultType == 3) {
            updateFunding(payer, 34000);
        } else if (resultType == 4) {
            updateFunding(payer, 35000);
        } else if (resultType == 5) {
            updateFunding(payer, 38000);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheBitXor(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte != 0x00) revert OnlyNonScalarOperationsAreSupported();
        if (resultType == 0) {
            updateFunding(payer, 26000);
        } else if (resultType == 1) {
            updateFunding(payer, 32000);
        } else if (resultType == 2) {
            updateFunding(payer, 34000);
        } else if (resultType == 3) {
            updateFunding(payer, 34000);
        } else if (resultType == 4) {
            updateFunding(payer, 35000);
        } else if (resultType == 5) {
            updateFunding(payer, 38000);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheShl(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 35000);
            } else if (resultType == 2) {
                updateFunding(payer, 35000);
            } else if (resultType == 3) {
                updateFunding(payer, 35000);
            } else if (resultType == 4) {
                updateFunding(payer, 35000);
            } else if (resultType == 5) {
                updateFunding(payer, 38000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 116000);
            } else if (resultType == 2) {
                updateFunding(payer, 133000);
            } else if (resultType == 3) {
                updateFunding(payer, 153000);
            } else if (resultType == 4) {
                updateFunding(payer, 183000);
            } else if (resultType == 5) {
                updateFunding(payer, 227000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheShr(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 35000);
            } else if (resultType == 2) {
                updateFunding(payer, 35000);
            } else if (resultType == 3) {
                updateFunding(payer, 35000);
            } else if (resultType == 4) {
                updateFunding(payer, 35000);
            } else if (resultType == 5) {
                updateFunding(payer, 38000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 116000);
            } else if (resultType == 2) {
                updateFunding(payer, 133000);
            } else if (resultType == 3) {
                updateFunding(payer, 153000);
            } else if (resultType == 4) {
                updateFunding(payer, 183000);
            } else if (resultType == 5) {
                updateFunding(payer, 227000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheRotl(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 35000);
            } else if (resultType == 2) {
                updateFunding(payer, 35000);
            } else if (resultType == 3) {
                updateFunding(payer, 35000);
            } else if (resultType == 4) {
                updateFunding(payer, 35000);
            } else if (resultType == 5) {
                updateFunding(payer, 38000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 116000);
            } else if (resultType == 2) {
                updateFunding(payer, 133000);
            } else if (resultType == 3) {
                updateFunding(payer, 153000);
            } else if (resultType == 4) {
                updateFunding(payer, 183000);
            } else if (resultType == 5) {
                updateFunding(payer, 227000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheRotr(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 35000);
            } else if (resultType == 2) {
                updateFunding(payer, 35000);
            } else if (resultType == 3) {
                updateFunding(payer, 35000);
            } else if (resultType == 4) {
                updateFunding(payer, 35000);
            } else if (resultType == 5) {
                updateFunding(payer, 38000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 116000);
            } else if (resultType == 2) {
                updateFunding(payer, 133000);
            } else if (resultType == 3) {
                updateFunding(payer, 153000);
            } else if (resultType == 4) {
                updateFunding(payer, 183000);
            } else if (resultType == 5) {
                updateFunding(payer, 227000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheEq(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 51000);
            } else if (resultType == 2) {
                updateFunding(payer, 53000);
            } else if (resultType == 3) {
                updateFunding(payer, 54000);
            } else if (resultType == 4) {
                updateFunding(payer, 82000);
            } else if (resultType == 5) {
                updateFunding(payer, 86000);
            } else if (resultType == 7) {
                updateFunding(payer, 90000);
            } else if (resultType == 11) {
                updateFunding(payer, 300000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 51000);
            } else if (resultType == 2) {
                updateFunding(payer, 53000);
            } else if (resultType == 3) {
                updateFunding(payer, 54000);
            } else if (resultType == 4) {
                updateFunding(payer, 82000);
            } else if (resultType == 5) {
                updateFunding(payer, 86000);
            } else if (resultType == 7) {
                updateFunding(payer, 90000);
            } else if (resultType == 11) {
                updateFunding(payer, 300000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheNe(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 51000);
            } else if (resultType == 2) {
                updateFunding(payer, 53000);
            } else if (resultType == 3) {
                updateFunding(payer, 54000);
            } else if (resultType == 4) {
                updateFunding(payer, 82000);
            } else if (resultType == 5) {
                updateFunding(payer, 86000);
            } else if (resultType == 7) {
                updateFunding(payer, 90000);
            } else if (resultType == 11) {
                updateFunding(payer, 300000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 51000);
            } else if (resultType == 2) {
                updateFunding(payer, 53000);
            } else if (resultType == 3) {
                updateFunding(payer, 54000);
            } else if (resultType == 4) {
                updateFunding(payer, 82000);
            } else if (resultType == 5) {
                updateFunding(payer, 86000);
            } else if (resultType == 7) {
                updateFunding(payer, 90000);
            } else if (resultType == 11) {
                updateFunding(payer, 300000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheGe(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 70000);
            } else if (resultType == 2) {
                updateFunding(payer, 82000);
            } else if (resultType == 3) {
                updateFunding(payer, 105000);
            } else if (resultType == 4) {
                updateFunding(payer, 128000);
            } else if (resultType == 5) {
                updateFunding(payer, 156000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 70000);
            } else if (resultType == 2) {
                updateFunding(payer, 82000);
            } else if (resultType == 3) {
                updateFunding(payer, 105000);
            } else if (resultType == 4) {
                updateFunding(payer, 128000);
            } else if (resultType == 5) {
                updateFunding(payer, 156000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheGt(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 70000);
            } else if (resultType == 2) {
                updateFunding(payer, 82000);
            } else if (resultType == 3) {
                updateFunding(payer, 105000);
            } else if (resultType == 4) {
                updateFunding(payer, 128000);
            } else if (resultType == 5) {
                updateFunding(payer, 156000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 70000);
            } else if (resultType == 2) {
                updateFunding(payer, 82000);
            } else if (resultType == 3) {
                updateFunding(payer, 105000);
            } else if (resultType == 4) {
                updateFunding(payer, 128000);
            } else if (resultType == 5) {
                updateFunding(payer, 156000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheLe(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 70000);
            } else if (resultType == 2) {
                updateFunding(payer, 82000);
            } else if (resultType == 3) {
                updateFunding(payer, 105000);
            } else if (resultType == 4) {
                updateFunding(payer, 128000);
            } else if (resultType == 5) {
                updateFunding(payer, 156000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 70000);
            } else if (resultType == 2) {
                updateFunding(payer, 82000);
            } else if (resultType == 3) {
                updateFunding(payer, 105000);
            } else if (resultType == 4) {
                updateFunding(payer, 128000);
            } else if (resultType == 5) {
                updateFunding(payer, 156000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheLt(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 70000);
            } else if (resultType == 2) {
                updateFunding(payer, 82000);
            } else if (resultType == 3) {
                updateFunding(payer, 105000);
            } else if (resultType == 4) {
                updateFunding(payer, 128000);
            } else if (resultType == 5) {
                updateFunding(payer, 156000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 70000);
            } else if (resultType == 2) {
                updateFunding(payer, 82000);
            } else if (resultType == 3) {
                updateFunding(payer, 105000);
            } else if (resultType == 4) {
                updateFunding(payer, 128000);
            } else if (resultType == 5) {
                updateFunding(payer, 156000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheMin(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 121000);
            } else if (resultType == 2) {
                updateFunding(payer, 128000);
            } else if (resultType == 3) {
                updateFunding(payer, 150000);
            } else if (resultType == 4) {
                updateFunding(payer, 164000);
            } else if (resultType == 5) {
                updateFunding(payer, 192000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 121000);
            } else if (resultType == 2) {
                updateFunding(payer, 128000);
            } else if (resultType == 3) {
                updateFunding(payer, 153000);
            } else if (resultType == 4) {
                updateFunding(payer, 183000);
            } else if (resultType == 5) {
                updateFunding(payer, 210000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheMax(address payer, uint8 resultType, bytes1 scalarByte) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(payer, 121000);
            } else if (resultType == 2) {
                updateFunding(payer, 128000);
            } else if (resultType == 3) {
                updateFunding(payer, 150000);
            } else if (resultType == 4) {
                updateFunding(payer, 164000);
            } else if (resultType == 5) {
                updateFunding(payer, 192000);
            }
        } else {
            if (resultType == 1) {
                updateFunding(payer, 121000);
            } else if (resultType == 2) {
                updateFunding(payer, 128000);
            } else if (resultType == 3) {
                updateFunding(payer, 153000);
            } else if (resultType == 4) {
                updateFunding(payer, 183000);
            } else if (resultType == 5) {
                updateFunding(payer, 210000);
            }
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheNeg(address payer, uint8 resultType) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        if (resultType == 1) {
            updateFunding(payer, 60000);
        } else if (resultType == 2) {
            updateFunding(payer, 95000);
        } else if (resultType == 3) {
            updateFunding(payer, 131000);
        } else if (resultType == 4) {
            updateFunding(payer, 160000);
        } else if (resultType == 5) {
            updateFunding(payer, 199000);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheNot(address payer, uint8 resultType) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        if (resultType == 0) {
            updateFunding(payer, 30000);
        } else if (resultType == 1) {
            updateFunding(payer, 33000);
        } else if (resultType == 2) {
            updateFunding(payer, 34000);
        } else if (resultType == 3) {
            updateFunding(payer, 35000);
        } else if (resultType == 4) {
            updateFunding(payer, 36000);
        } else if (resultType == 5) {
            updateFunding(payer, 37000);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForCast(address payer, uint8 resultType) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        if (resultType == 1) {
            updateFunding(payer, 200);
        } else if (resultType == 2) {
            updateFunding(payer, 200);
        } else if (resultType == 3) {
            updateFunding(payer, 200);
        } else if (resultType == 4) {
            updateFunding(payer, 200);
        } else if (resultType == 5) {
            updateFunding(payer, 200);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForTrivialEncrypt(address payer, uint8 resultType) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        if (resultType == 0) {
            updateFunding(payer, 100);
        } else if (resultType == 1) {
            updateFunding(payer, 100);
        } else if (resultType == 2) {
            updateFunding(payer, 100);
        } else if (resultType == 3) {
            updateFunding(payer, 200);
        } else if (resultType == 4) {
            updateFunding(payer, 300);
        } else if (resultType == 5) {
            updateFunding(payer, 600);
        } else if (resultType == 7) {
            updateFunding(payer, 700);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForIfThenElse(address payer, uint8 resultType) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        if (resultType == 1) {
            updateFunding(payer, 45000);
        } else if (resultType == 2) {
            updateFunding(payer, 47000);
        } else if (resultType == 3) {
            updateFunding(payer, 47000);
        } else if (resultType == 4) {
            updateFunding(payer, 50000);
        } else if (resultType == 5) {
            updateFunding(payer, 53000);
        } else if (resultType == 7) {
            updateFunding(payer, 80000);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheRand(address payer, uint8 resultType) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        if (resultType == 2) {
            updateFunding(payer, 100000);
        } else if (resultType == 3) {
            updateFunding(payer, 100000);
        } else if (resultType == 4) {
            updateFunding(payer, 100000);
        } else if (resultType == 5) {
            updateFunding(payer, 100000);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    function payForFheRandBounded(address payer, uint8 resultType) external {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        if (resultType == 2) {
            updateFunding(payer, 100000);
        } else if (resultType == 3) {
            updateFunding(payer, 100000);
        } else if (resultType == 4) {
            updateFunding(payer, 100000);
        } else if (resultType == 5) {
            updateFunding(payer, 100000);
        }
        if (currentBlockConsumption >= FHE_GAS_BLOCKLIMIT) revert FHEGasBlockLimitExceeded();
    }

    /// @notice Getter for the name and version of the contract
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
}
