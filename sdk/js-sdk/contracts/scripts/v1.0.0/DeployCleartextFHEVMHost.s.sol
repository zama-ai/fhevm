// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// Reference deploy for src/v1.0.0. Adapted from fhevm-mocks@8b91ae5
// host-contracts-cleartext/v1.0/pkg/forge/script/FhevmDeployScript.s.sol (+ VerifyFhevmDeploy.s.sol checks).
// Bootstrap values come from ./LocalHostBootstrap.sol (copied verbatim from
// host-contracts-cleartext/v1.0/pkg/forge/src/_internal/LocalHostBootstrap.sol).

import {Script, console} from "forge-std/Script.sol";
import {ERC1967Proxy} from "../../src/v1.0.0/erc1967/ERC1967Proxy.sol";
import {EmptyUUPSProxyACL} from "../../src/v1.0.0/host-contracts/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {EmptyUUPSProxy} from "../../src/v1.0.0/host-contracts/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {ACL} from "../../src/v1.0.0/host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "../../src/v1.0.0/host-contracts/contracts/FHEVMExecutor.sol";
import {KMSVerifier} from "../../src/v1.0.0/host-contracts/contracts/KMSVerifier.sol";
import {InputVerifier} from "../../src/v1.0.0/host-contracts/contracts/InputVerifier.sol";
import {HCULimit} from "../../src/v1.0.0/host-contracts/contracts/HCULimit.sol";
import {ProtocolConfig} from "../../src/v1.0.0/host-contracts/contracts/ProtocolConfig.sol";
import {ProtocolConfigReplica} from "../../src/v1.0.0/host-contracts/contracts/ProtocolConfigReplica.sol";
import {KMSGeneration} from "../../src/v1.0.0/host-contracts/contracts/KMSGeneration.sol";
import {PaddedInputter} from "../../src/v1.0.0/host-contracts/contracts/PaddedInputter.sol";
import {PauserSet} from "../../src/v1.0.0/host-contracts/contracts/immutable/PauserSet.sol";
import {EPOCH_COUNTER_BASE, KMS_CONTEXT_COUNTER_BASE} from "../../src/v1.0.0/host-contracts/contracts/shared/Constants.sol";
import {KmsNodeParams, KmsThresholds, PcrValues} from "../../src/v1.0.0/host-contracts/contracts/shared/Structs.sol";
import {CleartextACL} from "../../src/v1.0.0/cleartext/CleartextACL.sol";
import {CleartextFHEVMExecutor} from "../../src/v1.0.0/cleartext/CleartextFHEVMExecutor.sol";
import {CleartextKMSVerifier} from "../../src/v1.0.0/cleartext/CleartextKMSVerifier.sol";
import {CleartextInputVerifier} from "../../src/v1.0.0/cleartext/CleartextInputVerifier.sol";
import {CleartextHCULimit} from "../../src/v1.0.0/cleartext/CleartextHCULimit.sol";
import {CleartextPaddedInputter} from "../../src/v1.0.0/cleartext/CleartextPaddedInputter.sol";
import {CleartextArithmetic} from "../../src/v1.0.0/cleartext/CleartextArithmetic.sol";
import {CleartextDB} from "../../src/v1.0.0/cleartext/CleartextDB.sol";
import {ACLOwner} from "../../src/v1.0.0/upgrade/ACLOwner.sol";
import {LocalHostBootstrap} from "./LocalHostBootstrap.sol";
import {
    aclAdd,
    fhevmExecutorAdd,
    kmsVerifierAdd,
    inputVerifierAdd,
    hcuLimitAdd,
    protocolConfigAdd,
    kmsGenerationAdd,
    paddedInputterAdd,
    protocolConfigReplicaAdd,
    pauserSetAdd,
    cleartextArithmeticAdd,
    cleartextDbAdd
} from "../../src/v1.0.0/host-contracts/addresses/FHEVMHostAddresses.sol";

interface ICleartextMarkers {
    function IS_CLEARTEXT() external view returns (bool);
    function CLEARTEXT_PROTOCOL_VERSION() external view returns (uint256);
}

