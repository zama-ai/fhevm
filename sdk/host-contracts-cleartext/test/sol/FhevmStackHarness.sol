// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";

// Imported for its side effect: `deployCodeTo` looks the proxy up by artifact name, and solc only emits an
// artifact for a file something imports.
// forge-lint: disable-next-line(unused-import)
import {HarnessProxy} from "./HarnessProxy.sol";

import {ACL} from "../../src/contracts/ACL.sol";
import {FHEVMExecutor} from "../../src/contracts/FHEVMExecutor.sol";
import {HCULimit} from "../../src/contracts/HCULimit.sol";
import {ProtocolConfig} from "../../src/contracts/ProtocolConfig.sol";
import {PauserSet} from "../../src/contracts/immutable/PauserSet.sol";
import {EmptyUUPSProxy} from "../../src/contracts/emptyProxy/EmptyUUPSProxy.sol";
import {EmptyUUPSProxyACL} from "../../src/contracts/emptyProxyACL/EmptyUUPSProxyACL.sol";
import {IProtocolConfig} from "../../src/contracts/interfaces/IProtocolConfig.sol";
import {KmsNodeParams, PcrValues} from "../../src/contracts/shared/Structs.sol";

import {CleartextArithmetic} from "../../src/cleartext/CleartextArithmetic.sol";
import {CleartextDB} from "../../src/cleartext/CleartextDB.sol";
import {CleartextFHEVMExecutor} from "../../src/cleartext/CleartextFHEVMExecutor.sol";

import {
    aclAdd,
    fhevmExecutorAdd,
    hcuLimitAdd,
    protocolConfigAdd,
    pauserSetAdd,
    cleartextArithmeticAdd,
    cleartextDbAdd
} from "../../src/addresses/FHEVMHostAddresses.sol";

/**
 * Stands up the minimum stack needed to exercise the cleartext op table: the executor, the arithmetic
 * contract it delegates to, the DB that arithmetic persists into, plus the ACL and HCULimit every op
 * consults. The verifiers are not deployed — `trivialEncrypt` and the ops never touch them.
 *
 * Contracts must land on the addresses in `config/addresses.sol`, because they reference each other
 * through those compile-time constants. `deployCodeTo` runs a proxy's constructor at a chosen address to
 * make that possible.
 *
 * ACL goes first: it is the ownership root, and every other proxy's `_authorizeUpgrade` is `onlyACLOwner`,
 * which resolves through `ACL.owner()`.
 */
abstract contract FhevmStackHarness is Test {
    address internal constant PROXY_OWNER = address(0xBEEF);
    string private constant PROXY_ARTIFACT = "HarnessProxy.sol:HarnessProxy";

    CleartextFHEVMExecutor internal executor;
    CleartextDB internal db;

    function setUp() public virtual {
        _deployACL();
        _deployProtocolConfig();

        deployCodeTo("PauserSet.sol:PauserSet", pauserSetAdd);

        // Every HCU limit is maxed out. These tests are about arithmetic SEMANTICS; HCU accounting is a
        // separate concern, already covered by the host-contract suite upstream, and a live limit here would
        // only surface mid-fuzz as an opaque revert that has nothing to do with the op under test.
        _behindEmptyProxy(
            hcuLimitAdd,
            address(new HCULimit()),
            abi.encodeCall(
                HCULimit.initializeFromEmptyProxy, (type(uint48).max, type(uint48).max, type(uint48).max)
            )
        );

        // CleartextDB's only writer is CleartextArithmetic. If that link were wrong every op would silently
        // record nothing and every assertion below would read a zero.
        _behindEmptyProxy(
            cleartextArithmeticAdd,
            address(new CleartextArithmetic()),
            abi.encodeWithSelector(CleartextArithmetic.initializeFromEmptyProxy.selector)
        );
        _behindEmptyProxy(
            cleartextDbAdd,
            address(new CleartextDB()),
            abi.encodeCall(CleartextDB.initializeFromEmptyProxy, (cleartextArithmeticAdd))
        );
        _behindEmptyProxy(
            fhevmExecutorAdd,
            address(new CleartextFHEVMExecutor()),
            abi.encodeWithSelector(FHEVMExecutor.initializeFromEmptyProxy.selector)
        );

        executor = CleartextFHEVMExecutor(fhevmExecutorAdd);
        db = CleartextDB(cleartextDbAdd);
    }

    function _deployACL() private {
        address emptyImpl = address(new EmptyUUPSProxyACL());
        deployCodeTo(
            PROXY_ARTIFACT, abi.encode(emptyImpl, abi.encodeCall(EmptyUUPSProxyACL.initialize, (PROXY_OWNER))), aclAdd
        );

        // Deploy before arming the prank: `new` is itself a call and would consume it.
        address aclImpl = address(new ACL());
        vm.prank(PROXY_OWNER);
        EmptyUUPSProxyACL(aclAdd).upgradeToAndCall(aclImpl, abi.encodeCall(ACL.initializeFromEmptyProxy, ()));
    }

    function _deployProtocolConfig() private {
        KmsNodeParams[] memory nodes = new KmsNodeParams[](1);
        nodes[0] = KmsNodeParams({
            txSenderAddress: address(0xC0FFEE),
            signerAddress: address(0xC0FFEE),
            ipAddress: "127.0.0.1",
            storageUrl: "https://kms.local",
            partyId: 1,
            mpcIdentity: "kms-1",
            caCert: "",
            storagePrefix: ""
        });
        IProtocolConfig.KmsThresholds memory thresholds =
            IProtocolConfig.KmsThresholds({publicDecryption: 1, userDecryption: 1, kmsGen: 1, mpc: 1});

        _behindEmptyProxy(
            protocolConfigAdd,
            address(new ProtocolConfig()),
            abi.encodeCall(
                ProtocolConfig.initializeFromEmptyProxy, (nodes, thresholds, "0.0.0-test", new PcrValues[](0))
            )
        );
    }

    function _behindEmptyProxy(address target, address implementation, bytes memory initCall) private {
        address emptyImpl = address(new EmptyUUPSProxy());
        deployCodeTo(PROXY_ARTIFACT, abi.encode(emptyImpl, abi.encodeCall(EmptyUUPSProxy.initialize, ())), target);

        vm.prank(PROXY_OWNER);
        EmptyUUPSProxy(payable(target)).upgradeToAndCall(implementation, initCall);
    }
}
