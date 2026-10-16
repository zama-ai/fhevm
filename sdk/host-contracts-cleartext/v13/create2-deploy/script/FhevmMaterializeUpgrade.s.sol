// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {console} from "forge-std/Script.sol";
import {FhevmUpgradeBase} from "./FhevmUpgradeBase.s.sol";
import {IACLOwner, IOwnable2Step, IVersioned} from "./Interfaces.sol";

/**
 *  Applies the two materializations and five reinitializations in one admin-owned ACLOwner call.
 *
 *  The `require`s here are the load-bearing subset of `FhevmPreMaterializeCheck`, repeated because forge
 *  simulates this script before it broadcasts: a failed require ends the run before a transaction exists,
 *  whichever orchestrator invoked it and whether or not the gate was run first.
 */
contract FhevmMaterializeUpgrade is FhevmUpgradeBase {
    function run() external {
        _loadUpgradeConfig();
        _requireMinBlock();
        require(msg.sender == cfg.admin, "FhevmMaterializeUpgrade: sender is not the existing admin");
        require(
            IACLOwner(existingAclOwner).owner() == cfg.admin, "FhevmMaterializeUpgrade: admin does not own ACLOwner"
        );
        require(
            IOwnable2Step(existingAcl).owner() == existingAclOwner, "FhevmMaterializeUpgrade: ACLOwner does not own ACL"
        );

        string memory manifest = _loadManifest();
        string[] memory roles = _upgradeProxyRoles();
        uint256 complete;
        for (uint256 i; i < roles.length; i++) {
            address proxy = _readManifestAddress(manifest, roles[i]);
            address target = _readManifestAddress(manifest, _implRole(roles[i]));
            _requireImplementation(roles[i], target, _upgradeImplArtifact(i));
            address current = _implementationOf(proxy);
            if (current == target) {
                complete++;
            } else {
                require(
                    current == _expectedPreviousImplementation(manifest, roles[i]),
                    string.concat("FhevmMaterializeUpgrade: live proxy implementation drift: ", roles[i])
                );
            }
        }
        if (complete == roles.length) {
            console.log("  materialize already complete");
            return;
        }
        require(complete == 0, "FhevmMaterializeUpgrade: partially materialized state is fatal");
        _requireUntouchedProxies(manifest);

        (bool fresh, string memory why) = _migrationMatchesLive();
        require(fresh, string.concat("FhevmMaterializeUpgrade: ", why));

        IACLOwner.Op[] memory ops = _ops(manifest);
        if (vm.envOr("FHEVM_PREPARE_ONLY", false)) {
            string memory path = string.concat(cfg.outDir, "/materialize-calldata.txt");
            vm.writeFile(path, vm.toString(abi.encodeCall(IACLOwner.upgrade, (ops))));
            console.log("  wrote externally-signable ACLOwner.upgrade calldata", path);
            return;
        }
        vm.startBroadcast();
        IACLOwner(existingAclOwner).upgrade(ops);
        vm.stopBroadcast();

        for (uint256 i; i < roles.length; i++) {
            require(
                _implementationOf(_readManifestAddress(manifest, roles[i]))
                    == _readManifestAddress(manifest, _implRole(roles[i])),
                string.concat("FhevmMaterializeUpgrade: slot mismatch after upgrade: ", roles[i])
            );
        }
        console.log("  seven proxies upgraded atomically");
    }

    /// @dev The implementation exists, is this build's bytecode, and identifies itself as the role's contract.
    function _requireImplementation(string memory role, address target, string memory artifact) private view {
        require(target.code.length != 0, string.concat("FhevmMaterializeUpgrade: implementation has no code: ", role));
        require(
            _matchesDeployedCode(target, artifact),
            string.concat("FhevmMaterializeUpgrade: implementation is not this build's bytecode: ", role)
        );
        require(
            keccak256(bytes(IVersioned(target).getVersion())) == keccak256(bytes(_versionFor(role))),
            string.concat("FhevmMaterializeUpgrade: implementation reports the wrong contract: ", role)
        );
    }

    /// @dev The two proxies outside the op list still run what they ran at seal time.
    function _requireUntouchedProxies(string memory manifest) private view {
        string[] memory roles = _untouchedProxyRoles();
        for (uint256 i; i < roles.length; i++) {
            require(
                _implementationOf(_readManifestAddress(manifest, roles[i]))
                    == _sealedPreviousImplementation(manifest, roles[i]),
                string.concat("FhevmMaterializeUpgrade: untouched proxy implementation drift: ", roles[i])
            );
        }
    }
}
