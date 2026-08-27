// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";

import {ACL} from "../contracts/ACL.sol";
import {FHEVMExecutor} from "../contracts/FHEVMExecutor.sol";
import {HCULimit} from "../contracts/HCULimit.sol";
import {InputVerifier} from "../contracts/InputVerifier.sol";
import {KMSVerifier} from "../contracts/KMSVerifier.sol";
import {KMSGeneration} from "../contracts/KMSGeneration.sol";
import {ProtocolConfig} from "../contracts/ProtocolConfig.sol";
import {IProtocolConfig} from "../contracts/interfaces/IProtocolConfig.sol";
import {KmsNodeParams, PcrValues} from "../contracts/shared/Structs.sol";

// Referenced only through `vm.getDeployedCode`, but the import is what makes the consumer's build emit
// the artifact that call resolves.
// forge-lint: disable-next-line(unused-import)
import {PauserSet} from "../contracts/immutable/PauserSet.sol";

import {CleartextArithmetic} from "../cleartext/CleartextArithmetic.sol";
import {CleartextDB} from "../cleartext/CleartextDB.sol";

// Referenced only through `vm.getDeployedCode` (their initializers come from the base contracts above),
// so like PauserSet they need explicit imports for the consumer's build to emit their artifacts.
// forge-lint: disable-next-line(unused-import)
import {CleartextFHEVMExecutor} from "../cleartext/CleartextFHEVMExecutor.sol";
// forge-lint: disable-next-line(unused-import)
import {CleartextKMSVerifier} from "../cleartext/CleartextKMSVerifier.sol";
// forge-lint: disable-next-line(unused-import)
import {CleartextInputVerifier} from "../cleartext/CleartextInputVerifier.sol";

import {
    aclAdd,
    fhevmExecutorAdd,
    kmsVerifierAdd,
    inputVerifierAdd,
    hcuLimitAdd,
    protocolConfigAdd,
    kmsGenerationAdd,
    pauserSetAdd,
    cleartextArithmeticAdd,
    cleartextDbAdd
} from "../addresses/FHEVMHostAddresses.sol";

/**
 * Stands the cleartext stack up on an ALREADY RUNNING dev node (anvil or `hardhat node`) — the forge
 * counterpart of this package's `npm run deploy:local`, so a Foundry project needs nothing but forge:
 *
 *     anvil     # or: npx hardhat node
 *     forge script script/DeployFhevmStack.s.sol --rpc-url http://localhost:8545
 *
 * (The consumer's script is a one-line `contract DeployFhevmStack is DeployFhevmStackLocal {}`.)
 *
 * No `--broadcast` and no key: everything goes through `vm.rpc` — the node's own cheat codes place the
 * runtime bytecode at the fixed addresses (a forge script's `deployCodeTo`/`vm.etch` only affect its local
 * simulation, never the node), and the initializers are sent from an impersonated admin the node signs for.
 * The bytecode comes from THIS build via `vm.getDeployedCode`, so the addresses baked into it are exactly
 * the ones the consumer's `fhevm-config` remapping chose — placement and cross-references cannot disagree.
 *
 * The stack is initialized with the STANDARD MOCK IDENTITY — the same signer keys and gateway identity as
 * the hardhat plugin and the package's `internal/deploy-local` script. That is what lets the hardhat
 * plugin ADOPT a chain prepared here instead of redeploying. Change these values in lockstep or not at all.
 */
