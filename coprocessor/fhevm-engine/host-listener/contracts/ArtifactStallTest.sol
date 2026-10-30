// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "contracts/FHEEvents.sol";
import "./ACLTest.sol";

/// Emits real executor/ACL logs in the same transaction. This fixture exercises
/// ingestion and CPU execution; it deliberately does not implement host ACL checks.
contract ArtifactStallTest is FHEEvents {
    ACLTest private immutable acl;

    constructor(ACLTest acl_) {
        acl = acl_;
    }

    function seed(uint64 value, uint64 nonce) external returns (bytes32 result) {
        result = handle(nonce);
        emit TrivialEncrypt(msg.sender, value, FheType.Uint64, result);
        acl.allow(result, msg.sender);
    }

    function add(bytes32 lhs, uint64 rhs, uint64 nonce) external returns (bytes32 result) {
        result = handle(nonce);
        emit FheAdd(msg.sender, lhs, bytes32(uint256(rhs)), 0x01, result);
        acl.allow(result, msg.sender);
    }

    function handle(uint64 nonce) private view returns (bytes32) {
        // Preserve the production handle metadata: computed marker, chain ID,
        // uint64 type, version zero. The upper bytes keep fixture handles unique.
        uint256 digest = uint256(keccak256(abi.encode(address(this), nonce)));
        uint256 metadata = (uint256(0xff) << 80) | (block.chainid << 16) | (uint256(FheType.Uint64) << 8);
        return bytes32((digest & (type(uint256).max << 88)) | metadata);
    }
}
