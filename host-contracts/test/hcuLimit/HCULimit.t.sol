// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import {FheType} from "../../contracts/shared/FheType.sol";
import {HCULimit} from "../../contracts/HCULimit.sol";
import {HCULimitOperators} from "../../contracts/HCULimitOperators.sol";
import {HCUConfigurator} from "../../contracts/HCUConfigurator.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {HostContractsDeployerTestUtils} from "../../fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {fhevmExecutorAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {SupportedTypesConstants} from "../fhevmExecutor/fhevmExecutor.t.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";
import {IPauserSet} from "../../contracts/interfaces/IPauserSet.sol";
import {aclAdd, hcuConfiguratorAdd, hcuLimitAdd, pauserSetAdd} from "../../addresses/FHEVMHostAddresses.sol";

contract MockHCULimit is HCULimit {
    function getHCUForTransaction() external view returns (uint256) {
        return _getHCUForTransaction();
    }

    /// Returns `handle`'s depth within this transaction.
    function getHCUForHandle(bytes32 handle) external view returns (uint256) {
        (uint256 depths, ) = _loadDepths(handle);
        return uint128(depths);
    }

    function setHCUForTransaction(uint256 handleHCU) external {
        _setHCUForTransaction(handleHCU);
    }

    /// Records `depth` as `handle`'s depth within this transaction, with no carried depth.
    function setHCUForHandle(bytes32 handle, uint256 depth) external {
        _storeDepths(handle, depth);
    }

    /// Calls metering function `hook` (0 ZeroOp, 1 OneOp, 2 TwoOps, 3 ThreeOps, 4 Array, 5 ValueAndArray).
    function meterForTest(
        uint256 hook,
        uint256 opCost,
        address caller,
        bytes32[] calldata inputs,
        bytes32 result
    ) external {
        if (hook == 0) _adjustAndCheckFheTransactionLimitZeroOp(opCost, caller, result);
        else if (hook == 1) _adjustAndCheckFheTransactionLimitOneOp(opCost, caller, inputs[0], result);
        else if (hook == 2) _adjustAndCheckFheTransactionLimitTwoOps(opCost, caller, inputs[0], inputs[1], result);
        else if (hook == 3)
            _adjustAndCheckFheTransactionLimitThreeOps(opCost, caller, inputs[0], inputs[1], inputs[2], result);
        else if (hook == 4) _adjustAndCheckFheTransactionLimitArray(opCost, caller, inputs, result);
        else _adjustAndCheckFheTransactionLimitValueAndArray(opCost, caller, inputs[0], inputs[1:], result);
    }
}

contract HCULimitTest is HostContractsDeployerTestUtils, SupportedTypesConstants {
    MockHCULimit internal hcuLimit;
    HCUConfigurator internal hcuConfigurator;

    address internal constant owner = address(456);

    address internal proxy;
    address internal implementation;
    address internal fhevmExecutor;

    uint256 internal MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX = 20_000_000;

    uint16 internal constant DEFAULT_QUOTA = 0;
    uint16 internal constant PARTNER_QUOTA = 12;
    address internal constant PARTNER_ADMIN = address(0xAD);
    uint16 internal constant LOW_QUOTA = 20;
    uint16 internal constant HIGH_QUOTA = 21;
    address internal constant LOW = address(0xA1);
    address internal constant HIGH = address(0xB1);
    uint16 internal constant ALLOWLIST_QUOTA = 1;
    uint16 internal constant BLOCK_WHITELIST_QUOTA = 2;
    bytes32 internal constant HCU_LIMIT_STORAGE_LOCATION =
        0xc13af6c514bff8997f30c90003baa82bd02aad978179d1ce58d85c4319ad6500;

    bytes32 mockLHS = bytes32(uint256(int256(-1)));
    bytes32 mockRHS = bytes32(uint256(int256(-2)));
    bytes32 mockMiddle = bytes32(uint256(int256(-3)));
    bytes32 mockResult = bytes32(uint256(int256(-4)));

    function _isTypeSupported(FheType fheType, uint256 supportedTypes) internal pure returns (bool) {
        if ((1 << uint8(fheType)) & supportedTypes == 0) {
            return false;
        } else {
            return true;
        }
    }

    /**
     * @dev Sets up the testing environment by deploying a proxy contract and initializing signers.
     * This function is executed before each test to ensure a consistent and isolated state.
     */
    function setUp() public {
        _deployAndEtchACL();
        // HCULimit accepts settings from hcuConfiguratorAdd only, and HCUConfigurator writes to hcuLimitAdd.
        proxy = hcuLimitAdd;
        _deployEmptyProxyAt(proxy);

        implementation = address(new MockHCULimit());
        vm.startPrank(owner);
        /// @dev It uses UnsafeUpgrades for measuring code coverage.
        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(hcuLimit.initializeFromEmptyProxy, (type(uint48).max, 5_000_000, 20_000_000))
        );
        vm.stopPrank();
        _deployHCUConfigurator(owner);
        hcuConfigurator = HCUConfigurator(hcuConfiguratorAdd);
        hcuLimit = MockHCULimit(proxy);
        fhevmExecutor = hcuLimit.getFHEVMExecutorAddress();
    }

    /**
     * @dev Internal function to deploy and etch ACL contract at expected constant address.
     * Also stores `owner` as ACL's owner, this is needed for ownership of core contracts.
     */
    function _deployAndEtchACL() internal {
        address _acl = address(new ACL());
        bytes memory code = _acl.code;
        vm.etch(aclAdd, code);
        vm.store(
            aclAdd,
            0x9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300, // OwnableStorageLocation
            bytes32(uint256(uint160(owner)))
        );
    }

    /**
     * @dev Tests that the post-upgrade check for the proxy contract works as expected.
     * It checks that the version is correct and the owner is set to the expected address.
     */
    function test_PostProxyUpgradeCheck() public view {
        assertEq(hcuLimit.getVersion(), string(abi.encodePacked("HCULimit v0.6.0")));
        assertEq(hcuLimit.getFHEVMExecutorAddress(), fhevmExecutorAdd);
        assertEq(hcuLimit.getHCUConfiguratorAddress(), hcuConfiguratorAdd);
        assertEq(hcuConfigurator.getVersion(), "HCUConfigurator v0.1.0");
    }

    function test_ownerCannotChangeTheSettingsWithoutHCUConfigurator() public {
        _expectOnlyHCUConfiguratorChangesTheSettings(owner);
    }

    function test_onlyHCUConfiguratorChangesTheSettings(address caller) public {
        vm.assume(caller != hcuConfiguratorAdd);
        _expectOnlyHCUConfiguratorChangesTheSettings(caller);
    }

    function _expectOnlyHCUConfiguratorChangesTheSettings(address caller) internal {
        bytes[6] memory calls = [
            abi.encodeCall(HCULimit.setQuota, (3, 1, 1)),
            abi.encodeCall(HCULimit.assignQuota, (3, address(0xBEEF))),
            abi.encodeCall(HCULimit.setQuotaPaused, (3, true)),
            abi.encodeCall(HCULimit.setQuotaMaxStoredDepth, (3, 1)),
            abi.encodeCall(HCULimit.setTxLimits, (1, 1)),
            abi.encodeCall(HCULimit.setCrossTxDepthLimit, (1, 1))
        ];
        for (uint256 i = 0; i < calls.length; i++) {
            vm.prank(caller);
            (bool ok, bytes memory returned) = address(hcuLimit).call(calls[i]);
            assertFalse(ok);
            assertEq(returned, abi.encodeWithSelector(HCULimit.CallerMustBeHCUConfigurator.selector));
        }
    }

    function test_zeroHandleInputReverts() public {
        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.InvalidZeroHandle.selector);
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x00, bytes32(0), mockRHS, mockResult, fhevmExecutor);
    }

    function test_setHCUForHandleRevertsForZeroHandle() public {
        vm.expectRevert(HCULimit.InvalidZeroHandle.selector);
        hcuLimit.setHCUForHandle(bytes32(0), 1);
    }

    function test_setAndGetHCUForHandleWorksForNonZeroHandle() public {
        bytes32 handle = bytes32(uint256(1));
        uint256 handleHCU = 123;

        hcuLimit.setHCUForHandle(handle, handleHCU);

        assertEq(hcuLimit.getHCUForHandle(handle), handleHCU);
    }

    function test_checkHCUForFheAddWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheAdd));

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheAdd(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);

        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 84000);
            vm.assertLe(totalTransactionHCU, 172000);
        } else {
            vm.assertGe(totalTransactionHCU, 88000);
            vm.assertLe(totalTransactionHCU, 259000);
        }
    }

    function test_PayFheSubWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheSub));

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheSub(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);

        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 84000);
            vm.assertLe(totalTransactionHCU, 172000);
        } else {
            vm.assertGe(totalTransactionHCU, 91000);
            vm.assertLe(totalTransactionHCU, 260000);
        }
    }

    function test_PayFheMulWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMul));

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMul(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 122000);
            vm.assertLe(totalTransactionHCU, 696000);
        } else {
            vm.assertGe(totalTransactionHCU, 150000);
            vm.assertLe(totalTransactionHCU, 1686000);
        }
    }

    function test_checkHCUForFheMulDivRevertsForIllegalScalarByte(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMulDiv));
        vm.assume(scalarByte != FHE_MUL_DIV_FACTOR2_SCALAR && scalarByte != FHE_MUL_DIV_FACTOR2_ENCRYPTED);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimitOperators.OnlyScalarOperationsAreSupported.selector);
        hcuLimit.checkHCUForFheMulDiv(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_PayFheDivWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheDiv));
        bytes1 scalarByte = 0x01;

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheDiv(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 210000);
        vm.assertLe(totalTransactionHCU, 1225000);
    }

    function test_PayFheRemWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRem));
        bytes1 scalarByte = 0x01;

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRem(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 440000);
        vm.assertLe(totalTransactionHCU, 1943000);
    }

    function test_checkHCUForFheBitAndWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitAnd));

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitAnd(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 22000);
        vm.assertLe(totalTransactionHCU, 38000);
    }

    function test_checkHCUForFheBitOrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitOr));

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitOr(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 22000);
        vm.assertLe(totalTransactionHCU, 38000);
    }

    function test_PayFheBitXorWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitXor));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitXor(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 22000);
        vm.assertLe(totalTransactionHCU, 39000);
    }

    function test_checkHCUForFheShlWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShl));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheShl(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 32000);
            vm.assertLe(totalTransactionHCU, 39000);
        } else {
            vm.assertGe(totalTransactionHCU, 92000);
            vm.assertLe(totalTransactionHCU, 378000);
        }
    }

    function test_checkHCUForFheShrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShr));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheShr(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 32000);
            vm.assertLe(totalTransactionHCU, 38000);
        } else {
            vm.assertGe(totalTransactionHCU, 91000);
            vm.assertLe(totalTransactionHCU, 369000);
        }
    }

    function test_checkHCUForFheRotlWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotl));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRotl(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 31000);
            vm.assertLe(totalTransactionHCU, 38000);
        } else {
            vm.assertGe(totalTransactionHCU, 91000);
            vm.assertLe(totalTransactionHCU, 378000);
        }
    }

    function test_checkHCUForFheRotrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotr));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRotr(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 31000);
            vm.assertLe(totalTransactionHCU, 40000);
        } else {
            vm.assertGe(totalTransactionHCU, 91000);
            vm.assertLe(totalTransactionHCU, 378000);
        }
    }

    function test_checkHCUForFheEqWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheEq));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheEq(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 25000);
        vm.assertLe(totalTransactionHCU, 152000);
    }

    function test_checkHCUForFheNeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNe));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNe(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 23000);
        vm.assertLe(totalTransactionHCU, 150000);
    }

    function test_checkHCUForFheGeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGe));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheGe(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 52000);
            vm.assertLe(totalTransactionHCU, 149000);
        } else {
            vm.assertGe(totalTransactionHCU, 63000);
            vm.assertLe(totalTransactionHCU, 210000);
        }
    }

    function test_checkHCUForFheGtWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGt));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheGt(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 52000);
            vm.assertLe(totalTransactionHCU, 150000);
        } else {
            vm.assertGe(totalTransactionHCU, 59000);
            vm.assertLe(totalTransactionHCU, 218000);
        }
    }

    function test_checkHCUForFheLeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLe));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheLe(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 58000);
            vm.assertLe(totalTransactionHCU, 150000);
        } else {
            vm.assertGe(totalTransactionHCU, 58000);
            vm.assertLe(totalTransactionHCU, 218000);
        }
    }

    function test_checkHCUForFheLtWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLt));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheLt(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 52000);
            vm.assertLe(totalTransactionHCU, 149000);
        } else {
            vm.assertGe(totalTransactionHCU, 59000);
            vm.assertLe(totalTransactionHCU, 215000);
        }
    }

    function test_checkHCUForFheMinWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMin));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMin(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 84000);
            vm.assertLe(totalTransactionHCU, 186000);
        } else {
            vm.assertGe(totalTransactionHCU, 119000);
            vm.assertLe(totalTransactionHCU, 289000);
        }
    }

    function test_checkHCUForFheMaxWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMax));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMax(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 89000);
            vm.assertLe(totalTransactionHCU, 180000);
        } else {
            vm.assertGe(totalTransactionHCU, 121000);
            vm.assertLe(totalTransactionHCU, 290000);
        }
    }

    function test_checkHCUForFheNegWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNeg));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNeg(FheType(resultType), mockLHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 79000);
        vm.assertLe(totalTransactionHCU, 269000);
    }

    function test_checkHCUForFheNotWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNot));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNot(FheType(resultType), mockLHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 2);
        vm.assertLe(totalTransactionHCU, 130);
    }

    function test_checkHCUForCastWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesInputCast));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForCast(FheType(resultType), mockLHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertEq(totalTransactionHCU, 32);
    }

    function test_CheckGasLimitForIfThenElseWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheIfThenElse));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForIfThenElse(FheType(resultType), mockLHS, mockMiddle, mockRHS, mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 55000);
        vm.assertLe(totalTransactionHCU, 108000);
    }

    function test_checkHCUForFheRandWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRand));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRand(FheType(resultType), mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 19000);
        vm.assertLe(totalTransactionHCU, 30000);
    }

    function test_checkHCUForFheRandBoundedWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRandBounded));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRandBounded(FheType(resultType), mockResult, fhevmExecutor);
        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 23000);
        vm.assertLe(totalTransactionHCU, 30000);
    }

    function test_ZeroOperandOperationsRevertAboveDepthLimit(
        uint8 operationKind,
        uint8 typeIndex,
        bool unmetered
    ) public {
        (bytes memory callData, uint256 opHCU) = _zeroOperandOperation(operationKind, typeIndex);
        _configureZeroOperandDepthLimit(uint48(opHCU - 1), unmetered);
        _assertZeroOperandDepthRevert(callData);
    }

    function test_ZeroOperandOperationsRevertAtZeroDepthLimit(
        uint8 operationKind,
        uint8 typeIndex,
        bool unmetered
    ) public {
        (bytes memory callData, ) = _zeroOperandOperation(operationKind, typeIndex);
        _configureZeroOperandDepthLimit(0, unmetered);
        _assertZeroOperandDepthRevert(callData);
    }

    function test_ZeroOperandOperationsAcceptAtDepthLimit(uint8 operationKind, uint8 typeIndex, bool unmetered) public {
        (bytes memory callData, uint256 opHCU) = _zeroOperandOperation(operationKind, typeIndex);
        _configureZeroOperandDepthLimit(uint48(opHCU), unmetered);
        _assertZeroOperandSuccess(callData, opHCU, unmetered);
    }

    function test_ZeroOperandOperationsAcceptBelowDepthLimit(
        uint8 operationKind,
        uint8 typeIndex,
        bool unmetered
    ) public {
        (bytes memory callData, uint256 opHCU) = _zeroOperandOperation(operationKind, typeIndex);
        _configureZeroOperandDepthLimit(uint48(opHCU + 1), unmetered);
        _assertZeroOperandSuccess(callData, opHCU, unmetered);
    }

    function test_ZeroOperandOperationsIncludeDepthInDependentCast(uint8 operationKind, bool unmetered) public {
        (bytes memory callData, uint256 opHCU) = _zeroOperandOperation(operationKind, 0);
        _configureZeroOperandDepthLimit(uint48(opHCU + 31), unmetered);
        _assertZeroOperandSuccess(callData, opHCU, unmetered);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionDepthLimitExceeded.selector);
        hcuLimit.checkHCUForCast(FheType.Uint16, mockResult, mockLHS, address(0xBEEF));

        assertEq(hcuLimit.getHCUForTransaction(), opHCU);
        (, , uint48 usedHCU) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(usedHCU, unmetered ? 0 : opHCU);

        vm.prank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(uint48(opHCU + 32));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForCast(FheType.Uint16, mockResult, mockLHS, address(0xBEEF));

        assertEq(hcuLimit.getHCUForHandle(mockLHS), opHCU + 32);
        assertEq(hcuLimit.getHCUForTransaction(), opHCU + 32);
        (, , usedHCU) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(usedHCU, unmetered ? 0 : opHCU + 32);
    }

    function _zeroOperandOperation(uint8 operationKind, uint8 typeIndex) internal view returns (bytes memory, uint256) {
        operationKind %= 3;
        if (operationKind == 0) {
            FheType[8] memory types = [
                FheType.Bool,
                FheType.Uint8,
                FheType.Uint16,
                FheType.Uint32,
                FheType.Uint64,
                FheType.Uint128,
                FheType.Uint160,
                FheType.Uint256
            ];
            return (
                abi.encodeCall(
                    HCULimitOperators.checkHCUForTrivialEncrypt,
                    (types[typeIndex % types.length], mockResult, address(0xBEEF))
                ),
                32
            );
        }

        FheType[7] memory randomTypes = [
            FheType.Bool,
            FheType.Uint8,
            FheType.Uint16,
            FheType.Uint32,
            FheType.Uint64,
            FheType.Uint128,
            FheType.Uint256
        ];
        uint256[7] memory prices = [uint256(19000), 23000, 23000, 24000, 24000, 25000, 30000];
        uint256 index = operationKind == 1
            ? typeIndex % randomTypes.length
            : 1 + (typeIndex % (randomTypes.length - 1));
        bytes memory callData = operationKind == 1
            ? abi.encodeCall(HCULimitOperators.checkHCUForFheRand, (randomTypes[index], mockResult, address(0xBEEF)))
            : abi.encodeCall(
                HCULimitOperators.checkHCUForFheRandBounded,
                (randomTypes[index], mockResult, address(0xBEEF))
            );
        return (callData, prices[index]);
    }

    /// @dev The caller computes in the metered default quota, or, if `unmetered`, in an unmetered quota of its own.
    function _configureZeroOperandDepthLimit(uint48 depthLimit, bool unmetered) internal {
        vm.startPrank(owner);
        hcuConfigurator.setQuota(DEFAULT_QUOTA, 1_000_000, 1_000_000);
        hcuConfigurator.setMaxHCUDepthPerTx(depthLimit);
        if (unmetered) {
            hcuConfigurator.setQuota(PARTNER_QUOTA, type(uint48).max, type(uint48).max);
            hcuConfigurator.assignQuota(PARTNER_QUOTA, address(0xBEEF));
        }
        vm.stopPrank();
    }

    function _assertZeroOperandDepthRevert(bytes memory callData) internal {
        vm.prank(fhevmExecutor);
        (bool success, bytes memory reason) = address(hcuLimit).call(callData);
        assertFalse(success);
        assertEq(reason, abi.encodeWithSelector(HCULimit.HCUTransactionDepthLimitExceeded.selector));
        assertEq(hcuLimit.getHCUForHandle(mockResult), 0);
        assertEq(hcuLimit.getHCUForTransaction(), 0);
        (, , uint48 usedHCU) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(usedHCU, 0);
    }

    function _assertZeroOperandSuccess(bytes memory callData, uint256 opHCU, bool unmetered) internal {
        vm.prank(fhevmExecutor);
        (bool success, ) = address(hcuLimit).call(callData);
        assertTrue(success);
        assertEq(hcuLimit.getHCUForHandle(mockResult), opHCU);
        assertEq(hcuLimit.getHCUForTransaction(), opHCU);
        (, , uint48 usedHCU) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(usedHCU, unmetered ? 0 : opHCU);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheAdd(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheSub(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheSub(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheMul(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheMul(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheDiv(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheDiv(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRem(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheRem(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheBitAnd(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheBitAnd(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheBitOr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheBitOr(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheBitXor(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheBitXor(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheShl(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheShl(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheShr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheShr(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRotl(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheRotl(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRotr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheRotr(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheEq(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheEq(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheNe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheNe(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheGe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheGe(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheGt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheGt(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheLe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheLe(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheLt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheLt(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheMin(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheMin(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheMax(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheMax(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheNeg(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheNeg(FheType.Uint8, mockLHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheNot(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheNot(FheType.Uint8, mockLHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForCast(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForCast(FheType.Uint8, mockLHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallCheckGasLimitForTrivialEncrypt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForTrivialEncrypt(FheType.Uint8, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallCheckGasLimitForIfThenElse(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForIfThenElse(FheType.Uint8, mockLHS, mockMiddle, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRand(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheRand(FheType.Uint8, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRandBounded(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheRandBounded(FheType.Uint8, mockResult, fhevmExecutor);
    }

    /// @dev The cost rows give slots to FheType values below 64 only.
    function test_typesFromSixtyFourHaveNoPrice() public {
        FheType[2] memory types = [FheType(64), FheType.Int248];
        for (uint256 i = 0; i < types.length; i++) {
            vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
            vm.prank(fhevmExecutor);
            hcuLimit.checkHCUForFheAdd(types[i], 0x00, mockLHS, mockRHS, mockResult, fhevmExecutor);
        }
    }

    /// Every metering function charges work and adds latency to the depth. The depth limit sits between the two,
    /// so a function that swapped them would revert or record the wrong values.
    function test_meteringChargesWorkAndAddsLatencyToDepth() public {
        uint256 work = 300_000;
        uint256 latency = 100_000;
        vm.startPrank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(150_000);
        hcuConfigurator.setQuota(DEFAULT_QUOTA, uint48(6 * work), uint48(6 * work));
        vm.stopPrank();
        bytes32[] memory inputs = new bytes32[](3);
        for (uint256 i = 0; i < inputs.length; i++) {
            inputs[i] = bytes32(i + 1);
        }
        for (uint256 hook = 0; hook < 6; hook++) {
            bytes32 result = bytes32(0x100 + hook);
            hcuLimit.setHCUForTransaction(0);
            hcuLimit.meterForTest(hook, (work << 32) | latency, address(0x1234), inputs, result);
            assertEq(hcuLimit.getHCUForTransaction(), work);
            assertEq(hcuLimit.getHCUForHandle(result), latency);
        }
        (, , uint48 used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(used, 6 * work);
    }

    function test_checkHCUForFheAddRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheAdd));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheAdd(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheSubRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheSub));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheSub(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheMulRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMul));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMul(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheDivRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheDiv));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheDiv(FheType(fheType), 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRemRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRem));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRem(FheType(fheType), 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheBitAndRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitAnd));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitAnd(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheBitOrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitOr));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitOr(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheBitXorRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitXor));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitXor(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheShlRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheShl));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheShl(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheShrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheShr));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheShr(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRotlRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRotl));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRotl(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRotrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRotr));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRotr(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheEqRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheEq));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheEq(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheNeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNe));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNe(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheGeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheGe));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheGe(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheGtRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheGt));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheGt(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheLeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheLe));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheLe(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheLtRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheLt));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheLt(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheMinRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMin));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMin(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheMaxRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMax));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMax(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheNegRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNeg));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNeg(FheType(fheType), mockLHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheNotRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNot));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNot(FheType(fheType), mockLHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForCastRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesInputCast));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForCast(FheType(fheType), mockLHS, mockResult, fhevmExecutor);
    }

    function test_CheckGasLimitForIfThenElseRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheIfThenElse));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForIfThenElse(FheType(fheType), mockLHS, mockMiddle, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRandRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRand));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRand(FheType(fheType), mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRandBoundedRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRandBounded));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRandBounded(FheType(fheType), mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheDivRevertsIfNotScalar(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheDiv));
        bytes1 scalarByte = 0x00;
        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimitOperators.OnlyScalarOperationsAreSupported.selector);
        hcuLimit.checkHCUForFheDiv(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRemRevertsIfNotScalar(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRem));
        bytes1 scalarByte = 0x00;
        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimitOperators.OnlyScalarOperationsAreSupported.selector);
        hcuLimit.checkHCUForFheRem(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheAddRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheAdd));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheAdd(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheSubRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheSub));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheSub(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheMulRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMul));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheMul(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheDivRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheDiv));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheDiv(FheType(resultType), 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRemRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRem));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheRem(FheType(resultType), 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheBitAndRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitAnd));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheBitAnd(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheBitOrRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitOr));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheBitOr(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheBitXorRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitXor));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheBitXor(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheShlRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShl));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheShl(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheShrRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShr));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheShr(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRotlRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotl));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheRotl(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRotrRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotr));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheRotr(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheGeRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGe));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheGe(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheGtRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGt));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheGt(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheLeRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLe));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheLe(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheLtRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLt));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheLt(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheMinRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMin));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheMin(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheMaxRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMax));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheMax(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheNegRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNeg));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheNeg(FheType(resultType), mockLHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheNotRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNot));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheNot(FheType(resultType), mockLHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForCastRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesInputCast));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForCast(FheType(resultType), mockLHS, mockResult, fhevmExecutor);
    }

    function test_CheckGasLimitForIfThenElseRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheIfThenElse));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForIfThenElse(FheType(resultType), mockLHS, mockMiddle, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRandRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRand));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheRand(FheType(resultType), mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRandBoundedRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRandBounded));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheRandBounded(FheType(resultType), mockResult, fhevmExecutor);
    }

    /// The per-transaction limits count different units, work and latency, so neither bounds the other.
    function test_maxHCUDepthPerTxAndMaxHCUPerTxAreIndependent() public {
        vm.startPrank(owner);
        hcuConfigurator.setMaxHCUDepthPerTx(20_000_001);
        hcuConfigurator.setMaxHCUPerTx(1_000_000);
        vm.stopPrank();
        assertEq(hcuLimit.getMaxHCUDepthPerTx(), 20_000_001);
        assertEq(hcuLimit.getMaxHCUPerTx(), 1_000_000);
    }

    function test_reinitializeV6SucceedsOnUpgradePath() public {
        address proxyWithoutInitCall = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );

        address implementationWithoutInitCall = address(new MockHCULimit());
        vm.startPrank(owner);
        UnsafeUpgrades.upgradeProxy(proxyWithoutInitCall, implementationWithoutInitCall, "");

        MockHCULimit upgraded = MockHCULimit(proxyWithoutInitCall);
        upgraded.reinitializeV6();
        vm.stopPrank();
    }

    function test_checkHCUForFheIsInWorksAsExpectedForSupportedTypes(uint8 resultType, uint8 count) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheIsIn));
        count = uint8(bound(count, 1, 10));

        bytes32[] memory set = new bytes32[](count);
        for (uint256 i = 0; i < count; i++) {
            set[i] = bytes32(i + 1);
        }

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheIsIn(FheType(resultType), mockLHS, set, mockResult, fhevmExecutor);

        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 60000);
        vm.assertLe(totalTransactionHCU, 1_300_000);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheIsIn(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        bytes32[] memory set = new bytes32[](2);
        set[0] = bytes32(uint256(1));
        set[1] = bytes32(uint256(2));
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheIsIn(FheType.Uint8, mockLHS, set, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheIsInRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheIsIn));
        bytes32[] memory set = new bytes32[](2);
        set[0] = bytes32(uint256(1));
        set[1] = bytes32(uint256(2));
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheIsIn(FheType(fheType), mockLHS, set, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheIsInRevertsIfHCUTransactionIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheIsIn));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        bytes32[] memory set = new bytes32[](2);
        set[0] = bytes32(uint256(1));
        set[1] = bytes32(uint256(2));
        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheIsIn(FheType(resultType), mockLHS, set, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheSumWorksAsExpectedForSupportedTypes(uint8 resultType, uint8 count) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheSum));
        // Bound count in [2, 10] to stay well within per-type size limits and HCU budget.
        // Use bound() instead of assume() to avoid excessive fuzz input rejections.
        count = uint8(bound(count, 2, 10));

        bytes32[] memory values = new bytes32[](count);
        for (uint256 i = 0; i < count; i++) {
            values[i] = bytes32(uint256(i + 1));
        }

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheSum(FheType(resultType), values, mockResult, fhevmExecutor);

        uint256 totalTransactionHCU = hcuLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 88000);
        vm.assertLe(totalTransactionHCU, 259000);
    }

    /// FheSum and FheIsIn record their result at the operation's price above their deepest input.
    function test_arrayOperationsRecordResultDepthAboveDeepestInput() public {
        bytes32[] memory values = new bytes32[](3);
        for (uint256 i = 0; i < values.length; i++) {
            values[i] = bytes32(i + 1);
        }
        hcuLimit.setHCUForHandle(values[1], 7_000);

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheSum(FheType.Uint8, values, mockResult, fhevmExecutor);
        uint256 sumPrice = hcuLimit.getHCUForTransaction();
        assertEq(hcuLimit.getHCUForHandle(mockResult), sumPrice + 7_000);

        hcuLimit.setHCUForHandle(mockLHS, 9_000);
        bytes32 isInResult = bytes32(uint256(0xBEEF));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheIsIn(FheType.Uint8, mockLHS, values, isInResult, fhevmExecutor);
        uint256 isInPrice = hcuLimit.getHCUForTransaction() - sumPrice;
        assertEq(hcuLimit.getHCUForHandle(isInResult), isInPrice + 9_000);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheSum(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        bytes32[] memory values = new bytes32[](2);
        values[0] = mockLHS;
        values[1] = mockRHS;
        vm.prank(randomAccount);
        vm.expectRevert(HCULimitOperators.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheSum(FheType.Uint8, values, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheSumRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheSum));
        bytes32[] memory values = new bytes32[](2);
        values[0] = mockLHS;
        values[1] = mockRHS;
        vm.expectRevert(HCULimitOperators.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheSum(FheType(fheType), values, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheSumRevertsIfHCUTransactionIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheSum));

        hcuLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        bytes32[] memory values = new bytes32[](2);
        values[0] = mockLHS;
        values[1] = mockRHS;
        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.HCUTransactionLimitExceeded.selector);
        hcuLimit.checkHCUForFheSum(FheType(resultType), values, mockResult, fhevmExecutor);
    }

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade(address randomAccount) public {
        vm.assume(randomAccount != owner);
        /// @dev Have to use external call to this to avoid this issue:
        ///      https://github.com/foundry-rs/foundry/issues/5806
        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        this.upgrade(randomAccount);
    }

    /**
     * @dev This function is used to test that only the owner can authorize an upgrade.
     *      It attempts to upgrade the proxy contract to a new implementation using a random account.
     *      The upgrade should fail if the random account is not the owner.
     */
    function upgrade(address randomAccount) external {
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", randomAccount);
    }

    function upgradeHCUConfigurator(address randomAccount) external {
        UnsafeUpgrades.upgradeProxy(hcuConfiguratorAdd, address(new EmptyUUPSProxy()), "", randomAccount);
    }

    function test_OnlyOwnerCanAuthorizeHCUConfiguratorUpgrade(address randomAccount) public {
        vm.assume(randomAccount != owner);
        vm.expectPartialRevert(ACLOwnable.NotHostOwner.selector);
        this.upgradeHCUConfigurator(randomAccount);
        this.upgradeHCUConfigurator(owner);
    }

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade() public {
        /// @dev It does not revert since it called by the owner.
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", owner);
    }

    function test_quotaAllowsUsageUpToItsMaximum() public {
        // Cast costs 32 HCU, so a cap of 64 allows two casts and rejects the third.
        _setQuotaPerBlock(DEFAULT_QUOTA, 64);

        vm.startPrank(fhevmExecutor);
        hcuLimit.checkHCUForCast(FheType.Uint8, mockLHS, bytes32(uint256(0x2001)), address(0x1234));
        hcuLimit.checkHCUForCast(FheType.Uint8, mockLHS, bytes32(uint256(0x2002)), address(0x1234));
        vm.expectRevert(
            abi.encodeWithSelector(HCULimit.HCUQuotaExceeded.selector, address(0x1234), DEFAULT_QUOTA, uint256(1))
        );
        hcuLimit.checkHCUForCast(FheType.Uint8, mockLHS, bytes32(uint256(0x2003)), address(0x1234));
        vm.stopPrank();
    }

    function test_quotaRevertsAboveItsMaximum() public {
        _setQuotaPerBlock(DEFAULT_QUOTA, 100_000);
        _add(address(0x1234), 0x1001);
        _expectQuotaExceeded(address(0x1234), DEFAULT_QUOTA, 1, 0x1002);
    }

    function test_quotaReportsNoRetryWhenOneOperationExceedsTheMaximum() public {
        _setQuota(DEFAULT_QUOTA, 50_000, 50_000);
        _expectQuotaExceeded(address(0x1234), DEFAULT_QUOTA, type(uint256).max, 0x1001);
    }

    function test_quotaPerBlockFreesUpAfterOneBlock() public {
        _setQuotaPerBlock(DEFAULT_QUOTA, 100_000);
        _add(address(0x1234), 0x1001);
        (, , uint48 used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(used, 84_000);

        vm.roll(block.number + 1);
        (, , used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(used, 0);
        _add(address(0x1234), 0x1002);
    }

    function test_quotaUsageDecaysPerBlock() public {
        // A burst up to the maximum, then 50,000 HCU back per block.
        _setQuota(DEFAULT_QUOTA, 200_000, 50_000);
        _add(address(0x1234), 0x1001);
        _add(address(0x1234), 0x1002);
        _expectQuotaExceeded(address(0x1234), DEFAULT_QUOTA, 2, 0x1003);

        vm.roll(block.number + 1);
        _expectQuotaExceeded(address(0x1234), DEFAULT_QUOTA, 1, 0x1004);

        vm.roll(block.number + 1);
        _add(address(0x1234), 0x1005);
        (, , uint48 used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(used, 168_000 - 2 * 50_000 + 84_000);
    }

    function test_contractsAssignedToOneQuotaShareItsUsage() public {
        address first = address(0xA1);
        address second = address(0xA2);
        _setQuotaPerBlock(DEFAULT_QUOTA, 100_000);
        _setQuotaPerBlock(2, 100_000);
        _assign(2, first);
        _assign(2, second);

        _add(first, 0x1001);
        _expectQuotaExceeded(second, 2, 1, 0x1002);

        // The default quota has its own budget.
        _add(address(0x1234), 0x1003);
    }

    function test_unmeteredQuotaRecordsNoUsage() public {
        address account = address(0xBEEF);
        _setQuotaPerBlock(DEFAULT_QUOTA, 100_000);
        _setQuotaPerBlock(1, type(uint48).max);
        _assign(1, account);

        _add(account, 0x1001);
        _add(account, 0x1002);
        (, , uint48 used) = hcuLimit.getQuota(1);
        assertEq(used, 0);
        (, , used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(used, 0);

        // A default-quota caller is still metered.
        _add(address(0xCAFE), 0x1003);
        (, , used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(used, 84_000);
    }

    function test_unmeteredDefaultQuotaRecordsNoUsage() public {
        _add(address(0x1234), 0x1001);
        _add(address(0x1234), 0x1002);
        (uint48 maxHCU, , uint48 used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(maxHCU, type(uint48).max);
        assertEq(used, 0);
    }

    function test_quotaSetToZeroStopsItsContracts() public {
        address account = address(0xBEEF);
        _setQuotaPerBlock(3, 1_000_000);
        _assign(3, account);
        _setQuota(3, 0, 0);

        // The contract stays in its quota, which now refuses every operation.
        assertEq(hcuLimit.getContractQuota(account), 3);
        vm.prank(fhevmExecutor);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, account, uint16(3)));
        hcuLimit.checkHCUForCast(FheType.Uint8, mockLHS, mockResult, account);
    }

    function test_setQuotaByOwner() public {
        _setQuota(7, 25_000_000, 5_000_000);
        (uint48 maxHCU, uint48 decayPerBlock, ) = hcuLimit.getQuota(7);
        assertEq(maxHCU, 25_000_000);
        assertEq(decayPerBlock, 5_000_000);
    }

    function test_setQuotaRefusesOnlyOneOfMaximumAndDecayAtZero() public {
        vm.startPrank(owner);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidQuota.selector, uint16(5)));
        hcuConfigurator.setQuota(5, 100_000, 0);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidQuota.selector, uint16(5)));
        hcuConfigurator.setQuota(5, type(uint48).max, 0);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidQuota.selector, uint16(5)));
        hcuConfigurator.setQuota(5, 0, 100_000);
        hcuConfigurator.setQuota(5, 0, 0);
        vm.stopPrank();
    }

    function test_assignQuotaRefusesAStoppedQuotaButAcceptsTheDefaultQuota() public {
        _setQuotaPerBlock(3, 1_000_000);
        _assign(3, address(0xBEEF));
        _stopDefaultQuota();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidQuota.selector, uint16(7)));
        hcuConfigurator.assignQuota(7, address(0xBEEF));
        _assign(DEFAULT_QUOTA, address(0xBEEF));
    }

    function test_strangerCannotChangeQuotas(address stranger) public {
        vm.assume(stranger != owner && stranger != address(0));
        vm.mockCall(pauserSetAdd, abi.encodeCall(IPauserSet.isPauser, (stranger)), abi.encode(false));

        vm.startPrank(stranger);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.NotPauser.selector, stranger));
        hcuConfigurator.pauseQuota(DEFAULT_QUOTA);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.NotQuotaAdmin.selector, PARTNER_QUOTA, stranger));
        hcuConfigurator.grantQuota(PARTNER_QUOTA, address(0xBEEF));
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.NotQuotaAdmin.selector, PARTNER_QUOTA, stranger));
        hcuConfigurator.revokeQuota(PARTNER_QUOTA, address(0xBEEF));
        vm.stopPrank();
    }

    function test_pausedQuotaRevertsWithQuotaPausedAndKeepsItsSettings() public {
        address account = address(0xBEEF);
        _setQuota(3, 1_000_000, 500_000);
        _assign(3, account);
        _pause(_mockPauser(address(0x9A)), 3);

        _expectQuotaPaused(account, 3);
        assertEq(hcuLimit.getContractQuota(account), 3);
        (uint48 maxHCU, uint48 decayPerBlock, ) = hcuLimit.getQuota(3);
        assertEq(maxHCU, 1_000_000);
        assertEq(decayPerBlock, 500_000);
        assertTrue(hcuLimit.isQuotaPaused(3));
    }

    function test_pausedUnmeteredQuotaReverts() public {
        // The pause is checked before the unmetered return, so quotas with no limit can be stopped too.
        address account = address(0xBEEF);
        _setQuota(1, type(uint48).max, type(uint48).max);
        _assign(1, account);
        _pause(_mockPauser(address(0x9A)), 1);

        _expectQuotaPaused(account, 1);
    }

    function test_unpauseRestoresTheQuota() public {
        address account = address(0xBEEF);
        _setQuotaPerBlock(3, 1_000_000);
        _assign(3, account);
        _pause(_mockPauser(address(0x9A)), 3);

        vm.expectEmit(address(hcuConfigurator));
        emit HCUConfigurator.QuotaPauseSet(3, false, owner);
        vm.prank(owner);
        hcuConfigurator.unpauseQuota(3);

        assertFalse(hcuLimit.isQuotaPaused(3));
        _add(account, 0x1001);
    }

    function test_onlyOwnerUnpauses() public {
        address pauser = _mockPauser(address(0x9A));
        _pause(pauser, 3);
        vm.prank(pauser);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, pauser));
        hcuConfigurator.unpauseQuota(3);
    }

    function test_pauserCannotCallOwnerOnlyQuotaSetters() public {
        address pauser = _mockPauser(address(0x9A));
        vm.startPrank(pauser);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, pauser));
        hcuConfigurator.setQuota(DEFAULT_QUOTA, 0, 0);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, pauser));
        hcuConfigurator.assignQuota(DEFAULT_QUOTA, address(0xBEEF));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, pauser));
        hcuConfigurator.setQuotaAdmin(PARTNER_QUOTA, pauser);
        vm.stopPrank();
    }

    function test_pausingTheDefaultQuotaStopsUnassignedContractsOnly() public {
        address assigned = address(0xA1);
        _setQuotaPerBlock(3, 1_000_000);
        _assign(3, assigned);
        _pause(_mockPauser(address(0x9A)), DEFAULT_QUOTA);

        _expectQuotaPaused(address(0x1234), DEFAULT_QUOTA);
        _add(assigned, 0x1001);
    }

    function test_ownerMovesAContractOutOfAPausedQuota() public {
        address innocent = address(0xBEEF);
        _setQuotaPerBlock(3, 1_000_000);
        _setQuotaPerBlock(4, 1_000_000);
        _assign(3, innocent);
        _pause(_mockPauser(address(0x9A)), 3);

        _assign(4, innocent);
        _add(innocent, 0x1001);
    }

    function test_ownerKeepsOneContractStoppedInAPausedQuotaOfItsOwn() public {
        address abused = address(0xBAD);
        uint16 jail = 7;
        _setUpPartner();
        _setQuotaPerBlock(jail, 1);
        _pause(owner, jail);
        _assign(jail, abused);

        _expectQuotaPaused(abused, jail);
        // No admin can reach it: grants only take contracts from the default quota.
        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.ContractNotInQuota.selector, abused, DEFAULT_QUOTA));
        hcuConfigurator.grantQuota(PARTNER_QUOTA, abused);
    }

    function test_pausedQuotaFreezesItsMembersForItsAdmin() public {
        address auction = address(0xA1);
        _setUpPartner();
        _grant(auction);
        _pause(_mockPauser(address(0x9A)), PARTNER_QUOTA);

        // The admin can neither move its contracts out to escape the pause nor add others to it.
        vm.startPrank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.QuotaMembershipPaused.selector, PARTNER_QUOTA));
        hcuConfigurator.revokeQuota(PARTNER_QUOTA, auction);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.QuotaMembershipPaused.selector, PARTNER_QUOTA));
        hcuConfigurator.grantQuota(PARTNER_QUOTA, address(0xA2));
        vm.stopPrank();
    }

    function test_quotaAdminGrantsAndRevokesContractsOfItsQuota() public {
        address auction = address(0xA1);
        _setUpPartner();

        vm.expectEmit(address(hcuConfigurator));
        emit HCUConfigurator.ContractQuotaAssigned(PARTNER_QUOTA, auction, PARTNER_ADMIN);
        _grant(auction);
        assertEq(hcuLimit.getContractQuota(auction), PARTNER_QUOTA);
        _add(auction, 0x1001);

        vm.expectEmit(address(hcuConfigurator));
        emit HCUConfigurator.ContractQuotaAssigned(DEFAULT_QUOTA, auction, PARTNER_ADMIN);
        vm.prank(PARTNER_ADMIN);
        hcuConfigurator.revokeQuota(PARTNER_QUOTA, auction);
        vm.prank(fhevmExecutor);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, auction, DEFAULT_QUOTA));
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, bytes32(uint256(0x1002)), auction);
    }

    function test_quotaAdminRevokesAContractTheOwnerAssigned() public {
        address wrapper = address(0xB1);
        _setUpPartner();
        _assign(PARTNER_QUOTA, wrapper);

        vm.prank(PARTNER_ADMIN);
        hcuConfigurator.revokeQuota(PARTNER_QUOTA, wrapper);
        assertEq(hcuLimit.getContractQuota(wrapper), DEFAULT_QUOTA);
    }

    function test_quotaAdminCannotActOnAnotherQuota() public {
        _setUpPartner();
        _setQuotaPerBlock(13, 1_000_000);

        vm.startPrank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.NotQuotaAdmin.selector, uint16(13), PARTNER_ADMIN));
        hcuConfigurator.grantQuota(13, address(0xA1));
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.NotQuotaAdmin.selector, uint16(13), PARTNER_ADMIN));
        hcuConfigurator.revokeQuota(13, address(0xA1));
        vm.stopPrank();
    }

    function test_removedAdminLosesItsRightsAndItsContractsStay() public {
        address auction = address(0xA1);
        _setUpPartner();
        _grant(auction);

        _setAdmin(PARTNER_QUOTA, address(0));
        assertEq(hcuConfigurator.getQuotaAdmin(PARTNER_QUOTA), address(0));
        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.NotQuotaAdmin.selector, PARTNER_QUOTA, PARTNER_ADMIN));
        hcuConfigurator.revokeQuota(PARTNER_QUOTA, auction);
        assertEq(hcuLimit.getContractQuota(auction), PARTNER_QUOTA);
    }

    function test_setQuotaAdminRefusesTheDefaultQuota() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidQuota.selector, DEFAULT_QUOTA));
        hcuConfigurator.setQuotaAdmin(DEFAULT_QUOTA, PARTNER_ADMIN);
    }

    function test_grantQuotaRefusesWhileTheDefaultQuotaIsNotStopped() public {
        // setUp leaves the default quota unmetered: moving a contract out of it could leave it with less room.
        _setQuotaPerBlock(PARTNER_QUOTA, 1_000_000);
        _setAdmin(PARTNER_QUOTA, PARTNER_ADMIN);

        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(HCUConfigurator.DefaultQuotaNotStopped.selector);
        hcuConfigurator.grantQuota(PARTNER_QUOTA, address(0xA1));
    }

    function test_grantQuotaRefusesAStoppedQuota() public {
        _setUpPartner();
        _setQuota(PARTNER_QUOTA, 0, 0);

        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidQuota.selector, PARTNER_QUOTA));
        hcuConfigurator.grantQuota(PARTNER_QUOTA, address(0xA1));
    }

    function test_grantQuotaOnlyTakesContractsFromTheDefaultQuota() public {
        address other = address(0xB1);
        _setUpPartner();
        _setQuotaPerBlock(5, 1_000_000);
        _assign(5, other);

        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.ContractNotInQuota.selector, other, DEFAULT_QUOTA));
        hcuConfigurator.grantQuota(PARTNER_QUOTA, other);
    }

    function test_revokeQuotaOnlyReleasesContractsOfItsQuota() public {
        address other = address(0xB1);
        _setUpPartner();
        _setQuotaPerBlock(5, 1_000_000);
        _assign(5, other);

        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.ContractNotInQuota.selector, other, PARTNER_QUOTA));
        hcuConfigurator.revokeQuota(PARTNER_QUOTA, other);
    }

    function test_quotasThatReplacedTheAllowlistTakeNoAdmin() public {
        vm.startPrank(owner);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidQuota.selector, ALLOWLIST_QUOTA));
        hcuConfigurator.setQuotaAdmin(ALLOWLIST_QUOTA, PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCUConfigurator.InvalidQuota.selector, BLOCK_WHITELIST_QUOTA));
        hcuConfigurator.setQuotaAdmin(BLOCK_WHITELIST_QUOTA, PARTNER_ADMIN);
        vm.stopPrank();
    }

    /// RFC 040 relies on a quota's limits, usage, pause flag and stored-depth limit sharing one slot.
    function test_aQuotaFitsInOneSlot() public {
        vm.startPrank(owner);
        hcuConfigurator.setCrossTxDepthLimit(type(uint48).max, 1);
        hcuConfigurator.setQuota(PARTNER_QUOTA, 1_000_000, 1_000_000);
        hcuConfigurator.pauseQuota(PARTNER_QUOTA);
        hcuConfigurator.setQuotaMaxStoredDepth(DEFAULT_QUOTA, type(uint48).max);
        hcuConfigurator.setQuotaMaxStoredDepth(PARTNER_QUOTA, type(uint48).max);
        vm.stopPrank();

        // `quotas` is the sixth slot of HCULimit's namespaced storage.
        bytes32 quotaSlot = keccak256(abi.encode(uint256(PARTNER_QUOTA), uint256(HCU_LIMIT_STORAGE_LOCATION) + 5));
        uint256 word = uint256(vm.load(proxy, quotaSlot));
        assertEq(uint48(word), 1_000_000);
        assertEq(uint8(word >> 192), 1);
        assertEq(word >> 200, type(uint48).max);
        assertEq(vm.load(proxy, bytes32(uint256(quotaSlot) + 1)), bytes32(0));
    }

    /// @dev A quota that allows `hcuPerBlock` HCU per block: maximum and decay per block both `hcuPerBlock`.
    function _setQuotaPerBlock(uint16 quotaId, uint48 hcuPerBlock) internal {
        _setQuota(quotaId, hcuPerBlock, hcuPerBlock);
    }

    function _setQuota(uint16 quotaId, uint48 maxHCU, uint48 decayPerBlock) internal {
        vm.expectEmit(address(hcuConfigurator));
        emit HCUConfigurator.QuotaSet(quotaId, maxHCU, decayPerBlock, owner);
        vm.prank(owner);
        hcuConfigurator.setQuota(quotaId, maxHCU, decayPerBlock);
    }

    /// @dev Expects one 84,000 HCU addition by `caller` to exceed its quota with `retryAfterBlocks`.
    function _expectQuotaExceeded(address caller, uint16 quotaId, uint256 retryAfterBlocks, uint256 resultId) internal {
        vm.prank(fhevmExecutor);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaExceeded.selector, caller, quotaId, retryAfterBlocks));
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, bytes32(resultId), caller);
    }

    function _assign(uint16 quotaId, address account) internal {
        vm.expectEmit(address(hcuConfigurator));
        emit HCUConfigurator.ContractQuotaAssigned(quotaId, account, owner);
        vm.prank(owner);
        hcuConfigurator.assignQuota(quotaId, account);
    }

    function _stopDefaultQuota() internal {
        _setQuota(DEFAULT_QUOTA, 0, 0);
    }

    /// @dev A partner's quota with an admin, and a stopped default quota so the admin can add contracts.
    function _setUpPartner() internal {
        _stopDefaultQuota();
        _setQuotaPerBlock(PARTNER_QUOTA, 1_000_000);
        _setAdmin(PARTNER_QUOTA, PARTNER_ADMIN);
    }

    function _grant(address account) internal {
        vm.prank(PARTNER_ADMIN);
        hcuConfigurator.grantQuota(PARTNER_QUOTA, account);
    }

    function _setAdmin(uint16 quotaId, address admin) internal {
        vm.expectEmit(address(hcuConfigurator));
        emit HCUConfigurator.QuotaAdminSet(quotaId, admin);
        vm.prank(owner);
        hcuConfigurator.setQuotaAdmin(quotaId, admin);
    }

    function _pause(address pauser, uint16 quotaId) internal {
        vm.expectEmit(address(hcuConfigurator));
        emit HCUConfigurator.QuotaPauseSet(quotaId, true, pauser);
        vm.prank(pauser);
        hcuConfigurator.pauseQuota(quotaId);
    }

    /// @dev Expects one 84,000 HCU addition by `caller` to revert because its quota is stopped.
    function _expectQuotaStopped(address caller, uint16 quotaId) internal {
        vm.prank(fhevmExecutor);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, caller, quotaId));
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, bytes32(uint256(0x9002)), caller);
    }

    /// @dev Expects one 84,000 HCU addition by `caller` to revert because its quota is paused.
    function _expectQuotaPaused(address caller, uint16 quotaId) internal {
        vm.prank(fhevmExecutor);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaPaused.selector, caller, quotaId));
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, bytes32(uint256(0x9001)), caller);
    }

    /// @dev One 84,000 HCU addition by `caller`, written to `resultId`.
    function _add(address caller, uint256 resultId) internal {
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, bytes32(resultId), caller);
    }

    function _mockPauser(address pauser) internal returns (address) {
        vm.mockCall(pauserSetAdd, abi.encodeCall(IPauserSet.isPauser, (pauser)), abi.encode(true));
        return pauser;
    }
}