interface IVersioned {
    function getVersion() external view returns (string memory);
}

interface IEIP712Domain {
    function eip712Domain()
        external
        view
        returns (bytes1, string memory, string memory, uint256, address, bytes32, uint256[] memory);
}

uint256 constant PROXY_COUNT = 11;

library DeployV1 {
    // Must be called inside vm.startBroadcast(deployerKey), deployer at nonce 0 (or at the nonce the
    // addresses.sol was generated for). Nonces +0..+13 are pinned; +14 is ACLOwner (free).
    function deploy(address deployer) internal returns (ACLOwner aclOwner) {
        // +0 EmptyUUPSProxyACL, +1 ACL proxy
        EmptyUUPSProxyACL aclImpl = new EmptyUUPSProxyACL();
        ERC1967Proxy aclProxy =
            new ERC1967Proxy(address(aclImpl), abi.encodeCall(EmptyUUPSProxyACL.initialize, (deployer)));
        require(address(aclProxy) == aclAdd, "ACL proxy address mismatch");
        // +2 shared EmptyUUPSProxy implementation
        EmptyUUPSProxy sharedImpl = new EmptyUUPSProxy();
        _emptyProxy(sharedImpl, fhevmExecutorAdd); // +3
        _emptyProxy(sharedImpl, kmsVerifierAdd); // +4
        _emptyProxy(sharedImpl, inputVerifierAdd); // +5
        _emptyProxy(sharedImpl, hcuLimitAdd); // +6
        _emptyProxy(sharedImpl, protocolConfigAdd); // +7
        _emptyProxy(sharedImpl, kmsGenerationAdd); // +8
        _emptyProxy(sharedImpl, paddedInputterAdd); // +9
        _emptyProxy(sharedImpl, cleartextArithmeticAdd); // +10
        _emptyProxy(sharedImpl, cleartextDbAdd); // +11
        PauserSet ps = new PauserSet(); // +12
        require(address(ps) == pauserSetAdd, "PauserSet address mismatch");
        _emptyProxy(sharedImpl, protocolConfigReplicaAdd); // +13

        // +14 ACLOwner (owner = deployer), then pausers, then ACL ownership -> ACLOwner (two-step)
        aclOwner = new ACLOwner(deployer, aclAdd);
        PauserSet(pauserSetAdd).addPauser(address(aclOwner));
        ACL(aclAdd).transferOwnership(address(aclOwner));
        aclOwner.acceptACLOwnership();

        // implementations (free nonces) + one atomic ACLOwner.upgrade
        ACLOwner.Op[] memory ops = new ACLOwner.Op[](PROXY_COUNT);
        ops[0] = ACLOwner.Op(aclAdd, address(new CleartextACL()), abi.encodeCall(ACL.initializeFromEmptyProxy, ()));
        ops[1] = ACLOwner.Op(
            fhevmExecutorAdd,
            address(new CleartextFHEVMExecutor()),
            abi.encodeCall(FHEVMExecutor.initializeFromEmptyProxy, ())
        );
        ops[2] = ACLOwner.Op(kmsVerifierAdd, address(new CleartextKMSVerifier()), _kmsVerifierInit());
        ops[3] = ACLOwner.Op(inputVerifierAdd, address(new CleartextInputVerifier()), _inputVerifierInit());
        ops[4] = ACLOwner.Op(hcuLimitAdd, address(new CleartextHCULimit()), _hcuLimitInit());
        ops[5] = ACLOwner.Op(protocolConfigAdd, address(new ProtocolConfig()), _protocolConfigInit());
        ops[6] = ACLOwner.Op(
            kmsGenerationAdd, address(new KMSGeneration()), abi.encodeCall(KMSGeneration.initializeFromEmptyProxy, ())
        );
        ops[7] = ACLOwner.Op(paddedInputterAdd, address(new CleartextPaddedInputter()), _paddedInputterInit());
        ops[8] = ACLOwner.Op(
            cleartextArithmeticAdd,
            address(new CleartextArithmetic()),
            abi.encodeCall(CleartextArithmetic.initializeFromEmptyProxy, ())
        );
        ops[9] = ACLOwner.Op(
            cleartextDbAdd,
            address(new CleartextDB()),
            abi.encodeCall(CleartextDB.initializeFromEmptyProxy, (cleartextArithmeticAdd))
        );
        ops[10] = ACLOwner.Op(
            protocolConfigReplicaAdd,
            address(new ProtocolConfigReplica()),
            abi.encodeCall(
                ProtocolConfigReplica.initializeFromCanonical,
                (KMS_CONTEXT_COUNTER_BASE + 1, EPOCH_COUNTER_BASE + 1, _kmsNodes(), _kmsThresholds())
            )
        );
        aclOwner.upgrade(ops);
    }

    function _emptyProxy(EmptyUUPSProxy sharedImpl, address expected) private {
        ERC1967Proxy proxy = new ERC1967Proxy(address(sharedImpl), abi.encodeCall(EmptyUUPSProxy.initialize, ()));
        require(address(proxy) == expected, "proxy address mismatch");
    }

    function _paddedInputterInit() private pure returns (bytes memory) {
        return abi.encodeCall(PaddedInputter.initializeFromEmptyProxy, (LocalHostBootstrap.padPrfInstances()));
    }

    function _kmsVerifierInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            KMSVerifier.initializeFromEmptyProxy,
            (LocalHostBootstrap.DECRYPTION_ADDRESS, LocalHostBootstrap.GATEWAY_CHAIN_ID)
        );
    }

    function _inputVerifierInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            InputVerifier.initializeFromEmptyProxy,
            (
                LocalHostBootstrap.INPUT_VERIFICATION_ADDRESS,
                LocalHostBootstrap.GATEWAY_CHAIN_ID,
                LocalHostBootstrap.coprocessorSigners(),
                LocalHostBootstrap.COPROCESSOR_THRESHOLD
            )
        );
    }

    function _hcuLimitInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            HCULimit.initializeFromEmptyProxy,
            (
                LocalHostBootstrap.HCU_CAP_PER_BLOCK,
                LocalHostBootstrap.MAX_HCU_DEPTH_PER_TX,
                LocalHostBootstrap.MAX_HCU_PER_TX
            )
        );
    }

    function _protocolConfigInit() private pure returns (bytes memory) {
        return abi.encodeCall(
            ProtocolConfig.initializeFromEmptyProxy,
            (_kmsNodes(), _kmsThresholds(), LocalHostBootstrap.KMS_SOFTWARE_VERSION, new PcrValues[](0))
        );
    }

    function _kmsNodes() internal pure returns (KmsNodeParams[] memory nodes) {
        address[] memory signers = LocalHostBootstrap.kmsSigners();
        address[] memory txSenders = LocalHostBootstrap.kmsTxSenders();
        string[] memory ips = LocalHostBootstrap.kmsIpAddresses();
        string[] memory urls = LocalHostBootstrap.kmsStorageUrls();
        string[] memory identities = LocalHostBootstrap.kmsMpcIdentities();
        string[] memory prefixes = LocalHostBootstrap.kmsStoragePrefixes();
        int32[] memory partyIds = LocalHostBootstrap.kmsPartyIds();
        nodes = new KmsNodeParams[](LocalHostBootstrap.KMS_NODE_COUNT);
        for (uint256 i = 0; i < nodes.length; i++) {
            nodes[i] = KmsNodeParams({
                txSenderAddress: txSenders[i],
                signerAddress: signers[i],
                ipAddress: ips[i],
                storageUrl: urls[i],
                partyId: partyIds[i],
                mpcIdentity: identities[i],
                caCert: LocalHostBootstrap.KMS_NODE_CA_CERT,
                storagePrefix: prefixes[i]
            });
        }
    }

    function _kmsThresholds() internal pure returns (KmsThresholds memory) {
        uint256 c = LocalHostBootstrap.KMS_NODE_COUNT;
        return KmsThresholds({publicDecryption: c, userDecryption: c, kmsGen: c, mpc: c});
    }

    function _eq(string memory a, string memory b, string memory what) private pure {
        require(keccak256(bytes(a)) == keccak256(bytes(b)), string.concat("version mismatch: ", what, " = ", a));
    }

    function verify(address expectedAclOwner) internal view {
        // markers
        require(ICleartextMarkers(aclAdd).IS_CLEARTEXT(), "ACL.IS_CLEARTEXT");
        require(ICleartextMarkers(aclAdd).CLEARTEXT_PROTOCOL_VERSION() == 1_000_000, "CLEARTEXT_PROTOCOL_VERSION");
        require(ICleartextMarkers(hcuLimitAdd).IS_CLEARTEXT(), "HCULimit.IS_CLEARTEXT");
        require(ICleartextMarkers(cleartextArithmeticAdd).IS_CLEARTEXT(), "CleartextArithmetic.IS_CLEARTEXT");
        // versions
        _eq(IVersioned(aclAdd).getVersion(), "ACL v0.6.0", "ACL");
        _eq(IVersioned(fhevmExecutorAdd).getVersion(), "FHEVMExecutor v0.7.0", "FHEVMExecutor");
        _eq(IVersioned(kmsVerifierAdd).getVersion(), "KMSVerifier v0.5.0", "KMSVerifier");
        _eq(IVersioned(inputVerifierAdd).getVersion(), "InputVerifier v0.3.0", "InputVerifier");
        _eq(IVersioned(hcuLimitAdd).getVersion(), "HCULimit v0.4.0", "HCULimit");
        _eq(IVersioned(protocolConfigAdd).getVersion(), "ProtocolConfig v0.3.0", "ProtocolConfig");
        _eq(IVersioned(kmsGenerationAdd).getVersion(), "KMSGeneration v0.3.0", "KMSGeneration");
        _eq(IVersioned(paddedInputterAdd).getVersion(), "PaddedInputter v0.1.0", "PaddedInputter");
        _eq(IVersioned(cleartextArithmeticAdd).getVersion(), "CleartextArithmetic v0.6.0", "CleartextArithmetic");
        _eq(IVersioned(cleartextDbAdd).getVersion(), "CleartextDB v0.1.0", "CleartextDB");
        _eq(IVersioned(pauserSetAdd).getVersion(), "PauserSet v0.1.0", "PauserSet");
        _eq(
            IVersioned(protocolConfigReplicaAdd).getVersion(), "ProtocolConfigReplica v0.3.0", "ProtocolConfigReplica"
        );
        // wiring
        require(ACL(aclAdd).getFHEVMExecutorAddress() == fhevmExecutorAdd, "ACL.getFHEVMExecutorAddress");
        require(ACL(aclAdd).getPauserSetAddress() == pauserSetAdd, "ACL.getPauserSetAddress");
        require(ACL(aclAdd).getConfidentialBridgeAddress() == address(0), "ACL.getConfidentialBridgeAddress");
        require(FHEVMExecutor(fhevmExecutorAdd).getACLAddress() == aclAdd, "FHEVMExecutor.getACLAddress");
        require(FHEVMExecutor(fhevmExecutorAdd).getHCULimitAddress() == hcuLimitAdd, "FHEVMExecutor.getHCULimitAddress");
        require(
            FHEVMExecutor(fhevmExecutorAdd).getInputVerifierAddress() == inputVerifierAdd,
            "FHEVMExecutor.getInputVerifierAddress"
        );
        require(HCULimit(hcuLimitAdd).getFHEVMExecutorAddress() == fhevmExecutorAdd, "HCULimit.getFHEVMExecutorAddress");
        // ownership
        require(ACL(aclAdd).owner() == expectedAclOwner, "ACL.owner != ACLOwner");
        require(ACLOwner(expectedAclOwner).ACL_ADDRESS() == aclAdd, "ACLOwner.ACL_ADDRESS");
        require(PauserSet(pauserSetAdd).isPauser(expectedAclOwner), "ACLOwner not a pauser");
        // bootstrap
        address[] memory cop = InputVerifier(inputVerifierAdd).getCoprocessorSigners();
        require(cop.length == 4 && cop[0] == LocalHostBootstrap.coprocessorSigners()[0], "coprocessor signers");
        require(
            InputVerifier(inputVerifierAdd).getThreshold() == LocalHostBootstrap.COPROCESSOR_THRESHOLD,
            "coprocessor threshold"
        );
        address[] memory kms = ProtocolConfig(protocolConfigAdd).getKmsSigners();
        require(kms.length == 4 && kms[3] == LocalHostBootstrap.kmsSigners()[3], "kms signers");
        require(
            ProtocolConfig(protocolConfigAdd).getCurrentKmsContextId() == LocalHostBootstrap.INITIAL_KMS_CONTEXT_ID,
            "kms context id"
        );
        require(ProtocolConfig(protocolConfigAdd).getPublicDecryptionThreshold() == 4, "public decryption threshold");
        require(
            ProtocolConfigReplica(protocolConfigReplicaAdd).getCurrentKmsContextId()
                == LocalHostBootstrap.INITIAL_KMS_CONTEXT_ID,
            "replica context id"
        );
        require(
            PaddedInputter(paddedInputterAdd).isSupportedPrfInstance(LocalHostBootstrap.PAD_PRF_INSTANCE), "prf"
        );
        require(HCULimit(hcuLimitAdd).getMaxHCUPerTx() == LocalHostBootstrap.MAX_HCU_PER_TX, "maxHCUPerTx");
        (, string memory n1,, uint256 c1, address v1,,) = IEIP712Domain(kmsVerifierAdd).eip712Domain();
        require(keccak256(bytes(n1)) == keccak256("Decryption"), "KMSVerifier domain name");
        require(
            c1 == LocalHostBootstrap.GATEWAY_CHAIN_ID && v1 == LocalHostBootstrap.DECRYPTION_ADDRESS,
            "KMSVerifier domain"
        );
        (, string memory n2,, uint256 c2, address v2,,) = IEIP712Domain(inputVerifierAdd).eip712Domain();
        require(keccak256(bytes(n2)) == keccak256("InputVerification"), "InputVerifier domain name");
        require(
            c2 == LocalHostBootstrap.GATEWAY_CHAIN_ID && v2 == LocalHostBootstrap.INPUT_VERIFICATION_ADDRESS, "iv dom"
        );
    }
}

