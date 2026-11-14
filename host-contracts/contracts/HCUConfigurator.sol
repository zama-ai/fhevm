// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {aclAdd, hcuLimitAdd} from "../addresses/FHEVMHostAddresses.sol";
import {ACL} from "./ACL.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {HCULimit} from "./HCULimit.sol";
import {ALLOWLIST_QUOTA, BLOCK_WHITELIST_QUOTA, DEFAULT_QUOTA} from "./shared/Constants.sol";

/**
 * @title HCUConfigurator
 * @notice Changes the settings of HCULimit: who may change each setting, the checks on its value, and its event.
 * @dev HCULimit accepts its settings from this contract only and checks nothing, so the rules below hold only while
 *      this contract is correct. The quota admins live here, since no FHE operation reads them.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract HCUConfigurator is UUPSUpgradeableEmptyProxy, ACLOwnable {
    /// @notice Returned when the sender is neither the ACL owner nor a pauser.
    error NotPauser(address sender);

    /// @notice Returned when the sender is not the admin of the quota.
    error NotQuotaAdmin(uint16 quotaId, address sender);

    /// @notice Returned when a quota admin moves a contract into or out of a paused quota.
    error QuotaMembershipPaused(uint16 quotaId);

    /// @notice Returned when the contract is not in the quota it is moved out of.
    error ContractNotInQuota(address account, uint16 quotaId);

    /// @notice Returned when a quota admin adds a contract while the default quota's maximum is above 0: the
    ///         contract may then have more room in the default quota than in the admin's quota.
    error DefaultQuotaNotStopped();

    /// @notice Returned when:
    ///         - only one of a quota's maximum and decay is 0;
    ///         - a contract is assigned or granted to a quota whose maximum is 0, other than the default quota;
    ///         - an admin is set for the default quota or a quota that replaced the allowlist.
    error InvalidQuota(uint16 quotaId);

    /// @notice Returned when only one of the cross-transaction depth limit and its decay is zero.
    error InvalidCrossTxDepthLimit();

    /// @notice Returned when a stored-depth limit is set on quota 1 or 2, outside maxHCUDepthPerTx to maxDepth, or, for
    ///         a quota other than the default, below the default quota's limit or while the default quota has none.
    ///         Also returned, for the default quota, when maxHCUDepthPerTx is set above the default quota's limit.
    error InvalidMaxStoredDepth(uint16 quotaId);

    /// @notice Returned if a nonzero cross-transaction depth limit is below maxHCUDepthPerTx.
    error CrossTxDepthBelowMaxDepthPerTx();

    /// @notice Emitted when the cross-transaction depth limit is updated.
    event CrossTxDepthLimitSet(uint48 maxDepth, uint48 decayPerBlock);

    /// @notice Emitted when a quota's stored-depth limit is set.
    event QuotaMaxStoredDepthSet(uint16 indexed quotaId, uint48 maxStoredDepth, address indexed setBy);

    /// @notice Emitted when a quota's maximum and decay are set.
    event QuotaSet(uint16 indexed quotaId, uint48 maxHCU, uint48 decayPerBlock, address indexed setBy);

    /// @notice Emitted when the owner names a quota's admin; address(0) removes it.
    event QuotaAdminSet(uint16 indexed quotaId, address indexed admin);

    /// @notice Emitted when a quota is paused or unpaused.
    event QuotaPauseSet(uint16 indexed quotaId, bool paused, address indexed setBy);

    /// @notice Emitted when a contract is assigned to a quota, by the owner or the quota's admin.
    event ContractQuotaAssigned(uint16 indexed quotaId, address indexed account, address indexed setBy);

    /// @notice Emitted when the per-transaction HCU depth limit is updated.
    /// @param maxHCUDepthPerTx New depth limit.
    event MaxHCUDepthPerTxSet(uint48 maxHCUDepthPerTx);

    /// @notice Emitted when the per-transaction HCU limit is updated.
    /// @param maxHCUPerTx New transaction limit.
    event MaxHCUPerTxSet(uint48 maxHCUPerTx);

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "HCUConfigurator";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 1;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 2;

    /// @notice The contract whose settings this contract changes.
    HCULimit private constant HCU_LIMIT = HCULimit(hcuLimitAdd);

    /// @custom:storage-location erc7201:fhevm.storage.HCUConfigurator
    struct HCUConfiguratorStorage {
        /// @notice Address that moves contracts between the default quota and each quota; address(0) when none.
        mapping(uint16 quotaId => address admin) quotaAdmins;
    }

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.HCUConfigurator")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant HCU_CONFIGURATOR_STORAGE_LOCATION =
        0xb138efae62c1e8fe7282cda9cd237607a81b190b364e4655cc3a97b4b08ddc00;

    function _getHCUConfiguratorStorage() internal pure returns (HCUConfiguratorStorage storage $) {
        assembly {
            $.slot := HCU_CONFIGURATOR_STORAGE_LOCATION
        }
    }

    /// @dev Pausers are the ACL's pausers (PauserSet).
    modifier onlyACLOwnerOrPauser() {
        if (msg.sender != ACL(aclAdd).owner() && !ACL(aclAdd).isPauser(msg.sender)) revert NotPauser(msg.sender);
        _;
    }

    /// @dev A paused quota's membership is frozen for its admin; the owner can still move contracts with assignQuota.
    modifier onlyQuotaAdmin(uint16 quotaId) {
        if (msg.sender != _getHCUConfiguratorStorage().quotaAdmins[quotaId]) revert NotQuotaAdmin(quotaId, msg.sender);
        if (HCU_LIMIT.isQuotaPaused(quotaId)) revert QuotaMembershipPaused(quotaId);
        _;
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract. It starts with no quota admin.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy() public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @notice Sets a quota's maximum usage and how much usage decays per block, like setCrossTxDepthLimit.
     *         A maximum of type(uint48).max leaves the quota unmetered; (0, 0) stops it.
     */
    function setQuota(uint16 quotaId, uint48 maxHCU, uint48 decayPerBlock) external onlyACLOwner {
        // Both 0 stops the quota. A maximum without decay would never free up, and a decay without maximum means nothing.
        if ((maxHCU == 0) != (decayPerBlock == 0)) revert InvalidQuota(quotaId);
        HCU_LIMIT.setQuota(quotaId, maxHCU, decayPerBlock);
        emit QuotaSet(quotaId, maxHCU, decayPerBlock, msg.sender);
    }

    /**
     * @notice Names the address that moves contracts between the default quota and this quota.
     *         address(0) removes the admin. The default quota and the quotas that replaced the allowlist take none.
     */
    function setQuotaAdmin(uint16 quotaId, address admin) external onlyACLOwner {
        if (quotaId <= BLOCK_WHITELIST_QUOTA) revert InvalidQuota(quotaId);
        _getHCUConfiguratorStorage().quotaAdmins[quotaId] = admin;
        emit QuotaAdminSet(quotaId, admin);
    }

    /**
     * @notice Moves a contract to the default quota or to a quota whose maximum is above 0, from any quota,
     *         including a paused one.
     */
    function assignQuota(uint16 quotaId, address account) external onlyACLOwner {
        _assignQuota(quotaId, account);
    }

    /**
     * @notice Moves a contract from the default quota into the admin's quota.
     * @dev Only while the default quota is stopped: a contract then gains room by joining any quota, so an admin
     *      cannot starve a contract it does not own. If the owner reopens the default quota, granted contracts stay in
     *      the admin's quota until the admin revokes them or the owner moves them. An allowlisted contract nobody
     *      assigned computes in the allowlist quota, so it is not in the default quota and only the owner moves it.
     */
    function grantQuota(uint16 quotaId, address account) external onlyQuotaAdmin(quotaId) {
        if (_maxHCU(DEFAULT_QUOTA) != 0) revert DefaultQuotaNotStopped();
        if (HCU_LIMIT.getContractQuota(account) != DEFAULT_QUOTA) revert ContractNotInQuota(account, DEFAULT_QUOTA);
        _assignQuota(quotaId, account);
    }

    /**
     * @notice Moves a contract from the admin's quota back to the default quota.
     */
    function revokeQuota(uint16 quotaId, address account) external onlyQuotaAdmin(quotaId) {
        // getContractQuota differs from the assigned quota only by routing to quotas 1 and 2, which take no admin.
        if (HCU_LIMIT.getContractQuota(account) != quotaId) revert ContractNotInQuota(account, quotaId);
        _assignQuota(DEFAULT_QUOTA, account);
    }

    /**
     * @notice Stops every FHE operation from the quota's contracts and stops its admin from moving contracts,
     *         keeping its maximum, decay and contracts. Only the owner unpauses.
     */
    function pauseQuota(uint16 quotaId) external onlyACLOwnerOrPauser {
        _setQuotaPaused(quotaId, true);
    }

    /**
     * @notice Lifts a quota's pause.
     */
    function unpauseQuota(uint16 quotaId) external onlyACLOwner {
        _setQuotaPaused(quotaId, false);
    }

    /**
     * @notice Sets the per-transaction HCU depth limit.
     * @param maxHCUDepthPerTx New depth limit.
     * @dev At most a nonzero maxCrossTxDepth, and at most the default quota's stored-depth limit, so a transaction on
     *      fresh inputs can grant its results. It is not compared with maxHCUPerTx, which counts work while depth
     *      counts latency.
     */
    function setMaxHCUDepthPerTx(uint48 maxHCUDepthPerTx) external onlyACLOwner {
        (uint48 maxCrossTxDepth, ) = HCU_LIMIT.getCrossTxDepthLimit();
        if (maxCrossTxDepth != 0 && maxCrossTxDepth < maxHCUDepthPerTx) revert CrossTxDepthBelowMaxDepthPerTx();
        uint48 defaultMaxStoredDepth = HCU_LIMIT.getQuotaMaxStoredDepth(DEFAULT_QUOTA);
        if (defaultMaxStoredDepth != 0 && defaultMaxStoredDepth < maxHCUDepthPerTx) {
            revert InvalidMaxStoredDepth(DEFAULT_QUOTA);
        }
        HCU_LIMIT.setTxLimits(HCU_LIMIT.getMaxHCUPerTx(), maxHCUDepthPerTx);
        emit MaxHCUDepthPerTxSet(maxHCUDepthPerTx);
    }

    /**
     * @notice Sets the per-transaction HCU limit.
     * @param maxHCUPerTx New transaction limit.
     */
    function setMaxHCUPerTx(uint48 maxHCUPerTx) external onlyACLOwner {
        HCU_LIMIT.setTxLimits(maxHCUPerTx, HCU_LIMIT.getMaxHCUDepthPerTx());
        emit MaxHCUPerTxSet(maxHCUPerTx);
    }

    /**
     * @notice Sets the cross-transaction depth limit and how much carried depth decays per block.
     * @dev (0, 0) disables it. Decay must be nonzero when enabled, or a chain that reaches the limit stays
     *      blocked forever. The limit must be at least maxHCUDepthPerTx, so a transaction whose inputs carry no
     *      depth is never refused. Permissions granted while disabled carry no depth. A new decay rate also
     *      applies to depths already stored.
     */
    function setCrossTxDepthLimit(uint48 maxDepth, uint48 decayPerBlock) external onlyACLOwner {
        if ((maxDepth == 0) != (decayPerBlock == 0)) revert InvalidCrossTxDepthLimit();
        if (maxDepth != 0) {
            if (maxDepth < HCU_LIMIT.getMaxHCUDepthPerTx()) revert CrossTxDepthBelowMaxDepthPerTx();
            // Every operation on a stored handle reads ACL.getStoredDepth: refuse to enable before ACL has it.
            ACL(aclAdd).getStoredDepth(bytes32(0), address(0));
        }
        HCU_LIMIT.setCrossTxDepthLimit(maxDepth, decayPerBlock);
        emit CrossTxDepthLimitSet(maxDepth, decayPerBlock);
    }

    /**
     * @notice Sets the carried depth above which a quota's contracts cannot grant a handle, with allow or allowTransient.
     *         0 leaves only the cross-transaction depth limit. Otherwise the limit is from maxHCUDepthPerTx, so a
     *         transaction on fresh inputs always fits, to maxDepth.
     * @dev The default quota keeps the strictest limit: a quota admin can move its contract to the default quota, grant
     *      a deep handle there and move it back. So the default quota's limit is set first, and another quota's limit
     *      is refused below it; another quota's 0 counts as maxDepth. Raising the default quota's limit later is not
     *      checked against the others. Quotas 1 and 2 take no limit: they only hold allowlisted contracts until the
     *      owner assigns them.
     */
    function setQuotaMaxStoredDepth(uint16 quotaId, uint48 maxStoredDepth) external onlyACLOwner {
        if (quotaId == ALLOWLIST_QUOTA || quotaId == BLOCK_WHITELIST_QUOTA) revert InvalidMaxStoredDepth(quotaId);
        if (maxStoredDepth != 0) {
            (uint48 maxDepth, ) = HCU_LIMIT.getCrossTxDepthLimit();
            if (maxStoredDepth < HCU_LIMIT.getMaxHCUDepthPerTx() || maxStoredDepth > maxDepth) {
                revert InvalidMaxStoredDepth(quotaId);
            }
            if (quotaId != DEFAULT_QUOTA) {
                uint48 defaultLimit = HCU_LIMIT.getQuotaMaxStoredDepth(DEFAULT_QUOTA);
                if (defaultLimit == 0 || maxStoredDepth < defaultLimit) revert InvalidMaxStoredDepth(quotaId);
            }
        }
        HCU_LIMIT.setQuotaMaxStoredDepth(quotaId, maxStoredDepth);
        emit QuotaMaxStoredDepthSet(quotaId, maxStoredDepth, msg.sender);
    }

    /**
     * @notice Returns a quota's admin; address(0) when it has none.
     */
    function getQuotaAdmin(uint16 quotaId) external view returns (address) {
        return _getHCUConfiguratorStorage().quotaAdmins[quotaId];
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

    function _maxHCU(uint16 quotaId) private view returns (uint48 maxHCU) {
        (maxHCU, , ) = HCU_LIMIT.getQuota(quotaId);
    }

    /// @dev Every move goes through here, so a contract never lands in a stopped quota other than the default quota.
    function _assignQuota(uint16 quotaId, address account) private {
        if (quotaId != DEFAULT_QUOTA && _maxHCU(quotaId) == 0) revert InvalidQuota(quotaId);
        HCU_LIMIT.assignQuota(quotaId, account);
        emit ContractQuotaAssigned(quotaId, account, msg.sender);
    }

    function _setQuotaPaused(uint16 quotaId, bool paused) private {
        HCU_LIMIT.setQuotaPaused(quotaId, paused);
        emit QuotaPauseSet(quotaId, paused, msg.sender);
    }

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
