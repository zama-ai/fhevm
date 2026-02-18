// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {InputVerifier} from "../../../../contracts/InputVerifier.sol";
import {inputVerifierAdd} from "../../../../addresses/FHEVMHostAddresses.sol";
import {SignerGovernanceInvariantBase} from "./SignerGovernanceInvariantBase.t.sol";
import {InputVerifierSignerGovernanceHandler} from "./InputVerifierSignerGovernanceHandler.t.sol";

contract InputVerifierSignerGovernanceInvariants is SignerGovernanceInvariantBase {
    address internal constant OWNER = address(0x0A11CE);
    address internal constant ACTOR_1 = address(0x1001);
    address internal constant ACTOR_2 = address(0x1002);
    address internal constant ACTOR_3 = address(0x1003);

    InputVerifier internal inputVerifier;
    InputVerifierSignerGovernanceHandler internal handler;

    function setUp() public {
        _deployACL(OWNER);

        address[] memory initialSigners = new address[](3);
        initialSigners[0] = address(21);
        initialSigners[1] = address(22);
        initialSigners[2] = address(23);
        _deployInputVerifier(OWNER, address(0xBEEF), uint64(block.chainid), initialSigners, 2);

        inputVerifier = InputVerifier(inputVerifierAdd);
        handler = new InputVerifierSignerGovernanceHandler(inputVerifier, OWNER);

        address[] memory senders = new address[](4);
        senders[0] = OWNER;
        senders[1] = ACTOR_1;
        senders[2] = ACTOR_2;
        senders[3] = ACTOR_3;

        bytes4[] memory selectors = new bytes4[](4);
        selectors[0] = InputVerifierSignerGovernanceHandler.defineNewContext.selector;
        selectors[1] = InputVerifierSignerGovernanceHandler.setThreshold.selector;
        selectors[2] = InputVerifierSignerGovernanceHandler.verifyInput.selector;
        selectors[3] = InputVerifierSignerGovernanceHandler.cleanTransientStorage.selector;
        _targetInvariant(address(handler), selectors, senders);
    }

    /// @custom:invariant SG-INPUT-001
    function invariant_ThresholdWithinSignerBounds() public view {
        _assertThresholdWithinSignerBounds();
    }

    /// @custom:invariant SG-INPUT-002
    function invariant_SignerSetUniqueAndConsistent() public view {
        _assertSignerSetUniqueAndConsistent();
    }

    /// @custom:invariant SG-INPUT-003
    function invariant_VerificationAndCleanupDoNotMutateContext() public view {
        assertFalse(handler.unexpectedContextMutation());
    }

    /// @custom:invariant SG-INPUT-004
    function invariant_OnlyOwnerCanMutateSignerGovernance() public view {
        assertFalse(handler.privilegeViolation());
    }

    function _getSigners() internal view override returns (address[] memory) {
        return inputVerifier.getCoprocessorSigners();
    }

    function _getThreshold() internal view override returns (uint256) {
        return inputVerifier.getThreshold();
    }

    function _isSigner(address signer) internal view override returns (bool) {
        return inputVerifier.isSigner(signer);
    }

    function _candidateDomainSize() internal view override returns (uint256) {
        return handler.candidateDomainSize();
    }

    function _candidateAt(uint256 seed) internal view override returns (address) {
        return handler.candidateAt(seed);
    }
}
