/* SPDX-License-Identifier: MIT */
pragma solidity ^0.8.20;

contract Regression1 {
    error IndexOutOfBound();
    struct Metadata {
        uint256 created;
        uint256 lastUpdated;
        uint256 versionId;
    }
    struct Service {
        bytes32 id;
        string serviceType;
        string serviceEndpoint;
    }
    mapping(address id => Metadata) public metadata;
    mapping(address id => Service[] service) private _services;

    function addServices(Service[] calldata services) public {
        for (uint256 i = 0; i < services.length; i++) {
            _services[msg.sender].push(services[i]);
        }
        metadata[msg.sender].created = block.timestamp;
        metadata[msg.sender].lastUpdated = block.timestamp;
        metadata[msg.sender].versionId = 1;
    }

    function removeService(uint256 serviceIndex) public {
        if (serviceIndex >= _services[msg.sender].length) revert IndexOutOfBound();
        for (uint256 i = serviceIndex; i < _services[msg.sender].length - 1; i++) {
            _services[msg.sender][i] = _services[msg.sender][i + 1];
        }
        _services[msg.sender].pop();
        metadata[msg.sender].lastUpdated = block.timestamp;
        metadata[msg.sender].versionId++;
    }

    function getServices(address id) public view returns (Service[] memory) {
        return _services[id];
    }
}
