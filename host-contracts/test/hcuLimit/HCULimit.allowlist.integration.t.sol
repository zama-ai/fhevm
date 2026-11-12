// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {fhevmExecutorAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {FHEEvents} from "@fhevm-host-contracts/contracts/FHEEvents.sol";
import {FHEVMExecutor} from "@fhevm-host-contracts/contracts/FHEVMExecutor.sol";
import {HCULimit} from "@fhevm-host-contracts/contracts/HCULimit.sol";
import {HANDLE_VERSION} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {FheType} from "@fhevm-host-contracts/contracts/shared/FheType.sol";

contract AllowlistExecutorDapp {
    function add(bytes32 lhs) external returns (bytes32) {
        return FHEVMExecutor(fhevmExecutorAdd).fheAdd(lhs, bytes32(uint256(1)), 0x01);
    }

    function rand() external returns (bytes32) {
        return FHEVMExecutor(fhevmExecutorAdd).fheRand(FheType.Uint8);
    }
}

contract HCULimitAllowlistIntegrationTest is HostContractsDeployerTestUtils {
    address private constant OWNER = address(0xBEEF);
    uint48 private constant ADD_HCU = 84_000;
    uint48 private constant RAND_HCU = 23_000;
    bytes32 private constant EXECUTOR_STORAGE_SLOT = 0x4613e1771f6b755d243e536fb5a23c5b15e2826575fee921e8fe7a22a760c800;

    ACL private acl;
    HCULimit private hcuLimit;
    AllowlistExecutorDapp private dapp;
    bytes32 private input;

    function setUp() public {
        (acl,) = _deployACL(OWNER);
        _deployFHEVMExecutor(OWNER);
        (hcuLimit,) = _deployHCULimit(OWNER);
        dapp = new AllowlistExecutorDapp();
        input = bytes32(uint256(uint8(FheType.Uint8)) << 8);

        vm.prank(OWNER);
        hcuLimit.setAllowlistActivated(true);
    }

    function test_fheAddRejectsUnlistedDappWithoutSideEffectsThenSucceeds() public {
        vm.prank(fhevmExecutorAdd);
        acl.allowTransient(input, address(dapp));
        vm.prank(OWNER);
        hcuLimit.setMaxHCUDepthPerTx(ADD_HCU);
        vm.prank(OWNER);
        hcuLimit.setMaxHCUPerTx(ADD_HCU);
        bytes32 expectedResult = _addResult();
        assertFalse(acl.isAllowed(expectedResult, address(dapp)));
        (, uint48 meterBefore) = hcuLimit.getBlockMeter();

        vm.recordLogs();
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotAllowlisted.selector, address(dapp)));
        dapp.add(input);
        assertEq(vm.getRecordedLogs().length, 0);
        assertTrue(acl.isAllowed(input, address(dapp)));
        assertFalse(acl.isAllowed(expectedResult, address(dapp)));
        (, uint48 meterAfterRejection) = hcuLimit.getBlockMeter();
        assertEq(meterAfterRejection, meterBefore);

        _allowDapp();
        vm.expectEmit(true, false, false, true, fhevmExecutorAdd);
        emit FHEEvents.FheAdd(address(dapp), input, bytes32(uint256(1)), 0x01, expectedResult);
        assertEq(dapp.add(input), expectedResult);
        assertTrue(acl.isAllowed(expectedResult, address(dapp)));
        (, uint48 meterAfterSuccess) = hcuLimit.getBlockMeter();
        assertEq(meterAfterSuccess, meterBefore + ADD_HCU);
    }

    function test_fheRandRejectsUnlistedDappWithoutSideEffectsThenSucceeds() public {
        vm.prank(OWNER);
        hcuLimit.setMaxHCUDepthPerTx(RAND_HCU);
        vm.prank(OWNER);
        hcuLimit.setMaxHCUPerTx(RAND_HCU);
        uint256 counterBefore = uint256(vm.load(fhevmExecutorAdd, EXECUTOR_STORAGE_SLOT));
        bytes16 expectedSeed = _seed(counterBefore);
        bytes32 expectedResult = _withMetadata(
            keccak256(
                abi.encodePacked(bytes8("FHE_comp"), FHEVMExecutor.Operators.fheRand, FheType.Uint8, expectedSeed)
            )
        );
        (, uint48 meterBefore) = hcuLimit.getBlockMeter();

        vm.recordLogs();
        vm.expectRevert(abi.encodeWithSelector(HCULimit.CallerNotAllowlisted.selector, address(dapp)));
        dapp.rand();
        assertEq(vm.getRecordedLogs().length, 0);
        assertEq(uint256(vm.load(fhevmExecutorAdd, EXECUTOR_STORAGE_SLOT)), counterBefore);
        assertFalse(acl.isAllowed(expectedResult, address(dapp)));
        (, uint48 meterAfterRejection) = hcuLimit.getBlockMeter();
        assertEq(meterAfterRejection, meterBefore);

        _allowDapp();
        vm.expectEmit(true, false, false, true, fhevmExecutorAdd);
        emit FHEEvents.FheRand(address(dapp), FheType.Uint8, expectedSeed, expectedResult);
        assertEq(dapp.rand(), expectedResult);
        assertEq(uint256(vm.load(fhevmExecutorAdd, EXECUTOR_STORAGE_SLOT)), counterBefore + 1);
        assertTrue(acl.isAllowed(expectedResult, address(dapp)));
        (, uint48 meterAfterSuccess) = hcuLimit.getBlockMeter();
        assertEq(meterAfterSuccess, meterBefore + RAND_HCU);
    }

    function _allowDapp() private {
        address[] memory accounts = new address[](1);
        accounts[0] = address(dapp);
        vm.prank(OWNER);
        hcuLimit.addToAllowlist(accounts);
    }

    function _addResult() private view returns (bytes32) {
        return _withMetadata(
            keccak256(
                abi.encodePacked(
                    bytes8("FHE_comp"),
                    FHEVMExecutor.Operators.fheAdd,
                    input,
                    bytes32(uint256(1)),
                    bytes1(0x01),
                    acl,
                    block.chainid,
                    blockhash(block.number - 1),
                    block.timestamp
                )
            )
        );
    }

    function _seed(uint256 counter) private view returns (bytes16) {
        return bytes16(
            keccak256(
                abi.encodePacked(
                    bytes8("FHE_seed"), counter, acl, block.chainid, blockhash(block.number - 1), block.timestamp
                )
            )
        );
    }

    function _withMetadata(bytes32 prehandle) private view returns (bytes32) {
        return (prehandle & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000)
            | (bytes32(uint256(0xff)) << 80) | (bytes32(uint256(uint64(block.chainid))) << 16)
            | (bytes32(uint256(uint8(FheType.Uint8))) << 8) | bytes32(uint256(HANDLE_VERSION));
    }
}
