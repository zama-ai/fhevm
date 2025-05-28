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
    bytes32 private constant FHEGasLimitStorageLocation =
        0xb5c80b3bbe0bcbcea690f6dbe62b32a45bd1ad263b78db2f25ef8414efe9bc00;

    function getHCUForTransaction() external view returns (uint256) {
        return _getHCUForTransaction();
    }

    function setHCUForTransaction(uint256 handleHCU) external {
        _setHCUForTransaction(handleHCU);
    }

    function resetTotalTransactionHCU() external {
        bytes32 slot = keccak256(abi.encodePacked(FHEGasLimitStorageLocation, "HCU"));

        assembly {
            tstore(slot, 0)
        }
    }
}

contract FHEGasLimitTest is Test, SupportedTypesConstants {
    MockFheGasLimit internal fheGasLimit;

    address internal constant owner = address(456);

    address internal proxy;
    address internal implementation;
    address internal fhevmExecutor;

    uint256 internal MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX = 20_000_000 - 1;

    bytes32 mockLHS;
    bytes32 mockRHS;
    bytes32 mockMiddle;
    bytes32 mockResult;

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

    function test_checkHCUForFheAddWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheAdd));

        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheAdd(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);

        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 94000);
        vm.assertLe(totalTransactionHCU, 218000);
    }

    function test_PayFheSubWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheSub));

        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheSub(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);

        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 94000);
        vm.assertLe(totalTransactionHCU, 218000);
    }

    function test_PayFheMulWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMul));

        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheMul(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 159000);
            vm.assertLe(totalTransactionHCU, 480000);
        } else {
            vm.assertGe(totalTransactionHCU, 197000);
            vm.assertLe(totalTransactionHCU, 1145000);
        }
    }

    function test_PayFheDivWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheDiv));
        bytes1 scalarByte = 0x01;

        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheDiv(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 238000);
        vm.assertLe(totalTransactionHCU, 857000);
    }

    function test_PayFheRemWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRem));
        bytes1 scalarByte = 0x01;

        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheRem(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 460000);
        vm.assertLe(totalTransactionHCU, 1499000);
    }

    function test_checkHCUForFheBitAndWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitAnd));

        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheBitAnd(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 26000);
        vm.assertLe(totalTransactionHCU, 44000);
    }

    function test_checkHCUForFheBitOrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitOr));

        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheBitOr(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 26000);
        vm.assertLe(totalTransactionHCU, 44000);
    }

    function test_PayFheBitXorWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitXor));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheBitXor(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 26000);
        vm.assertLe(totalTransactionHCU, 44000);
    }

    function test_checkHCUForFheShlWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShl));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheShl(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 35000);
            vm.assertLe(totalTransactionHCU, 44000);
        } else {
            vm.assertGe(totalTransactionHCU, 133000);
            vm.assertLe(totalTransactionHCU, 350000);
        }
    }

    function test_checkHCUForFheShrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShr));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheShr(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 35000);
            vm.assertLe(totalTransactionHCU, 44000);
        } else {
            vm.assertGe(totalTransactionHCU, 133000);
            vm.assertLe(totalTransactionHCU, 350000);
        }
    }

    function test_checkHCUForFheRotlWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotl));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheRotl(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 35000);
            vm.assertLe(totalTransactionHCU, 44000);
        } else {
            vm.assertGe(totalTransactionHCU, 133000);
            vm.assertLe(totalTransactionHCU, 350000);
        }
    }

    function test_checkHCUForFheRotrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotr));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheRotr(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 35000);
            vm.assertLe(totalTransactionHCU, 44000);
        } else {
            vm.assertGe(totalTransactionHCU, 133000);
            vm.assertLe(totalTransactionHCU, 350000);
        }
    }

    function test_checkHCUForFheEqWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheEq) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheEqWithBytes)
        );
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheEq(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 49000);
        vm.assertLe(totalTransactionHCU, 300000);
    }

    function test_checkHCUForFheEqBytesWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheEq) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheEqWithBytes)
        );
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheEqBytes(FheType(resultType), scalarByte, mockLHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 49000);
        vm.assertLe(totalTransactionHCU, 300000);
    }

    function test_checkHCUForFheNeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheNe) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheNeWithBytes)
        );
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheNe(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 49000);
        vm.assertLe(totalTransactionHCU, 300000);
    }

    function test_checkHCUForFheNeBytesWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheNe) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheNeWithBytes)
        );
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheNeBytes(FheType(resultType), scalarByte, mockLHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 49000);
        vm.assertLe(totalTransactionHCU, 300000);
    }

    function test_checkHCUForFheGeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGe));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheGe(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 82000);
        vm.assertLe(totalTransactionHCU, 190000);
    }

    function test_checkHCUForFheGtWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGt));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheGt(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 82000);
        vm.assertLe(totalTransactionHCU, 190000);
    }

    function test_checkHCUForFheLeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLe));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheLe(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 82000);
        vm.assertLe(totalTransactionHCU, 190000);
    }

    function test_checkHCUForFheLtWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLt));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheLt(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 82000);
        vm.assertLe(totalTransactionHCU, 190000);
    }

    function test_checkHCUForFheMinWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMin));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheMin(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 128000);
            vm.assertLe(totalTransactionHCU, 225000);
        } else {
            vm.assertGe(totalTransactionHCU, 128000);
            vm.assertLe(totalTransactionHCU, 241000);
        }
    }

    function test_checkHCUForFheMaxWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMax));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheMax(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();

        if (scalarByte == 0x01) {
            vm.assertGe(totalTransactionHCU, 128000);
            vm.assertLe(totalTransactionHCU, 225000);
        } else {
            vm.assertGe(totalTransactionHCU, 128000);
            vm.assertLe(totalTransactionHCU, 241000);
        }
    }

    function test_checkHCUForFheNegWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNeg));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheNeg(FheType(resultType), mockLHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 95000);
        vm.assertLe(totalTransactionHCU, 309000);
    }

    function test_checkHCUForFheNotWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNot));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheNot(FheType(resultType), mockLHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 30000);
        vm.assertLe(totalTransactionHCU, 39000);
    }

    function test_checkHCUForCastWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesInputCast));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForCast(FheType(resultType), mockLHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertEq(totalTransactionHCU, 200);
    }

    function test_CheckGasLimitForTrivialEncryptWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesTrivialEncrypt) ||
                _isTypeSupported(FheType(resultType), supportedTypesTrivialEncryptWithBytes)
        );
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForTrivialEncrypt(FheType(resultType), mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 100);
        vm.assertLe(totalTransactionHCU, 6400);
    }

    function test_CheckGasLimitForIfThenElseWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheIfThenElse));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForIfThenElse(FheType(resultType), mockLHS, mockMiddle, mockRHS, mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 43000);
        vm.assertLe(totalTransactionHCU, 300000);
    }

    function test_checkHCUForFheRandWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRand));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheRand(FheType(resultType), mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertGe(totalTransactionHCU, 100000);
        vm.assertLe(totalTransactionHCU, 400000);
    }

    function test_checkHCUForFheRandBoundedWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRandBounded));
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheRandBounded(FheType(resultType), mockResult);
        uint256 totalTransactionHCU = fheGasLimit.getHCUForTransaction();
        vm.assertEq(totalTransactionHCU, 100000);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheAdd(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheSub(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheSub(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheMul(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheMul(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheDiv(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheDiv(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRem(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheRem(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheBitAnd(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheBitAnd(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheBitOr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheBitOr(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheBitXor(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheBitXor(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheShl(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheShl(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheShr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheShr(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRotl(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheRotl(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRotr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheRotr(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheEq(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheEq(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheEqBytes(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheEqBytes(FheType.Uint8, 0x01, mockLHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheNe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheNe(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheNeBytes(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheNeBytes(FheType.Uint8, 0x01, mockLHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheGe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheGe(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheGt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheGt(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheLe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheLe(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheLt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheLt(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheMin(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheMin(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheMax(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheMax(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheNeg(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheNeg(FheType.Uint8, mockLHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheNot(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheNot(FheType.Uint8, mockLHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForCast(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForCast(FheType.Uint8, mockLHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallCheckGasLimitForTrivialEncrypt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForTrivialEncrypt(FheType.Uint8, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallCheckGasLimitForIfThenElse(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForIfThenElse(FheType.Uint8, mockLHS, mockMiddle, mockRHS, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRand(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheRand(FheType.Uint8, mockResult);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRandBounded(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(FHEGasLimit.CallerMustBeFHEVMExecutorContract.selector);
        fheGasLimit.checkHCUForFheRandBounded(FheType.Uint8, mockResult);
    }

    function test_checkHCUForFheAddRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheAdd));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheAdd(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheSubRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheSub));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheSub(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheMulRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMul));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheMul(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheDivRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheDiv));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheDiv(FheType(fheType), 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheRemRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRem));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheRem(FheType(fheType), 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheBitAndRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitAnd));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheBitAnd(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheBitOrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitOr));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheBitOr(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheBitXorRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitXor));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheBitXor(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheShlRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheShl));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheShl(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheShrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheShr));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheShr(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheRotlRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRotl));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheRotl(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheRotrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRotr));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheRotr(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheEqRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheEq));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheEq(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheEqBytesRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheEq));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheEqBytes(FheType(fheType), scalarByte, mockLHS, mockResult);
    }

    function test_checkHCUForFheNeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNe));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheNe(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheNeBytesRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNe));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheNeBytes(FheType(fheType), scalarByte, mockLHS, mockResult);
    }

    function test_checkHCUForFheGeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheGe));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheGe(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheGtRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheGt));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheGt(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheLeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheLe));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheLe(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheLtRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheLt));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheLt(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheMinRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMin));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheMin(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheMaxRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMax));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheMax(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheNegRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNeg));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheNeg(FheType(fheType), mockLHS, mockResult);
    }

    function test_checkHCUForFheNotRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNot));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheNot(FheType(fheType), mockLHS, mockResult);
    }

    function test_checkHCUForCastRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesInputCast));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForCast(FheType(fheType), mockLHS, mockResult);
    }

    function test_CheckGasLimitForTrivialEncryptRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(
            !_isTypeSupported(FheType(fheType), supportedTypesTrivialEncrypt) &&
                !_isTypeSupported(FheType(fheType), supportedTypesTrivialEncryptWithBytes)
        );
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForTrivialEncrypt(FheType(fheType), mockResult);
    }

    function test_CheckGasLimitForIfThenElseRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheIfThenElse));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForIfThenElse(FheType(fheType), mockLHS, mockMiddle, mockRHS, mockResult);
    }

    function test_checkHCUForFheRandRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRand));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheRand(FheType(fheType), mockResult);
    }

    function test_checkHCUForFheRandBoundedRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRandBounded));
        vm.expectRevert(FHEGasLimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        fheGasLimit.checkHCUForFheRandBounded(FheType(fheType), mockResult);
    }

    function test_checkHCUForFheDivRevertsIfNotScalar(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheDiv));
        bytes1 scalarByte = 0x00;
        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.OnlyScalarOperationsAreSupported.selector);
        fheGasLimit.checkHCUForFheDiv(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheRemRevertsIfNotScalar(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRem));
        bytes1 scalarByte = 0x00;
        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.OnlyScalarOperationsAreSupported.selector);
        fheGasLimit.checkHCUForFheRem(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheAddRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheAdd));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheAdd(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheSubRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheSub));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheSub(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheMulRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMul));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheMul(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheDivRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheDiv));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheDiv(FheType(resultType), 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheRemRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRem));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheRem(FheType(resultType), 0x01, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheBitAndRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitAnd));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheBitAnd(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheBitOrRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitOr));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheBitOr(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheBitXorRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitXor));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheBitXor(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheShlRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShl));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheShl(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheShrRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShr));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheShr(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheRotlRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotl));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheRotl(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheRotrRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotr));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheRotr(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheEqRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheEq) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheEqWithBytes)
        );

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheEq(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheEqBytesRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheEq) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheEqWithBytes)
        );

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheEqBytes(FheType(resultType), scalarType, mockLHS, mockResult);
    }

    function test_checkHCUForFheNeRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheNe) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheNeWithBytes)
        );

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheNe(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheNeBytesRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesFheNe) ||
                _isTypeSupported(FheType(resultType), supportedTypesFheNeWithBytes)
        );

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheNeBytes(FheType(resultType), scalarType, mockLHS, mockResult);
    }

    function test_checkHCUForFheGeRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGe));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheGe(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheGtRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGt));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheGt(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheLeRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLe));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheLe(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheLtRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLt));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheLt(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheMinRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMin));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheMin(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheMaxRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType,
        bytes1 scalarType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMax));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheMax(FheType(resultType), scalarType, mockLHS, mockRHS, mockResult);
    }

    function test_checkHCUForFheNegRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNeg));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheNeg(FheType(resultType), mockLHS, mockResult);
    }

    function test_checkHCUForFheNotRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNot));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheNot(FheType(resultType), mockLHS, mockResult);
    }

    function test_checkHCUForCastRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesInputCast));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForCast(FheType(resultType), mockLHS, mockResult);
    }

    function test_CheckGasLimitForTrivialEncryptRevertsIfHCUTransationIsAboveHCUTransactionLimit(
        uint8 resultType
    ) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(resultType), supportedTypesTrivialEncrypt) ||
                _isTypeSupported(FheType(resultType), supportedTypesTrivialEncryptWithBytes)
        );

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForTrivialEncrypt(FheType(resultType), mockResult);
    }

    function test_CheckGasLimitForIfThenElseRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheIfThenElse));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForIfThenElse(FheType(resultType), mockLHS, mockMiddle, mockRHS, mockResult);
    }

    function test_checkHCUForFheRandRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRand));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheRand(FheType(resultType), mockResult);
    }

    function test_checkHCUForFheRandBoundedRevertsIfHCUTransationIsAboveHCUTransactionLimit(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRandBounded));

        fheGasLimit.setHCUForTransaction(MAX_HOMOMORPHIC_COMPUTE_UNITS_PER_TX);

        vm.prank(fhevmExecutor);
        vm.expectRevert(FHEGasLimit.HCUTransactionLimitExceeded.selector);
        fheGasLimit.checkHCUForFheRandBounded(FheType(resultType), mockResult);
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