abstract contract DeployFhevmStackLocal is Script {
    /// @dev Becomes ACL's owner and sends every initializer. Impersonated — the node signs for it.
    address internal constant ADMIN = address(0x1);

    uint256 internal constant MOCK_KMS_SIGNER_PK = 0x388b7680e4e1afa06efbfd45cdd1fe39f3c6af381df6555a19661f283b97de91;
    uint256 internal constant MOCK_INPUT_SIGNER_PK =
        0x7ec8ada6642fc4ccfb7729bc29c17cf8d21b61abd5642d1db992c0b8672ab901;
    uint64 internal constant GATEWAY_CHAIN_ID = 10901;
    address internal constant GATEWAY_DECRYPTION_ADDRESS = 0x5ffdaAB0373E62E2ea2944776209aEf29E631A64;
    address internal constant GATEWAY_INPUT_VERIFICATION_ADDRESS = 0x812b06e1CDCE800494b79fFE4f925A504a9A9810;

    /// @dev Block cap maxed out: throttling long dev sessions is a production concern, not a testing one.
    uint48 private constant HCU_CAP_PER_BLOCK = type(uint48).max;
    uint48 private constant HCU_MAX_DEPTH_PER_TX = 5_000_000;
    uint48 private constant HCU_MAX_PER_TX = 20_000_000;

    /// @dev ERC-7201 storage locations (OpenZeppelin upgradeable) — layout facts, not configuration.
    bytes32 private constant INITIALIZABLE_STORAGE_SLOT =
        0xf0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00;
    bytes32 private constant OWNABLE_STORAGE_SLOT = 0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300;

    /// @dev "anvil" or "hardhat" — which cheat-code RPC namespace the node speaks.
    string private _ns;

    function run() external {
        require(block.chainid == 31337, "ZamaConfig pins the local stack addresses for chainid 31337");
        _ns = _detectNamespace();

        // No overwrite path: setCode replaces code but NOT storage, so re-initializing over a used stack
        // trips on surviving state (e.g. ProtocolConfig's KMS context counter). Fresh stack = fresh node.
        // Code presence alone is not enough — a run that died between placement and initialization leaves
        // code with no live stack behind it, and adopting that silently would fail much later, in a test.
        if (_rpcBytes("eth_getCode", string.concat('["', vm.toString(fhevmExecutorAdd), '","latest"]')).length > 0) {
            // The same readiness probe the hardhat plugin uses for adoption: the mock KMS signer must be
            // registered. An uninitialized stack does NOT revert here — it returns an empty signer set.
            require(
                _stackIsInitialized(),
                "host-contract code is present but the stack is not initialized (a previous run died "
                "half-way?) - restart the node and run this again"
            );
            console2.log("FHEVM cleartext stack already present on this node (%s); nothing to do.", _ns);
            console2.log("For a fresh stack, restart the node and run this again.");
            return;
        }

        console2.log("Deploying the FHEVM cleartext stack (%s)...", _ns);

        // 1. Place every contract's runtime code — compiled by THIS project, addresses already baked.
        _setCode(aclAdd, "ACL.sol:ACL");
        _setCode(fhevmExecutorAdd, "CleartextFHEVMExecutor.sol:CleartextFHEVMExecutor");
        _setCode(kmsVerifierAdd, "CleartextKMSVerifier.sol:CleartextKMSVerifier");
        _setCode(inputVerifierAdd, "CleartextInputVerifier.sol:CleartextInputVerifier");
        _setCode(hcuLimitAdd, "HCULimit.sol:HCULimit");
        _setCode(protocolConfigAdd, "ProtocolConfig.sol:ProtocolConfig");
        _setCode(kmsGenerationAdd, "KMSGeneration.sol:KMSGeneration");
        _setCode(pauserSetAdd, "PauserSet.sol:PauserSet");
        _setCode(cleartextArithmeticAdd, "CleartextArithmetic.sol:CleartextArithmetic");
        _setCode(cleartextDbAdd, "CleartextDB.sol:CleartextDB");

        // 2. Fake the post-`EmptyUUPSProxy.initialize()` state `onlyFromEmptyProxy` checks for (PauserSet is
        //    immutable — no proxy, no initializer), and give ACL its owner BEFORE its initializer runs:
        //    `ACL.initializeFromEmptyProxy()` preserves the owner already in the slot rather than taking one.
        address[9] memory proxies = [
            aclAdd,
            fhevmExecutorAdd,
            kmsVerifierAdd,
            inputVerifierAdd,
            hcuLimitAdd,
            protocolConfigAdd,
            kmsGenerationAdd,
            cleartextArithmeticAdd,
            cleartextDbAdd
        ];
        for (uint256 i = 0; i < proxies.length; i++) {
            _setStorage(proxies[i], INITIALIZABLE_STORAGE_SLOT, bytes32(uint256(1)));
        }
        _setStorage(aclAdd, OWNABLE_STORAGE_SLOT, bytes32(uint256(uint160(ADMIN))));

        // 3. Run the real initializers from the impersonated admin, in `bootstrapInitCalls` order.
        _rpc("_setBalance", string.concat('["', vm.toString(ADMIN), '","0x56BC75E2D63100000"]'));
        _rpc("_impersonateAccount", string.concat('["', vm.toString(ADMIN), '"]'));

        _send(protocolConfigAdd, _protocolConfigInitData());
        _send(aclAdd, abi.encodeCall(ACL.initializeFromEmptyProxy, ()));
        _send(fhevmExecutorAdd, abi.encodePacked(FHEVMExecutor.initializeFromEmptyProxy.selector));
        _send(kmsGenerationAdd, abi.encodePacked(KMSGeneration.initializeFromEmptyProxy.selector));
        _send(
            kmsVerifierAdd,
            abi.encodeCall(KMSVerifier.initializeFromEmptyProxy, (GATEWAY_DECRYPTION_ADDRESS, GATEWAY_CHAIN_ID))
        );
        address[] memory inputSigners = new address[](1);
        inputSigners[0] = vm.addr(MOCK_INPUT_SIGNER_PK);
        _send(
            inputVerifierAdd,
            abi.encodeCall(
                InputVerifier.initializeFromEmptyProxy,
                (GATEWAY_INPUT_VERIFICATION_ADDRESS, GATEWAY_CHAIN_ID, inputSigners, 1)
            )
        );
        _send(
            hcuLimitAdd,
            abi.encodeCall(HCULimit.initializeFromEmptyProxy, (HCU_CAP_PER_BLOCK, HCU_MAX_DEPTH_PER_TX, HCU_MAX_PER_TX))
        );
        _send(cleartextArithmeticAdd, abi.encodePacked(CleartextArithmetic.initializeFromEmptyProxy.selector));
        _send(cleartextDbAdd, abi.encodeCall(CleartextDB.initializeFromEmptyProxy, (cleartextArithmeticAdd)));

        // 4. Read back through the stack itself, so a silent misconfiguration fails here and not in a test.
        address[] memory kmsSigners =
            abi.decode(_call(kmsVerifierAdd, abi.encodeCall(KMSVerifier.getKmsSigners, ())), (address[]));
        require(
            kmsSigners.length == 1 && kmsSigners[0] == vm.addr(MOCK_KMS_SIGNER_PK),
            "post-deploy check failed: KMSVerifier.getKmsSigners() does not match the mock KMS signer"
        );
        bool isWriter =
            abi.decode(_call(cleartextDbAdd, abi.encodeCall(CleartextDB.isWriter, (cleartextArithmeticAdd))), (bool));
        require(isWriter, "post-deploy check failed: CleartextDB does not have CleartextArithmetic as a writer");

        console2.log("Done. Stack addresses (pinned by ZamaConfig for chainid 31337):");
        console2.log("  ACL              %s", aclAdd);
        console2.log("  FHEVMExecutor    %s", fhevmExecutorAdd);
        console2.log("  KMSVerifier      %s", kmsVerifierAdd);
    }

    function _protocolConfigInitData() private view returns (bytes memory) {
        KmsNodeParams[] memory nodes = new KmsNodeParams[](1);
        nodes[0] = KmsNodeParams({
            txSenderAddress: address(0xC0FFEE),
            signerAddress: vm.addr(MOCK_KMS_SIGNER_PK),
            ipAddress: "127.0.0.1",
            storageUrl: "https://kms.local",
            partyId: 1,
            mpcIdentity: "kms-1",
            caCert: "",
            storagePrefix: ""
        });
        IProtocolConfig.KmsThresholds memory thresholds =
            IProtocolConfig.KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1});
        return abi.encodeCall(
            ProtocolConfig.initializeFromEmptyProxy, (nodes, thresholds, "0.0.0-mock", new PcrValues[](0))
        );
    }

    function _stackIsInitialized() private returns (bool) {
        (bool ok, bytes memory ret) = address(vm).call(
            abi.encodeWithSignature(
                "rpc(string,string)",
                "eth_call",
                string.concat(
                    '[{"to":"',
                    vm.toString(kmsVerifierAdd),
                    '","data":"',
                    vm.toString(abi.encodeCall(KMSVerifier.getKmsSigners, ())),
                    '"},"latest"]'
                )
            )
        );
        if (!ok) {
            return false;
        }
        bytes memory result = abi.decode(ret, (bytes));
        if (result.length < 96) {
            return false;
        }
        address[] memory signers = abi.decode(result, (address[]));
        return signers.length == 1 && signers[0] == vm.addr(MOCK_KMS_SIGNER_PK);
    }

    /// @dev anvil aliases much of the `hardhat_*` namespace, but its own is authoritative — check it first.
    function _detectNamespace() private returns (string memory) {
        bytes memory client = _rpcBytes("web3_clientVersion", "[]");
        if (client.length >= 5 && client[0] == "a" && client[1] == "n" && client[2] == "v" && client[3] == "i") {
            return "anvil";
        }
        return "hardhat";
    }

    /// @dev Places runtime bytecode from this build's artifact — never a locally patched blob, so the
    ///      addresses the code carries are the ones this project compiled with.
    function _setCode(address target, string memory artifact) private {
        bytes memory code = vm.getDeployedCode(artifact);
        _rpc("_setCode", string.concat('["', vm.toString(target), '","', vm.toString(code), '"]'));
    }

    function _setStorage(address target, bytes32 slot, bytes32 value) private {
        _rpc(
            "_setStorageAt",
            string.concat(
                '["', vm.toString(target), '","', vm.toString(slot), '","', vm.toString(value), '"]'
            )
        );
    }

    /// @dev Both target nodes automine and surface a revert as an RPC error, which fails the script.
    function _send(address to, bytes memory data) private {
        _rpcBytes(
            "eth_sendTransaction",
            string.concat(
                '[{"from":"',
                vm.toString(ADMIN),
                '","to":"',
                vm.toString(to),
                '","data":"',
                vm.toString(data),
                // 10M — comfortably above any initializer, and under `hardhat node`'s 2^24 tx gas cap.
                '","gas":"0x989680"}]'
            )
        );
    }

    function _call(address to, bytes memory data) private returns (bytes memory) {
        return _rpcBytes(
            "eth_call",
            string.concat('[{"to":"', vm.toString(to), '","data":"', vm.toString(data), '"},"latest"]')
        );
    }

    /// @dev Sends `method` with the detected namespace prefix (`anvil_setCode` / `hardhat_setCode`, ...).
    ///      Low-level on purpose: several of these return a JSON boolean (`hardhat_setCode`,
    ///      `anvil_setStorageAt`, ...), which `vm.rpc`'s declared `bytes` return cannot decode — the RPC
    ///      succeeds on the node and then the decoding reverts in the caller. Ignoring the return data
    ///      sidesteps the decode; failures still surface through `ok`.
    function _rpc(string memory method, string memory params) private {
        (bool ok, ) = address(vm).call(abi.encodeWithSignature("rpc(string,string)", string.concat(_ns, method), params));
        require(ok, string.concat(_ns, method, " failed"));
    }

    function _rpcBytes(string memory method, string memory params) private returns (bytes memory) {
        return vm.rpc(method, params);
    }
}
