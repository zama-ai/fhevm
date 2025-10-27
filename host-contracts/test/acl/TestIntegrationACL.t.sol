// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {euint64} from "encrypted-types/EncryptedTypes.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ACL} from "../../contracts/ACL.sol";
import {PauserSet} from "../../contracts/immutable/PauserSet.sol";
import {FHE} from "../../lib/FHE.sol";
import {FHEEvents} from "../../contracts/FHEEvents.sol";
import {CoprocessorConfig} from "../../lib/Impl.sol";
import {aclAdd, fhevmExecutorAdd, kmsVerifierAdd, pauserSetAdd} from "../../addresses/FHEVMHostAddresses.sol";

contract TestIntegrationACL is HostContractsDeployerTestUtils {
    ACL internal acl;
    PauserSet internal pauserSet;
    FHELibCaller internal caller;

    address private constant OWNER = address(0xBEEF);
    address private constant PAUSER = address(0xCAFE);
    address[] private kmsSigners;
    address[] private inputSigners;

    function setUp() public {
        kmsSigners = new address[](1);
        kmsSigners[0] = address(0x7777);
        inputSigners = new address[](1);
        inputSigners[0] = address(0x8888);

        _deployFullHostStack(OWNER, PAUSER, address(0x1234), address(0x5678), 31337, kmsSigners, 1, inputSigners, 1);

        acl = ACL(aclAdd);
        pauserSet = PauserSet(pauserSetAdd);

        caller = new FHELibCaller(address(acl));
    }

    function test_BlockingAccountPreventsFHEAddCompute() public {
        bytes32 lhs = caller.trivialEncrypt(7);
        bytes32 rhs = caller.trivialEncrypt(11);
        caller.add(lhs, rhs);

        vm.prank(OWNER);
        acl.blockAccount(address(caller));

        vm.expectRevert(abi.encodeWithSelector(ACL.SenderDenied.selector, address(caller)));
        caller.add(lhs, rhs);

        vm.prank(OWNER);
        acl.unblockAccount(address(caller));

        caller.add(lhs, rhs);
    }

    function test_BlockingAccountPreventsFHESubCompute() public {
        bytes32 lhs = caller.trivialEncrypt(20);
        bytes32 rhs = caller.trivialEncrypt(5);
        caller.sub(lhs, rhs);

        vm.prank(OWNER);
        acl.blockAccount(address(caller));

        vm.expectRevert(abi.encodeWithSelector(ACL.SenderDenied.selector, address(caller)));
        caller.sub(lhs, rhs);

        vm.prank(OWNER);
        acl.unblockAccount(address(caller));

        caller.sub(lhs, rhs);
    }

    // A denied account can still call the FHEVM Executor and emit Computation events.
    // However, these events are most likely ignored on the coprocessor side.
    // This happens because every single handle resulting from these computations will not be ACL-persistent.
    // On the coprocessor side, any computation branch leading to a non-ACL-persistent handle (using allow/allowForDecryption)
    // will ultimately be automatically discarded.
    function test_BlockingAccountDoesNotPreventAddEventEmission() public {
        bytes32 lhs = caller.trivialEncrypt(7);
        bytes32 rhs = caller.trivialEncrypt(11);
        caller.addWithoutAllowing(lhs, rhs);

        vm.prank(OWNER);
        acl.blockAccount(address(caller));

        // Only check for the event emission and topic, not the exact event data
        vm.expectEmit(true, false, false, false);
        emit FHEEvents.FheAdd(address(caller), lhs, rhs, bytes1(0x00), bytes32(0));
        caller.addWithoutAllowing(lhs, rhs);
    }
}

contract FHELibCaller {
    constructor(address aclAddress) {
        CoprocessorConfig memory config = CoprocessorConfig({
            ACLAddress: aclAddress,
            CoprocessorAddress: fhevmExecutorAdd,
            DecryptionOracleAddress: address(0),
            KMSVerifierAddress: kmsVerifierAdd
        });
        FHE.setCoprocessor(config);
    }

    function trivialEncrypt(uint64 value) external returns (bytes32 handle) {
        euint64 encrypted = FHE.asEuint64(value);
        FHE.allowThis(encrypted);
        handle = euint64.unwrap(encrypted);
    }

    function add(bytes32 lhs, bytes32 rhs) external returns (bytes32 handle) {
        euint64 result = FHE.add(euint64.wrap(lhs), euint64.wrap(rhs));
        FHE.allowThis(result);
        handle = euint64.unwrap(result);
    }

    function addWithoutAllowing(bytes32 lhs, bytes32 rhs) external returns (bytes32 handle) {
        euint64 result = FHE.add(euint64.wrap(lhs), euint64.wrap(rhs));
        handle = euint64.unwrap(result);
    }

    function sub(bytes32 lhs, bytes32 rhs) external returns (bytes32 handle) {
        euint64 result = FHE.sub(euint64.wrap(lhs), euint64.wrap(rhs));
        FHE.allowThis(result);
        handle = euint64.unwrap(result);
    }
}
