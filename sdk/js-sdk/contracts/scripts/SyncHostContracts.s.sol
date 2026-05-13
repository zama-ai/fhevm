// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Script, console} from "forge-std/Script.sol";
import {Vm} from "forge-std/Vm.sol";

/// @notice Syncs `<root>/host-contracts/contracts/` into `sdk/js-sdk/contracts/src/host-contracts/`.
///
/// Behaviour:
///   - Destination missing          → recursive copy.
///   - Destination exists, matches  → no-op.
///   - Destination exists, differs  → revert with the offending relative path.
///
/// Run with:
///   forge script script/SyncHostContracts.s.sol
///
/// Requires this entry in foundry.toml:
///   fs_permissions = [
///     { access = "read",  path = "../../.." },   // already present
///     { access = "write", path = "src" },        // ADD THIS
///   ]
contract SyncHostContracts is Script {
    string constant SRC_ROOT = "../../../host-contracts/contracts";
    string constant DST_ROOT = "src/host-contracts";

    function run() external {
        if (!vm.exists(DST_ROOT)) {
            console.log("Destination missing, copying tree...");
            _copyTree();
            console.log("Copy complete.");
            return;
        }

        console.log("Destination exists, verifying contents...");
        _assertIdentical();
        console.log("Contents identical, nothing to do.");
    }

    function _copyTree() internal {
        Vm.DirEntry[] memory entries = vm.readDir(SRC_ROOT, type(uint64).max);
        vm.createDir(DST_ROOT, true);
        for (uint256 i = 0; i < entries.length; i++) {
            Vm.DirEntry memory e = entries[i];
            string memory rel = _stripPrefix(e.path, SRC_ROOT);
            string memory dst = string.concat(DST_ROOT, "/", rel);
            if (e.isDir) {
                vm.createDir(dst, true);
            } else {
                vm.copyFile(e.path, dst);
            }
        }
    }

    function _assertIdentical() internal view {
        // 1. Every source file must exist at the destination with matching bytes.
        Vm.DirEntry[] memory srcEntries = vm.readDir(SRC_ROOT, type(uint64).max);
        for (uint256 i = 0; i < srcEntries.length; i++) {
            Vm.DirEntry memory e = srcEntries[i];
            if (e.isDir) continue;

            string memory rel = _stripPrefix(e.path, SRC_ROOT);
            string memory dst = string.concat(DST_ROOT, "/", rel);

            if (!vm.exists(dst)) {
                revert(
                    string.concat(
                        "Sync drift: '",
                        rel,
                        "' exists in host-contracts/contracts but is missing in sdk/js-sdk/contracts/src/host-contracts. ",
                        "Delete src/host-contracts/ and re-run, or reconcile manually."
                    )
                );
            }

            bytes memory s = vm.readFileBinary(e.path);
            bytes memory d = vm.readFileBinary(dst);
            if (keccak256(s) != keccak256(d)) {
                revert(
                    string.concat(
                        "Sync drift: '",
                        rel,
                        "' differs between host-contracts/contracts and sdk/js-sdk/contracts/src/host-contracts. ",
                        "Delete src/host-contracts/ and re-run, or reconcile manually."
                    )
                );
            }
        }

        // 2. No extra files at the destination.
        Vm.DirEntry[] memory dstEntries = vm.readDir(DST_ROOT, type(uint64).max);
        for (uint256 i = 0; i < dstEntries.length; i++) {
            Vm.DirEntry memory e = dstEntries[i];
            if (e.isDir) continue;

            string memory rel = _stripPrefix(e.path, DST_ROOT);
            string memory src = string.concat(SRC_ROOT, "/", rel);

            if (!vm.exists(src)) {
                revert(
                    string.concat(
                        "Sync drift: '",
                        rel,
                        "' exists in sdk/js-sdk/contracts/src/host-contracts but is missing in host-contracts/contracts. ",
                        "Delete src/host-contracts/ and re-run, or reconcile manually."
                    )
                );
            }
        }
    }

    function _stripPrefix(string memory path, string memory prefix) internal pure returns (string memory) {
        bytes memory p = bytes(path);
        bytes memory pre = bytes(prefix);
        require(p.length > pre.length, "path shorter than prefix");
        for (uint256 i = 0; i < pre.length; i++) {
            require(p[i] == pre[i], "prefix mismatch");
        }
        uint256 start = pre.length;
        if (p[start] == bytes1("/")) {
            start++;
        }
        bytes memory out = new bytes(p.length - start);
        for (uint256 i = 0; i < out.length; i++) {
            out[i] = p[start + i];
        }
        return string(out);
    }
}
