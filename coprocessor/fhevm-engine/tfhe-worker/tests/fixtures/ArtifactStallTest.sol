// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEEvents} from "host-contracts/contracts/FHEEvents.sol";
import {ACLEvents} from "host-contracts/contracts/ACLEvents.sol";
import {FheType} from "host-contracts/contracts/shared/FheType.sol";

// Regenerate the checked-in Alloy JSON fixtures from the repository root with
// solc 0.8.26, optimizer enabled (200 runs), producing ABI, bytecode and runtime
// bytecode. Wrap these as {abi, bytecode: {object}, deployedBytecode: {object}}.
// Like host-listener's RawLog fixture, no Solidity compiler is needed in Cargo CI.
contract ArtifactStallAcl is ACLEvents {
    function allow(bytes32 handle, address account) external {
        emit Allowed(msg.sender, account, handle);
    }
}

/// Emits real executor/ACL logs in the same transaction. This fixture exercises
/// ingestion and CPU execution; it deliberately does not implement host ACL checks.
contract ArtifactStallTest is FHEEvents {
    ArtifactStallAcl private immutable acl;

    constructor(ArtifactStallAcl acl_) {
        acl = acl_;
    }

    function seed(uint64 value, uint64 nonce) external returns (bytes32 result) {
        result = handle(nonce);
        emit TrivialEncrypt(msg.sender, value, FheType.Uint64, result);
        acl.allow(result, msg.sender);
    }

    // Exact trivialEncrypt preimage and metadata from FHEVMExecutor. Unlike the
    // nonce fixture, this changes across a same-height reorg with a new timestamp.
    function seedBlock(uint64 value) external returns (bytes32 result) {
        uint256 digest = uint256(keccak256(abi.encodePacked(
            bytes8("FHE_comp"), uint8(24), uint256(value), FheType.Uint64,
            address(acl), block.chainid, blockhash(block.number - 1), block.timestamp
        )));
        uint256 metadata = (uint256(0xff) << 80) | (block.chainid << 16) | (uint256(FheType.Uint64) << 8);
        result = bytes32((digest & (type(uint256).max << 88)) | metadata);
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
