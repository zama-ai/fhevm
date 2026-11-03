// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {ProtocolConfigReplicaTestBase} from "./ProtocolConfigReplicaTestBase.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfigReplica, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {PcrValues} from "@fhevm-host-contracts/contracts/shared/Structs.sol";
import {KMS_CONTEXT_COUNTER_BASE, EPOCH_COUNTER_BASE} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {protocolConfigAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

contract ProtocolConfigReplicaTest is ProtocolConfigReplicaTestBase {
    // -----------------------------------------------------------------------
    // Canonical mirror initializer
    // -----------------------------------------------------------------------

    function _expectInitializeFromCanonicalReverts(
        uint256 contextId,
        uint256 epochId,
        uint64 chainId,
        address canonical,
        bytes memory err
    ) internal {
        _deployACL(owner);
        _deployEmptyProtocolConfigProxy();
        address impl = address(new ProtocolConfigReplica());

        vm.prank(owner);
        vm.expectRevert(err);
        EmptyUUPSProxy(protocolConfigAdd).upgradeToAndCall(
            impl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (contextId, epochId, _nodes(SEED0), _thresholds(), chainId, canonical)
            )
        );
    }

    function test_revertInitializeFromCanonicalInvalidEpochId() public {
        _expectInitializeFromCanonicalReverts(
            CTX1,
            EPOCH_COUNTER_BASE,
            CANONICAL_CHAIN_ID,
            CANONICAL_ADDRESS,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsEpoch.selector, EPOCH_COUNTER_BASE)
        );
    }

    function test_revertInitializeFromCanonicalInvalidContextId() public {
        _expectInitializeFromCanonicalReverts(
            KMS_CONTEXT_COUNTER_BASE,
            EPOCH1,
            CANONICAL_CHAIN_ID,
            CANONICAL_ADDRESS,
            abi.encodeWithSelector(IProtocolConfigBase.InvalidKmsContext.selector, KMS_CONTEXT_COUNTER_BASE)
        );
    }

    function test_revertInitializeFromCanonicalZeroChainId() public {
        _expectInitializeFromCanonicalReverts(
            CTX1,
            EPOCH1,
            0,
            CANONICAL_ADDRESS,
            abi.encodeWithSelector(IProtocolConfigReplica.InvalidCanonicalDomain.selector, 0, CANONICAL_ADDRESS)
        );
    }

    function test_revertInitializeFromCanonicalZeroAddress() public {
        _expectInitializeFromCanonicalReverts(
            CTX1,
            EPOCH1,
            CANONICAL_CHAIN_ID,
            address(0),
            abi.encodeWithSelector(
                IProtocolConfigReplica.InvalidCanonicalDomain.selector,
                CANONICAL_CHAIN_ID,
                address(0)
            )
        );
    }

    // -----------------------------------------------------------------------
    // Upgrade from ProtocolConfig
    // -----------------------------------------------------------------------

    address internal replicaImplementation;

    /// @dev Deploys a canonical ProtocolConfig with 2 nodes of committee SEED0, and the replica implementation.
    ///      `_upgradeToReplica` then makes no deployment, so a test can expect a revert on the upgrade.
    function _setupCanonical() internal returns (ProtocolConfig pc) {
        _deployACL(owner);
        (pc, ) = _deployProtocolConfig(owner, _makeKmsNodeParams(2), _defaultThresholds());
        replicaImplementation = address(new ProtocolConfigReplica());
    }

    function _upgradeToReplica(uint64 chainId, address canonical) internal {
        // Existing replica proxies run the v0.15 ProtocolConfig, which is at initialized version 4.
        vm.store(protocolConfigAdd, INITIALIZABLE_STORAGE, bytes32(uint256(4)));
        vm.prank(owner);
        ProtocolConfig(protocolConfigAdd).upgradeToAndCall(
            replicaImplementation,
            abi.encodeCall(ProtocolConfigReplica.reinitializeV4, (chainId, canonical))
        );
        protocolConfigReplica = ProtocolConfigReplica(protocolConfigAdd);
    }

    function _expectReinitializeV4Reverts(uint64 chainId, address canonical) internal {
        _setupCanonical();
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.InvalidCanonicalDomain.selector, chainId, canonical)
        );
        _upgradeToReplica(chainId, canonical);
    }

    function test_revertReinitializeV4ZeroChainId() public {
        _expectReinitializeV4Reverts(0, CANONICAL_ADDRESS);
    }

    function test_revertReinitializeV4ZeroAddress() public {
        _expectReinitializeV4Reverts(CANONICAL_CHAIN_ID, address(0));
    }

    // -----------------------------------------------------------------------
    // Re-initialization protection
    // -----------------------------------------------------------------------

    function test_revertInitializeFromCanonicalAfterInit() public {
        _setupReplica();

        uint256 canonicalContextId = KMS_CONTEXT_COUNTER_BASE + 5;
        uint256 canonicalEpochId = EPOCH_COUNTER_BASE + 5;
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        protocolConfigReplica.initializeFromCanonical(
            canonicalContextId,
            canonicalEpochId,
            _nodes(SEED0),
            _thresholds(),
            CANONICAL_CHAIN_ID,
            CANONICAL_ADDRESS
        );
    }

    function test_revertInitializeFromCanonicalOnUpgradeFromProtocolConfig() public {
        ProtocolConfig pc = _setupCanonical();

        address replicaImpl = address(new ProtocolConfigReplica());
        vm.prank(owner);
        vm.expectRevert(UUPSUpgradeableEmptyProxy.NotInitializingFromEmptyProxy.selector);
        pc.upgradeToAndCall(
            replicaImpl,
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (
                    KMS_CONTEXT_COUNTER_BASE + 1,
                    EPOCH_COUNTER_BASE + 1,
                    _nodes(SEED0),
                    _thresholds(),
                    CANONICAL_CHAIN_ID,
                    CANONICAL_ADDRESS
                )
            )
        );
    }

    function test_reinitializeV4OnUpgradeFromProtocolConfigKeepsStateAndMirrors() public {
        ProtocolConfig pc = _setupCanonical();

        (uint256 contextId, uint256 epochId) = pc.getCurrentKmsContextAndEpoch();
        address[] memory signers = pc.getKmsSignersForContext(contextId);
        uint256 publicDecryption = pc.getPublicDecryptionThresholdForContext(contextId);
        uint256 userDecryption = pc.getUserDecryptionThresholdForContext(contextId);
        uint256 kmsGen = pc.getKmsGenThresholdForContext(contextId);
        uint256 mpc = pc.getMpcThresholdForContext(contextId);

        _upgradeToReplica(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);

        assertEq(uint256(vm.load(protocolConfigAdd, INITIALIZABLE_STORAGE)), 5);
        _assertCanonicalDomain();
        assertEq(protocolConfigReplica.getVersion(), "ProtocolConfigReplica v0.4.0");
        _assertActive(contextId, epochId);
        assertEq(protocolConfigReplica.getKmsSignersForContext(contextId), signers);
        assertEq(protocolConfigReplica.getPublicDecryptionThresholdForContext(contextId), publicDecryption);
        assertEq(protocolConfigReplica.getUserDecryptionThresholdForContext(contextId), userDecryption);
        assertEq(protocolConfigReplica.getKmsGenThresholdForContext(contextId), kmsGen);
        assertEq(protocolConfigReplica.getMpcThresholdForContext(contextId), mpc);

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV4(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpoch(contextId, epochId + 1);
        _assertActive(contextId, epochId + 1);
    }

    /// @dev A pending context leaves the canonical counter above the active ID. The replica reports the active ID.
    function test_reinitializeV4CounterReturnsActiveContextId() public {
        ProtocolConfig pc = _setupCanonical();
        vm.prank(owner);
        pc.defineNewKmsContextAndEpoch(_makeKmsNodeParams(2), _defaultThresholds(), "", new PcrValues[](0));
        assertEq(pc.getCurrentKmsContextIdCounter(), CTX1 + 1);

        _upgradeToReplica(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);

        assertEq(protocolConfigReplica.getCurrentKmsContextIdCounter(), CTX1);

        // The canonical stored the pending 2-node SEED0 committee under `CTX1 + 1`. Mirroring that ID replaces it.
        vm.prank(owner);
        protocolConfigReplica.mirrorKmsContextAndEpoch(CTX1 + 1, EPOCH1 + 1, _nodes(SEED1), _thresholds());
        assertEq(protocolConfigReplica.getCurrentKmsContextIdCounter(), CTX1 + 1);
        _assertActive(CTX1 + 1, EPOCH1 + 1);
        _assertMembers(CTX1 + 1, SEED1);
        _assertNotMembers(CTX1 + 1, SEED0);
    }

    /// @dev The replica verifies signatures against the committee that ProtocolConfig stored before the upgrade.
    function test_reinitializeV4ThenSubmitsKeyResharing() public {
        _setupCanonical();
        _upgradeToReplica(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);

        protocolConfigReplica.submitKmsKeyResharing(_epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, 2));
        _assertActive(CTX1, EPOCH1 + 1);
    }

    function test_revertReinitializeV4OnFreshReplica() public {
        _setupReplica();

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV4(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);
    }

    function test_revertUpgradeNotOwner() public {
        _setupReplica();
        address newImpl = address(new ProtocolConfigReplica());

        _expectNotOwner();
        protocolConfigReplica.upgradeToAndCall(newImpl, "");
    }

    // -----------------------------------------------------------------------
    // EIP-712 domain
    // -----------------------------------------------------------------------

    function _assertCanonicalDomain() internal view {
        (
            ,
            string memory name,
            string memory version,
            uint256 chainId,
            address verifyingContract,
            ,

        ) = protocolConfigReplica.eip712Domain();
        assertEq(name, "ProtocolConfig");
        assertEq(version, "1");
        assertEq(chainId, CANONICAL_CHAIN_ID);
        assertEq(verifyingContract, CANONICAL_ADDRESS);
    }

    function test_eip712DomainIsCanonical() public {
        _setupReplica();
        _assertCanonicalDomain();
    }

    /// @dev Expects a resharing signed under the domain `(chainId, verifyingContract)` to be rejected.
    function _expectRejectsDomain(uint256 chainId, address verifyingContract) internal {
        _setupReplica();
        (signingChainId, signingVerifyingContract) = (chainId, verifyingContract);
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);

        // The replica recovers the signer over the canonical digest.
        (signingChainId, signingVerifyingContract) = (CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);
        _assertResharingRejected(epoch);
    }

    function test_revertSubmitSignedForLocalDomain() public {
        _expectRejectsDomain(block.chainid, protocolConfigAdd);
    }

    /// @dev A canonical at the same address on another chain, such as a testnet.
    function test_revertSubmitSignedForOtherCanonicalChain() public {
        _expectRejectsDomain(CANONICAL_CHAIN_ID + 1, CANONICAL_ADDRESS);
    }
}
