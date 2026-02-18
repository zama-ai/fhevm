// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {InputVerifier} from "../../../../contracts/InputVerifier.sol";
import {FHEVMExecutor} from "../../../../contracts/FHEVMExecutor.sol";

contract InputVerifierSignerGovernanceHandler is Test {
    uint8 internal constant SIGNER_DOMAIN = 32;

    InputVerifier internal immutable inputVerifier;
    address internal immutable owner;

    bool public unexpectedContextMutation;
    bool public privilegeViolation;

    constructor(InputVerifier _inputVerifier, address _owner) {
        inputVerifier = _inputVerifier;
        owner = _owner;
    }

    function defineNewContext(uint8 signerCountSeed, uint8 thresholdSeed, uint8 baseSeed, bool forceDuplicate) external {
        uint256 signerCount = bound(uint256(signerCountSeed), 1, 5);
        uint256 threshold = bound(uint256(thresholdSeed), 0, 6);
        address[] memory signers = _buildSignerSet(signerCount, baseSeed, forceDuplicate);

        vm.prank(msg.sender);
        (bool ok, ) = address(inputVerifier).call(abi.encodeCall(InputVerifier.defineNewContext, (signers, threshold)));
        if (ok && msg.sender != owner) {
            privilegeViolation = true;
        }
    }

    function setThreshold(uint8 thresholdSeed) external {
        uint256 threshold = bound(uint256(thresholdSeed), 0, 6);

        vm.prank(msg.sender);
        (bool ok, ) = address(inputVerifier).call(abi.encodeCall(InputVerifier.setThreshold, (threshold)));
        if (ok && msg.sender != owner) {
            privilegeViolation = true;
        }
    }

    function verifyInput(
        bytes32 inputHandle,
        uint8 proofLenSeed,
        uint8 userSeed,
        uint8 contractSeed,
        uint256 proofSeed
    ) external {
        bytes32 beforeHash = _contextHash();
        FHEVMExecutor.ContextUserInputs memory context = FHEVMExecutor.ContextUserInputs({
            userAddress: _seededAddress(userSeed, 200),
            contractAddress: _seededAddress(contractSeed, 240)
        });
        bytes memory inputProof = _buildProof(bound(uint256(proofLenSeed), 0, 48), proofSeed);

        vm.prank(msg.sender);
        try inputVerifier.verifyInput(context, inputHandle, inputProof) {} catch {}

        if (beforeHash != _contextHash()) {
            unexpectedContextMutation = true;
        }
    }

    function cleanTransientStorage() external {
        bytes32 beforeHash = _contextHash();

        vm.prank(msg.sender);
        inputVerifier.cleanTransientStorage();

        if (beforeHash != _contextHash()) {
            unexpectedContextMutation = true;
        }
    }

    function candidateDomainSize() external pure returns (uint256) {
        return SIGNER_DOMAIN;
    }

    function candidateAt(uint256 seed) external pure returns (address) {
        return _seededAddress(seed, 20);
    }

    function _contextHash() internal view returns (bytes32) {
        return keccak256(abi.encode(inputVerifier.getThreshold(), inputVerifier.getCoprocessorSigners()));
    }

    function _buildSignerSet(
        uint256 signerCount,
        uint8 baseSeed,
        bool forceDuplicate
    ) internal pure returns (address[] memory signers) {
        signers = new address[](signerCount);
        for (uint256 i = 0; i < signerCount; i++) {
            signers[i] = _seededAddress(uint256(baseSeed) + i, 20);
        }

        if (forceDuplicate && signerCount > 1) {
            signers[signerCount - 1] = signers[0];
        }
    }

    function _buildProof(uint256 len, uint256 seed) internal pure returns (bytes memory proof) {
        proof = new bytes(len);
        for (uint256 i = 0; i < len; i++) {
            proof[i] = bytes1(uint8(uint256(keccak256(abi.encode(seed, i)))));
        }
    }

    function _seededAddress(uint256 seed, uint160 offset) internal pure returns (address) {
        return address(offset + uint160(seed % SIGNER_DOMAIN) + 1);
    }
}