// -----------------------------------------------------------------------------------------------------------------
// Entry points called by scripts/fhevm-deploy.sh, the same names as every other generation. Only the deployer comes
// from the environment (DEPLOYER_MNEMONIC, DEPLOYER_MNEMONIC_INDEX); every other value is LocalHostBootstrap's.
// src/v1.0.0 is read-only: its committed config/addresses.sol (canonical deployer, start nonce 0) is checked, never
// rewritten, so only a deployer whose addresses match it can deploy.
// -----------------------------------------------------------------------------------------------------------------

/// @dev Nonce offset of ACLOwner, the first address `config/addresses.sol` does not hold (DEPLOY.md §4).
uint64 constant ACL_OWNER_NONCE_OFFSET = 14;

abstract contract DeployerScript is Script {
    function _deployerKey() internal view returns (uint256) {
        return vm.deriveKey(vm.envString("DEPLOYER_MNEMONIC"), uint32(vm.envUint("DEPLOYER_MNEMONIC_INDEX")));
    }

    function _deployer() internal view returns (address) {
        return vm.addr(_deployerKey());
    }

    /// @dev Reverts unless `config/addresses.sol` holds the addresses `deployer` creates from `startNonce`.
    function _requireAddressesFor(address deployer, uint64 startNonce) internal pure {
        _requireAt("ACL", aclAdd, deployer, startNonce + 1);
        _requireAt("FHEVMExecutor", fhevmExecutorAdd, deployer, startNonce + 3);
        _requireAt("KMSVerifier", kmsVerifierAdd, deployer, startNonce + 4);
        _requireAt("InputVerifier", inputVerifierAdd, deployer, startNonce + 5);
        _requireAt("HCULimit", hcuLimitAdd, deployer, startNonce + 6);
        _requireAt("ProtocolConfig", protocolConfigAdd, deployer, startNonce + 7);
        _requireAt("KMSGeneration", kmsGenerationAdd, deployer, startNonce + 8);
        _requireAt("PaddedInputter", paddedInputterAdd, deployer, startNonce + 9);
        _requireAt("CleartextArithmetic", cleartextArithmeticAdd, deployer, startNonce + 10);
        _requireAt("CleartextDB", cleartextDbAdd, deployer, startNonce + 11);
        _requireAt("PauserSet", pauserSetAdd, deployer, startNonce + 12);
        _requireAt("ProtocolConfigReplica", protocolConfigReplicaAdd, deployer, startNonce + 13);
    }

    function _requireAt(string memory name, address expected, address deployer, uint64 nonce) private pure {
        if (vm.computeCreateAddress(deployer, nonce) != expected) {
            revert(
                string.concat(
                    "src/v1.0.0/config/addresses.sol does not match deployer ",
                    vm.toString(deployer),
                    " at start nonce ",
                    vm.toString(uint256(nonce)),
                    " (",
                    name,
                    "). It holds the canonical set (DEPLOY.md section 4) and src/v1.0.0 is read-only."
                )
            );
        }
    }
}

