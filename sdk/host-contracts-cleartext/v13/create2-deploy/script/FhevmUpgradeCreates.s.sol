// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {console} from "forge-std/Script.sol";
import {FhevmUpgradeBase} from "./FhevmUpgradeBase.s.sol";

/**
 *  Permissionlessly deploys the ten sealed CREATE2 objects required by the upgrade.
 */
contract FhevmUpgradeCreates is FhevmUpgradeBase {
    uint256 private created;
    uint256 private skipped;

    function run() external {
        _loadUpgradeConfig();
        _requireMinBlock();
        require(msg.sender == cfg.deployer, "FhevmUpgradeCreates: sender is not the sealed deployer");
        string memory manifest = _loadManifest();
        Create[] memory creates = _upgradeCreates(manifest);

        _banner("upgrade creates");
        vm.startBroadcast();
        for (uint256 i; i < creates.length; i++) {
            _create(manifest, creates[i]);
        }
        vm.stopBroadcast();
        console.log("  created", created);
        console.log("  already present", skipped);
    }

    function _create(string memory manifest, Create memory item) private {
        address sealedAddress = _readManifestAddress(manifest, item.role);
        address predicted = _predictCreate2Address(item.role, item.initCode);
        require(predicted == sealedAddress, string.concat("FhevmUpgradeCreates: build drift in ", item.role));
        if (_deployed(predicted)) {
            skipped++;
            _logRole(item.role, predicted, true);
            return;
        }
        _factoryCreate2(_salt(item.role), item.initCode);
        require(_deployed(predicted), string.concat("FhevmUpgradeCreates: no code after creating ", item.role));
        created++;
        _logRole(item.role, predicted, false);
    }
}
