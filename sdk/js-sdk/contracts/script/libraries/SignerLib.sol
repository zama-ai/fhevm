// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {VmSafe} from "forge-std/Vm.sol";
import {console} from "forge-std/console.sol";
import {Signer} from "./structs/SignerStruct.sol";

library SignerLib {
    function boxMessage(string memory message) private pure returns (string memory) {
        return string.concat(
            "\n",
            "================================================================================\n",
            message,
            "\n",
            "================================================================================\n",
            "\n"
        );
    }

    /// Resolves a `Signer` from a family of env vars.
    ///
    /// Resolution order:
    ///   1. If `$<privateKeyEnv>` is set, use it directly as a `uint256` private key.
    ///   2. Otherwise, require `$<mnemonicEnv>` to be set; derive the key from
    ///      the mnemonic at `${<mnemonicEnv>_PATH}${<mnemonicEnv>_INDEX}`.
    ///      Each path component falls back to a default if its env var is unset:
    ///        - `$<mnemonicEnv>_PATH`  → `"m/44'/60'/0'/0/"`
    ///        - `$<mnemonicEnv>_INDEX` → `0`
    ///
    /// Reverts with a multi-line message listing the env vars to set when
    /// neither path can produce a key.
    ///
    /// Example:
    /// ```
    ///   Signer memory deployer = SignerLib.resolvePrivateKeyFromEnv(
    ///       vm,
    ///       "DEPLOYER_PRIVATE_KEY",
    ///       "DEPLOYER_MNEMONIC"
    ///   );
    ///   // Reads (in order, first-match-wins):
    ///   //   $DEPLOYER_PRIVATE_KEY      uint256 private key
    ///   //   $DEPLOYER_MNEMONIC         BIP-39 mnemonic
    ///   //   $DEPLOYER_MNEMONIC_PATH    BIP-32 prefix         (default "m/44'/60'/0'/0/")
    ///   //   $DEPLOYER_MNEMONIC_INDEX   uint32 account index  (default 0)
    /// ```
    function resolvePrivateKeyFromEnv(VmSafe vm, string memory privateKeyEnv, string memory mnemonicEnv)
        internal
        view
        returns (Signer memory d)
    {
        if (vm.envExists(privateKeyEnv)) {
            d.privateKey = vm.envUint(privateKeyEnv);
            d.addr = vm.addr(d.privateKey);
            return d;
        }

        require(
            vm.envExists(mnemonicEnv),
            boxMessage(
                string.concat(
                    "At least one of the following env vars must be set:\n",
                    "  - ",
                    privateKeyEnv,
                    "\n",
                    "  - ",
                    mnemonicEnv
                )
            )
        );
        return _deriveSignerFromMnemonicEnv(vm, mnemonicEnv, _resolveMnemonicIndexFromEnv(vm, mnemonicEnv, 0));
    }

    function canResolvePrivateKeyFromEnv(VmSafe vm, string memory privateKeyEnv, string memory mnemonicEnv)
        internal
        view
        returns (bool)
    {
        return vm.envExists(privateKeyEnv) || vm.envExists(mnemonicEnv);
    }

    function resolvePrivateKeyFromMnemonicEnv(VmSafe vm, string memory mnemonicEnv)
        internal
        view
        returns (Signer memory)
    {
        require(vm.envExists(mnemonicEnv), boxMessage(string.concat("Env var must be set:\n", "  - ", mnemonicEnv)));
        return _deriveSignerFromMnemonicEnv(vm, mnemonicEnv, _resolveMnemonicIndexFromEnv(vm, mnemonicEnv, 0));
    }

    /// Derives a `Signer` from `$<mnemonicEnv>` (+ optional `$<mnemonicEnv>_PATH`,
    /// default `"m/44'/60'/0'/0/"`) at the given `mnemonicIndex`.
    ///
    /// Caller MUST:
    ///   - verify `$<mnemonicEnv>` is set before calling — this helper assumes it;
    ///   - pass the leaf index explicitly. The `$<mnemonicEnv>_INDEX` env var is
    ///     intentionally NOT consulted here (use `_resolveMnemonicIndexFromEnv`
    ///     to read it). This keeps the helper safe to call inside a list loop
    ///     where `mnemonicIndex` is the loop counter.
    function _deriveSignerFromMnemonicEnv(VmSafe vm, string memory mnemonicEnv, uint32 mnemonicIndex)
        private
        view
        returns (Signer memory d)
    {
        string memory mnemonic = vm.envString(mnemonicEnv);
        string memory mnemonicPathEnv = string.concat(mnemonicEnv, "_PATH");
        string memory mnemonicPath = vm.envExists(mnemonicPathEnv) ? vm.envString(mnemonicPathEnv) : "m/44'/60'/0'/0/";

        d.privateKey = vm.deriveKey(mnemonic, mnemonicPath, mnemonicIndex);
        d.addr = vm.addr(d.privateKey);
    }

    /// Reads `$<mnemonicEnv>_INDEX` if set, otherwise returns `defaultIndex`.
    /// Reverts if the env value exceeds `uint32`.
    function _resolveMnemonicIndexFromEnv(VmSafe vm, string memory mnemonicEnv, uint32 defaultIndex)
        private
        view
        returns (uint32)
    {
        string memory mnemonicIndexEnv = string.concat(mnemonicEnv, "_INDEX");
        if (!vm.envExists(mnemonicIndexEnv)) {
            return defaultIndex;
        }
        uint256 raw = vm.envUint(mnemonicIndexEnv);
        require(raw <= type(uint32).max, boxMessage(string.concat(mnemonicIndexEnv, " exceeds uint32")));
        return uint32(raw);
    }

    function resolveListFromEnv(VmSafe vm, string memory numEnv, string memory mnemonicEnv, uint32 defaultStartIndex)
        internal
        view
        returns (Signer[] memory)
    {
        require(vm.envExists(mnemonicEnv), boxMessage(string.concat("Env var must be set:\n", "  - ", mnemonicEnv)));

        uint32 mnemonicStartIndex = _resolveMnemonicIndexFromEnv(vm, mnemonicEnv, defaultStartIndex);

        uint32 num = 1;
        if (vm.envExists(numEnv)) {
            uint256 raw = vm.envUint(numEnv);
            require(raw <= type(uint32).max, boxMessage(string.concat(numEnv, " exceeds uint32")));
            num = uint32(raw);
        }

        Signer[] memory signers = new Signer[](num);
        for (uint32 i = 0; i < num; ++i) {
            signers[i] = _deriveSignerFromMnemonicEnv(vm, mnemonicEnv, i + mnemonicStartIndex);
        }

        return signers;
    }

    function addressesOf(Signer[] memory signers) internal pure returns (address[] memory addrs) {
        uint256 n = signers.length;
        addrs = new address[](n);
        for (uint256 i = 0; i < n; ++i) {
            addrs[i] = signers[i].addr;
        }
    }
}
