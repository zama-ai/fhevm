// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import {FheType} from "../../contracts/FheType.sol";
import {FHEGasLimit} from "../../contracts/FHEGasLimit.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {fhevmExecutorAdd} from "../../addresses/FHEVMExecutorAddress.sol";
import {SupportedTypesConstants} from "../fhevmExecutor/fhevmExecutor.t.sol";

contract MockFheGasLimit is FHEGasLimit {
    /// @dev This function allows reading the current block consumption from the storage.
    function getCurrentBlockConsumption() public view returns (uint256) {
        FHEGasLimitStorage storage $ = _getFHEGasLimitStorage();
        return $.currentBlockConsumption;
    }

    /// @dev This function is used for testing purposes to increase the paidAmountGas (for checking revertion paths).
    function updateFunding(uint256 paidAmountGas) public {
        _checkIfNewBlock();
        _updateFunding(paidAmountGas);
    }
}

contract FHEGasLimitTest is Test, SupportedTypesConstants {
    MockFheGasLimit internal fheGasLimit;

    address internal constant owner = address(456);

    address internal proxy;
    address internal implementation;
    address internal fhevmExecutor;

    uint256 internal FHE_GAS_BLOCKLIMIT = 20_000_000;

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
        /// @dev It uses UnsafeUpgrades for measuring code coverage.
        proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, owner)
        );

        implementation = address(new MockFheGasLimit());
        UnsafeUpgrades.upgradeProxy(proxy, implementation, abi.encodeCall(fheGasLimit.reinitialize, ()), owner);
        fheGasLimit = MockFheGasLimit(proxy);
        fhevmExecutor = fheGasLimit.getFHEVMExecutorAddress();
    }

    /**
     * @dev Tests that the post-upgrade check for the proxy contract works as expected.
     * It checks that the version is correct and the owner is set to the expected address.
     */
    function test_PostProxyUpgradeCheck() public view {
        assertEq(fheGasLimit.getVersion(), string(abi.encodePacked("FHEGasLimit v0.1.0")));
        assertEq(fheGasLimit.owner(), owner);
        assertEq(fheGasLimit.getFHEVMExecutorAddress(), fhevmExecutorAdd);
    }

    function test_PayForFheAddWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheAdd));

        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheAdd(FheType(resultType), scalarByte);

        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 94000);
        vm.assertLe(currentBlockConsumption, 218000);
    }

    function test_PayFheSubWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheSub));

        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheSub(FheType(resultType), scalarByte);

        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 94000);
        vm.assertLe(currentBlockConsumption, 218000);
    }

    function test_PayFheMulWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMul));

        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheMul(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();

        if (scalarByte == 0x01) {
            vm.assertGe(currentBlockConsumption, 159000);
            vm.assertLe(currentBlockConsumption, 480000);
        } else {
            vm.assertGe(currentBlockConsumption, 197000);
            vm.assertLe(currentBlockConsumption, 1145000);
        }
    }

    function test_PayFheDivWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheDiv));
        bytes1 scalarByte = 0x01;

        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheDiv(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 238000);
        vm.assertLe(currentBlockConsumption, 857000);
    }

    function test_PayFheRemWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRem));
        bytes1 scalarByte = 0x01;

        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheRem(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 460000);
        vm.assertLe(currentBlockConsumption, 1499000);
    }

    function test_PayForFheBitAndWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitAnd));

        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheBitAnd(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 26000);
        vm.assertLe(currentBlockConsumption, 44000);
    }

    function test_PayForFheBitOrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitOr));

        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheBitOr(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 26000);
        vm.assertLe(currentBlockConsumption, 44000);
    }

    function test_PayFheBitXorWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitXor));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheBitXor(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 26000);
        vm.assertLe(currentBlockConsumption, 44000);
    }

    function test_PayForFheShlWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShl));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheShl(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();

        if (scalarByte == 0x01) {
            vm.assertGe(currentBlockConsumption, 35000);
            vm.assertLe(currentBlockConsumption, 44000);
        } else {
            vm.assertGe(currentBlockConsumption, 133000);
            vm.assertLe(currentBlockConsumption, 350000);
        }
    }

    function test_PayForFheShrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShr));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheShr(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();

        if (scalarByte == 0x01) {
            vm.assertGe(currentBlockConsumption, 35000);
            vm.assertLe(currentBlockConsumption, 44000);
        } else {
            vm.assertGe(currentBlockConsumption, 133000);
            vm.assertLe(currentBlockConsumption, 350000);
        }
    }

    function test_PayForFheRotlWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotl));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheRotl(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();

        if (scalarByte == 0x01) {
            vm.assertGe(currentBlockConsumption, 35000);
            vm.assertLe(currentBlockConsumption, 44000);
        } else {
            vm.assertGe(currentBlockConsumption, 133000);
            vm.assertLe(currentBlockConsumption, 350000);
        }
    }
    function test_PayForFheRotrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotr));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheRotr(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();

        if (scalarByte == 0x01) {
            vm.assertGe(currentBlockConsumption, 35000);
            vm.assertLe(currentBlockConsumption, 44000);
        } else {
            vm.assertGe(currentBlockConsumption, 133000);
            vm.assertLe(currentBlockConsumption, 350000);
        }
    }

    function test_PayForFheEqWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheEq) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheEqWithBytes)
        );
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheEq(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 49000);
        vm.assertLe(currentBlockConsumption, 300000);
    }

    function test_PayForFheNeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheNe) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheNeWithBytes)
        );
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheNe(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 49000);
        vm.assertLe(currentBlockConsumption, 300000);
    }

    function test_PayForFheGeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGe));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheGe(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 82000);
        vm.assertLe(currentBlockConsumption, 190000);
    }

    function test_PayForFheGtWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGt));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheGt(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 82000);
        vm.assertLe(currentBlockConsumption, 190000);
    }

    function test_PayForFheLeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLe));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheLe(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 82000);
        vm.assertLe(currentBlockConsumption, 190000);
    }

    function test_PayForFheLtWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLt));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheLt(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 82000);
        vm.assertLe(currentBlockConsumption, 190000);
    }

    function test_PayForFheMinWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMin));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheMin(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();

        if (scalarByte == 0x01) {
            vm.assertGe(currentBlockConsumption, 128000);
            vm.assertLe(currentBlockConsumption, 225000);
        } else {
            vm.assertGe(currentBlockConsumption, 128000);
            vm.assertLe(currentBlockConsumption, 241000);
        }
    }

    function test_PayForFheMaxWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMax));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheMax(FheType(resultType), scalarByte);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();

        if (scalarByte == 0x01) {
            vm.assertGe(currentBlockConsumption, 128000);
            vm.assertLe(currentBlockConsumption, 225000);
        } else {
            vm.assertGe(currentBlockConsumption, 128000);
            vm.assertLe(currentBlockConsumption, 241000);
        }
    }

    function test_PayForFheNegWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNeg));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheNeg(FheType(resultType));
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 95000);
        vm.assertLe(currentBlockConsumption, 309000);
    }

    function test_PayForFheNotWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNot));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheNot(FheType(resultType));
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 30000);
        vm.assertLe(currentBlockConsumption, 39000);
    }

    function test_PayForCastWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesInputCast));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForCast(FheType(resultType));
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertEq(currentBlockConsumption, 200);
    }

    function test_PayForTrivialEncryptWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesTrivialEncrypt) ||
                _isTypeSupported(FheType(resultType), supportedTypesTrivialEncryptWithBytes)
        );
        vm.prank(fhevmExecutor);
        fheGasLimit.payForTrivialEncrypt(FheType(resultType));
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 100);
        vm.assertLe(currentBlockConsumption, 6400);
    }

    function test_PayForIfThenElseWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheIfThenElse));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForIfThenElse(FheType(resultType));
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 43000);
        vm.assertLe(currentBlockConsumption, 300000);
    }

    function test_PayForFheRandWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRand));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheRand(FheType(resultType));
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertGe(currentBlockConsumption, 100000);
        vm.assertLe(currentBlockConsumption, 400000);
    }

    function test_PayForFheRandBoundedWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRandBounded));
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheRandBounded(FheType(resultType));
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertEq(currentBlockConsumption, 100000);
    }

    function test_OnlyFHEVMExecutorCanCallPayForFheAdd(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheAdd(FheType.Uint8, 0x01);
    }

    function test_OnlyFHEVMExecutorCanCallPayForFheSub(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheSub(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheMul(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheMul(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheDiv(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheDiv(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheRem(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheRem(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheBitAnd(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheBitAnd(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheBitOr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheBitOr(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheBitXor(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheBitXor(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheShl(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheShl(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheShr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheShr(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheRotl(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheRotl(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheRotr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheRotr(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheEq(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheEq(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheNe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheNe(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheGe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheGe(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheGt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheGt(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheLe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheLe(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheLt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheLt(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheMin(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheMin(FheType.Uint8, 0x01);
    }

    function test_OnlyFHEVMExecutorCanCallPayForFheMax(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheMax(FheType.Uint8, 0x01);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheNeg(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheNeg(FheType.Uint8);
    }

    function test_OnlyFHEVMExecutorCanCallPayForFheNot(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheNot(FheType.Uint8);
    }

    function test_OnlyFHEVMExecutorCanCallPayForCast(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForCast(FheType.Uint8);
    }

    function test_OnlyFHEVMExecutorCanCallPayForTrivialEncrypt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForTrivialEncrypt(FheType.Uint8);
    }

    function test_OnlyFHEVMExecutorCanCallPayForIfThenElse(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForIfThenElse(FheType.Uint8);
    }
    function test_OnlyFHEVMExecutorCanCallPayForFheRand(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheRand(FheType.Uint8);
    }

    function test_OnlyFHEVMExecutorCanCallPayForFheRandBounded(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.payForFheRandBounded(FheType.Uint8);
    }

    function test_PayForFheAddRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheAdd));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheAdd(FheType(fheType), scalarByte);
    }

    function test_PayForFheSubRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheSub));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheSub(FheType(fheType), scalarByte);
    }

    function test_PayForFheMulRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMul));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheMul(FheType(fheType), scalarByte);
    }

    function test_PayForFheDivRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheDiv));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheDiv(FheType(fheType), 0x01);
    }
    function test_PayForFheRemRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRem));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheRem(FheType(fheType), 0x01);
    }

    function test_PayForFheBitAndRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitAnd));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheBitAnd(FheType(fheType), scalarByte);
    }

    function test_PayForFheBitOrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitOr));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheBitOr(FheType(fheType), scalarByte);
    }

    function test_PayForFheBitXorRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitXor));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheBitXor(FheType(fheType), scalarByte);
    }

    function test_PayForFheShlRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheShl));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheShl(FheType(fheType), scalarByte);
    }

    function test_PayForFheShrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheShr));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheShr(FheType(fheType), scalarByte);
    }

    function test_PayForFheRotlRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRotl));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheRotl(FheType(fheType), scalarByte);
    }

    function test_PayForFheRotrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRotr));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheRotr(FheType(fheType), scalarByte);
    }

    function test_PayForFheEqRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheEq));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheEq(FheType(fheType), scalarByte);
    }

    function test_PayForFheNeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNe));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheNe(FheType(fheType), scalarByte);
    }

    function test_PayForFheGeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheGe));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheGe(FheType(fheType), scalarByte);
    }

    function test_PayForFheGtRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheGt));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheGt(FheType(fheType), scalarByte);
    }

    function test_PayForFheLeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheLe));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheLe(FheType(fheType), scalarByte);
    }

    function test_PayForFheLtRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheLt));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheLt(FheType(fheType), scalarByte);
    }

    function test_PayForFheMinRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMin));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheMin(FheType(fheType), scalarByte);
    }

    function test_PayForFheMaxRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMax));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheMax(FheType(fheType), scalarByte);
    }

    function test_PayForFheNegRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNeg));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheNeg(FheType(fheType));
    }

    function test_PayForFheNotRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNot));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheNot(FheType(fheType));
    }

    function test_PayForCastRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesInputCast));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForCast(FheType(fheType));
    }

    function test_PayForTrivialEncryptRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(
            !_isTypeSupported(FheType(fheType), supportedTypesTrivialEncrypt) &&
                !_isTypeSupported(FheType(fheType), supportedTypesTrivialEncryptWithBytes)
        );
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForTrivialEncrypt(FheType(fheType));
    }

    function test_PayForIfThenElseRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheIfThenElse));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForIfThenElse(FheType(fheType));
    }
    function test_PayForFheRandRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRand));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheRand(FheType(fheType));
    }

    function test_PayForFheRandBoundedRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRandBounded));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.payForFheRandBounded(FheType(fheType));
    }

    function test_PayForFheDivRevertsIfNotScalar(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheDiv));
        bytes1 scalarByte = 0x00;
        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.OnlyScalarOperationsAreSupported.selector);
        fheGasLimit.payForFheDiv(FheType(resultType), scalarByte);
    }

    function test_PayForFheRemRevertsIfNotScalar(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRem));
        bytes1 scalarByte = 0x00;
        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.OnlyScalarOperationsAreSupported.selector);
        fheGasLimit.payForFheRem(FheType(resultType), scalarByte);
    }

    function test_PayForFheAddRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheAdd));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheAdd(FheType(resultType), scalarType);
    }

    function test_PayForFheSubRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheSub));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheSub(FheType(resultType), scalarType);
    }

    function test_PayForFheMulRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMul));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheMul(FheType(resultType), scalarType);
    }

    function test_PayForFheDivRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheDiv));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheDiv(FheType(resultType), 0x01);
    }

    function test_PayForFheRemRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRem));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheRem(FheType(resultType), 0x01);
    }

    function test_PayForFheBitAndRevertsIfFheGasBlockLimitIsAboveBlockLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitAnd));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheBitAnd(FheType(resultType), scalarType);
    }

    function test_PayForFheBitOrRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitOr));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheBitOr(FheType(resultType), scalarType);
    }

    function test_PayForFheBitXorRevertsIfFheGasBlockLimitIsAboveBlockLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitXor));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheBitXor(FheType(resultType), scalarType);
    }

    function test_PayForFheShlRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShl));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheShl(FheType(resultType), scalarType);
    }

    function test_PayForFheShrRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShr));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheShr(FheType(resultType), scalarType);
    }

    function test_PayForFheRotlRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotl));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheRotl(FheType(resultType), scalarType);
    }

    function test_PayForFheRotrRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotr));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheRotr(FheType(resultType), scalarType);
    }

    function test_PayForFheEqRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheEq) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheEqWithBytes)
        );

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheEq(FheType(resultType), scalarType);
    }

    function test_PayForFheNeRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheNe) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheNeWithBytes)
        );

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheNe(FheType(resultType), scalarType);
    }

    function test_PayForFheGeRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGe));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheGe(FheType(resultType), scalarType);
    }

    function test_PayForFheGtRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGt));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheGt(FheType(resultType), scalarType);
    }

    function test_PayForFheLeRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLe));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheLe(FheType(resultType), scalarType);
    }

    function test_PayForFheLtRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLt));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheLt(FheType(resultType), scalarType);
    }

    function test_PayForFheMinRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMin));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheMin(FheType(resultType), scalarType);
    }

    function test_PayForFheMaxRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType, bytes1 scalarType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMax));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheMax(FheType(resultType), scalarType);
    }

    function test_PayForFheNegRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNeg));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheNeg(FheType(resultType));
    }

    function test_PayForFheNotRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNot));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheNot(FheType(resultType));
    }

    function test_PayForCastRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesInputCast));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForCast(FheType(resultType));
    }

    function test_PayForTrivialEncryptRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesTrivialEncrypt) ||
                _isTypeSupported(FheType(resultType), supportedTypesTrivialEncryptWithBytes)
        );

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForTrivialEncrypt(FheType(resultType));
    }

    function test_PayForIfThenElseRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheIfThenElse));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForIfThenElse(FheType(resultType));
    }
    function test_PayForFheRandRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRand));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheRand(FheType(resultType));
    }

    function test_PayForFheRandBoundedRevertsIfFheGasBlockLimitIsAboveBlockLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRandBounded));

        fheGasLimit.updateFunding(FHE_GAS_BLOCKLIMIT);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.FHEGasBlockLimitExceeded.selector);
        fheGasLimit.payForFheRandBounded(FheType(resultType));
    }

    function test_CurrentBlockConsumptionRestartsWhenNewBlock() public {
        vm.startPrank(fhevmExecutor);
        fheGasLimit.payForFheAdd(FheType.Uint16, 0x01);
        uint256 currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertEq(currentBlockConsumption, 133000);

        /// @dev In the same block, it should be 2x.
        fheGasLimit.payForFheAdd(FheType.Uint16, 0x01);
        currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertEq(currentBlockConsumption, 133000 * 2);

        // It should reset, so it should be 1x.
        vm.roll(block.number + 1);
        fheGasLimit.payForFheAdd(FheType.Uint16, 0x01);
        currentBlockConsumption = fheGasLimit.getCurrentBlockConsumption();
        vm.assertEq(currentBlockConsumption, 133000);

        vm.stopPrank();
    }

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade(address randomAccount) public {
        vm.assume(randomAccount != owner);
        /// @dev Have to use external call to this to avoid this issue:
        ///      https://github.com/foundry-rs/foundry/issues/5806
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
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

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade() public {
        /// @dev It does not revert since it called by the owner.
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", owner);
    }
}
