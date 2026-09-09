// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FhevmCreate2Base} from "./FhevmCreate2Base.s.sol";
import {UpgradeInitData} from "./UpgradeInitData.sol";
import {IACLOwner, IWiredKMSVerifier} from "./Interfaces.sol";
import {LocalHostVersions} from "../../pkg/forge/src/_internal/LocalHostVersions.sol";
import {ProtocolConfig} from "../../pkg/src/contracts/ProtocolConfig.sol";
import {IProtocolConfig} from "../../pkg/src/contracts/interfaces/IProtocolConfig.sol";
import {KmsNode} from "../../pkg/src/contracts/shared/Structs.sol";

/**
 *  Shared role, artifact, create, version and migration tables for the v12 -> v13 CREATE2 upgrade.
 *
 *  Every script of the upgrade — compute, creates, precheck, materialize, verify — reads its tables from
 *  here, so a check and the operation it checks cannot disagree about what "the upgrade" is.
 */
abstract contract FhevmUpgradeBase is FhevmCreate2Base {
    address internal existingAcl;
    address internal existingExecutor;
    address internal existingKmsVerifier;
    address internal existingInputVerifier;
    address internal existingHcuLimit;
    address internal existingArithmetic;
    address internal existingDb;
    address internal existingPauserSet;
    address internal existingAclOwner;

    function _loadUpgradeConfig() internal {
        _loadConfig();
        existingAcl = vm.envAddress("FHEVM_EXISTING_ACL_ADDRESS");
        existingExecutor = vm.envAddress("FHEVM_EXISTING_FHEVM_EXECUTOR_ADDRESS");
        existingKmsVerifier = vm.envAddress("FHEVM_EXISTING_KMS_VERIFIER_ADDRESS");
        existingInputVerifier = vm.envAddress("FHEVM_EXISTING_INPUT_VERIFIER_ADDRESS");
        existingHcuLimit = vm.envAddress("FHEVM_EXISTING_HCU_LIMIT_ADDRESS");
        existingArithmetic = vm.envAddress("FHEVM_EXISTING_CLEARTEXT_ARITHMETIC_ADDRESS");
        existingDb = vm.envAddress("FHEVM_EXISTING_CLEARTEXT_DB_ADDRESS");
        existingPauserSet = vm.envAddress("FHEVM_EXISTING_PAUSER_SET_ADDRESS");
        existingAclOwner = vm.envAddress("FHEVM_EXISTING_ACL_OWNER");
    }

    // ---------------------------------------------------------------------------------------
    // Roles
    // ---------------------------------------------------------------------------------------

    /**
     * Seven proxies affected by the atomic upgrade, in ACLOwner.Op order.
     */
    function _upgradeProxyRoles() internal pure returns (string[] memory roles) {
        roles = new string[](7);
        roles[0] = R_PROTOCOL_CONFIG;
        roles[1] = R_KMS_GENERATION;
        roles[2] = R_ACL;
        roles[3] = R_FHEVM_EXECUTOR;
        roles[4] = R_HCU_LIMIT;
        roles[5] = R_KMS_VERIFIER;
        roles[6] = R_CLEARTEXT_ARITHMETIC;
    }

    /// @dev The two live proxies the op list must NOT touch: byte-identical across the two generations.
    function _untouchedProxyRoles() internal pure returns (string[] memory roles) {
        roles = new string[](2);
        roles[0] = R_INPUT_VERIFIER;
        roles[1] = R_CLEARTEXT_DB;
    }

    /// @dev The five live proxies the op list re-points; the other two ops target the new proxies.
    function _liveUpgradedProxyRoles() internal pure returns (string[] memory roles) {
        roles = new string[](5);
        roles[0] = R_ACL;
        roles[1] = R_FHEVM_EXECUTOR;
        roles[2] = R_HCU_LIMIT;
        roles[3] = R_KMS_VERIFIER;
        roles[4] = R_CLEARTEXT_ARITHMETIC;
    }

    function _isNewProxyRole(string memory role) internal pure returns (bool) {
        bytes32 key = keccak256(bytes(role));
        return key == keccak256(bytes(R_PROTOCOL_CONFIG)) || key == keccak256(bytes(R_KMS_GENERATION));
    }

    function _upgradeImplArtifact(uint256 i) internal pure returns (string memory) {
        if (i == 0) return "pkg/src/contracts/ProtocolConfig.sol:ProtocolConfig";
        if (i == 1) return "pkg/src/contracts/KMSGeneration.sol:KMSGeneration";
        if (i == 2) return "pkg/src/contracts/ACL.sol:ACL";
        if (i == 3) return "pkg/src/cleartext/CleartextFHEVMExecutor.sol:CleartextFHEVMExecutor";
        if (i == 4) return "pkg/src/contracts/HCULimit.sol:HCULimit";
        if (i == 5) return "pkg/src/cleartext/CleartextKMSVerifier.sol:CleartextKMSVerifier";
        if (i == 6) return "pkg/src/cleartext/CleartextArithmetic.sol:CleartextArithmetic";
        revert("FhevmUpgradeBase: implementation index out of range");
    }

    /**
     * @dev The `getVersion()` string a proxy role must report once it runs this generation's code. Read
     *      from the generated `LocalHostVersions`, never a literal. Called on an IMPLEMENTATION address it
     *      says which contract that implementation is — `getVersion` is pure, so no proxy is needed.
     */
    function _versionFor(string memory role) internal pure returns (string memory) {
        bytes32 key = keccak256(bytes(role));
        if (key == keccak256(bytes(R_ACL))) return LocalHostVersions.ACL;
        if (key == keccak256(bytes(R_FHEVM_EXECUTOR))) return LocalHostVersions.FHEVM_EXECUTOR;
        if (key == keccak256(bytes(R_HCU_LIMIT))) return LocalHostVersions.HCU_LIMIT;
        if (key == keccak256(bytes(R_KMS_VERIFIER))) return LocalHostVersions.KMS_VERIFIER;
        if (key == keccak256(bytes(R_CLEARTEXT_ARITHMETIC))) return LocalHostVersions.CLEARTEXT_ARITHMETIC;
        if (key == keccak256(bytes(R_PROTOCOL_CONFIG))) return LocalHostVersions.PROTOCOL_CONFIG;
        if (key == keccak256(bytes(R_KMS_GENERATION))) return LocalHostVersions.KMS_GENERATION;
        if (key == keccak256(bytes(R_INPUT_VERIFIER))) return LocalHostVersions.INPUT_VERIFIER;
        if (key == keccak256(bytes(R_CLEARTEXT_DB))) return LocalHostVersions.CLEARTEXT_DB;
        if (key == keccak256(bytes(R_PAUSER_SET))) return LocalHostVersions.PAUSER_SET;
        revert("FhevmUpgradeBase: no version for role");
    }

    function _existingAddress(string memory role) internal view returns (address) {
        bytes32 key = keccak256(bytes(role));
        if (key == keccak256(bytes(R_ACL))) return existingAcl;
        if (key == keccak256(bytes(R_FHEVM_EXECUTOR))) return existingExecutor;
        if (key == keccak256(bytes(R_KMS_VERIFIER))) return existingKmsVerifier;
        if (key == keccak256(bytes(R_INPUT_VERIFIER))) return existingInputVerifier;
        if (key == keccak256(bytes(R_HCU_LIMIT))) return existingHcuLimit;
        if (key == keccak256(bytes(R_CLEARTEXT_ARITHMETIC))) return existingArithmetic;
        if (key == keccak256(bytes(R_CLEARTEXT_DB))) return existingDb;
        if (key == keccak256(bytes(R_PAUSER_SET))) return existingPauserSet;
        if (key == keccak256(bytes(R_ACL_OWNER))) return existingAclOwner;
        revert("FhevmUpgradeBase: unknown existing role");
    }

    // ---------------------------------------------------------------------------------------
    // The ten creates
    // ---------------------------------------------------------------------------------------

    /// @dev One create with the artifact it was built from, so a check can ask the artifact what the
    ///      chain should hold. `Create` (role, initCode) is what the deploy path's loop consumes.
    struct UpgradeCreate {
        string role;
        string artifact;
        bytes initCode;
    }

    /**
     * The ten permissionless CREATE2 operations, in dependency order, each with its artifact.
     */
    function _upgradeCreateTable(string memory manifest) internal view returns (UpgradeCreate[] memory t) {
        string[] memory roles = _upgradeProxyRoles();
        address emptyImpl = _readManifestAddress(manifest, R_IMPL_EMPTY_SHARED);
        bytes memory proxyCode = _proxyInitCode(emptyImpl, _sharedProxyInitData());

        t = new UpgradeCreate[](roles.length + 3);
        uint256 n;
        t[n++] = UpgradeCreate(R_IMPL_EMPTY_SHARED, A_EMPTY_SHARED, _initCode(A_EMPTY_SHARED));
        t[n++] = UpgradeCreate(R_PROTOCOL_CONFIG, A_ERC1967_PROXY, proxyCode);
        t[n++] = UpgradeCreate(R_KMS_GENERATION, A_ERC1967_PROXY, proxyCode);
        for (uint256 i; i < roles.length; i++) {
            string memory artifact = _upgradeImplArtifact(i);
            t[n++] = UpgradeCreate(_implRole(roles[i]), artifact, _initCode(artifact));
        }
        require(n == t.length, "FhevmUpgradeBase: create table length mismatch");
    }

    function _upgradeCreates(string memory manifest) internal view returns (Create[] memory creates) {
        UpgradeCreate[] memory t = _upgradeCreateTable(manifest);
        creates = new Create[](t.length);
        for (uint256 i; i < t.length; i++) {
            creates[i] = Create(t[i].role, t[i].initCode);
        }
    }

    // ---------------------------------------------------------------------------------------
    // The pre-upgrade state the seal recorded
    // ---------------------------------------------------------------------------------------

    /// @dev The implementation a live proxy ran when `compute` sealed, from `.preUpgrade.implementation`.
    function _sealedPreviousImplementation(string memory manifest, string memory role) internal pure returns (address) {
        return vm.parseJsonAddress(manifest, string.concat(".preUpgrade.implementation.", role));
    }

    /**
     * @dev What a proxy's implementation slot must hold BEFORE materialize: the fresh shared empty
     *      implementation for the two new proxies, the snapshotted v12 implementation for a live one.
     */
    function _expectedPreviousImplementation(string memory manifest, string memory role)
        internal
        pure
        returns (address)
    {
        if (_isNewProxyRole(role)) return _readManifestAddress(manifest, R_IMPL_EMPTY_SHARED);
        return _sealedPreviousImplementation(manifest, role);
    }

    /**
     * @dev Does the code at `a` equal the artifact's deployed bytecode, allowing only the UUPS `__self`
     *      immutable to differ: 32 zero bytes in the artifact, `a` itself on chain. Anything else — a stale
     *      build, another contract, a patched byte — is a mismatch.
     */
    function _matchesDeployedCode(address a, string memory artifact) internal view returns (bool) {
        bytes memory want = vm.getDeployedCode(artifact);
        bytes memory got = a.code;
        if (want.length == 0 || got.length != want.length) return false;

        bytes32 self = bytes32(uint256(uint160(a)));
        uint256 firstNonZero;
        while (firstNonZero < 32 && self[firstNonZero] == 0) firstNonZero++;

        for (uint256 i; i < want.length; i++) {
            if (got[i] == want[i]) continue;
            // The first differing byte is the address's first non-zero byte; the window starts before it.
            if (i < firstNonZero || i - firstNonZero + 32 > want.length) return false;
            uint256 start = i - firstNonZero;
            for (uint256 j; j < 32; j++) {
                if (want[start + j] != 0 || got[start + j] != self[j]) return false;
            }
            i = start + 31;
        }
        return true;
    }

    // ---------------------------------------------------------------------------------------
    // The seven ops
    // ---------------------------------------------------------------------------------------

    function _ops(string memory manifest) internal view returns (IACLOwner.Op[] memory ops) {
        string[] memory roles = _upgradeProxyRoles();
        ops = new IACLOwner.Op[](roles.length);
        bytes memory protocolConfigInit = _protocolConfigInit();
        for (uint256 i; i < roles.length; i++) {
            ops[i] = IACLOwner.Op({
                proxy: _readManifestAddress(manifest, roles[i]),
                implementation: _readManifestAddress(manifest, _implRole(roles[i])),
                initData: UpgradeInitData.initData(i, protocolConfigInit)
            });
        }
    }

    /// @dev The exact `ACLOwner.upgrade` payload, whether sent by a key here or by a multisig elsewhere.
    function _upgradeCalldata(string memory manifest) internal view returns (bytes memory) {
        return abi.encodeCall(IACLOwner.upgrade, (_ops(manifest)));
    }

    function _protocolConfigInit() internal view returns (bytes memory) {
        return abi.encodeCall(
            ProtocolConfig.initializeFromMigration, (_migrationContextId(), _migrationNodes(), _migrationThresholds())
        );
    }

    // ---------------------------------------------------------------------------------------
    // The sealed KMS migration, from FHEVM_MIGRATION_*
    // ---------------------------------------------------------------------------------------

    function _migrationContextId() internal view returns (uint256) {
        return vm.envUint("FHEVM_MIGRATION_CONTEXT_ID");
    }

    function _migrationNodes() internal view returns (KmsNode[] memory nodes) {
        uint256 count = vm.envUint("FHEVM_MIGRATION_NODE_COUNT");
        require(count != 0, "FhevmUpgradeBase: sealed migration has no KMS nodes");
        nodes = new KmsNode[](count);
        for (uint256 i; i < count; i++) {
            string memory prefix = string.concat("FHEVM_MIGRATION_NODE_", vm.toString(i));
            nodes[i] = KmsNode({
                txSenderAddress: vm.envAddress(string.concat(prefix, "_TX_SENDER")),
                signerAddress: vm.envAddress(string.concat(prefix, "_SIGNER")),
                ipAddress: vm.envString(string.concat(prefix, "_IP")),
                storageUrl: vm.envString(string.concat(prefix, "_STORAGE"))
            });
        }
    }

    function _migrationSigners() internal view returns (address[] memory signers) {
        KmsNode[] memory nodes = _migrationNodes();
        signers = new address[](nodes.length);
        for (uint256 i; i < nodes.length; i++) {
            signers[i] = nodes[i].signerAddress;
        }
    }

    function _migrationThresholds() internal view returns (IProtocolConfig.KmsThresholds memory) {
        return IProtocolConfig.KmsThresholds({
            publicDecryption: vm.envUint("FHEVM_MIGRATION_PUBLIC_DECRYPTION_THRESHOLD"),
            userDecryption: vm.envUint("FHEVM_MIGRATION_USER_DECRYPTION_THRESHOLD"),
            kmsGen: vm.envUint("FHEVM_MIGRATION_KMS_GEN_THRESHOLD"),
            mpc: vm.envUint("FHEVM_MIGRATION_MPC_THRESHOLD")
        });
    }

    /**
     * @dev Is the sealed migration still what the live v12 KMSVerifier holds? `compute` checked this when it
     *      sealed; days may pass before materialize, and a signer rotation in between would make the
     *      migration REPLACE the live set rather than carry it over. Returns why not, for a require or a report.
     */
    function _migrationMatchesLive() internal view returns (bool ok, string memory why) {
        IWiredKMSVerifier live = IWiredKMSVerifier(existingKmsVerifier);
        address[] memory liveSigners = live.getKmsSigners();
        address[] memory sealedSigners = _migrationSigners();
        if (liveSigners.length != sealedSigners.length) {
            return (false, "sealed migration signer count differs from live KMSVerifier");
        }
        for (uint256 i; i < sealedSigners.length; i++) {
            if (liveSigners[i] != sealedSigners[i]) {
                return (false, "sealed migration signer differs from live KMSVerifier");
            }
        }
        if (live.getCurrentKmsContextId() != _migrationContextId()) {
            return (false, "sealed migration context id differs from live KMSVerifier");
        }
        uint256 threshold = live.getThreshold();
        IProtocolConfig.KmsThresholds memory t = _migrationThresholds();
        if (
            threshold == 0 || t.publicDecryption != threshold || t.userDecryption != threshold || t.kmsGen != threshold
                || t.mpc != threshold
        ) {
            return (false, "sealed migration thresholds differ from live KMSVerifier threshold");
        }
        return (true, "");
    }
}
