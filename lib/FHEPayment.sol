// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "./TFHEExecutorAddress.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";

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
        require(success, "Withdrawal failed");
    }

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
        uint64 lastBlock_ = uint64(block.number);
        if (block.number > lastBlock) {
            lastBlock = lastBlock_;
            currentBlockConsumption = 0;
        }
    }

    function payForFheAdd(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 65000;
                currentBlockConsumption += 65000;
                claimableUsedFHEGas += 65000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 94000;
                currentBlockConsumption += 94000;
                claimableUsedFHEGas += 94000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 133000;
                currentBlockConsumption += 133000;
                claimableUsedFHEGas += 133000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 162000;
                currentBlockConsumption += 162000;
                claimableUsedFHEGas += 162000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 188000;
                currentBlockConsumption += 188000;
                claimableUsedFHEGas += 188000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 65000;
                currentBlockConsumption += 65000;
                claimableUsedFHEGas += 65000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 94000;
                currentBlockConsumption += 94000;
                claimableUsedFHEGas += 94000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 133000;
                currentBlockConsumption += 133000;
                claimableUsedFHEGas += 133000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 162000;
                currentBlockConsumption += 162000;
                claimableUsedFHEGas += 162000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 188000;
                currentBlockConsumption += 188000;
                claimableUsedFHEGas += 188000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheSub(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 65000;
                currentBlockConsumption += 65000;
                claimableUsedFHEGas += 65000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 94000;
                currentBlockConsumption += 94000;
                claimableUsedFHEGas += 94000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 133000;
                currentBlockConsumption += 133000;
                claimableUsedFHEGas += 133000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 162000;
                currentBlockConsumption += 162000;
                claimableUsedFHEGas += 162000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 188000;
                currentBlockConsumption += 188000;
                claimableUsedFHEGas += 188000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 65000;
                currentBlockConsumption += 65000;
                claimableUsedFHEGas += 65000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 94000;
                currentBlockConsumption += 94000;
                claimableUsedFHEGas += 94000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 133000;
                currentBlockConsumption += 133000;
                claimableUsedFHEGas += 133000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 162000;
                currentBlockConsumption += 162000;
                claimableUsedFHEGas += 162000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 188000;
                currentBlockConsumption += 188000;
                claimableUsedFHEGas += 188000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheMul(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 88000;
                currentBlockConsumption += 88000;
                claimableUsedFHEGas += 88000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 159000;
                currentBlockConsumption += 159000;
                claimableUsedFHEGas += 159000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 208000;
                currentBlockConsumption += 208000;
                claimableUsedFHEGas += 208000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 264000;
                currentBlockConsumption += 264000;
                claimableUsedFHEGas += 264000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 356000;
                currentBlockConsumption += 356000;
                claimableUsedFHEGas += 356000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 150000;
                currentBlockConsumption += 150000;
                claimableUsedFHEGas += 150000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 197000;
                currentBlockConsumption += 197000;
                claimableUsedFHEGas += 197000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 262000;
                currentBlockConsumption += 262000;
                claimableUsedFHEGas += 262000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 359000;
                currentBlockConsumption += 359000;
                claimableUsedFHEGas += 359000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 641000;
                currentBlockConsumption += 641000;
                claimableUsedFHEGas += 641000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheDiv(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        require(scalarByte == 0x01, "Only scalar operations are supported");
        if (resultType == 1) {
            depositsETH[payer] -= 139000;
            currentBlockConsumption += 139000;
            claimableUsedFHEGas += 139000;
        } else if (resultType == 2) {
            depositsETH[payer] -= 238000;
            currentBlockConsumption += 238000;
            claimableUsedFHEGas += 238000;
        } else if (resultType == 3) {
            depositsETH[payer] -= 314000;
            currentBlockConsumption += 314000;
            claimableUsedFHEGas += 314000;
        } else if (resultType == 4) {
            depositsETH[payer] -= 398000;
            currentBlockConsumption += 398000;
            claimableUsedFHEGas += 398000;
        } else if (resultType == 5) {
            depositsETH[payer] -= 584000;
            currentBlockConsumption += 584000;
            claimableUsedFHEGas += 584000;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheRem(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        require(scalarByte == 0x01, "Only scalar operations are supported");
        if (resultType == 1) {
            depositsETH[payer] -= 286000;
            currentBlockConsumption += 286000;
            claimableUsedFHEGas += 286000;
        } else if (resultType == 2) {
            depositsETH[payer] -= 460000;
            currentBlockConsumption += 460000;
            claimableUsedFHEGas += 460000;
        } else if (resultType == 3) {
            depositsETH[payer] -= 622000;
            currentBlockConsumption += 622000;
            claimableUsedFHEGas += 622000;
        } else if (resultType == 4) {
            depositsETH[payer] -= 805000;
            currentBlockConsumption += 805000;
            claimableUsedFHEGas += 805000;
        } else if (resultType == 5) {
            depositsETH[payer] -= 1095000;
            currentBlockConsumption += 1095000;
            claimableUsedFHEGas += 1095000;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheBitAnd(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        require(scalarByte == 0x00, "Only non-scalar operations are supported");
        if (resultType == 0) {
            depositsETH[payer] -= 26000;
            currentBlockConsumption += 26000;
            claimableUsedFHEGas += 26000;
        } else if (resultType == 1) {
            depositsETH[payer] -= 32000;
            currentBlockConsumption += 32000;
            claimableUsedFHEGas += 32000;
        } else if (resultType == 2) {
            depositsETH[payer] -= 34000;
            currentBlockConsumption += 34000;
            claimableUsedFHEGas += 34000;
        } else if (resultType == 3) {
            depositsETH[payer] -= 34000;
            currentBlockConsumption += 34000;
            claimableUsedFHEGas += 34000;
        } else if (resultType == 4) {
            depositsETH[payer] -= 35000;
            currentBlockConsumption += 35000;
            claimableUsedFHEGas += 35000;
        } else if (resultType == 5) {
            depositsETH[payer] -= 38000;
            currentBlockConsumption += 38000;
            claimableUsedFHEGas += 38000;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheBitOr(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        require(scalarByte == 0x00, "Only non-scalar operations are supported");
        if (resultType == 0) {
            depositsETH[payer] -= 26000;
            currentBlockConsumption += 26000;
            claimableUsedFHEGas += 26000;
        } else if (resultType == 1) {
            depositsETH[payer] -= 32000;
            currentBlockConsumption += 32000;
            claimableUsedFHEGas += 32000;
        } else if (resultType == 2) {
            depositsETH[payer] -= 34000;
            currentBlockConsumption += 34000;
            claimableUsedFHEGas += 34000;
        } else if (resultType == 3) {
            depositsETH[payer] -= 34000;
            currentBlockConsumption += 34000;
            claimableUsedFHEGas += 34000;
        } else if (resultType == 4) {
            depositsETH[payer] -= 35000;
            currentBlockConsumption += 35000;
            claimableUsedFHEGas += 35000;
        } else if (resultType == 5) {
            depositsETH[payer] -= 38000;
            currentBlockConsumption += 38000;
            claimableUsedFHEGas += 38000;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheBitXor(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        require(scalarByte == 0x00, "Only non-scalar operations are supported");
        if (resultType == 0) {
            depositsETH[payer] -= 26000;
            currentBlockConsumption += 26000;
            claimableUsedFHEGas += 26000;
        } else if (resultType == 1) {
            depositsETH[payer] -= 32000;
            currentBlockConsumption += 32000;
            claimableUsedFHEGas += 32000;
        } else if (resultType == 2) {
            depositsETH[payer] -= 34000;
            currentBlockConsumption += 34000;
            claimableUsedFHEGas += 34000;
        } else if (resultType == 3) {
            depositsETH[payer] -= 34000;
            currentBlockConsumption += 34000;
            claimableUsedFHEGas += 34000;
        } else if (resultType == 4) {
            depositsETH[payer] -= 35000;
            currentBlockConsumption += 35000;
            claimableUsedFHEGas += 35000;
        } else if (resultType == 5) {
            depositsETH[payer] -= 38000;
            currentBlockConsumption += 38000;
            claimableUsedFHEGas += 38000;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheShl(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 38000;
                currentBlockConsumption += 38000;
                claimableUsedFHEGas += 38000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 116000;
                currentBlockConsumption += 116000;
                claimableUsedFHEGas += 116000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 133000;
                currentBlockConsumption += 133000;
                claimableUsedFHEGas += 133000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 153000;
                currentBlockConsumption += 153000;
                claimableUsedFHEGas += 153000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 183000;
                currentBlockConsumption += 183000;
                claimableUsedFHEGas += 183000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 227000;
                currentBlockConsumption += 227000;
                claimableUsedFHEGas += 227000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheShr(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 38000;
                currentBlockConsumption += 38000;
                claimableUsedFHEGas += 38000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 116000;
                currentBlockConsumption += 116000;
                claimableUsedFHEGas += 116000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 133000;
                currentBlockConsumption += 133000;
                claimableUsedFHEGas += 133000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 153000;
                currentBlockConsumption += 153000;
                claimableUsedFHEGas += 153000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 183000;
                currentBlockConsumption += 183000;
                claimableUsedFHEGas += 183000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 227000;
                currentBlockConsumption += 227000;
                claimableUsedFHEGas += 227000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheRotl(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 38000;
                currentBlockConsumption += 38000;
                claimableUsedFHEGas += 38000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 116000;
                currentBlockConsumption += 116000;
                claimableUsedFHEGas += 116000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 133000;
                currentBlockConsumption += 133000;
                claimableUsedFHEGas += 133000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 153000;
                currentBlockConsumption += 153000;
                claimableUsedFHEGas += 153000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 183000;
                currentBlockConsumption += 183000;
                claimableUsedFHEGas += 183000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 227000;
                currentBlockConsumption += 227000;
                claimableUsedFHEGas += 227000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheRotr(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 35000;
                currentBlockConsumption += 35000;
                claimableUsedFHEGas += 35000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 38000;
                currentBlockConsumption += 38000;
                claimableUsedFHEGas += 38000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 116000;
                currentBlockConsumption += 116000;
                claimableUsedFHEGas += 116000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 133000;
                currentBlockConsumption += 133000;
                claimableUsedFHEGas += 133000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 153000;
                currentBlockConsumption += 153000;
                claimableUsedFHEGas += 153000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 183000;
                currentBlockConsumption += 183000;
                claimableUsedFHEGas += 183000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 227000;
                currentBlockConsumption += 227000;
                claimableUsedFHEGas += 227000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheEq(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 51000;
                currentBlockConsumption += 51000;
                claimableUsedFHEGas += 51000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 53000;
                currentBlockConsumption += 53000;
                claimableUsedFHEGas += 53000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 54000;
                currentBlockConsumption += 54000;
                claimableUsedFHEGas += 54000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 86000;
                currentBlockConsumption += 86000;
                claimableUsedFHEGas += 86000;
            } else if (resultType == 7) {
                depositsETH[payer] -= 90000;
                currentBlockConsumption += 90000;
                claimableUsedFHEGas += 90000;
            } else if (resultType == 11) {
                depositsETH[payer] -= 300000;
                currentBlockConsumption += 300000;
                claimableUsedFHEGas += 300000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 51000;
                currentBlockConsumption += 51000;
                claimableUsedFHEGas += 51000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 53000;
                currentBlockConsumption += 53000;
                claimableUsedFHEGas += 53000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 54000;
                currentBlockConsumption += 54000;
                claimableUsedFHEGas += 54000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 86000;
                currentBlockConsumption += 86000;
                claimableUsedFHEGas += 86000;
            } else if (resultType == 7) {
                depositsETH[payer] -= 90000;
                currentBlockConsumption += 90000;
                claimableUsedFHEGas += 90000;
            } else if (resultType == 11) {
                depositsETH[payer] -= 300000;
                currentBlockConsumption += 300000;
                claimableUsedFHEGas += 300000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheNe(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 51000;
                currentBlockConsumption += 51000;
                claimableUsedFHEGas += 51000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 53000;
                currentBlockConsumption += 53000;
                claimableUsedFHEGas += 53000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 54000;
                currentBlockConsumption += 54000;
                claimableUsedFHEGas += 54000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 86000;
                currentBlockConsumption += 86000;
                claimableUsedFHEGas += 86000;
            } else if (resultType == 7) {
                depositsETH[payer] -= 90000;
                currentBlockConsumption += 90000;
                claimableUsedFHEGas += 90000;
            } else if (resultType == 11) {
                depositsETH[payer] -= 300000;
                currentBlockConsumption += 300000;
                claimableUsedFHEGas += 300000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 51000;
                currentBlockConsumption += 51000;
                claimableUsedFHEGas += 51000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 53000;
                currentBlockConsumption += 53000;
                claimableUsedFHEGas += 53000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 54000;
                currentBlockConsumption += 54000;
                claimableUsedFHEGas += 54000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 86000;
                currentBlockConsumption += 86000;
                claimableUsedFHEGas += 86000;
            } else if (resultType == 7) {
                depositsETH[payer] -= 90000;
                currentBlockConsumption += 90000;
                claimableUsedFHEGas += 90000;
            } else if (resultType == 11) {
                depositsETH[payer] -= 300000;
                currentBlockConsumption += 300000;
                claimableUsedFHEGas += 300000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheGe(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 70000;
                currentBlockConsumption += 70000;
                claimableUsedFHEGas += 70000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 105000;
                currentBlockConsumption += 105000;
                claimableUsedFHEGas += 105000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 156000;
                currentBlockConsumption += 156000;
                claimableUsedFHEGas += 156000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 70000;
                currentBlockConsumption += 70000;
                claimableUsedFHEGas += 70000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 105000;
                currentBlockConsumption += 105000;
                claimableUsedFHEGas += 105000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 156000;
                currentBlockConsumption += 156000;
                claimableUsedFHEGas += 156000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheGt(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 70000;
                currentBlockConsumption += 70000;
                claimableUsedFHEGas += 70000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 105000;
                currentBlockConsumption += 105000;
                claimableUsedFHEGas += 105000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 156000;
                currentBlockConsumption += 156000;
                claimableUsedFHEGas += 156000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 70000;
                currentBlockConsumption += 70000;
                claimableUsedFHEGas += 70000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 105000;
                currentBlockConsumption += 105000;
                claimableUsedFHEGas += 105000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 156000;
                currentBlockConsumption += 156000;
                claimableUsedFHEGas += 156000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheLe(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 70000;
                currentBlockConsumption += 70000;
                claimableUsedFHEGas += 70000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 105000;
                currentBlockConsumption += 105000;
                claimableUsedFHEGas += 105000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 156000;
                currentBlockConsumption += 156000;
                claimableUsedFHEGas += 156000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 70000;
                currentBlockConsumption += 70000;
                claimableUsedFHEGas += 70000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 105000;
                currentBlockConsumption += 105000;
                claimableUsedFHEGas += 105000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 156000;
                currentBlockConsumption += 156000;
                claimableUsedFHEGas += 156000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheLt(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 70000;
                currentBlockConsumption += 70000;
                claimableUsedFHEGas += 70000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 105000;
                currentBlockConsumption += 105000;
                claimableUsedFHEGas += 105000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 156000;
                currentBlockConsumption += 156000;
                claimableUsedFHEGas += 156000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 70000;
                currentBlockConsumption += 70000;
                claimableUsedFHEGas += 70000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 82000;
                currentBlockConsumption += 82000;
                claimableUsedFHEGas += 82000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 105000;
                currentBlockConsumption += 105000;
                claimableUsedFHEGas += 105000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 156000;
                currentBlockConsumption += 156000;
                claimableUsedFHEGas += 156000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheMin(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 121000;
                currentBlockConsumption += 121000;
                claimableUsedFHEGas += 121000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 150000;
                currentBlockConsumption += 150000;
                claimableUsedFHEGas += 150000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 164000;
                currentBlockConsumption += 164000;
                claimableUsedFHEGas += 164000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 192000;
                currentBlockConsumption += 192000;
                claimableUsedFHEGas += 192000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 121000;
                currentBlockConsumption += 121000;
                claimableUsedFHEGas += 121000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 153000;
                currentBlockConsumption += 153000;
                claimableUsedFHEGas += 153000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 183000;
                currentBlockConsumption += 183000;
                claimableUsedFHEGas += 183000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 210000;
                currentBlockConsumption += 210000;
                claimableUsedFHEGas += 210000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheMax(address payer, uint8 resultType, bytes1 scalarByte) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                depositsETH[payer] -= 121000;
                currentBlockConsumption += 121000;
                claimableUsedFHEGas += 121000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 150000;
                currentBlockConsumption += 150000;
                claimableUsedFHEGas += 150000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 164000;
                currentBlockConsumption += 164000;
                claimableUsedFHEGas += 164000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 192000;
                currentBlockConsumption += 192000;
                claimableUsedFHEGas += 192000;
            }
        } else {
            if (resultType == 1) {
                depositsETH[payer] -= 121000;
                currentBlockConsumption += 121000;
                claimableUsedFHEGas += 121000;
            } else if (resultType == 2) {
                depositsETH[payer] -= 128000;
                currentBlockConsumption += 128000;
                claimableUsedFHEGas += 128000;
            } else if (resultType == 3) {
                depositsETH[payer] -= 153000;
                currentBlockConsumption += 153000;
                claimableUsedFHEGas += 153000;
            } else if (resultType == 4) {
                depositsETH[payer] -= 183000;
                currentBlockConsumption += 183000;
                claimableUsedFHEGas += 183000;
            } else if (resultType == 5) {
                depositsETH[payer] -= 210000;
                currentBlockConsumption += 210000;
                claimableUsedFHEGas += 210000;
            }
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheNeg(address payer, uint8 resultType) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        if (resultType == 1) {
            depositsETH[payer] -= 60000;
            currentBlockConsumption += 60000;
            claimableUsedFHEGas += 60000;
        } else if (resultType == 2) {
            depositsETH[payer] -= 95000;
            currentBlockConsumption += 95000;
            claimableUsedFHEGas += 95000;
        } else if (resultType == 3) {
            depositsETH[payer] -= 131000;
            currentBlockConsumption += 131000;
            claimableUsedFHEGas += 131000;
        } else if (resultType == 4) {
            depositsETH[payer] -= 160000;
            currentBlockConsumption += 160000;
            claimableUsedFHEGas += 160000;
        } else if (resultType == 5) {
            depositsETH[payer] -= 199000;
            currentBlockConsumption += 199000;
            claimableUsedFHEGas += 199000;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheNot(address payer, uint8 resultType) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        if (resultType == 0) {
            depositsETH[payer] -= 30000;
            currentBlockConsumption += 30000;
            claimableUsedFHEGas += 30000;
        } else if (resultType == 1) {
            depositsETH[payer] -= 33000;
            currentBlockConsumption += 33000;
            claimableUsedFHEGas += 33000;
        } else if (resultType == 2) {
            depositsETH[payer] -= 34000;
            currentBlockConsumption += 34000;
            claimableUsedFHEGas += 34000;
        } else if (resultType == 3) {
            depositsETH[payer] -= 35000;
            currentBlockConsumption += 35000;
            claimableUsedFHEGas += 35000;
        } else if (resultType == 4) {
            depositsETH[payer] -= 36000;
            currentBlockConsumption += 36000;
            claimableUsedFHEGas += 36000;
        } else if (resultType == 5) {
            depositsETH[payer] -= 37000;
            currentBlockConsumption += 37000;
            claimableUsedFHEGas += 37000;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForCast(address payer, uint8 resultType) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        if (resultType == 1) {
            depositsETH[payer] -= 200;
            currentBlockConsumption += 200;
            claimableUsedFHEGas += 200;
        } else if (resultType == 2) {
            depositsETH[payer] -= 200;
            currentBlockConsumption += 200;
            claimableUsedFHEGas += 200;
        } else if (resultType == 3) {
            depositsETH[payer] -= 200;
            currentBlockConsumption += 200;
            claimableUsedFHEGas += 200;
        } else if (resultType == 4) {
            depositsETH[payer] -= 200;
            currentBlockConsumption += 200;
            claimableUsedFHEGas += 200;
        } else if (resultType == 5) {
            depositsETH[payer] -= 200;
            currentBlockConsumption += 200;
            claimableUsedFHEGas += 200;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForTrivialEncrypt(address payer, uint8 resultType) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        if (resultType == 0) {
            depositsETH[payer] -= 100;
            currentBlockConsumption += 100;
            claimableUsedFHEGas += 100;
        } else if (resultType == 1) {
            depositsETH[payer] -= 100;
            currentBlockConsumption += 100;
            claimableUsedFHEGas += 100;
        } else if (resultType == 2) {
            depositsETH[payer] -= 100;
            currentBlockConsumption += 100;
            claimableUsedFHEGas += 100;
        } else if (resultType == 3) {
            depositsETH[payer] -= 200;
            currentBlockConsumption += 200;
            claimableUsedFHEGas += 200;
        } else if (resultType == 4) {
            depositsETH[payer] -= 300;
            currentBlockConsumption += 300;
            claimableUsedFHEGas += 300;
        } else if (resultType == 5) {
            depositsETH[payer] -= 600;
            currentBlockConsumption += 600;
            claimableUsedFHEGas += 600;
        } else if (resultType == 7) {
            depositsETH[payer] -= 700;
            currentBlockConsumption += 700;
            claimableUsedFHEGas += 700;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForIfThenElse(address payer, uint8 resultType) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        if (resultType == 1) {
            depositsETH[payer] -= 45000;
            currentBlockConsumption += 45000;
            claimableUsedFHEGas += 45000;
        } else if (resultType == 2) {
            depositsETH[payer] -= 47000;
            currentBlockConsumption += 47000;
            claimableUsedFHEGas += 47000;
        } else if (resultType == 3) {
            depositsETH[payer] -= 47000;
            currentBlockConsumption += 47000;
            claimableUsedFHEGas += 47000;
        } else if (resultType == 4) {
            depositsETH[payer] -= 50000;
            currentBlockConsumption += 50000;
            claimableUsedFHEGas += 50000;
        } else if (resultType == 5) {
            depositsETH[payer] -= 53000;
            currentBlockConsumption += 53000;
            claimableUsedFHEGas += 53000;
        } else if (resultType == 7) {
            depositsETH[payer] -= 80000;
            currentBlockConsumption += 80000;
            claimableUsedFHEGas += 80000;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheRand(address payer, uint8 resultType) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        if (resultType == 2) {
            depositsETH[payer] -= 100000;
            currentBlockConsumption += 100000;
            claimableUsedFHEGas += 100000;
        } else if (resultType == 3) {
            depositsETH[payer] -= 100000;
            currentBlockConsumption += 100000;
            claimableUsedFHEGas += 100000;
        } else if (resultType == 4) {
            depositsETH[payer] -= 100000;
            currentBlockConsumption += 100000;
            claimableUsedFHEGas += 100000;
        } else if (resultType == 5) {
            depositsETH[payer] -= 100000;
            currentBlockConsumption += 100000;
            claimableUsedFHEGas += 100000;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
    }

    function payForFheRandBounded(address payer, uint8 resultType) external {
        require(msg.sender == tfheExecutorAddress, "Caller must be TFHEExecutor contract");
        if (resultType == 2) {
            depositsETH[payer] -= 100000;
            currentBlockConsumption += 100000;
            claimableUsedFHEGas += 100000;
        } else if (resultType == 3) {
            depositsETH[payer] -= 100000;
            currentBlockConsumption += 100000;
            claimableUsedFHEGas += 100000;
        } else if (resultType == 4) {
            depositsETH[payer] -= 100000;
            currentBlockConsumption += 100000;
            claimableUsedFHEGas += 100000;
        } else if (resultType == 5) {
            depositsETH[payer] -= 100000;
            currentBlockConsumption += 100000;
            claimableUsedFHEGas += 100000;
        }
        require(currentBlockConsumption <= FHE_GAS_BLOCKLIMIT, "FHEGas block limit exceeded");
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
