// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import {KMS_CONTEXT_COUNTER_BASE} from "../../contracts/shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "../../contracts/shared/UUPSUpgradeableEmptyProxy.sol";

/// @dev Migration-only legacy KMSVerifier mock used by `test/tasks/migration.ts`.
contract KMSVerifier is UUPSUpgradeableEmptyProxy {
    uint64 private constant REINITIALIZER_VERSION = 3;

    struct Context {
        address[] signers;
        uint256 threshold;
    }

    uint256 private _currentKmsContextId;
    mapping(uint256 => Context) private _contexts;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        address,
        uint64,
        address[] calldata initialSigners,
        uint256 initialThreshold
    ) public onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        _currentKmsContextId = KMS_CONTEXT_COUNTER_BASE;
        _defineContext(initialSigners, initialThreshold);
    }

    function defineNewContext(address[] memory newSignersSet, uint256 newThreshold) public {
        _defineContext(newSignersSet, newThreshold);
    }

    function getKmsSigners() public view returns (address[] memory) {
        return _contexts[_currentKmsContextId].signers;
    }

    function getThreshold() public view returns (uint256) {
        return _contexts[_currentKmsContextId].threshold;
    }

    function getCurrentKmsContextId() public view returns (uint256) {
        return _currentKmsContextId;
    }

    function _defineContext(address[] memory signers, uint256 threshold) internal {
        uint256 newContextId = _currentKmsContextId + 1;
        _currentKmsContextId = newContextId;

        Context storage context = _contexts[newContextId];
        context.threshold = threshold;
        for (uint256 i = 0; i < signers.length; ++i) {
            context.signers.push(signers[i]);
        }
    }

    function _authorizeUpgrade(address) internal override {}
}
