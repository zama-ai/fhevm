// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {KMSVerifier} from "../../../../contracts/KMSVerifier.sol";

contract KMSSignerGovernanceHandler is Test {
    uint8 internal constant SIGNER_DOMAIN = 32;

    KMSVerifier internal immutable kmsVerifier;
    address internal immutable owner;

    bool public unexpectedContextMutation;
    bool public privilegeViolation;

    constructor(KMSVerifier _kmsVerifier, address _owner) {
        kmsVerifier = _kmsVerifier;
        owner = _owner;
    }

    function defineNewContext(uint8 signerCountSeed, uint8 thresholdSeed, uint8 baseSeed, bool forceDuplicate) external {
        uint256 signerCount = bound(uint256(signerCountSeed), 1, 5);
        uint256 threshold = bound(uint256(thresholdSeed), 0, 6);
        address[] memory signers = _buildSignerSet(signerCount, baseSeed, forceDuplicate);

        vm.prank(msg.sender);
        (bool ok, ) = address(kmsVerifier).call(abi.encodeCall(KMSVerifier.defineNewContext, (signers, threshold)));
        if (ok && msg.sender != owner) {
            privilegeViolation = true;
        }
    }

    function setThreshold(uint8 thresholdSeed) external {
        uint256 threshold = bound(uint256(thresholdSeed), 0, 6);

        vm.prank(msg.sender);
        (bool ok, ) = address(kmsVerifier).call(abi.encodeCall(KMSVerifier.setThreshold, (threshold)));
        if (ok && msg.sender != owner) {
            privilegeViolation = true;
        }
    }

    function verifyDecryptionEIP712KMSSignatures(
        uint8 handlesLenSeed,
        uint8 decryptedLenSeed,
        uint8 proofLenSeed,
        uint256 dataSeed
    ) external {
        bytes32 beforeHash = _contextHash();
        bytes32[] memory handles = _buildHandles(bound(uint256(handlesLenSeed), 0, 4), dataSeed);
        bytes memory decryptedResult = _buildBytes(bound(uint256(decryptedLenSeed), 0, 32), dataSeed ^ 0xAAA1);
        bytes memory decryptionProof = _buildBytes(bound(uint256(proofLenSeed), 0, 64), dataSeed ^ 0xBBB2);

        vm.prank(msg.sender);
        try kmsVerifier.verifyDecryptionEIP712KMSSignatures(handles, decryptedResult, decryptionProof) {} catch {}

        if (beforeHash != _contextHash()) {
            unexpectedContextMutation = true;
        }
    }

    function _contextHash() internal view returns (bytes32) {
        return keccak256(abi.encode(kmsVerifier.getThreshold(), kmsVerifier.getKmsSigners()));
    }

    function candidateDomainSize() external pure returns (uint256) {
        return SIGNER_DOMAIN;
    }

    function candidateAt(uint256 seed) external pure returns (address) {
        return _seededAddress(seed, 40);
    }

    function _buildSignerSet(
        uint256 signerCount,
        uint8 baseSeed,
        bool forceDuplicate
    ) internal pure returns (address[] memory signers) {
        signers = new address[](signerCount);
        for (uint256 i = 0; i < signerCount; i++) {
            signers[i] = _seededAddress(uint256(baseSeed) + i, 40);
        }

        if (forceDuplicate && signerCount > 1) {
            signers[signerCount - 1] = signers[0];
        }
    }

    function _buildHandles(uint256 len, uint256 seed) internal pure returns (bytes32[] memory handles) {
        handles = new bytes32[](len);
        for (uint256 i = 0; i < len; i++) {
            handles[i] = keccak256(abi.encode(seed, i));
        }
    }

    function _buildBytes(uint256 len, uint256 seed) internal pure returns (bytes memory data) {
        data = new bytes(len);
        for (uint256 i = 0; i < len; i++) {
            data[i] = bytes1(uint8(uint256(keccak256(abi.encode(seed, i)))));
        }
    }

    function _seededAddress(uint256 seed, uint160 offset) internal pure returns (address) {
        return address(offset + uint160(seed % SIGNER_DOMAIN) + 1);
    }
}
