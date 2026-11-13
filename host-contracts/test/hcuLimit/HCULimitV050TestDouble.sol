// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {UUPSUpgradeableEmptyProxy} from "../../contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";

/// @dev Reproduces the v0.5.0 storage layout (v0.14.7-0, with the allowlist) and consumed initializer version, for
///      upgrade and rollback tests. Its getters read the fields as v0.5.0 does.
contract HCULimitV050TestDouble is UUPSUpgradeableEmptyProxy, ACLOwnable {
    /// @custom:storage-location erc7201:fhevm.storage.HCULimit
    struct HCULimitStorage {
        uint48 globalHCUCapPerBlock;
        uint48 usedBlockHCU;
        uint48 lastSeenBlockNumber;
        uint48 maxHCUDepthPerTx;
        uint48 maxHCUPerTx;
        mapping(address account => bool isWhitelisted) blockHCUWhitelist;
        mapping(address account => bool allowed) isAllowlisted;
        mapping(address account => uint256 deadline) allowlisters;
        bool isAllowlistActivated;
    }

    function _getHCULimitStorage() private pure returns (HCULimitStorage storage $) {
        assembly {
            $.slot := 0xc13af6c514bff8997f30c90003baa82bd02aad978179d1ce58d85c4319ad6500
        }
    }

    function initializeFromEmptyProxy(
        uint48 globalCap,
        bool allowlistActivated,
        address[] calldata allowlisted,
        address[] calldata whitelisted
    ) external onlyFromEmptyProxy reinitializer(6) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        $.globalHCUCapPerBlock = globalCap;
        $.usedBlockHCU = 12_345;
        $.lastSeenBlockNumber = uint48(block.number);
        $.maxHCUDepthPerTx = 5_000_000;
        $.maxHCUPerTx = 20_000_000;
        $.isAllowlistActivated = allowlistActivated;
        for (uint256 i; i < allowlisted.length; ++i) $.isAllowlisted[allowlisted[i]] = true;
        for (uint256 i; i < whitelisted.length; ++i) $.blockHCUWhitelist[whitelisted[i]] = true;
    }

    function getGlobalHCUCapPerBlock() external view returns (uint48) {
        return _getHCULimitStorage().globalHCUCapPerBlock;
    }

    function getMaxHCUDepthPerTx() external view returns (uint48) {
        return _getHCULimitStorage().maxHCUDepthPerTx;
    }

    function getMaxHCUPerTx() external view returns (uint48) {
        return _getHCULimitStorage().maxHCUPerTx;
    }

    function isAllowlistActivated() external view returns (bool) {
        return _getHCULimitStorage().isAllowlistActivated;
    }

    function isAllowlisted(address account) external view returns (bool) {
        return _getHCULimitStorage().isAllowlisted[account];
    }

    function isBlockHCUWhitelisted(address account) external view returns (bool) {
        return _getHCULimitStorage().blockHCUWhitelist[account];
    }

    function _authorizeUpgrade(address) internal override onlyACLOwner {}
}
