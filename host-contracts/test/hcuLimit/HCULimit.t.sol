// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import {FheType} from "../../contracts/shared/FheType.sol";
import {HCULimit} from "../../contracts/HCULimit.sol";
import {ACL} from "../../contracts/ACL.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {fhevmExecutorAdd} from "../../addresses/FHEVMHostAddresses.sol";
import {SupportedTypesConstants} from "../fhevmExecutor/fhevmExecutor.t.sol";
import {ACLOwnable} from "../../contracts/shared/ACLOwnable.sol";
import {IPauserSet} from "../../contracts/interfaces/IPauserSet.sol";
import {aclAdd, pauserSetAdd} from "../../addresses/FHEVMHostAddresses.sol";

contract MockHCULimit is HCULimit {
    // The cost table's rows, by the operation and branch that read them.
    uint256 public constant ROW_ADD_SUB_SCALAR = COSTS_ADD_SUB_SCALAR;
    uint256 public constant ROW_ADD = COSTS_ADD;
    uint256 public constant ROW_SUB = COSTS_SUB;
    uint256 public constant ROW_MUL_SCALAR = COSTS_MUL_SCALAR;
    uint256 public constant ROW_MUL = COSTS_MUL;
    uint256 public constant ROW_DIV_SCALAR = COSTS_DIV_SCALAR;
    uint256 public constant ROW_REM_SCALAR = COSTS_REM_SCALAR;
    uint256 public constant ROW_BIT_AND_SCALAR = COSTS_BIT_AND_SCALAR;
    uint256 public constant ROW_BIT_AND = COSTS_BIT_AND;
    uint256 public constant ROW_BIT_OR_SCALAR = COSTS_BIT_OR_SCALAR;
    uint256 public constant ROW_BIT_OR = COSTS_BIT_OR;
    uint256 public constant ROW_BIT_XOR_SCALAR = COSTS_BIT_XOR_SCALAR;
    uint256 public constant ROW_BIT_XOR = COSTS_BIT_XOR;
    uint256 public constant ROW_SHL_SCALAR = COSTS_SHL_SCALAR;
    uint256 public constant ROW_SHL = COSTS_SHL;
    uint256 public constant ROW_SHR_SCALAR = COSTS_SHR_SCALAR;
    uint256 public constant ROW_SHR = COSTS_SHR;
    uint256 public constant ROW_ROTL_SCALAR = COSTS_ROTL_SCALAR;
    uint256 public constant ROW_ROTL = COSTS_ROTL;
    uint256 public constant ROW_ROTR_SCALAR = COSTS_ROTR_SCALAR;
    uint256 public constant ROW_ROTR = COSTS_ROTR;
    uint256 public constant ROW_EQ_SCALAR = COSTS_EQ_SCALAR;
    uint256 public constant ROW_EQ = COSTS_EQ;
    uint256 public constant ROW_NE_SCALAR = COSTS_NE_SCALAR;
    uint256 public constant ROW_NE = COSTS_NE;
    uint256 public constant ROW_GE_SCALAR = COSTS_GE_SCALAR;
    uint256 public constant ROW_GE = COSTS_GE;
    uint256 public constant ROW_GT_SCALAR = COSTS_GT_SCALAR;
    uint256 public constant ROW_GT = COSTS_GT;
    uint256 public constant ROW_LE_SCALAR = COSTS_LE_SCALAR;
    uint256 public constant ROW_LE = COSTS_LE;
    uint256 public constant ROW_LT_SCALAR = COSTS_LT_SCALAR;
    uint256 public constant ROW_LT = COSTS_LT;
    uint256 public constant ROW_MIN_SCALAR = COSTS_MIN_SCALAR;
    uint256 public constant ROW_MIN = COSTS_MIN;
    uint256 public constant ROW_MAX_SCALAR = COSTS_MAX_SCALAR;
    uint256 public constant ROW_MAX = COSTS_MAX;
    uint256 public constant ROW_NEG = COSTS_NEG;
    uint256 public constant ROW_IF_THEN_ELSE = COSTS_IF_THEN_ELSE;
    uint256 public constant ROW_RAND = COSTS_RAND;
    uint256 public constant ROW_RAND_BOUNDED = COSTS_RAND_BOUNDED;
    uint256 public constant ROW_MUL_DIV_SCALAR = COSTS_MUL_DIV_SCALAR;
    uint256 public constant ROW_MUL_DIV = COSTS_MUL_DIV;

    /// @dev The latency and work of `fheType` in `row`, decoded here rather than by the metering path, so a test
    ///      catches a metering function that reads the wrong column.
    function cost(uint256 row, FheType fheType) external pure returns (uint256 latencyHCU, uint256 workHCU) {
        uint256 opCost = _cost(row, fheType);
        return (uint32(opCost), (opCost >> 32) * 1000);
    }

    /// @dev The depth of `handle` within the current transaction, in latency HCU.
    function getTransactionDepth(bytes32 handle) external view returns (uint256 depth) {
        assembly {
            depth := tload(handle)
        }
        return uint128(depth);
    }

    function getHCUForTransaction() external view returns (uint256) {
        return _getHCUForTransaction();
    }

    function setHCUForTransaction(uint256 handleHCU) external {
        _setHCUForTransaction(handleHCU);
    }

    function setHCUForHandle(bytes32 handle, uint256 handleHCU) external {
        _setHCUForHandle(handle, handleHCU);
    }

    function setLegacyGlobalCapForTest(uint48 cap) external {
        _getHCULimitStorage().globalHCUCapPerBlock = cap;
    }

    function setAllowlistActivatedForTest(bool activated) external {
        _getHCULimitStorage().isAllowlistActivated = activated;
    }

    function setAllowlistedForTest(address account) external {
        _getHCULimitStorage().isAllowlisted[account] = true;
    }

    function setBlockHCUWhitelistedForTest(address account) external {
        _getHCULimitStorage().blockHCUWhitelist[account] = true;
    }
}

