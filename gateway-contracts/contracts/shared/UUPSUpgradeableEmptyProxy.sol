// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { UUPSUpgradeable } from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

/**
 * @title UUPSUpgradeableEmptyProxy
 * @dev Abstract base contract for upgradeable contracts that are intended to be deployed behind
 * empty proxies. This contract provides a modifier that ensures functions can only be called
 * during the first initialization phase (i.e., when initialized version is 1), enforcing
 * correct deployment from an empty proxy using the UUPSUpgradeable pattern.
 *
 * Inheriting contracts should use the `onlyFromEmptyProxy` modifier to protect initialization logic
 * that must not run on upgrades or reinitializations.
 */
abstract contract UUPSUpgradeableEmptyProxy is UUPSUpgradeable {
    error NotInitializingFromEmptyProxy();

    /**
     * @dev IMPORTANT: This modifier MUST appear before `reinitializer(N)` in the modifier list.
     *      It relies on `_getInitializedVersion()` still returning 1 at the time of the check,
     *      which is only true because Solidity evaluates modifiers left-to-right and
     *      `reinitializer(N)` has not yet bumped the version. Reversing the order would
     *      silently disable this guard.
     */
    modifier onlyFromEmptyProxy() {
        if (_getInitializedVersion() != 1) {
            revert NotInitializingFromEmptyProxy();
        }
        _;
    }
}
