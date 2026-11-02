// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @dev Mock of ProtocolConfig for the CiphertextRoots tests, which deploy it at ProtocolConfig's
 *      address. It implements the coprocessor context reads CiphertextRoots makes. Unlike the real
 *      contract, it reverts on any read of a context that is invalid, or that is neither the current
 *      one nor the one before it, so the tests prove CiphertextRoots never makes such a read. Tests
 *      define contexts with consecutive IDs.
 */
contract ProtocolConfigMock {
    error InvalidCoprocessorContext(uint256 coprocessorContextId);
    error NeitherCurrentNorPreviousContext(uint256 coprocessorContextId);

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

    /// @dev CiphertextRoots must only read the current context and the one before it, while valid.
    function _requireValid(uint256 contextId) private view {
        if (contextId != _currentId && contextId + 1 != _currentId) {
            revert NeitherCurrentNorPreviousContext(contextId);
        }
        if (!_valid[contextId]) {
            revert InvalidCoprocessorContext(contextId);
        }
    }
}
