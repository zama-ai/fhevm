// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {fhevmExecutorAdd, hcuConfiguratorAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {FHEEvents} from "@fhevm-host-contracts/contracts/FHEEvents.sol";
import {FHEVMExecutor} from "@fhevm-host-contracts/contracts/FHEVMExecutor.sol";
import {HCULimit} from "@fhevm-host-contracts/contracts/HCULimit.sol";
import {HCUConfigurator} from "@fhevm-host-contracts/contracts/HCUConfigurator.sol";
import {HANDLE_VERSION} from "@fhevm-host-contracts/contracts/shared/Constants.sol";
import {FheType} from "@fhevm-host-contracts/contracts/shared/FheType.sol";

contract QuotaExecutorDapp {
    function add(bytes32 lhs) external returns (bytes32) {
        return FHEVMExecutor(fhevmExecutorAdd).fheAdd(lhs, bytes32(uint256(1)), 0x01);
    }

    function rand() external returns (bytes32) {
        return FHEVMExecutor(fhevmExecutorAdd).fheRand(FheType.Uint8);
    }
}

/// @dev A contract outside every quota is refused through FHEVMExecutor without side effects, then computes once the
///      owner assigns it to a quota.
contract HCULimitQuotasIntegrationTest is HostContractsDeployerTestUtils {
    address private constant OWNER = address(0xBEEF);
    uint16 private constant DEFAULT_QUOTA = 0;
    uint16 private constant DAPP_QUOTA = 3;
    uint48 private constant ADD_HCU = 84_000;
    uint48 private constant RAND_HCU = 23_000;
    bytes32 private constant EXECUTOR_STORAGE_SLOT = 0x4613e1771f6b755d243e536fb5a23c5b15e2826575fee921e8fe7a22a760c800;

    ACL private acl;
    HCULimit private hcuLimit;
    HCUConfigurator private constant hcuConfigurator = HCUConfigurator(hcuConfiguratorAdd);
    QuotaExecutorDapp private dapp;
    bytes32 private input;

    function setUp() public {
        (acl, ) = _deployACL(OWNER);
        _deployFHEVMExecutor(OWNER);
        (hcuLimit, ) = _deployHCULimit(OWNER);
        _deployHCUConfigurator(OWNER);
        dapp = new QuotaExecutorDapp();
        input = bytes32(uint256(uint8(FheType.Uint8)) << 8);

        vm.prank(OWNER);
        hcuConfigurator.setQuota(DEFAULT_QUOTA, 0, 0);
    }

    function test_fheAddRefusesADappInAStoppedQuotaWithoutSideEffectsThenSucceeds() public {
        vm.prank(fhevmExecutorAdd);
        acl.allowTransient(input, address(dapp));
        vm.prank(OWNER);
        hcuConfigurator.setMaxHCUDepthPerTx(ADD_HCU);
        vm.prank(OWNER);
        hcuConfigurator.setMaxHCUPerTx(ADD_HCU);
        bytes32 expectedResult = _addResult();
        assertFalse(acl.isAllowed(expectedResult, address(dapp)));

        vm.recordLogs();
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, address(dapp), DEFAULT_QUOTA));
        dapp.add(input);
        assertEq(vm.getRecordedLogs().length, 0);
        assertTrue(acl.isAllowed(input, address(dapp)));
        assertFalse(acl.isAllowed(expectedResult, address(dapp)));

        _admitDapp();
        vm.expectEmit(true, false, false, true, fhevmExecutorAdd);
        emit FHEEvents.FheAdd(address(dapp), input, bytes32(uint256(1)), 0x01, expectedResult);
        assertEq(dapp.add(input), expectedResult);
        assertTrue(acl.isAllowed(expectedResult, address(dapp)));
        (, , uint48 usedHCU) = hcuLimit.getQuota(DAPP_QUOTA);
        assertEq(usedHCU, ADD_HCU);
    }

    function test_fheRandRefusesADappInAStoppedQuotaWithoutSideEffectsThenSucceeds() public {
        vm.prank(OWNER);
        hcuConfigurator.setMaxHCUDepthPerTx(RAND_HCU);
        vm.prank(OWNER);
        hcuConfigurator.setMaxHCUPerTx(RAND_HCU);
        uint256 counterBefore = uint256(vm.load(fhevmExecutorAdd, EXECUTOR_STORAGE_SLOT));
        bytes16 expectedSeed = _seed(counterBefore);
        bytes32 expectedResult = _withMetadata(
            keccak256(
                abi.encodePacked(bytes8("FHE_comp"), FHEVMExecutor.Operators.fheRand, FheType.Uint8, expectedSeed)
            )
        );

        vm.recordLogs();
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, address(dapp), DEFAULT_QUOTA));
        dapp.rand();
        assertEq(vm.getRecordedLogs().length, 0);
        assertEq(uint256(vm.load(fhevmExecutorAdd, EXECUTOR_STORAGE_SLOT)), counterBefore);
        assertFalse(acl.isAllowed(expectedResult, address(dapp)));

        _admitDapp();
        vm.expectEmit(true, false, false, true, fhevmExecutorAdd);
        emit FHEEvents.FheRand(address(dapp), FheType.Uint8, expectedSeed, expectedResult);
        assertEq(dapp.rand(), expectedResult);
        assertEq(uint256(vm.load(fhevmExecutorAdd, EXECUTOR_STORAGE_SLOT)), counterBefore + 1);
        assertTrue(acl.isAllowed(expectedResult, address(dapp)));
        (, , uint48 usedHCU) = hcuLimit.getQuota(DAPP_QUOTA);
        assertEq(usedHCU, RAND_HCU);
    }

    function _admitDapp() private {
        vm.startPrank(OWNER);
        hcuConfigurator.setQuota(DAPP_QUOTA, 1_000_000, 1_000_000);
        hcuConfigurator.assignQuota(DAPP_QUOTA, address(dapp));
        vm.stopPrank();
    }

    function _addResult() private view returns (bytes32) {
        return
            _withMetadata(
                keccak256(
                    abi.encodePacked(
                        bytes8("FHE_comp"),
                        FHEVMExecutor.Operators.fheAdd,
                        input,
                        bytes32(uint256(1)),
                        bytes1(0x01),
                        uint256(1),
                        acl,
                        block.chainid,
                        blockhash(block.number - 1),
                        block.timestamp
                    )
                )
            );
    }

    function _seed(uint256 counter) private view returns (bytes16) {
        return
            bytes16(
                keccak256(
                    abi.encodePacked(
                        bytes8("FHE_seed"),
                        counter,
                        acl,
                        block.chainid,
                        blockhash(block.number - 1),
                        block.timestamp
                    )
                )
            );
    }

    function _withMetadata(bytes32 prehandle) private view returns (bytes32) {
        return
            (prehandle & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000) |
            (bytes32(uint256(0xff)) << 80) |
            (bytes32(uint256(uint64(block.chainid))) << 16) |
            (bytes32(uint256(uint8(FheType.Uint8))) << 8) |
            bytes32(uint256(HANDLE_VERSION));
    }
}
