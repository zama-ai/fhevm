// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";
import {ProtocolConfigReplicaTestBase} from "./ProtocolConfigReplicaTestBase.sol";
import {ProtocolConfig} from "@fhevm-host-contracts/contracts/ProtocolConfig.sol";
import {ProtocolConfigReplica} from "@fhevm-host-contracts/contracts/ProtocolConfigReplica.sol";
import {IProtocolConfigReplica, KmsEpochSubmission} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigReplica.sol";
import {IProtocolConfigBase} from "@fhevm-host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {EmptyUUPSProxy} from "@fhevm-host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {UUPSUpgradeableEmptyProxy} from "@fhevm-host-contracts/contracts/shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "@fhevm-host-contracts/contracts/shared/ACLOwnable.sol";
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

    function _expectReinitializeV4Reverts(uint64 chainId, address canonical) internal {
        ProtocolConfig pc = _setupCanonical();
        vm.store(protocolConfigAdd, INITIALIZABLE_STORAGE, bytes32(uint256(4)));
        address replicaImpl = address(new ProtocolConfigReplica());

        vm.prank(owner);
        vm.expectRevert(
            abi.encodeWithSelector(IProtocolConfigReplica.InvalidCanonicalDomain.selector, chainId, canonical)
        );
        pc.upgradeToAndCall(replicaImpl, abi.encodeCall(ProtocolConfigReplica.reinitializeV4, (chainId, canonical)));
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

        protocolConfigReplica = _upgradeToReplica(pc);

        assertEq(uint256(vm.load(protocolConfigAdd, INITIALIZABLE_STORAGE)), 5);
        _assertCanonicalDomain();
        assertEq(protocolConfigReplica.getVersion(), "ProtocolConfigReplica v0.4.0");
        (uint256 newContextId, uint256 newEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(newContextId, contextId);
        assertEq(newEpochId, epochId);
        assertEq(protocolConfigReplica.getKmsSignersForContext(contextId), signers);
        assertEq(protocolConfigReplica.getPublicDecryptionThresholdForContext(contextId), publicDecryption);
        assertEq(protocolConfigReplica.getUserDecryptionThresholdForContext(contextId), userDecryption);
        assertEq(protocolConfigReplica.getKmsGenThresholdForContext(contextId), kmsGen);
        assertEq(protocolConfigReplica.getMpcThresholdForContext(contextId), mpc);

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV4(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);

        vm.prank(owner);
        protocolConfigReplica.mirrorKmsEpoch(contextId, epochId + 1);
        (newContextId, newEpochId) = protocolConfigReplica.getCurrentKmsContextAndEpoch();
        assertEq(newContextId, contextId);
        assertEq(newEpochId, epochId + 1);
    }

    function test_revertReinitializeV4OnFreshReplica() public {
        _setupReplica();

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        protocolConfigReplica.reinitializeV4(CANONICAL_CHAIN_ID, CANONICAL_ADDRESS);
    }

    function test_revertUpgradeNotOwner() public {
        _setupReplica();
        address newImpl = address(new ProtocolConfigReplica());

        vm.prank(address(0x999));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, address(0x999)));
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
        bytes32 structHash = keccak256(
            abi.encode(EIP712_EPOCH_ACTIVATION_TYPE_HASH, CTX1, EPOCH1, EPOCH1 + 1, MATERIAL, keccak256(EXTRA))
        );
        bytes32 digest = MessageHashUtils.toTypedDataHash(
            _computeProtocolConfigDomainSeparator(chainId, verifyingContract),
            structHash
        );
        KmsEpochSubmission memory epoch = _epochSubmission(CTX1, EPOCH1, EPOCH1 + 1, SEED0, N);
        epoch.signatures = _sign(digest, SEED0, N);

        _expectNotInCommittee(_epochDigest(CTX1, EPOCH1, epoch), epoch.signatures[0], CTX1);
        protocolConfigReplica.submitKmsKeyResharing(epoch);
    }

    function test_revertSubmitSignedForLocalDomain() public {
        _expectRejectsDomain(block.chainid, protocolConfigAdd);
    }

    /// @dev A canonical at the same address on another chain, such as a testnet.
    function test_revertSubmitSignedForOtherCanonicalChain() public {
        _expectRejectsDomain(CANONICAL_CHAIN_ID + 1, CANONICAL_ADDRESS);
    }
}
