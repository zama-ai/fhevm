// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @notice Contract for managing services and metadata
contract Regression1 {
    error IndexOutOfBound();

    /// @dev Struct to store metadata information
    struct Metadata {
        uint256 created;
        uint256 lastUpdated;
        uint256 versionId;
    }

    /// @dev Struct to represent a service
    struct Service {
        bytes32 id;
        string serviceType;
        string serviceEndpoint;
    }

    /// @notice Mapping to store metadata for each address
    mapping(address id => Metadata) public metadata;

    /// @dev Mapping to store services for each address
    mapping(address id => Service[] service) private _services;

    /// @notice Function to add services
    function addServices(Service[] calldata services) public {
        for (uint256 i = 0; i < services.length; i++) {
            _services[msg.sender].push(services[i]);
        }
        metadata[msg.sender].created = block.timestamp;
        metadata[msg.sender].lastUpdated = block.timestamp;
        metadata[msg.sender].versionId = 1;
    }

    /// @notice Function to remove a service
    function removeService(uint256 serviceIndex) public {
        if (serviceIndex >= _services[msg.sender].length) revert IndexOutOfBound();
        for (uint256 i = serviceIndex; i < _services[msg.sender].length - 1; i++) {
            _services[msg.sender][i] = _services[msg.sender][i + 1];
        }
        _services[msg.sender].pop();
        metadata[msg.sender].lastUpdated = block.timestamp;
        metadata[msg.sender].versionId++;
    }

    /// @notice Function to get services for a given address
    /// @param id The address to get services for
    /// @return An array of Service structs
    function getServices(address id) public view returns (Service[] memory) {
        return _services[id];
    }
}
