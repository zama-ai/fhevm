// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @dev Test double for the coprocessor context reads CiphertextRoots makes on ProtocolConfig
 *      (RFC 039), deployed at `protocolConfigAdd`. Stricter than ProtocolConfig on purpose: every
 *      per-context read of an invalid context reverts, so a test proves CiphertextRoots never makes
 *      one. Contexts get consecutive IDs, as the previous context is the current ID minus one.
 */
contract CoprocessorContextsMock {
    error InvalidCoprocessorContext(uint256 coprocessorContextId);

    uint256 private _currentId;
    mapping(uint256 contextId => bool valid) private _valid;
    mapping(uint256 contextId => uint256 threshold) private _majorityThreshold;
    mapping(uint256 contextId => mapping(address txSender => bool registered)) private _isTxSender;

    /// @dev Stores a context and makes it current.
    function defineContext(uint256 contextId, address[] calldata txSenders, uint256 majorityThreshold) external {
        _currentId = contextId;
        _valid[contextId] = true;
        _majorityThreshold[contextId] = majorityThreshold;
        for (uint256 i = 0; i < txSenders.length; i++) {
            _isTxSender[contextId][txSenders[i]] = true;
        }
    }

    function destroyContext(uint256 contextId) external {
        _valid[contextId] = false;
    }

    function setCurrentContextId(uint256 contextId) external {
        _currentId = contextId;
    }

    function getCurrentCoprocessorContextId() external view returns (uint256) {
        return _currentId;
    }

    function isValidCoprocessorContext(uint256 contextId) external view returns (bool) {
        return _valid[contextId];
    }

    function getCoprocessorMajorityThresholdForContext(uint256 contextId) external view returns (uint256) {
        _requireValid(contextId);
        return _majorityThreshold[contextId];
    }

    function isCoprocessorTxSenderForContext(uint256 contextId, address txSender) external view returns (bool) {
        _requireValid(contextId);
        return _isTxSender[contextId][txSender];
    }

    function _requireValid(uint256 contextId) private view {
        if (!_valid[contextId]) {
            revert InvalidCoprocessorContext(contextId);
        }
    }
}
