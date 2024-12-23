// SPDX-License-Identifier: BSD-3-Clause-Clear

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

    /// @notice Initializes the contract setting `initialOwner` as the initial owner
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
    }

    function payForFheAdd(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(65000);
            } else if (resultType == 2) {
                updateFunding(94000);
            } else if (resultType == 3) {
                updateFunding(133000);
            } else if (resultType == 4) {
                updateFunding(162000);
            } else if (resultType == 5) {
                updateFunding(188000);
            } else if (resultType == 6) {
                updateFunding(218000);
            } else if (resultType == 8) {
                updateFunding(253000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(65000);
            } else if (resultType == 2) {
                updateFunding(94000);
            } else if (resultType == 3) {
                updateFunding(133000);
            } else if (resultType == 4) {
                updateFunding(162000);
            } else if (resultType == 5) {
                updateFunding(188000);
            } else if (resultType == 6) {
                updateFunding(218000);
            } else if (resultType == 8) {
                updateFunding(253000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheSub(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(65000);
            } else if (resultType == 2) {
                updateFunding(94000);
            } else if (resultType == 3) {
                updateFunding(133000);
            } else if (resultType == 4) {
                updateFunding(162000);
            } else if (resultType == 5) {
                updateFunding(188000);
            } else if (resultType == 6) {
                updateFunding(218000);
            } else if (resultType == 8) {
                updateFunding(253000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(65000);
            } else if (resultType == 2) {
                updateFunding(94000);
            } else if (resultType == 3) {
                updateFunding(133000);
            } else if (resultType == 4) {
                updateFunding(162000);
            } else if (resultType == 5) {
                updateFunding(188000);
            } else if (resultType == 6) {
                updateFunding(218000);
            } else if (resultType == 8) {
                updateFunding(253000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheMul(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(88000);
            } else if (resultType == 2) {
                updateFunding(159000);
            } else if (resultType == 3) {
                updateFunding(208000);
            } else if (resultType == 4) {
                updateFunding(264000);
            } else if (resultType == 5) {
                updateFunding(356000);
            } else if (resultType == 6) {
                updateFunding(480000);
            } else if (resultType == 8) {
                updateFunding(647000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(150000);
            } else if (resultType == 2) {
                updateFunding(197000);
            } else if (resultType == 3) {
                updateFunding(262000);
            } else if (resultType == 4) {
                updateFunding(359000);
            } else if (resultType == 5) {
                updateFunding(641000);
            } else if (resultType == 6) {
                updateFunding(1145000);
            } else if (resultType == 8) {
                updateFunding(2045000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheDiv(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();
        if (resultType == 1) {
            updateFunding(139000);
        } else if (resultType == 2) {
            updateFunding(238000);
        } else if (resultType == 3) {
            updateFunding(314000);
        } else if (resultType == 4) {
            updateFunding(398000);
        } else if (resultType == 5) {
            updateFunding(584000);
        } else if (resultType == 6) {
            updateFunding(857000);
        } else if (resultType == 8) {
            updateFunding(1258000);
        } else {
            revert UnsupportedOperation();
        }
        checkFHEGasBlockLimit();
    }

    function payForFheRem(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte != 0x01) revert OnlyScalarOperationsAreSupported();
        if (resultType == 1) {
            updateFunding(286000);
        } else if (resultType == 2) {
            updateFunding(460000);
        } else if (resultType == 3) {
            updateFunding(622000);
        } else if (resultType == 4) {
            updateFunding(805000);
        } else if (resultType == 5) {
            updateFunding(1095000);
        } else if (resultType == 6) {
            updateFunding(1499000);
        } else if (resultType == 8) {
            updateFunding(2052000);
        } else {
            revert UnsupportedOperation();
        }
        checkFHEGasBlockLimit();
    }

    function payForFheBitAnd(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 0) {
                updateFunding(26000);
            } else if (resultType == 1) {
                updateFunding(32000);
            } else if (resultType == 2) {
                updateFunding(34000);
            } else if (resultType == 3) {
                updateFunding(34000);
            } else if (resultType == 4) {
                updateFunding(35000);
            } else if (resultType == 5) {
                updateFunding(38000);
            } else if (resultType == 6) {
                updateFunding(41000);
            } else if (resultType == 8) {
                updateFunding(44000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 0) {
                updateFunding(26000);
            } else if (resultType == 1) {
                updateFunding(32000);
            } else if (resultType == 2) {
                updateFunding(34000);
            } else if (resultType == 3) {
                updateFunding(34000);
            } else if (resultType == 4) {
                updateFunding(35000);
            } else if (resultType == 5) {
                updateFunding(38000);
            } else if (resultType == 6) {
                updateFunding(41000);
            } else if (resultType == 8) {
                updateFunding(44000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheBitOr(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 0) {
                updateFunding(26000);
            } else if (resultType == 1) {
                updateFunding(32000);
            } else if (resultType == 2) {
                updateFunding(34000);
            } else if (resultType == 3) {
                updateFunding(34000);
            } else if (resultType == 4) {
                updateFunding(35000);
            } else if (resultType == 5) {
                updateFunding(38000);
            } else if (resultType == 6) {
                updateFunding(41000);
            } else if (resultType == 8) {
                updateFunding(44000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 0) {
                updateFunding(26000);
            } else if (resultType == 1) {
                updateFunding(32000);
            } else if (resultType == 2) {
                updateFunding(34000);
            } else if (resultType == 3) {
                updateFunding(34000);
            } else if (resultType == 4) {
                updateFunding(35000);
            } else if (resultType == 5) {
                updateFunding(38000);
            } else if (resultType == 6) {
                updateFunding(41000);
            } else if (resultType == 8) {
                updateFunding(44000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheBitXor(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 0) {
                updateFunding(26000);
            } else if (resultType == 1) {
                updateFunding(32000);
            } else if (resultType == 2) {
                updateFunding(34000);
            } else if (resultType == 3) {
                updateFunding(34000);
            } else if (resultType == 4) {
                updateFunding(35000);
            } else if (resultType == 5) {
                updateFunding(38000);
            } else if (resultType == 6) {
                updateFunding(41000);
            } else if (resultType == 8) {
                updateFunding(44000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 0) {
                updateFunding(26000);
            } else if (resultType == 1) {
                updateFunding(32000);
            } else if (resultType == 2) {
                updateFunding(34000);
            } else if (resultType == 3) {
                updateFunding(34000);
            } else if (resultType == 4) {
                updateFunding(35000);
            } else if (resultType == 5) {
                updateFunding(38000);
            } else if (resultType == 6) {
                updateFunding(41000);
            } else if (resultType == 8) {
                updateFunding(44000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheShl(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(35000);
            } else if (resultType == 2) {
                updateFunding(35000);
            } else if (resultType == 3) {
                updateFunding(35000);
            } else if (resultType == 4) {
                updateFunding(35000);
            } else if (resultType == 5) {
                updateFunding(38000);
            } else if (resultType == 6) {
                updateFunding(41000);
            } else if (resultType == 8) {
                updateFunding(44000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(116000);
            } else if (resultType == 2) {
                updateFunding(133000);
            } else if (resultType == 3) {
                updateFunding(153000);
            } else if (resultType == 4) {
                updateFunding(183000);
            } else if (resultType == 5) {
                updateFunding(227000);
            } else if (resultType == 6) {
                updateFunding(282000);
            } else if (resultType == 8) {
                updateFunding(350000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheShr(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(35000);
            } else if (resultType == 2) {
                updateFunding(35000);
            } else if (resultType == 3) {
                updateFunding(35000);
            } else if (resultType == 4) {
                updateFunding(35000);
            } else if (resultType == 5) {
                updateFunding(38000);
            } else if (resultType == 6) {
                updateFunding(41000);
            } else if (resultType == 8) {
                updateFunding(44000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(116000);
            } else if (resultType == 2) {
                updateFunding(133000);
            } else if (resultType == 3) {
                updateFunding(153000);
            } else if (resultType == 4) {
                updateFunding(183000);
            } else if (resultType == 5) {
                updateFunding(227000);
            } else if (resultType == 6) {
                updateFunding(282000);
            } else if (resultType == 8) {
                updateFunding(350000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheRotl(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(35000);
            } else if (resultType == 2) {
                updateFunding(35000);
            } else if (resultType == 3) {
                updateFunding(35000);
            } else if (resultType == 4) {
                updateFunding(35000);
            } else if (resultType == 5) {
                updateFunding(38000);
            } else if (resultType == 6) {
                updateFunding(41000);
            } else if (resultType == 8) {
                updateFunding(44000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(116000);
            } else if (resultType == 2) {
                updateFunding(133000);
            } else if (resultType == 3) {
                updateFunding(153000);
            } else if (resultType == 4) {
                updateFunding(183000);
            } else if (resultType == 5) {
                updateFunding(227000);
            } else if (resultType == 6) {
                updateFunding(282000);
            } else if (resultType == 8) {
                updateFunding(350000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheRotr(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(35000);
            } else if (resultType == 2) {
                updateFunding(35000);
            } else if (resultType == 3) {
                updateFunding(35000);
            } else if (resultType == 4) {
                updateFunding(35000);
            } else if (resultType == 5) {
                updateFunding(38000);
            } else if (resultType == 6) {
                updateFunding(41000);
            } else if (resultType == 8) {
                updateFunding(44000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(116000);
            } else if (resultType == 2) {
                updateFunding(133000);
            } else if (resultType == 3) {
                updateFunding(153000);
            } else if (resultType == 4) {
                updateFunding(183000);
            } else if (resultType == 5) {
                updateFunding(227000);
            } else if (resultType == 6) {
                updateFunding(282000);
            } else if (resultType == 8) {
                updateFunding(350000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheEq(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 0) {
                updateFunding(49000);
            } else if (resultType == 1) {
                updateFunding(51000);
            } else if (resultType == 2) {
                updateFunding(53000);
            } else if (resultType == 3) {
                updateFunding(54000);
            } else if (resultType == 4) {
                updateFunding(82000);
            } else if (resultType == 5) {
                updateFunding(86000);
            } else if (resultType == 6) {
                updateFunding(88000);
            } else if (resultType == 7) {
                updateFunding(90000);
            } else if (resultType == 8) {
                updateFunding(100000);
            } else if (resultType == 9) {
                updateFunding(150000);
            } else if (resultType == 10) {
                updateFunding(200000);
            } else if (resultType == 11) {
                updateFunding(300000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 0) {
                updateFunding(49000);
            } else if (resultType == 1) {
                updateFunding(51000);
            } else if (resultType == 2) {
                updateFunding(53000);
            } else if (resultType == 3) {
                updateFunding(54000);
            } else if (resultType == 4) {
                updateFunding(82000);
            } else if (resultType == 5) {
                updateFunding(86000);
            } else if (resultType == 6) {
                updateFunding(88000);
            } else if (resultType == 7) {
                updateFunding(90000);
            } else if (resultType == 8) {
                updateFunding(100000);
            } else if (resultType == 9) {
                updateFunding(150000);
            } else if (resultType == 10) {
                updateFunding(200000);
            } else if (resultType == 11) {
                updateFunding(300000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheNe(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 0) {
                updateFunding(49000);
            } else if (resultType == 1) {
                updateFunding(51000);
            } else if (resultType == 2) {
                updateFunding(53000);
            } else if (resultType == 3) {
                updateFunding(54000);
            } else if (resultType == 4) {
                updateFunding(82000);
            } else if (resultType == 5) {
                updateFunding(86000);
            } else if (resultType == 6) {
                updateFunding(88000);
            } else if (resultType == 7) {
                updateFunding(90000);
            } else if (resultType == 8) {
                updateFunding(100000);
            } else if (resultType == 9) {
                updateFunding(150000);
            } else if (resultType == 10) {
                updateFunding(200000);
            } else if (resultType == 11) {
                updateFunding(300000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 0) {
                updateFunding(49000);
            } else if (resultType == 1) {
                updateFunding(51000);
            } else if (resultType == 2) {
                updateFunding(53000);
            } else if (resultType == 3) {
                updateFunding(54000);
            } else if (resultType == 4) {
                updateFunding(82000);
            } else if (resultType == 5) {
                updateFunding(86000);
            } else if (resultType == 6) {
                updateFunding(88000);
            } else if (resultType == 7) {
                updateFunding(90000);
            } else if (resultType == 8) {
                updateFunding(100000);
            } else if (resultType == 9) {
                updateFunding(150000);
            } else if (resultType == 10) {
                updateFunding(200000);
            } else if (resultType == 11) {
                updateFunding(300000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheGe(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(70000);
            } else if (resultType == 2) {
                updateFunding(82000);
            } else if (resultType == 3) {
                updateFunding(105000);
            } else if (resultType == 4) {
                updateFunding(128000);
            } else if (resultType == 5) {
                updateFunding(156000);
            } else if (resultType == 6) {
                updateFunding(190000);
            } else if (resultType == 8) {
                updateFunding(231000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(70000);
            } else if (resultType == 2) {
                updateFunding(82000);
            } else if (resultType == 3) {
                updateFunding(105000);
            } else if (resultType == 4) {
                updateFunding(128000);
            } else if (resultType == 5) {
                updateFunding(156000);
            } else if (resultType == 6) {
                updateFunding(190000);
            } else if (resultType == 8) {
                updateFunding(231000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheGt(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(70000);
            } else if (resultType == 2) {
                updateFunding(82000);
            } else if (resultType == 3) {
                updateFunding(105000);
            } else if (resultType == 4) {
                updateFunding(128000);
            } else if (resultType == 5) {
                updateFunding(156000);
            } else if (resultType == 6) {
                updateFunding(190000);
            } else if (resultType == 8) {
                updateFunding(231000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(70000);
            } else if (resultType == 2) {
                updateFunding(82000);
            } else if (resultType == 3) {
                updateFunding(105000);
            } else if (resultType == 4) {
                updateFunding(128000);
            } else if (resultType == 5) {
                updateFunding(156000);
            } else if (resultType == 6) {
                updateFunding(190000);
            } else if (resultType == 8) {
                updateFunding(231000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheLe(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(70000);
            } else if (resultType == 2) {
                updateFunding(82000);
            } else if (resultType == 3) {
                updateFunding(105000);
            } else if (resultType == 4) {
                updateFunding(128000);
            } else if (resultType == 5) {
                updateFunding(156000);
            } else if (resultType == 6) {
                updateFunding(190000);
            } else if (resultType == 8) {
                updateFunding(231000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(70000);
            } else if (resultType == 2) {
                updateFunding(82000);
            } else if (resultType == 3) {
                updateFunding(105000);
            } else if (resultType == 4) {
                updateFunding(128000);
            } else if (resultType == 5) {
                updateFunding(156000);
            } else if (resultType == 6) {
                updateFunding(190000);
            } else if (resultType == 8) {
                updateFunding(231000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheLt(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(70000);
            } else if (resultType == 2) {
                updateFunding(82000);
            } else if (resultType == 3) {
                updateFunding(105000);
            } else if (resultType == 4) {
                updateFunding(128000);
            } else if (resultType == 5) {
                updateFunding(156000);
            } else if (resultType == 6) {
                updateFunding(190000);
            } else if (resultType == 8) {
                updateFunding(231000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(70000);
            } else if (resultType == 2) {
                updateFunding(82000);
            } else if (resultType == 3) {
                updateFunding(105000);
            } else if (resultType == 4) {
                updateFunding(128000);
            } else if (resultType == 5) {
                updateFunding(156000);
            } else if (resultType == 6) {
                updateFunding(190000);
            } else if (resultType == 8) {
                updateFunding(231000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheMin(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(121000);
            } else if (resultType == 2) {
                updateFunding(128000);
            } else if (resultType == 3) {
                updateFunding(150000);
            } else if (resultType == 4) {
                updateFunding(164000);
            } else if (resultType == 5) {
                updateFunding(192000);
            } else if (resultType == 6) {
                updateFunding(225000);
            } else if (resultType == 8) {
                updateFunding(264000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(121000);
            } else if (resultType == 2) {
                updateFunding(128000);
            } else if (resultType == 3) {
                updateFunding(153000);
            } else if (resultType == 4) {
                updateFunding(183000);
            } else if (resultType == 5) {
                updateFunding(210000);
            } else if (resultType == 6) {
                updateFunding(241000);
            } else if (resultType == 8) {
                updateFunding(277000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheMax(uint8 resultType, bytes1 scalarByte) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (scalarByte == 0x01) {
            if (resultType == 1) {
                updateFunding(121000);
            } else if (resultType == 2) {
                updateFunding(128000);
            } else if (resultType == 3) {
                updateFunding(150000);
            } else if (resultType == 4) {
                updateFunding(164000);
            } else if (resultType == 5) {
                updateFunding(192000);
            } else if (resultType == 6) {
                updateFunding(225000);
            } else if (resultType == 8) {
                updateFunding(264000);
            } else {
                revert UnsupportedOperation();
            }
        } else {
            if (resultType == 1) {
                updateFunding(121000);
            } else if (resultType == 2) {
                updateFunding(128000);
            } else if (resultType == 3) {
                updateFunding(153000);
            } else if (resultType == 4) {
                updateFunding(183000);
            } else if (resultType == 5) {
                updateFunding(210000);
            } else if (resultType == 6) {
                updateFunding(241000);
            } else if (resultType == 8) {
                updateFunding(277000);
            } else {
                revert UnsupportedOperation();
            }
        }
        checkFHEGasBlockLimit();
    }

    function payForFheNeg(uint8 resultType) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (resultType == 1) {
            updateFunding(60000);
        } else if (resultType == 2) {
            updateFunding(95000);
        } else if (resultType == 3) {
            updateFunding(131000);
        } else if (resultType == 4) {
            updateFunding(160000);
        } else if (resultType == 5) {
            updateFunding(199000);
        } else if (resultType == 6) {
            updateFunding(248000);
        } else if (resultType == 8) {
            updateFunding(309000);
        } else {
            revert UnsupportedOperation();
        }
        checkFHEGasBlockLimit();
    }

    function payForFheNot(uint8 resultType) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (resultType == 0) {
            updateFunding(30000);
        } else if (resultType == 1) {
            updateFunding(33000);
        } else if (resultType == 2) {
            updateFunding(34000);
        } else if (resultType == 3) {
            updateFunding(35000);
        } else if (resultType == 4) {
            updateFunding(36000);
        } else if (resultType == 5) {
            updateFunding(37000);
        } else if (resultType == 6) {
            updateFunding(38000);
        } else if (resultType == 8) {
            updateFunding(39000);
        } else {
            revert UnsupportedOperation();
        }
        checkFHEGasBlockLimit();
    }

    function payForCast(uint8 resultType) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (resultType == 0) {
            updateFunding(200);
        } else if (resultType == 1) {
            updateFunding(200);
        } else if (resultType == 2) {
            updateFunding(200);
        } else if (resultType == 3) {
            updateFunding(200);
        } else if (resultType == 4) {
            updateFunding(200);
        } else if (resultType == 5) {
            updateFunding(200);
        } else if (resultType == 6) {
            updateFunding(200);
        } else if (resultType == 8) {
            updateFunding(200);
        } else {
            revert UnsupportedOperation();
        }
        checkFHEGasBlockLimit();
    }

    function payForTrivialEncrypt(uint8 resultType) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (resultType == 0) {
            updateFunding(100);
        } else if (resultType == 1) {
            updateFunding(100);
        } else if (resultType == 2) {
            updateFunding(100);
        } else if (resultType == 3) {
            updateFunding(200);
        } else if (resultType == 4) {
            updateFunding(300);
        } else if (resultType == 5) {
            updateFunding(600);
        } else if (resultType == 6) {
            updateFunding(650);
        } else if (resultType == 7) {
            updateFunding(700);
        } else if (resultType == 8) {
            updateFunding(800);
        } else if (resultType == 9) {
            updateFunding(1600);
        } else if (resultType == 10) {
            updateFunding(3200);
        } else if (resultType == 11) {
            updateFunding(6400);
        } else {
            revert UnsupportedOperation();
        }
        checkFHEGasBlockLimit();
    }

    function payForIfThenElse(uint8 resultType) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (resultType == 0) {
            updateFunding(43000);
        } else if (resultType == 1) {
            updateFunding(45000);
        } else if (resultType == 2) {
            updateFunding(47000);
        } else if (resultType == 3) {
            updateFunding(47000);
        } else if (resultType == 4) {
            updateFunding(50000);
        } else if (resultType == 5) {
            updateFunding(53000);
        } else if (resultType == 6) {
            updateFunding(70000);
        } else if (resultType == 7) {
            updateFunding(80000);
        } else if (resultType == 8) {
            updateFunding(90000);
        } else if (resultType == 9) {
            updateFunding(150000);
        } else if (resultType == 10) {
            updateFunding(200000);
        } else if (resultType == 11) {
            updateFunding(300000);
        } else {
            revert UnsupportedOperation();
        }
        checkFHEGasBlockLimit();
    }

    function payForFheRand(uint8 resultType) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (resultType == 0) {
            updateFunding(100000);
        } else if (resultType == 1) {
            updateFunding(100000);
        } else if (resultType == 2) {
            updateFunding(100000);
        } else if (resultType == 3) {
            updateFunding(100000);
        } else if (resultType == 4) {
            updateFunding(100000);
        } else if (resultType == 5) {
            updateFunding(100000);
        } else if (resultType == 6) {
            updateFunding(100000);
        } else if (resultType == 8) {
            updateFunding(100000);
        } else if (resultType == 9) {
            updateFunding(200000);
        } else if (resultType == 10) {
            updateFunding(300000);
        } else if (resultType == 11) {
            updateFunding(400000);
        } else {
            revert UnsupportedOperation();
        }
        checkFHEGasBlockLimit();
    }

    function payForFheRandBounded(uint8 resultType) external virtual {
        if (msg.sender != tfheExecutorAddress) revert CallerMustBeTFHEExecutorContract();
        checkIfNewBlock();
        if (resultType == 1) {
            updateFunding(100000);
        } else if (resultType == 2) {
            updateFunding(100000);
        } else if (resultType == 3) {
            updateFunding(100000);
        } else if (resultType == 4) {
            updateFunding(100000);
        } else if (resultType == 5) {
            updateFunding(100000);
        } else if (resultType == 6) {
            updateFunding(100000);
        } else if (resultType == 8) {
            updateFunding(100000);
        } else {
            revert UnsupportedOperation();
        }
        checkFHEGasBlockLimit();
    }

    /// @notice Getter for the name and version of the contract
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
}
