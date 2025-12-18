// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity 0.8.27;

import {FHE, ebool, euint64 } from "@fhevm/solidity/lib/FHE.sol";
import {EthereumConfigUpgradeable} from "../fhevm/EthereumConfigUpgradeable.sol";
import {ERC7984Upgradeable} from "./ERC7984Upgradeable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import {AccessControlDefaultAdminRulesUpgradeable} from "@openzeppelin/contracts-upgradeable/access/extensions/AccessControlDefaultAdminRulesUpgradeable.sol";
import {IERC20Errors} from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import {SanctionsList} from "../admin/SanctionsList.sol";
import {AdminProvider} from "../admin/AdminProvider.sol";
import {IDeploymentCoordinator} from "../interfaces/IDeploymentCoordinator.sol";

interface FHEErrors {
    /**
     * @notice Returned when an address is sanctioned.
     */
    error SanctionedAddress(address account);

    /**
     * @notice Returned when rate parameter is invalid.
     */
    error InvalidRate();

    /**
     * @notice Returned when wrapper has already been set.
     */
    error WrapperAlreadySet();

    /**
     * @notice FHE.isSenderAllowed(encryptedValue) returned false
     */
    error SenderNotAllowed();
}

/// @custom:security-contact contact@zaiffer.org
contract RegulatedERC7984Upgradeable is
    EthereumConfigUpgradeable,
    AccessControlDefaultAdminRulesUpgradeable,
    ERC7984Upgradeable,
    UUPSUpgradeable,
    IERC20Errors,
    FHEErrors
{
    bytes32 public constant WRAPPER_ROLE = keccak256("WRAPPER_ROLE");
    bytes32 public constant WRAPPER_SETTER_ROLE = keccak256("WRAPPER_SETTER_ROLE");
    bytes32 public constant UPGRADER_ROLE = keccak256("UPGRADER_ROLE");

    /// @custom:storage-location erc7201:zaiffer.storage.RegulatedERC7984
    struct RegulatedERC7984Storage {
        uint256 _rate;
        uint256 _nextTxId;
        address _underlying;
        uint8 _decimals;
        bool _wrapperSet;
        IDeploymentCoordinator _deploymentCoordinator;
        address _tokenRegulator;
    }

    event TransferInfo(address indexed from, address indexed to, euint64 encryptedAmount, uint256 txId);
    event MintInfo(address indexed to, uint64 amount, uint256 txId);
    event BurnInfo(address indexed from, euint64 amount, uint256 txId);
    event TokenRegulatorUpdated(address indexed oldRegulator, address indexed newRegulator);

    // keccak256(abi.encode(uint256(keccak256("zaiffer.storage.RegulatedERC7984")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant RegulatedERC7984StorageLocation =
        0x825f52e2e3e00f0b167e52d69cfe92f4ec69b657f7c0aaefb00a7607a3ca0100;

    function _getRegulatedERC7984Storage() internal pure returns (RegulatedERC7984Storage storage $) {
        assembly {
            $.slot := RegulatedERC7984StorageLocation
        }
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(
        string memory name_,
        string memory symbol_,
        uint8 decimals_,
        address admin_,
        uint256 rate_,
        address underlying_,
        IDeploymentCoordinator deploymentCoordinator_,
        address wrapperSetter_
    ) public initializer {
        __EthereumConfig_init();
        __AccessControlDefaultAdminRules_init(0, admin_); // 0 delay for admin transfer
        __ERC7984_init(name_, symbol_, "");

        _grantRole(WRAPPER_SETTER_ROLE, wrapperSetter_);

        // Validate rate bounds
        // For tokens with decimals <= 6: rate must be 1
        // For tokens with decimals > 6: rate = 10^(decimals - 6)
        // Maximum supported is 30 decimals, so max rate = 10^24
        require(rate_ >= 1 && rate_ <= 10**24, InvalidRate());

        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();

        $._decimals = decimals_;
        $._rate = rate_;
        $._deploymentCoordinator = deploymentCoordinator_;
        $._nextTxId = 1;
        $._underlying = underlying_;

       _setTotalSupply(FHE.asEuint64(0));
    }

    function _incrementNextTxId() internal {
        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();
        $._nextTxId += 1;
    }

    function nextTxId() public view returns (uint256) {
        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();
        return $._nextTxId;
    }

    function underlying() public view returns (address) {
        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();

        return $._underlying;
    }

    function decimals() public view override returns (uint8) {
        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();

        return $._decimals;
    }

    function rate() public view returns (uint256) {
        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();

        return $._rate;
    }

    function deploymentCoordinator() public view virtual returns (IDeploymentCoordinator) {
        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();

        return $._deploymentCoordinator;
    }

    function adminProvider() public view virtual returns (AdminProvider) {
        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();

        return $._deploymentCoordinator.adminProvider();
    }

    function regulator() public view virtual returns (address) {
        return adminProvider().regulator();
    }

    function tokenRegulator() public view virtual returns (address) {
        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();

        return $._tokenRegulator;
    }

    function setTokenRegulator(address newRegulator) public virtual onlyRole(DEFAULT_ADMIN_ROLE) {
        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();
        address oldRegulator = $._tokenRegulator;
        $._tokenRegulator = newRegulator;

        emit TokenRegulatorUpdated(oldRegulator, newRegulator);
    }

    function burn(euint64 amount, address proxyingFor) public virtual onlyRole(WRAPPER_ROLE) returns (euint64) {
        require(FHE.isSenderAllowed(amount), SenderNotAllowed());
        address from = _msgSender();
        uint256 txId = nextTxId();

        euint64 balance = confidentialBalanceOf(from);
        ebool isBurnable = FHE.le(amount, balance);
        euint64 burnAmount = FHE.select(isBurnable, amount, FHE.asEuint64(0));

        euint64 burned = _burn(from, burnAmount);

        FHE.allow(burned, proxyingFor);

        emit BurnInfo(proxyingFor, burned, txId);

        return burned;
    }

    function mint(address to, uint64 amount) public virtual onlyRole(WRAPPER_ROLE) {
        uint256 txId = nextTxId();

        _mint(to, FHE.asEuint64(amount));

        emit MintInfo(to, amount, txId);
    }

    /// @notice Set the wrapper address and grant it WRAPPER_ROLE (can only be called once)
    /// @param wrapper_ Address of the wrapper contract
    function setWrapper(address wrapper_) external onlyRole(WRAPPER_SETTER_ROLE) {
        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();

        if ($._wrapperSet) {
            revert WrapperAlreadySet();
        }

        $._wrapperSet = true;
        _grantRole(WRAPPER_ROLE, wrapper_);
    }

    function _authorizeUpgrade(address) internal override onlyRole(UPGRADER_ROLE) {}

    /**
     * @notice Check if an address is sanctioned.
     * @param account The address to check.
     */
    function _checkSanctions(SanctionsList sanctionsList, address account) internal view {
        if (sanctionsList.isSanctioned(account)) {
            revert SanctionedAddress(account);
        }
    }

    // Overrides

    function _setTotalSupply(euint64 totalSupply) internal virtual override {
        ERC7984Storage storage $ = _getERC7984Storage();
        $._totalSupply = totalSupply;
        FHE.allowThis($._totalSupply);
        FHE.makePubliclyDecryptable($._totalSupply);
    }

    function _update(address from, address to, euint64 amount) internal override returns (euint64 transferred) {
        RegulatedERC7984Storage storage $ = _getRegulatedERC7984Storage();

        transferred = super._update(from, to, amount);

        SanctionsList sanctionsList = adminProvider().sanctionsList();
        address cachedRegulator = regulator();
        address cachedTokenRegulator = $._tokenRegulator;

        // Check sanctions on msg.sender (operator) - prevents sanctioned addresses from operating on behalf of others
        address sender = _msgSender();
        if (sender != from) {
            _checkSanctions(sanctionsList, sender);
        }

        // Grant admin access to transferred amount and updated balances
        if (from != address(0)) {
            _checkSanctions(sanctionsList, from);
            euint64 fromBalance = confidentialBalanceOf(from);
            FHE.allow(fromBalance, cachedRegulator);
            if (cachedTokenRegulator != address(0)) {
                FHE.allow(fromBalance, cachedTokenRegulator);
            }
        }
        if (to != address(0)) {
            _checkSanctions(sanctionsList, to);
            euint64 toBalance = confidentialBalanceOf(to);
            FHE.allow(toBalance, cachedRegulator);
            if (cachedTokenRegulator != address(0)) {
                FHE.allow(toBalance, cachedTokenRegulator);
            }
        }

        FHE.allow(transferred, cachedRegulator);
        if (cachedTokenRegulator != address(0)) {
            FHE.allow(transferred, cachedTokenRegulator);
        }

        emit TransferInfo(from, to, transferred, $._nextTxId);

        _incrementNextTxId();

        return transferred;
    }

    function supportsInterface(bytes4 interfaceId) public view override(AccessControlDefaultAdminRulesUpgradeable, ERC7984Upgradeable) returns (bool) {
        return super.supportsInterface(interfaceId);
    }
}