/// The deployer, in the JSON shape fhevm-deploy.sh reads. v1.0.0 uses a single key: no empty-proxy deployer.
contract PrintFhevmSigners is DeployerScript {
    function run() external view {
        console.log("JSON_RESULT_START");
        console.log(
            string.concat(
                '{"deployer":{"address":"',
                vm.toString(_deployer()),
                '"},"emptyUupsDeployer":{"address":"0x0000000000000000000000000000000000000000"}}'
            )
        );
        console.log("JSON_RESULT_END");
    }
}

/// Named as for the other generations; here it CHECKS the committed config/addresses.sol instead of writing it.
contract WriteFHEVMHostAddressesDotSol is DeployerScript {
    function run() external view {
        _requireAddressesFor(_deployer(), vm.getNonce(_deployer()));
        console.log("src/v1.0.0/config/addresses.sol matches the deployer; nothing written (read-only folder).");
    }
}

contract PrintFHEVMHostAddressesDotSol is DeployerScript {
    function run() external view {
        address d = _deployer();
        console.log("FHEVM host addresses (v1.0.0, src/v1.0.0/config/addresses.sol)");
        console.log("  acl                   :", aclAdd);
        console.log("  fhevmExecutor         :", fhevmExecutorAdd);
        console.log("  kmsVerifier           :", kmsVerifierAdd);
        console.log("  inputVerifier         :", inputVerifierAdd);
        console.log("  hcuLimit              :", hcuLimitAdd);
        console.log("  protocolConfig        :", protocolConfigAdd);
        console.log("  kmsGeneration         :", kmsGenerationAdd);
        console.log("  paddedInputter        :", paddedInputterAdd);
        console.log("  cleartextArithmetic   :", cleartextArithmeticAdd);
        console.log("  cleartextDb           :", cleartextDbAdd);
        console.log("  pauserSet             :", pauserSetAdd);
        console.log("  protocolConfigReplica :", protocolConfigReplicaAdd);
        console.log("  aclOwner              :", vm.computeCreateAddress(d, ACL_OWNER_NONCE_OFFSET));
        console.log("  deployer              :", d);
    }
}

contract Deploy is DeployerScript {
    function run() external {
        uint256 key = _deployerKey();
        address deployer = vm.addr(key);
        _requireAddressesFor(deployer, vm.getNonce(deployer));
        vm.startBroadcast(key);
        ACLOwner aclOwner = DeployV1.deploy(deployer);
        vm.stopBroadcast();
        console.log("ACLOwner:", address(aclOwner));
    }
}

/// Read-only checks against the deployed stack (DEPLOY.md section 9). The stack starts at nonce 0 (checked at deploy).
contract Verify is DeployerScript {
    function run() external view {
        DeployV1.verify(vm.computeCreateAddress(_deployer(), ACL_OWNER_NONCE_OFFSET));
        console.log("v1.0.0 cleartext stack verified");
    }
}