contract HCULimitTest is Test, SupportedTypesConstants {
    MockHCULimit internal hcuLimit;

    address internal constant owner = address(456);

    address internal proxy;
    address internal implementation;
    address internal fhevmExecutor;
    /// @dev What `_add` charges to quotas and the capacity: the work of a scalar 8-bit add, from the cost table.
    ///      Work is stored in thousands, so halves, quarters and eighths of it are exact.
    uint48 internal addWork;

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
    address internal constant ALLOWED = address(0xC1);
    address internal constant ALLOWED_BLOCK_WHITELISTED = address(0xC2);
    address internal constant BLOCK_WHITELISTED_ONLY = address(0xC3);
    address internal constant UNKNOWN = address(0xC4);

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
        /// @dev It uses UnsafeUpgrades for measuring code coverage.
        proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );

        implementation = address(new MockHCULimit());
        vm.startPrank(owner);
        UnsafeUpgrades.upgradeProxy(
            proxy,
            implementation,
            abi.encodeCall(hcuLimit.initializeFromEmptyProxy, (type(uint48).max, 5_000_000, 20_000_000))
        );
        vm.stopPrank();
        hcuLimit = MockHCULimit(proxy);
        fhevmExecutor = hcuLimit.getFHEVMExecutorAddress();
        (, uint256 work) = hcuLimit.cost(hcuLimit.ROW_ADD_SUB_SCALAR(), FheType.Uint8);
        addWork = uint48(work);
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
    }

    function test_setHCUForHandleRevertsForZeroHandle() public {
        vm.expectRevert(HCULimit.InvalidZeroHandle.selector);
        hcuLimit.setHCUForHandle(bytes32(0), 1);
    }

    function test_checkHCUForFheAddWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheAdd));

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheAdd(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);

        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_ADD_SUB_SCALAR() : hcuLimit.ROW_ADD(), FheType(resultType));
    }

    function test_PayFheSubWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheSub));

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheSub(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);

        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_ADD_SUB_SCALAR() : hcuLimit.ROW_SUB(), FheType(resultType));
    }

    function test_PayFheMulWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMul));

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMul(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_MUL_SCALAR() : hcuLimit.ROW_MUL(), FheType(resultType));
    }

    function test_checkHCUForFheMulDivRevertsForIllegalScalarByte(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMulDiv));
        vm.assume(scalarByte != FHE_MUL_DIV_FACTOR2_SCALAR && scalarByte != FHE_MUL_DIV_FACTOR2_ENCRYPTED);

        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.OnlyScalarOperationsAreSupported.selector);
        hcuLimit.checkHCUForFheMulDiv(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_PayFheDivWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheDiv));
        bytes1 scalarByte = 0x01;

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheDiv(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(hcuLimit.ROW_DIV_SCALAR(), FheType(resultType));
    }

    function test_PayFheRemWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRem));
        bytes1 scalarByte = 0x01;

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRem(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(hcuLimit.ROW_REM_SCALAR(), FheType(resultType));
    }

    function test_checkHCUForFheBitAndWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitAnd));

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitAnd(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_BIT_AND_SCALAR() : hcuLimit.ROW_BIT_AND(), FheType(resultType));
    }

    function test_checkHCUForFheBitOrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitOr));

        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitOr(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_BIT_OR_SCALAR() : hcuLimit.ROW_BIT_OR(), FheType(resultType));
    }

    function test_PayFheBitXorWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheBitXor));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitXor(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_BIT_XOR_SCALAR() : hcuLimit.ROW_BIT_XOR(), FheType(resultType));
    }

    function test_checkHCUForFheShlWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShl));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheShl(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_SHL_SCALAR() : hcuLimit.ROW_SHL(), FheType(resultType));
    }

    function test_checkHCUForFheShrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheShr));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheShr(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_SHR_SCALAR() : hcuLimit.ROW_SHR(), FheType(resultType));
    }

    function test_checkHCUForFheRotlWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotl));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRotl(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_ROTL_SCALAR() : hcuLimit.ROW_ROTL(), FheType(resultType));
    }

    function test_checkHCUForFheRotrWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRotr));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRotr(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_ROTR_SCALAR() : hcuLimit.ROW_ROTR(), FheType(resultType));
    }

    function test_checkHCUForFheEqWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheEq));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheEq(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_EQ_SCALAR() : hcuLimit.ROW_EQ(), FheType(resultType));
    }

    function test_checkHCUForFheNeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNe));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNe(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_NE_SCALAR() : hcuLimit.ROW_NE(), FheType(resultType));
    }

    function test_checkHCUForFheGeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGe));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheGe(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_GE_SCALAR() : hcuLimit.ROW_GE(), FheType(resultType));
    }

    function test_checkHCUForFheGtWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheGt));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheGt(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_GT_SCALAR() : hcuLimit.ROW_GT(), FheType(resultType));
    }

    function test_checkHCUForFheLeWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLe));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheLe(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_LE_SCALAR() : hcuLimit.ROW_LE(), FheType(resultType));
    }

    function test_checkHCUForFheLtWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheLt));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheLt(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_LT_SCALAR() : hcuLimit.ROW_LT(), FheType(resultType));
    }

    function test_checkHCUForFheMinWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMin));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMin(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_MIN_SCALAR() : hcuLimit.ROW_MIN(), FheType(resultType));
    }

    function test_checkHCUForFheMaxWorksAsExpectedForSupportedTypes(uint8 resultType, bytes1 scalarByte) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheMax));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMax(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(scalarByte == 0x01 ? hcuLimit.ROW_MAX_SCALAR() : hcuLimit.ROW_MAX(), FheType(resultType));
    }

    function test_checkHCUForFheNegWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNeg));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNeg(FheType(resultType), mockLHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(hcuLimit.ROW_NEG(), FheType(resultType));
    }

    function test_checkHCUForFheNotWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheNot));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNot(FheType(resultType), mockLHS, mockResult, fhevmExecutor);
        // No table row: the latency bounds hold for the result's depth, and some work is charged.
        uint256 latencyHCU = hcuLimit.getTransactionDepth(mockResult);
        vm.assertGe(latencyHCU, 2);
        vm.assertLe(latencyHCU, 130);
        vm.assertGt(hcuLimit.getHCUForTransaction(), 0);
    }

    function test_checkHCUForCastWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesInputCast));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForCast(FheType(resultType), mockLHS, mockResult, fhevmExecutor);
        // No table row: the latency bounds hold for the result's depth, and some work is charged.
        uint256 latencyHCU = hcuLimit.getTransactionDepth(mockResult);
        vm.assertEq(latencyHCU, 32);
        vm.assertGt(hcuLimit.getHCUForTransaction(), 0);
    }

    function test_CheckGasLimitForIfThenElseWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheIfThenElse));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForIfThenElse(FheType(resultType), mockLHS, mockMiddle, mockRHS, mockResult, fhevmExecutor);
        _assertChargedFromTable(hcuLimit.ROW_IF_THEN_ELSE(), FheType(resultType));
    }

    function test_checkHCUForFheRandWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRand));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRand(FheType(resultType), mockResult, fhevmExecutor);
        _assertChargedFromTable(hcuLimit.ROW_RAND(), FheType(resultType));
    }

    function test_checkHCUForFheRandBoundedWorksAsExpectedForSupportedTypes(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRandBounded));
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRandBounded(FheType(resultType), mockResult, fhevmExecutor);
        _assertChargedFromTable(hcuLimit.ROW_RAND_BOUNDED(), FheType(resultType));
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheAdd(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheSub(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheSub(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheMul(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheMul(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheDiv(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheDiv(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRem(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheRem(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheBitAnd(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheBitAnd(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheBitOr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheBitOr(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheBitXor(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheBitXor(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheShl(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheShl(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheShr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheShr(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRotl(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheRotl(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRotr(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheRotr(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheEq(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheEq(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheNe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheNe(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheGe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheGe(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheGt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheGt(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheLe(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheLe(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheLt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheLt(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheMin(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheMin(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheMax(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheMax(FheType.Uint8, 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheNeg(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheNeg(FheType.Uint8, mockLHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheNot(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheNot(FheType.Uint8, mockLHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForCast(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForCast(FheType.Uint8, mockLHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallCheckGasLimitForTrivialEncrypt(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForTrivialEncrypt(FheType.Uint8, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallCheckGasLimitForIfThenElse(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForIfThenElse(FheType.Uint8, mockLHS, mockMiddle, mockRHS, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRand(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheRand(FheType.Uint8, mockResult, fhevmExecutor);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheRandBounded(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheRandBounded(FheType.Uint8, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheAddRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheAdd));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheAdd(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheSubRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheSub));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheSub(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheMulRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMul));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMul(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheDivRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheDiv));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheDiv(FheType(fheType), 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRemRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRem));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRem(FheType(fheType), 0x01, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheBitAndRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitAnd));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitAnd(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheBitOrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitOr));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitOr(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheBitXorRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitXor));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheBitXor(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheShlRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheShl));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheShl(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheShrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheShr));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheShr(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRotlRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRotl));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRotl(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRotrRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRotr));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRotr(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheEqRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheEq));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheEq(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheNeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNe));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNe(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheGeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheGe));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheGe(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheGtRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheGt));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheGt(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheLeRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheLe));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheLe(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheLtRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheLt));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheLt(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheMinRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMin));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMin(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheMaxRevertsForUnsupportedTypes(uint8 fheType, bytes1 scalarByte) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMax));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheMax(FheType(fheType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheNegRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNeg));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNeg(FheType(fheType), mockLHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheNotRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNot));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheNot(FheType(fheType), mockLHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForCastRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesInputCast));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForCast(FheType(fheType), mockLHS, mockResult, fhevmExecutor);
    }

    function test_CheckGasLimitForIfThenElseRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheIfThenElse));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForIfThenElse(FheType(fheType), mockLHS, mockMiddle, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRandRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRand));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRand(FheType(fheType), mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRandBoundedRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRandBounded));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
        vm.prank(fhevmExecutor);
        hcuLimit.checkHCUForFheRandBounded(FheType(fheType), mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheDivRevertsIfNotScalar(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheDiv));
        bytes1 scalarByte = 0x00;
        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.OnlyScalarOperationsAreSupported.selector);
        hcuLimit.checkHCUForFheDiv(FheType(resultType), scalarByte, mockLHS, mockRHS, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheRemRevertsIfNotScalar(uint8 resultType) public {
        vm.assume(resultType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(resultType), supportedTypesFheRem));
        bytes1 scalarByte = 0x00;
        vm.prank(fhevmExecutor);
        vm.expectRevert(HCULimit.OnlyScalarOperationsAreSupported.selector);
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

    function test_setQuotaByOwner() public {
        _setQuota(7, 25_000_000, 5_000_000);
        (uint48 maxHCU, uint48 decayPerBlock, ) = hcuLimit.getQuota(7);
        assertEq(maxHCU, 25_000_000);
        assertEq(decayPerBlock, 5_000_000);
    }

    function test_setQuotaRefusesOnlyOneOfMaximumAndDecayAtZero() public {
        vm.startPrank(owner);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.InvalidQuota.selector, uint16(5)));
        hcuLimit.setQuota(5, 100_000, 0);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.InvalidQuota.selector, uint16(5)));
        hcuLimit.setQuota(5, type(uint48).max, 0);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.InvalidQuota.selector, uint16(5)));
        hcuLimit.setQuota(5, 0, 100_000);
        hcuLimit.setQuota(5, 0, 0);
        vm.stopPrank();
    }

    function test_quotaUsageDecaysPerBlock() public {
        // A burst up to the maximum, two adds and a quarter, then half an add back per block.
        _setQuota(DEFAULT_QUOTA, 2 * addWork + addWork / 4, addWork / 2);
        _add(address(0x1234), 0x1001);
        _add(address(0x1234), 0x1002);
        _expectQuotaExceeded(address(0x1234), DEFAULT_QUOTA, 2, 0x1003);

        vm.roll(block.number + 1);
        _expectQuotaExceeded(address(0x1234), DEFAULT_QUOTA, 1, 0x1004);

        vm.roll(block.number + 1);
        _add(address(0x1234), 0x1005);
        (, , uint48 used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(used, 2 * addWork - 2 * (addWork / 2) + addWork);
    }

    function test_quotaReportsNoRetryWhenOneOperationExceedsTheMaximum() public {
        _setQuota(DEFAULT_QUOTA, addWork / 2, addWork / 2);
        _expectQuotaExceeded(address(0x1234), DEFAULT_QUOTA, type(uint256).max, 0x1001);
    }

    /// Depth is latency and the per-transaction total is work, so neither limit bounds the other.
    function test_maxHCUDepthPerTxCanExceedMaxHCUPerTx() public {
        vm.prank(owner);
        hcuLimit.setMaxHCUDepthPerTx(20_000_001);
        assertEq(hcuLimit.getMaxHCUDepthPerTx(), 20_000_001);
    }

    function test_unmeteredDefaultQuotaRecordsNoUsage() public {
        _add(address(0x1234), 0x1001);
        _add(address(0x1234), 0x1002);
        (uint48 maxHCU, , uint48 used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(maxHCU, type(uint48).max);
        assertEq(used, 0);
    }

    function test_quotaRevertsAboveItsMaximum() public {
        _setQuotaPerBlock(DEFAULT_QUOTA, addWork + addWork / 4);
        _add(address(0x1234), 0x1001);
        _expectQuotaExceeded(address(0x1234), DEFAULT_QUOTA, 1, 0x1002);
    }

    function test_quotaAllowsUsageUpToItsMaximum() public {
        // A cap of two adds allows two and rejects the third.
        _setQuotaPerBlock(DEFAULT_QUOTA, 2 * addWork);
        _add(address(0x1234), 0x2001);
        _add(address(0x1234), 0x2002);
        _expectQuotaExceeded(address(0x1234), DEFAULT_QUOTA, 1, 0x2003);
    }

    function test_contractsAssignedToOneQuotaShareItsUsage() public {
        address first = address(0xA1);
        address second = address(0xA2);
        _setQuotaPerBlock(DEFAULT_QUOTA, addWork + addWork / 4);
        _setQuotaPerBlock(2, addWork + addWork / 4);
        _assign(2, first);
        _assign(2, second);

        _add(first, 0x1001);
        _expectQuotaExceeded(second, 2, 1, 0x1002);

        // The default quota has its own budget.
        _add(address(0x1234), 0x1003);
    }

    function test_unmeteredQuotaRecordsNoUsage() public {
        address account = address(0xBEEF);
        _setQuotaPerBlock(DEFAULT_QUOTA, addWork + addWork / 4);
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
        assertEq(used, addWork);
    }

    function test_quotaPerBlockFreesUpAfterOneBlock() public {
        _setQuotaPerBlock(DEFAULT_QUOTA, addWork + addWork / 4);
        _add(address(0x1234), 0x1001);
        (, , uint48 used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(used, addWork);

        vm.roll(block.number + 1);
        (, , used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(used, 0);
        _add(address(0x1234), 0x1002);
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

    function test_pausingTheDefaultQuotaStopsUnassignedContractsOnly() public {
        address assigned = address(0xA1);
        _setQuotaPerBlock(3, 1_000_000);
        _assign(3, assigned);
        _pause(_mockPauser(address(0x9A)), DEFAULT_QUOTA);

        _expectQuotaPaused(address(0x1234), DEFAULT_QUOTA);
        _add(assigned, 0x1001);
    }

    function test_unpauseRestoresTheQuota() public {
        address account = address(0xBEEF);
        _setQuotaPerBlock(3, 1_000_000);
        _assign(3, account);
        _pause(_mockPauser(address(0x9A)), 3);

        vm.expectEmit(address(hcuLimit));
        emit HCULimit.QuotaPauseSet(3, false, owner);
        vm.prank(owner);
        hcuLimit.unpauseQuota(3);

        assertFalse(hcuLimit.isQuotaPaused(3));
        _add(account, 0x1001);
    }

    function test_onlyOwnerUnpauses() public {
        address pauser = _mockPauser(address(0x9A));
        _pause(pauser, 3);
        vm.prank(pauser);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, pauser));
        hcuLimit.unpauseQuota(3);
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

    function test_assignQuotaRefusesAStoppedQuotaButAcceptsTheDefaultQuota() public {
        _setQuotaPerBlock(3, 1_000_000);
        _assign(3, address(0xBEEF));
        _stopDefaultQuota();

        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.InvalidQuota.selector, uint16(7)));
        hcuLimit.assignQuota(7, address(0xBEEF));
        _assign(DEFAULT_QUOTA, address(0xBEEF));
    }

    function test_setQuotaAdminRefusesTheDefaultQuota() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.InvalidQuota.selector, DEFAULT_QUOTA));
        hcuLimit.setQuotaAdmin(DEFAULT_QUOTA, PARTNER_ADMIN);
    }

    function test_quotaAdminGrantsAndRevokesContractsOfItsQuota() public {
        address auction = address(0xA1);
        _setUpPartner();

        vm.expectEmit(address(hcuLimit));
        emit HCULimit.ContractQuotaAssigned(PARTNER_QUOTA, auction, PARTNER_ADMIN);
        _grant(auction);
        assertEq(hcuLimit.getContractQuota(auction), PARTNER_QUOTA);
        _add(auction, 0x1001);

        vm.expectEmit(address(hcuLimit));
        emit HCULimit.ContractQuotaAssigned(DEFAULT_QUOTA, auction, PARTNER_ADMIN);
        vm.prank(PARTNER_ADMIN);
        hcuLimit.revokeQuota(PARTNER_QUOTA, auction);
        vm.prank(fhevmExecutor);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaStopped.selector, auction, DEFAULT_QUOTA));
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, bytes32(uint256(0x1002)), auction);
    }

    function test_quotaAdminRevokesAContractTheOwnerAssigned() public {
        address wrapper = address(0xB1);
        _setUpPartner();
        _assign(PARTNER_QUOTA, wrapper);

        vm.prank(PARTNER_ADMIN);
        hcuLimit.revokeQuota(PARTNER_QUOTA, wrapper);
        assertEq(hcuLimit.getContractQuota(wrapper), DEFAULT_QUOTA);
    }

    function test_grantQuotaRefusesWhileTheDefaultQuotaIsNotStopped() public {
        // setUp leaves the default quota unmetered: moving a contract out of it could leave it with less room.
        _setQuotaPerBlock(PARTNER_QUOTA, 1_000_000);
        _setAdmin(PARTNER_QUOTA, PARTNER_ADMIN);

        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(HCULimit.DefaultQuotaNotStopped.selector);
        hcuLimit.grantQuota(PARTNER_QUOTA, address(0xA1));
    }

    function test_grantQuotaOnlyTakesContractsFromTheDefaultQuota() public {
        address other = address(0xB1);
        _setUpPartner();
        _setQuotaPerBlock(5, 1_000_000);
        _assign(5, other);

        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.ContractNotInQuota.selector, other, DEFAULT_QUOTA));
        hcuLimit.grantQuota(PARTNER_QUOTA, other);
    }

    function test_revokeQuotaOnlyReleasesContractsOfItsQuota() public {
        address other = address(0xB1);
        _setUpPartner();
        _setQuotaPerBlock(5, 1_000_000);
        _assign(5, other);

        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.ContractNotInQuota.selector, other, PARTNER_QUOTA));
        hcuLimit.revokeQuota(PARTNER_QUOTA, other);
    }

    function test_grantQuotaRefusesAStoppedQuota() public {
        _setUpPartner();
        _setQuota(PARTNER_QUOTA, 0, 0);

        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.InvalidQuota.selector, PARTNER_QUOTA));
        hcuLimit.grantQuota(PARTNER_QUOTA, address(0xA1));
    }

    function test_quotaAdminCannotActOnAnotherQuota() public {
        _setUpPartner();
        _setQuotaPerBlock(13, 1_000_000);

        vm.startPrank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.NotQuotaAdmin.selector, uint16(13), PARTNER_ADMIN));
        hcuLimit.grantQuota(13, address(0xA1));
        vm.expectRevert(abi.encodeWithSelector(HCULimit.NotQuotaAdmin.selector, uint16(13), PARTNER_ADMIN));
        hcuLimit.revokeQuota(13, address(0xA1));
        vm.stopPrank();
    }

    function test_pausedQuotaFreezesItsMembersForItsAdmin() public {
        address auction = address(0xA1);
        _setUpPartner();
        _grant(auction);
        _pause(_mockPauser(address(0x9A)), PARTNER_QUOTA);

        // The admin can neither move its contracts out to escape the pause nor add others to it.
        vm.startPrank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.QuotaMembershipPaused.selector, PARTNER_QUOTA));
        hcuLimit.revokeQuota(PARTNER_QUOTA, auction);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.QuotaMembershipPaused.selector, PARTNER_QUOTA));
        hcuLimit.grantQuota(PARTNER_QUOTA, address(0xA2));
        vm.stopPrank();
    }

    function test_removedAdminLosesItsRightsAndItsContractsStay() public {
        address auction = address(0xA1);
        _setUpPartner();
        _grant(auction);

        _setAdmin(PARTNER_QUOTA, address(0));
        assertEq(hcuLimit.getQuotaAdmin(PARTNER_QUOTA), address(0));
        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.NotQuotaAdmin.selector, PARTNER_QUOTA, PARTNER_ADMIN));
        hcuLimit.revokeQuota(PARTNER_QUOTA, auction);
        assertEq(hcuLimit.getContractQuota(auction), PARTNER_QUOTA);
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
        vm.expectRevert(abi.encodeWithSelector(HCULimit.ContractNotInQuota.selector, abused, DEFAULT_QUOTA));
        hcuLimit.grantQuota(PARTNER_QUOTA, abused);
    }

    function test_pauserCannotCallOwnerOnlyQuotaSetters() public {
        address pauser = _mockPauser(address(0x9A));
        vm.startPrank(pauser);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, pauser));
        hcuLimit.setQuota(DEFAULT_QUOTA, 0, 0);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, pauser));
        hcuLimit.assignQuota(DEFAULT_QUOTA, address(0xBEEF));
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, pauser));
        hcuLimit.setQuotaAdmin(PARTNER_QUOTA, pauser);
        vm.stopPrank();
    }

    function test_strangerCannotChangeQuotas(address stranger) public {
        vm.assume(stranger != owner && stranger != address(0));
        vm.mockCall(pauserSetAdd, abi.encodeCall(IPauserSet.isPauser, (stranger)), abi.encode(false));

        vm.startPrank(stranger);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.NotPauser.selector, stranger));
        hcuLimit.pauseQuota(DEFAULT_QUOTA);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.NotQuotaAdmin.selector, PARTNER_QUOTA, stranger));
        hcuLimit.grantQuota(PARTNER_QUOTA, address(0xBEEF));
        vm.expectRevert(abi.encodeWithSelector(HCULimit.NotQuotaAdmin.selector, PARTNER_QUOTA, stranger));
        hcuLimit.revokeQuota(PARTNER_QUOTA, address(0xBEEF));
        vm.stopPrank();
    }

    function test_reinitializeV6CopiesGlobalCapIntoDefaultQuota() public {
        _upgradeFromV5(false);

        (uint48 maxHCU, uint48 decayPerBlock, ) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(maxHCU, _legacyGlobalCap());
        assertEq(decayPerBlock, _legacyGlobalCap());
        (maxHCU, , ) = hcuLimit.getQuota(ALLOWLIST_QUOTA);
        assertEq(maxHCU, 0);
        (maxHCU, , ) = hcuLimit.getCapacity();
        assertEq(maxHCU, type(uint48).max);
        assertEq(hcuLimit.getTier(0), 10_000);
    }

    function test_reinitializeV6WithTheAllowlistActiveStopsTheDefaultQuota() public {
        _upgradeFromV5(true);

        (uint48 maxHCU, uint48 decayPerBlock, ) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(maxHCU, 0);
        assertEq(decayPerBlock, 0);
        (maxHCU, decayPerBlock, ) = hcuLimit.getQuota(ALLOWLIST_QUOTA);
        assertEq(maxHCU, _legacyGlobalCap());
        assertEq(decayPerBlock, _legacyGlobalCap());
        (maxHCU, , ) = hcuLimit.getQuota(BLOCK_WHITELIST_QUOTA);
        assertEq(maxHCU, type(uint48).max);
        (maxHCU, , ) = hcuLimit.getCapacity();
        assertEq(maxHCU, type(uint48).max);
        assertEq(hcuLimit.getTier(0), 10_000);
    }

    function test_allowlistedContractsKeepTheLimitsTheyHadUntilAssigned() public {
        _upgradeFromV5(true);
        assertEq(hcuLimit.getContractQuota(ALLOWED), ALLOWLIST_QUOTA);
        assertEq(hcuLimit.getContractQuota(ALLOWED_BLOCK_WHITELISTED), BLOCK_WHITELIST_QUOTA);
        assertEq(hcuLimit.getContractQuota(BLOCK_WHITELISTED_ONLY), DEFAULT_QUOTA);
        assertEq(hcuLimit.getContractQuota(UNKNOWN), DEFAULT_QUOTA);

        // Allowlisted contracts share the old global cap, an add and a quarter per block.
        _add(ALLOWED, 0x1001);
        (, , uint48 used) = hcuLimit.getQuota(ALLOWLIST_QUOTA);
        assertEq(used, addWork);
        _expectQuotaExceeded(ALLOWED, ALLOWLIST_QUOTA, 1, 0x1002);
        // Block-whitelisted ones skipped that cap, and stay unmetered.
        _add(ALLOWED_BLOCK_WHITELISTED, 0x1003);
        _add(ALLOWED_BLOCK_WHITELISTED, 0x1004);
        // Contracts the allowlist refused stay refused, block-whitelisted or not.
        _expectQuotaStopped(UNKNOWN, DEFAULT_QUOTA);
        _expectQuotaStopped(BLOCK_WHITELISTED_ONLY, DEFAULT_QUOTA);
    }

    function test_assignmentTakesPrecedenceOverTheAllowlist() public {
        _upgradeFromV5(true);
        _setQuotaPerBlock(PARTNER_QUOTA, 1_000_000);
        _assign(PARTNER_QUOTA, ALLOWED);

        assertEq(hcuLimit.getContractQuota(ALLOWED), PARTNER_QUOTA);
        _add(ALLOWED, 0x1001);
        (, , uint48 used) = hcuLimit.getQuota(PARTNER_QUOTA);
        assertEq(used, addWork);

        // Back in the default quota, it has left the allowlist and stays stopped.
        _assign(DEFAULT_QUOTA, ALLOWED);
        assertEq(hcuLimit.getContractQuota(ALLOWED), DEFAULT_QUOTA);
        _expectQuotaStopped(ALLOWED, DEFAULT_QUOTA);
    }

    function test_revokingAnAllowlistedContractStopsIt() public {
        _upgradeFromV5(true);
        _setQuotaPerBlock(PARTNER_QUOTA, 1_000_000);
        _setAdmin(PARTNER_QUOTA, PARTNER_ADMIN);
        _assign(PARTNER_QUOTA, ALLOWED_BLOCK_WHITELISTED);

        // Without leaving the allowlist, it would compute unmetered in quota 2 instead.
        vm.prank(PARTNER_ADMIN);
        hcuLimit.revokeQuota(PARTNER_QUOTA, ALLOWED_BLOCK_WHITELISTED);
        _expectQuotaStopped(ALLOWED_BLOCK_WHITELISTED, DEFAULT_QUOTA);
    }

    function test_quotasThatReplacedTheAllowlistTakeNoAdmin() public {
        vm.startPrank(owner);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.InvalidQuota.selector, ALLOWLIST_QUOTA));
        hcuLimit.setQuotaAdmin(ALLOWLIST_QUOTA, PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.InvalidQuota.selector, BLOCK_WHITELIST_QUOTA));
        hcuLimit.setQuotaAdmin(BLOCK_WHITELIST_QUOTA, PARTNER_ADMIN);
        vm.stopPrank();
    }

    function test_pausingTheAllowlistQuotaPausesTheContractsItRoutes() public {
        _upgradeFromV5(true);
        _pause(_mockPauser(address(0xFA)), ALLOWLIST_QUOTA);

        _expectQuotaPaused(ALLOWED, ALLOWLIST_QUOTA);
    }

    function test_routedContractsChargeTheCapacity() public {
        _upgradeFromV5(true);
        _setCapacity(1_000_000, 100_000);

        _add(ALLOWED_BLOCK_WHITELISTED, 0x1001);
        (, , uint48 load) = hcuLimit.getCapacity();
        assertEq(load, addWork);
    }

    function test_quotaAdminCannotTakeAnAllowlistedContract() public {
        _upgradeFromV5(true);
        _setQuotaPerBlock(PARTNER_QUOTA, 1_000_000);
        _setAdmin(PARTNER_QUOTA, PARTNER_ADMIN);

        vm.prank(PARTNER_ADMIN);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.ContractNotInQuota.selector, ALLOWED, DEFAULT_QUOTA));
        hcuLimit.grantQuota(PARTNER_QUOTA, ALLOWED);

        _grant(UNKNOWN);
        assertEq(hcuLimit.getContractQuota(UNKNOWN), PARTNER_QUOTA);
    }

    function test_stoppingTheAllowlistQuotaStopsContractsNobodyAssigned() public {
        _upgradeFromV5(true);
        _setQuota(ALLOWLIST_QUOTA, 0, 0);

        _expectQuotaStopped(ALLOWED, ALLOWLIST_QUOTA);
    }

    function test_openingTheDefaultQuotaEndsTheAllowlistRouting() public {
        _upgradeFromV5(true);
        _setQuotaPerBlock(DEFAULT_QUOTA, 1_000_000);

        assertEq(hcuLimit.getContractQuota(ALLOWED), DEFAULT_QUOTA);
        _add(ALLOWED, 0x1001);
        (, , uint48 used) = hcuLimit.getQuota(DEFAULT_QUOTA);
        assertEq(used, addWork);
    }

    function test_inactiveAllowlistRoutesNothing() public {
        _upgradeFromV5(false);
        _stopDefaultQuota();

        assertEq(hcuLimit.getContractQuota(ALLOWED), DEFAULT_QUOTA);
        _expectQuotaStopped(ALLOWED, DEFAULT_QUOTA);
    }

    function test_initializeLeavesCapacityUnmeteredAndTheDefaultTierFull() public view {
        (uint48 maxHCU, , ) = hcuLimit.getCapacity();
        assertEq(maxHCU, type(uint48).max);
        assertEq(hcuLimit.getTier(0), 10_000);
        assertEq(hcuLimit.getQuotaTier(PARTNER_QUOTA), 0);
    }

    function test_lowTierIsRefusedOnceChainLoadPassesItsThreshold() public {
        // A capacity of 12.5 adds draining 1.25 per block; tier 1 is refused above 2.5 adds of load.
        _setUpTiers();

        _add(LOW, 0x1001);
        _add(LOW, 0x1002);
        _expectChainLoadTooHigh(LOW, LOW_QUOTA, 1, 0x1003);
        // The default tier still has room up to the full capacity.
        _add(HIGH, 0x1004);
        (, , uint48 load) = hcuLimit.getCapacity();
        assertEq(load, 3 * addWork);

        // The default tier's addition pushed tier 1's wait from one block to two.
        vm.roll(block.number + 1);
        _expectChainLoadTooHigh(LOW, LOW_QUOTA, 1, 0x1005);
        vm.roll(block.number + 1);
        _add(LOW, 0x1006);
    }

    function test_otherTiersLoadRefusesTheLowTier() public {
        _setUpTiers();

        _add(HIGH, 0x1001);
        _add(HIGH, 0x1002);
        _add(HIGH, 0x1003);
        // Four adds after the operation, 1.5 above tier 1's threshold: two blocks of decay.
        _expectChainLoadTooHigh(LOW, LOW_QUOTA, 2, 0x1004);
    }

    function test_unmeteredQuotaStillChargesTheCapacity() public {
        // setUp leaves the default quota unmetered.
        _setCapacity(addWork + addWork / 4, addWork + addWork / 4);
        _add(address(0x1234), 0x1001);
        (, , uint48 load) = hcuLimit.getCapacity();
        assertEq(load, addWork);
        _expectChainLoadTooHigh(address(0x1234), DEFAULT_QUOTA, 1, 0x1002);
    }

    function test_refusedOperationSpendsNeitherMeter() public {
        _setUpTiers();
        _setQuotaPerBlock(LOW_QUOTA, 1_000_000);
        _add(HIGH, 0x1001);
        _add(HIGH, 0x1002);
        _add(HIGH, 0x1003);

        _expectChainLoadTooHigh(LOW, LOW_QUOTA, 2, 0x1004);
        (, , uint48 used) = hcuLimit.getQuota(LOW_QUOTA);
        assertEq(used, 0);

        _setQuotaPerBlock(HIGH_QUOTA, addWork + addWork / 4);
        vm.roll(block.number + 1);
        _add(HIGH, 0x1005);
        _expectQuotaExceeded(HIGH, HIGH_QUOTA, 1, 0x1006);
        (, , uint48 load) = hcuLimit.getCapacity();
        assertEq(load, 3 * addWork - (addWork + addWork / 4) + addWork);
    }

    function test_theLongerWaitNamesTheErrorWhenBothMetersRefuse() public {
        // The second addition is three quarters of an add over both meters, which drain at different rates:
        // half an add per block takes two blocks, an eighth takes six.
        _setQuota(DEFAULT_QUOTA, addWork + addWork / 4, addWork / 2);
        _setCapacity(addWork + addWork / 4, addWork / 8);
        _add(address(0x1234), 0x1001);
        _expectChainLoadTooHigh(address(0x1234), DEFAULT_QUOTA, 6, 0x1002);

        _setQuota(DEFAULT_QUOTA, addWork + addWork / 4, addWork / 8);
        _setCapacity(addWork + addWork / 4, addWork / 2);
        _expectQuotaExceeded(address(0x1234), DEFAULT_QUOTA, 6, 0x1003);
    }

    function test_retunedTierAppliesToAllItsQuotas() public {
        address lowSibling = address(0xA2);
        _setUpTiers();
        _assign(LOW_QUOTA, lowSibling);
        _setTier(1, 1_000);

        _add(LOW, 0x1001);
        _expectChainLoadTooHigh(lowSibling, LOW_QUOTA, 1, 0x1002);
        _setTier(1, 5_000);
        _add(lowSibling, 0x1003);
    }

    function test_tierSetToZeroStopsItsQuotas() public {
        _setUpTiers();
        _setTier(1, 0);
        _expectTierStopped(LOW, LOW_QUOTA);
    }

    function test_tierThresholdsApplyOnlyWhileTheCapacityIsMetered() public {
        _setUpTiers();
        _setTier(1, 0);
        _setCapacity(type(uint48).max, type(uint48).max);
        _add(LOW, 0x1001);
    }

    function test_capacitySetToZeroStopsEveryOperation() public {
        _setCapacity(0, 0);
        _expectTierStopped(address(0x1234), DEFAULT_QUOTA);
    }

    function test_setQuotaTierRefusesATierWithoutThreshold() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.InvalidTier.selector, uint8(9)));
        hcuLimit.setQuotaTier(LOW_QUOTA, 9);
    }

    function test_setTierRefusesAboveTenThousandBasisPoints() public {
        vm.prank(owner);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.InvalidTier.selector, uint8(1)));
        hcuLimit.setTier(1, 10_001);
    }

    function test_setCapacityRefusesOnlyOneOfMaximumAndDecayAtZero() public {
        vm.startPrank(owner);
        vm.expectRevert(HCULimit.InvalidCapacity.selector);
        hcuLimit.setCapacity(100_000, 0);
        vm.expectRevert(HCULimit.InvalidCapacity.selector);
        hcuLimit.setCapacity(0, 100_000);
        vm.stopPrank();
    }

    function test_pauserCannotSetCapacityOrTiers() public {
        address pauser = _mockPauser(address(0x9A));
        vm.startPrank(pauser);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, pauser));
        hcuLimit.setCapacity(0, 0);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, pauser));
        hcuLimit.setTier(1, 0);
        vm.expectRevert(abi.encodeWithSelector(ACLOwnable.NotHostOwner.selector, pauser));
        hcuLimit.setQuotaTier(PARTNER_QUOTA, 0);
        vm.stopPrank();
    }

    /// @dev Capacity of 1,000,000 HCU draining 100,000 per block. LOW is in an unmetered quota of tier 1, refused
    ///      above 2,000 basis points (200,000 HCU of load); HIGH in an unmetered quota of the default tier.
    function _setUpTiers() internal {
        _setCapacity(12 * addWork + addWork / 2, addWork + addWork / 4);
        _setTier(1, 2_000);
        _setQuota(LOW_QUOTA, type(uint48).max, type(uint48).max);
        _setQuota(HIGH_QUOTA, type(uint48).max, type(uint48).max);
        _setQuotaTier(LOW_QUOTA, 1);
        _assign(LOW_QUOTA, LOW);
        _assign(HIGH_QUOTA, HIGH);
    }

    function _setCapacity(uint48 maxHCU, uint48 decayPerBlock) internal {
        vm.expectEmit(address(hcuLimit));
        emit HCULimit.CapacitySet(maxHCU, decayPerBlock, owner);
        vm.prank(owner);
        hcuLimit.setCapacity(maxHCU, decayPerBlock);
    }

    function _setTier(uint8 tier, uint16 maxLoadBps) internal {
        vm.expectEmit(address(hcuLimit));
        emit HCULimit.TierSet(tier, maxLoadBps, owner);
        vm.prank(owner);
        hcuLimit.setTier(tier, maxLoadBps);
    }

    function _setQuotaTier(uint16 quotaId, uint8 tier) internal {
        vm.expectEmit(address(hcuLimit));
        emit HCULimit.QuotaTierSet(quotaId, tier, owner);
        vm.prank(owner);
        hcuLimit.setQuotaTier(quotaId, tier);
        assertEq(hcuLimit.getQuotaTier(quotaId), tier);
    }

    function _expectTierStopped(address caller, uint16 quotaId) internal {
        vm.prank(fhevmExecutor);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUTierStopped.selector, caller, quotaId));
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, bytes32(uint256(0x9002)), caller);
    }

    /// @dev Expects one 84,000 HCU addition by `caller` to be refused by the chain's load with `retryAfterBlocks`.
    function _expectChainLoadTooHigh(
        address caller,
        uint16 quotaId,
        uint256 retryAfterBlocks,
        uint256 resultId
    ) internal {
        vm.prank(fhevmExecutor);
        vm.expectRevert(
            abi.encodeWithSelector(HCULimit.HCUChainLoadTooHigh.selector, caller, quotaId, retryAfterBlocks)
        );
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, bytes32(resultId), caller);
    }

    /// @dev After one operation on fresh inputs: the transaction's total is the row's work, and the result's depth
    ///      its latency.
    function _assertChargedFromTable(uint256 row, FheType fheType) internal view {
        (uint256 latencyHCU, uint256 workHCU) = hcuLimit.cost(row, fheType);
        vm.assertEq(hcuLimit.getHCUForTransaction(), workHCU);
        vm.assertEq(hcuLimit.getTransactionDepth(mockResult), latencyHCU);
    }

    /// @dev A quota that allows `hcuPerBlock` HCU per block: maximum and decay per block both `hcuPerBlock`.
    function _setQuotaPerBlock(uint16 quotaId, uint48 hcuPerBlock) internal {
        _setQuota(quotaId, hcuPerBlock, hcuPerBlock);
    }

    function _setQuota(uint16 quotaId, uint48 maxHCU, uint48 decayPerBlock) internal {
        vm.expectEmit(address(hcuLimit));
        emit HCULimit.QuotaSet(quotaId, maxHCU, decayPerBlock, owner);
        vm.prank(owner);
        hcuLimit.setQuota(quotaId, maxHCU, decayPerBlock);
    }

    /// @dev Expects one 84,000 HCU addition by `caller` to exceed its quota with `retryAfterBlocks`.
    function _expectQuotaExceeded(address caller, uint16 quotaId, uint256 retryAfterBlocks, uint256 resultId) internal {
        vm.prank(fhevmExecutor);
        vm.expectRevert(abi.encodeWithSelector(HCULimit.HCUQuotaExceeded.selector, caller, quotaId, retryAfterBlocks));
        hcuLimit.checkHCUForFheAdd(FheType.Uint8, 0x01, mockLHS, mockRHS, bytes32(resultId), caller);
    }

    function _assign(uint16 quotaId, address account) internal {
        vm.expectEmit(address(hcuLimit));
        emit HCULimit.ContractQuotaAssigned(quotaId, account, owner);
        vm.prank(owner);
        hcuLimit.assignQuota(quotaId, account);
    }

    /// @dev The V5 global cap `_upgradeFromV5` sets: one add fits a block, two do not.
    function _legacyGlobalCap() internal view returns (uint48) {
        return addWork + addWork / 4;
    }

    /// @dev Replaces the proxy under test with one upgraded from V5: a global cap of `_legacyGlobalCap()` per block,
    ///      ALLOWED and ALLOWED_BLOCK_WHITELISTED allowlisted, ALLOWED_BLOCK_WHITELISTED and BLOCK_WHITELISTED_ONLY
    ///      block-whitelisted.
    function _upgradeFromV5(bool allowlistActivated) internal {
        address upgradedProxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, ())
        );
        vm.startPrank(owner);
        UnsafeUpgrades.upgradeProxy(upgradedProxy, address(new MockHCULimit()), "");
        MockHCULimit upgraded = MockHCULimit(upgradedProxy);
        upgraded.setLegacyGlobalCapForTest(_legacyGlobalCap());
        upgraded.setAllowlistActivatedForTest(allowlistActivated);
        upgraded.setAllowlistedForTest(ALLOWED);
        upgraded.setAllowlistedForTest(ALLOWED_BLOCK_WHITELISTED);
        upgraded.setBlockHCUWhitelistedForTest(ALLOWED_BLOCK_WHITELISTED);
        upgraded.setBlockHCUWhitelistedForTest(BLOCK_WHITELISTED_ONLY);
        upgraded.reinitializeV6();
        // A deployed V5 already has these limits; the upgrade keeps them.
        upgraded.setMaxHCUPerTx(20_000_000);
        upgraded.setMaxHCUDepthPerTx(5_000_000);
        vm.stopPrank();
        hcuLimit = upgraded;
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
        hcuLimit.grantQuota(PARTNER_QUOTA, account);
    }

    function _setAdmin(uint16 quotaId, address admin) internal {
        vm.expectEmit(address(hcuLimit));
        emit HCULimit.QuotaAdminSet(quotaId, admin);
        vm.prank(owner);
        hcuLimit.setQuotaAdmin(quotaId, admin);
    }

    function _pause(address pauser, uint16 quotaId) internal {
        vm.expectEmit(address(hcuLimit));
        emit HCULimit.QuotaPauseSet(quotaId, true, pauser);
        vm.prank(pauser);
        hcuLimit.pauseQuota(quotaId);
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

        // No table row: the latency bounds hold for the result's depth, and some work is charged.
        uint256 latencyHCU = hcuLimit.getTransactionDepth(mockResult);
        vm.assertGe(latencyHCU, 60000);
        vm.assertLe(latencyHCU, 1_300_000);
        vm.assertGt(hcuLimit.getHCUForTransaction(), 0);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheIsIn(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        bytes32[] memory set = new bytes32[](2);
        set[0] = bytes32(uint256(1));
        set[1] = bytes32(uint256(2));
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheIsIn(FheType.Uint8, mockLHS, set, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheIsInRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheIsIn));
        bytes32[] memory set = new bytes32[](2);
        set[0] = bytes32(uint256(1));
        set[1] = bytes32(uint256(2));
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
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

        // No table row: the latency bounds hold for the result's depth, and some work is charged.
        uint256 latencyHCU = hcuLimit.getTransactionDepth(mockResult);
        vm.assertGe(latencyHCU, 88000);
        vm.assertLe(latencyHCU, 259000);
        vm.assertGt(hcuLimit.getHCUForTransaction(), 0);
    }

    function test_OnlyFHEVMExecutorCanCallcheckHCUForFheSum(address randomAccount) public {
        vm.assume(randomAccount != fhevmExecutor);
        bytes32[] memory values = new bytes32[](2);
        values[0] = mockLHS;
        values[1] = mockRHS;
        vm.prank(randomAccount);
        vm.expectRevert(HCULimit.CallerMustBeFHEVMExecutorContract.selector);
        hcuLimit.checkHCUForFheSum(FheType.Uint8, values, mockResult, fhevmExecutor);
    }

    function test_checkHCUForFheSumRevertsForUnsupportedTypes(uint8 fheType) public {
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheSum));
        bytes32[] memory values = new bytes32[](2);
        values[0] = mockLHS;
        values[1] = mockRHS;
        vm.expectRevert(HCULimit.UnsupportedOperation.selector);
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

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade() public {
        /// @dev It does not revert since it called by the owner.
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", owner);
    }
}
