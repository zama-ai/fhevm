// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KMSVerifier} from "../../../../contracts/KMSVerifier.sol";
import {kmsVerifierAdd} from "../../../../addresses/FHEVMHostAddresses.sol";
import {SignerGovernanceInvariantBase} from "./SignerGovernanceInvariantBase.t.sol";
import {KMSSignerGovernanceHandler} from "./KMSSignerGovernanceHandler.t.sol";

contract KMSSignerGovernanceInvariants is SignerGovernanceInvariantBase {
    address internal constant OWNER = address(0x0B0B);
    address internal constant ACTOR_1 = address(0x1101);
    address internal constant ACTOR_2 = address(0x1102);
    address internal constant ACTOR_3 = address(0x1103);

    KMSVerifier internal kmsVerifier;
    KMSSignerGovernanceHandler internal handler;

    function setUp() public {
        _deployACL(OWNER);

        address[] memory initialSigners = new address[](3);
        initialSigners[0] = address(41);
        initialSigners[1] = address(42);
        initialSigners[2] = address(43);
        _deployKMSVerifier(OWNER, address(0xCAFE), uint64(block.chainid), initialSigners, 2);

        kmsVerifier = KMSVerifier(kmsVerifierAdd);
        handler = new KMSSignerGovernanceHandler(kmsVerifier, OWNER);

        address[] memory senders = new address[](4);
        senders[0] = OWNER;
        senders[1] = ACTOR_1;
        senders[2] = ACTOR_2;
        senders[3] = ACTOR_3;

        bytes4[] memory selectors = new bytes4[](3);
        selectors[0] = KMSSignerGovernanceHandler.defineNewContext.selector;
        selectors[1] = KMSSignerGovernanceHandler.setThreshold.selector;
        selectors[2] = KMSSignerGovernanceHandler.verifyDecryptionEIP712KMSSignatures.selector;
        _targetInvariant(address(handler), selectors, senders);
    }

    /// @custom:invariant SG-KMS-001
    function invariant_ThresholdWithinSignerBounds() public view {
        _assertThresholdWithinSignerBounds();
    }

    /// @custom:invariant SG-KMS-002
    function invariant_SignerSetUniqueAndConsistent() public view {
        _assertSignerSetUniqueAndConsistent();
    }

    /// @custom:invariant SG-KMS-003
    function invariant_VerificationDoesNotMutateContext() public view {
        assertFalse(handler.unexpectedContextMutation());
    }

    /// @custom:invariant SG-KMS-004
    function invariant_OnlyOwnerCanMutateSignerGovernance() public view {
        assertFalse(handler.privilegeViolation());
    }

    function _getSigners() internal view override returns (address[] memory) {
        return kmsVerifier.getKmsSigners();
    }

    function _getThreshold() internal view override returns (uint256) {
        return kmsVerifier.getThreshold();
    }

    function _isSigner(address signer) internal view override returns (bool) {
        return kmsVerifier.isSigner(signer);
    }

    function _candidateDomainSize() internal view override returns (uint256) {
        return handler.candidateDomainSize();
    }

    function _candidateAt(uint256 seed) internal view override returns (address) {
        return handler.candidateAt(seed);
    }
}
