// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {UUPSUpgradeableEmptyProxy} from "../../contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";

/// @dev Reproduces the v0.4.0 storage layout and consumed initializer version for upgrade continuity tests.
contract HCULimitV040TestDouble is UUPSUpgradeableEmptyProxy, ACLOwnable {
    /// @custom:storage-location erc7201:fhevm.storage.HCULimit
    struct HCULimitStorage {
        uint48 globalHCUCapPerBlock;
        uint48 usedBlockHCU;
        uint48 lastSeenBlockNumber;
        uint48 maxHCUDepthPerTx;
        uint48 maxHCUPerTx;
        mapping(address account => bool isWhitelisted) blockHCUWhitelist;
    }

    function _getHCULimitStorage() private pure returns (HCULimitStorage storage $) {
        assembly {
            $.slot := 0xc13af6c514bff8997f30c90003baa82bd02aad978179d1ce58d85c4319ad6500
        }
    }

    function initializeFromEmptyProxy(address whitelisted) external onlyFromEmptyProxy reinitializer(5) {
        HCULimitStorage storage $ = _getHCULimitStorage();
        $.globalHCUCapPerBlock = 30_000_000;
        $.usedBlockHCU = 12_345;
        $.lastSeenBlockNumber = uint48(block.number);
        $.maxHCUDepthPerTx = 5_000_000;
        $.maxHCUPerTx = 20_000_000;
        $.blockHCUWhitelist[whitelisted] = true;
    }

    function _authorizeUpgrade(address) internal override onlyACLOwner {}
}
