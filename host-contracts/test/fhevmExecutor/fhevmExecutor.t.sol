// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {UnsafeUpgrades} from "@openzeppelin/foundry-upgrades/src/Upgrades.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {MessageHashUtils} from "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

import {FHEVMExecutor} from "../../contracts/FHEVMExecutor.sol";
import {FHEEvents} from "../../contracts/FHEEvents.sol";
import {FHEVMExecutorNoEvents} from "../../contracts/FHEVMExecutorNoEvents.sol";
import {EmptyUUPSProxy} from "../../contracts/emptyProxy/EmptyUUPSProxy.sol";
import {FheType} from "../../contracts/FheType.sol";

import {aclAdd} from "../../addresses/ACLAddress.sol";
import {fheGasLimitAdd} from "../../addresses/FHEGasLimitAddress.sol";
import {inputVerifierAdd} from "../../addresses/InputVerifierAddress.sol";

contract SupportedTypesConstants {
    uint256 internal supportedTypesFheAdd =
        (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));

    uint256 internal supportedTypesFheSub = supportedTypesFheAdd;
    uint256 internal supportedTypesFheMul = supportedTypesFheSub;
    uint256 internal supportedTypesFheDiv = supportedTypesFheMul;
    uint256 internal supportedTypesFheRem = supportedTypesFheDiv;

    uint256 internal supportedTypesFheBitAnd =
        (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));

    uint256 internal supportedTypesFheBitOr = supportedTypesFheBitAnd;
    uint256 internal supportedTypesFheBitXor = supportedTypesFheBitOr;
    uint256 internal supportedTypesFheShl =
        (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));
    uint256 internal supportedTypesFheShr = supportedTypesFheShl;
    uint256 internal supportedTypesFheRotl = supportedTypesFheShr;
    uint256 internal supportedTypesFheRotr = supportedTypesFheRotl;

    uint256 internal supportedTypesFheEq =
        (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint160)) +
            (1 << uint8(FheType.Uint256)) +
            (1 << uint8(FheType.Uint512)) +
            (1 << uint8(FheType.Uint1024)) +
            (1 << uint8(FheType.Uint2048));

    uint256 internal supportedTypesFheEqWithBytes =
        (1 << uint8(FheType.Uint512)) + (1 << uint8(FheType.Uint1024)) + (1 << uint8(FheType.Uint2048));

    uint256 internal supportedTypesFheNe = supportedTypesFheEq;
    uint256 internal supportedTypesFheNeWithBytes = supportedTypesFheEqWithBytes;

    uint256 internal supportedTypesFheGe =
        (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128));

    uint256 internal supportedTypesFheGt = supportedTypesFheGe;
    uint256 internal supportedTypesFheLe = supportedTypesFheGt;
    uint256 internal supportedTypesFheLt = supportedTypesFheLe;
    uint256 internal supportedTypesFheMin = supportedTypesFheLt;
    uint256 internal supportedTypesFheMax = supportedTypesFheMin;

    uint256 internal supportedTypesFheNeg =
        (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));

    uint256 internal supportedTypesFheNot =
        (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));

    uint256 internal supportedTypesFheIfThenElse =
        (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint160)) +
            (1 << uint8(FheType.Uint256)) +
            (1 << uint8(FheType.Uint512)) +
            (1 << uint8(FheType.Uint1024)) +
            (1 << uint8(FheType.Uint2048));

    uint256 internal supportedTypesFheRand =
        (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256)) +
            (1 << uint8(FheType.Uint512)) +
            (1 << uint8(FheType.Uint1024)) +
            (1 << uint8(FheType.Uint2048));

    uint256 internal supportedTypesFheRandBounded =
        (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));

    uint256 internal supportedTypesInputCast =
        (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));

    uint256 internal supportedTypesOutputCast =
        (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint256));

    uint256 internal supportedTypesTrivialEncrypt =
        (1 << uint8(FheType.Bool)) +
            (1 << uint8(FheType.Uint8)) +
            (1 << uint8(FheType.Uint16)) +
            (1 << uint8(FheType.Uint32)) +
            (1 << uint8(FheType.Uint64)) +
            (1 << uint8(FheType.Uint128)) +
            (1 << uint8(FheType.Uint160)) +
            (1 << uint8(FheType.Uint256));

    uint256 internal supportedTypesTrivialEncryptWithBytes =
        (1 << uint8(FheType.Uint512)) + (1 << uint8(FheType.Uint1024)) + (1 << uint8(FheType.Uint2048));
}

/// @dev This contract is a mock implementation of the ACL interface.
/// It provides a simple mapping to check if an account is allowed for a given handle.
/// For mock purposes, it doesn't distinguish between allowTransient and allow.
contract MockACL {
    mapping(bytes32 handle => mapping(address => bool)) internal allowed;
    function allowTransient(bytes32 handle, address account) external {
        allowed[handle][account] = true;
    }

    function allow(bytes32 handle, address account) external {
        allowed[handle][account] = true;
    }

    function isAllowed(bytes32 handle, address account) external view returns (bool) {
        return allowed[handle][account];
    }
}

/// @dev This contract is a mock implementation of the InputVerifier.
/// @dev It never reverts and always returns the handle back.
contract MockInputVerifier {
    /// @dev This function is a placeholder for the actual input verification logic.
    function verifyCiphertext(
        FHEVMExecutorNoEvents.ContextUserInputs memory,
        bytes32 inputHandle,
        bytes memory
    ) external pure returns (bytes32) {
        return inputHandle;
    }
}

/// @dev This contract is a mock implementation of the FHEGasLimit.
/// It includes a fallback function not to revert.
contract MockFHEGasLimit {
    fallback() external payable {}
}

contract FHEVMExecutorTest is SupportedTypesConstants, Test {
    FHEVMExecutor internal fhevmExecutor;

    uint256 internal randomCounterForMockHandle;

    uint8 internal constant HANDLE_VERSION = 0;
    address internal constant owner = address(456);

    MockACL internal acl;
    MockFHEGasLimit internal fheGasLimit;
    MockInputVerifier internal inputVerifier;

    /// @dev Proxy and implementation variables
    address internal proxy;

    /**
     * @dev Internal function to deploy a UUPS proxy contract.
     * The proxy is deployed using the UnsafeUpgrades library and initialized with the owner address.
     */
    function _deployProxy() internal {
        proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new EmptyUUPSProxy()),
            abi.encodeCall(EmptyUUPSProxy.initialize, owner)
        );
    }

    /**
     * @dev Internal function to upgrade the deployed proxy to a new implementation.
     * The new implementation is an instance of the FHEVMExecutor contract.
     * The proxy is upgraded using the UnsafeUpgrades library and the owner address.
     */
    function _upgradeProxy(bool hasEvents) internal {
        if (hasEvents) {
            UnsafeUpgrades.upgradeProxy(
                proxy,
                address(new FHEVMExecutor()),
                abi.encodeCall(FHEVMExecutorNoEvents.reinitialize, ()),
                owner
            );
        } else {
            UnsafeUpgrades.upgradeProxy(
                proxy,
                address(new FHEVMExecutorNoEvents()),
                abi.encodeCall(FHEVMExecutorNoEvents.reinitialize, ()),
                owner
            );
        }
        fhevmExecutor = FHEVMExecutor(proxy);
    }

    function _deployMockContracts() internal {
        vm.etch(aclAdd, address(new MockACL()).code);
        vm.etch(fheGasLimitAdd, address(new MockFHEGasLimit()).code);
        vm.etch(inputVerifierAdd, address(new MockInputVerifier()).code);
        acl = MockACL(aclAdd);
        inputVerifier = MockInputVerifier(inputVerifierAdd);
    }

    function _generateMockHandle(FheType fheType) internal returns (bytes32 handle) {
        handle = _appendMetadataToPrehandle(fheType, _generateMockPrehandle(fheType), block.chainid, HANDLE_VERSION);
    }

    function _generateMockPrehandle(FheType fheType) internal returns (bytes32 preHandle) {
        preHandle = keccak256(abi.encodePacked(fheType, randomCounterForMockHandle++));
    }

    function _generateMockBytes(FheType fheType) internal returns (bytes memory) {
        assert(fheType == FheType.Uint512 || fheType == FheType.Uint1024 || fheType == FheType.Uint2048);

        uint256 length;
        if (fheType == FheType.Uint512) {
            length = 64;
        } else if (fheType == FheType.Uint1024) {
            length = 128;
        } else if (fheType == FheType.Uint2048) {
            length = 256;
        }

        bytes memory input = abi.encode(_generateMockHandle(fheType));
        uint256 inputLength = input.length;

        bytes memory result = new bytes(length);
        uint256 paddingLength = 64 - inputLength;

        for (uint256 i = 0; i < paddingLength; i++) {
            result[i] = 0;
        }

        for (uint256 i = 0; i < inputLength; i++) {
            result[paddingLength + i] = input[i];
        }

        return result;
    }

    /**
     * @dev Appends metadata to `prehandle` by modifying specific bytes.
     * - Clears bytes 21-31.
     * - Sets byte 21 to `0xff`.
     * - Inserts `chainId`, `fheType`, and `handleVersion` into respective bytes.
     * @return result Modified `prehandle` with metadata.
     */
    function _appendMetadataToPrehandle(
        FheType fheType,
        bytes32 prehandle,
        uint256 chainId,
        uint8 handleVersion
    ) internal view virtual returns (bytes32 result) {
        /// @dev Clear bytes 21-31.
        result = prehandle & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        /// @dev Set byte 21 to 0xff
        result = result | (bytes32(uint256(0xff)) << 80);
        /// @dev chainId is cast to uint64 first to make sure it does not take more than 8 bytes before shifting.
        /// If EIP2294 gets approved, it will force the chainID's size to be lower than MAX_UINT64.
        result = result | (bytes32(uint256(uint64(chainId))) << 16);
        /// @dev Insert handleType into byte 30.
        result = result | (bytes32(uint256(uint8(fheType))) << 8);
        /// @dev Insert HANDLE_VERSION into byte 31.
        result = result | bytes32(uint256(handleVersion));
    }

    function _approveHandleInACL(bytes32 handle, address account) internal {
        acl.allow(handle, account);
    }

    function _isTypeSupported(FheType fheType, uint256 supportedTypes) internal pure returns (bool) {
        if ((1 << uint8(fheType)) & supportedTypes == 0) {
            return false;
        } else {
            return true;
        }
    }

    function _computeExpectedResultUnaryOp(
        FHEVMExecutorNoEvents.Operators op,
        bytes32 handle,
        FheType resultType
    ) internal view returns (bytes32 result) {
        result = keccak256(abi.encodePacked(op, handle, acl, block.chainid));
        result = _appendMetadataToPrehandle(resultType, result, block.chainid, HANDLE_VERSION);
    }

    function _computeExpectedResultBinaryOp(
        FHEVMExecutorNoEvents.Operators op,
        bytes32 lhs,
        bytes32 rhs,
        bytes1 scalar,
        FheType resultType
    ) internal view returns (bytes32 result) {
        scalar = scalar & 0x01;
        result = keccak256(abi.encodePacked(op, lhs, rhs, scalar, acl, block.chainid));
        result = _appendMetadataToPrehandle(resultType, result, block.chainid, HANDLE_VERSION);
    }

    function _computeExpectedResultBinaryOpWithScalar(
        FHEVMExecutorNoEvents.Operators op,
        bytes32 lhs,
        bytes memory rhs,
        bytes1 scalar,
        FheType resultType
    ) internal view returns (bytes32 result) {
        scalar = scalar & 0x01;
        result = keccak256(abi.encodePacked(op, lhs, rhs, scalar, acl, block.chainid));
        result = _appendMetadataToPrehandle(resultType, result, block.chainid, HANDLE_VERSION);
    }

    function _computeExpectedResultTernaryOp(
        FHEVMExecutorNoEvents.Operators op,
        bytes32 lhs,
        bytes32 middle,
        bytes32 rhs,
        FheType middleFheType
    ) internal view returns (bytes32 result) {
        result = keccak256(abi.encodePacked(op, lhs, middle, rhs, acl, block.chainid));
        result = _appendMetadataToPrehandle(middleFheType, result, block.chainid, HANDLE_VERSION);
    }

    function upgradeProxyAndDeployMockContracts(bool withEvents) internal {
        _upgradeProxy(withEvents);
        _deployMockContracts();
    }

    /**
     * @dev Public function to set up the test environment.
     * This function deploys the proxy, upgrades it to the FHEVMExecutorNoEvents implementation.
     */
    function setUp() public {
        _deployProxy();
    }

    /**
     * @dev Tests that the contract is reinitialized correctly.
     */
    function test_PostProxyUpgradeCheck(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        // Check if the owner is set correctly
        assertEq(fhevmExecutor.owner(), owner);
        assertEq(fhevmExecutor.getInputVerifierAddress(), inputVerifierAdd);
        assertEq(fhevmExecutor.getACLAddress(), aclAdd);
        assertEq(fhevmExecutor.getFHEGasLimitAddress(), fheGasLimitAdd);
        assertEq(fhevmExecutor.getVersion(), string(abi.encodePacked("FHEVMExecutor v0.1.0")));
    }

    /// @dev This function exists for the test below to call it externally.
    function upgrade(address randomAccount) external {
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", randomAccount);
    }

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade(address randomAccount, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(randomAccount != owner);
        /// @dev Have to use external call to this to avoid this issue:
        ///      https://github.com/foundry-rs/foundry/issues/5806
        vm.expectPartialRevert(OwnableUpgradeable.OwnableUnauthorizedAccount.selector);
        this.upgrade(randomAccount);
    }

    /**
     * @dev Tests that only the owner can authorize an upgrade.
     */
    function test_OnlyOwnerCanAuthorizeUpgrade(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        /// @dev It does not revert since it called by the owner.
        UnsafeUpgrades.upgradeProxy(proxy, address(new EmptyUUPSProxy()), "", owner);
    }

    /**
     * @dev The following tests will verify that only the supported types are allowed for each operation.
     */

    function test_FheAddSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);

        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheAdd));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheAdd,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheAdd(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheAdd(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheSubSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheSub));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheSub,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheSub(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheSub(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheMulSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheMul));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheMul,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheMul(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheMul(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheDivSupportedTypesWorkAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheDiv));
        address sender = address(123);
        /// @dev The scalar byte is used in the division operation at the moment.
        bytes1 scalarByte = bytes1(0x01);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheDiv,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheDiv(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheDiv(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheRemSupportedTypesWorkAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheRem));
        address sender = address(123);
        /// @dev The scalar byte is used in the rem operation at the moment.
        bytes1 scalarByte = bytes1(0x01);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheRem,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheRem(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheRem(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheBitAndSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheBitAnd));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheBitAnd,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheBitAnd(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheBitAnd(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheBitOrSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheBitOr));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheBitOr,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheBitOr(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheBitOr(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheBitXorSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheBitXor));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheBitXor,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheBitXor(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheBitXor(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheShlSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheShl));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheShl,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheShl(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheShl(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheShrSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheShr));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheShr,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheShr(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheShr(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheRotlSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheRotl));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheRotl,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheRotl(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheRotl(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheRotrSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheRotr));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheRotr,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheRotr(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheRotr(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheEqSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheEq));
        vm.assume(fheType <= uint8(FheType.Uint256) || (scalarByte & 0x01) == 0x00);
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheEq,
            lhs,
            rhs,
            scalarByte,
            FheType.Bool /// @dev The result type is always Bool for the equality operator.
        );

        vm.prank(sender);

        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheEq(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheEq(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheEqWithBytesSupportedTypesWorkAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheEqWithBytes));
        /// @dev The second operand must be scalar.
        bytes1 scalarByte = 0x01;
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes memory rhs = _generateMockBytes(FheType(fheType));

        _approveHandleInACL(lhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOpWithScalar(
            FHEVMExecutorNoEvents.Operators.fheEq,
            lhs,
            rhs, /// @dev Since the rhs is converted to bytes without any modification, it is passed as is.
            scalarByte,
            FheType.Bool /// @dev The result type is always Bool for the equality operator.
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheEqBytes(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheEq(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheNeSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheNe));
        vm.assume(fheType <= uint8(FheType.Uint256) || (scalarByte & 0x01) == 0x00);
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheNe,
            lhs,
            rhs,
            scalarByte,
            FheType.Bool /// @dev The result type is always Bool for the non-equality operator.
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheNe(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheNe(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheNeWithBytesSupportedTypesWorkAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheNeWithBytes));
        /// @dev The second operand must be scalar.
        bytes1 scalarByte = 0x01;
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes memory rhs = _generateMockBytes(FheType(fheType));

        _approveHandleInACL(lhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOpWithScalar(
            FHEVMExecutorNoEvents.Operators.fheNe,
            lhs,
            rhs, /// @dev Since the rhs is converted to bytes without any modification, it is passed as is.
            scalarByte,
            FheType.Bool /// @dev The result type is always Bool for the non-equality operator.
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheNeBytes(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheNe(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheGeSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheGe));
        vm.assume(fheType <= uint8(FheType.Uint256) || (scalarByte & 0x01) == 0x00);
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheGe,
            lhs,
            rhs,
            scalarByte,
            FheType.Bool /// @dev The result type is always Bool for the greater than or equal operator.
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheGe(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheGe(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheGtSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheGt));
        vm.assume(fheType <= uint8(FheType.Uint256) || (scalarByte & 0x01) == 0x00);
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheGt,
            lhs,
            rhs,
            scalarByte,
            FheType.Bool /// @dev The result type is always Bool for the greater than operator.
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheGt(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheGt(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheLeSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheLe));
        vm.assume(fheType <= uint8(FheType.Uint256) || (scalarByte & 0x01) == 0x00);
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheLe,
            lhs,
            rhs,
            scalarByte,
            FheType.Bool /// @dev The result type is always Bool for the less than or equal operator.
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheLe(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheLe(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheLtSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheLt));
        vm.assume(fheType <= uint8(FheType.Uint256) || (scalarByte & 0x01) == 0x00);
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheLt,
            lhs,
            rhs,
            scalarByte,
            FheType.Bool /// @dev The result type is always Bool for the less than operator.
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheLt(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheLt(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheMinSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheMin));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheMin,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheMin(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheMin(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheMaxSupportedTypesWorkAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheMax));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultBinaryOp(
            FHEVMExecutorNoEvents.Operators.fheMax,
            lhs,
            rhs,
            scalarByte,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheMax(sender, lhs, rhs, scalarByte, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheMax(lhs, rhs, scalarByte);
        assertEq(result, expectedResult);
    }

    function test_FheNegSupportedTypesWorkAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheNeg));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);

        bytes32 expectedResult = _computeExpectedResultUnaryOp(
            FHEVMExecutorNoEvents.Operators.fheNeg,
            lhs,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheNeg(sender, lhs, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheNeg(lhs);
        assertEq(result, expectedResult);
    }

    function test_FheNotSupportedTypesWorkAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheNot));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);

        bytes32 expectedResult = _computeExpectedResultUnaryOp(
            FHEVMExecutorNoEvents.Operators.fheNot,
            lhs,
            FheType(fheType)
        );

        vm.prank(sender);

        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheNot(sender, lhs, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheNot(lhs);
        assertEq(result, expectedResult);
    }

    function test_FheIfThenElseSupportedTypesWorkAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheIfThenElse));

        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType.Bool);
        bytes32 middle = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(middle, sender);
        _approveHandleInACL(rhs, sender);

        bytes32 expectedResult = _computeExpectedResultTernaryOp(
            FHEVMExecutorNoEvents.Operators.fheIfThenElse,
            lhs,
            middle,
            rhs,
            FheType(fheType)
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.FheIfThenElse(sender, lhs, middle, rhs, expectedResult);
        }
        bytes32 result = fhevmExecutor.fheIfThenElse(lhs, middle, rhs);
        assertEq(result, expectedResult);
    }

    function test_FheRandSupportedTypesWorkAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheRand));
        address sender = address(123);

        for (uint256 i = 0; i < 30; i++) {
            /// @dev The first argument is the counterRand, which should be 0 for the first call.
            bytes16 expectedSeed = bytes16(
                keccak256(
                    abi.encodePacked(uint256(i), acl, block.chainid, blockhash(block.number - 1), block.timestamp)
                )
            );

            bytes32 expectedResult = keccak256(
                abi.encodePacked(FHEVMExecutorNoEvents.Operators.fheRand, FheType(fheType), expectedSeed)
            );

            expectedResult = _appendMetadataToPrehandle(
                FheType(fheType),
                expectedResult,
                block.chainid,
                HANDLE_VERSION
            );

            vm.prank(sender);
            if (withEvents) {
                vm.expectEmit(true, true, true, true);
                emit FHEEvents.FheRand(sender, FheType(fheType), expectedSeed, expectedResult);
            }

            bytes32 result = fhevmExecutor.fheRand(FheType(fheType));
            assertEq(result, expectedResult);
        }
    }

    function test_FheRandBoundedSupportedTypesWorkAsExpected(
        uint8 upperBoundExponent,
        uint8 fheType,
        bool withEvents
    ) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        /// @dev The upperBound must be a power of 2.
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesFheRandBounded));
        address sender = address(123);

        uint256 upperBound = 2 ** upperBoundExponent;

        for (uint256 i = 0; i < 30; i++) {
            /// @dev The first argument is the counterRand, which should be 0 for the first call.
            bytes16 expectedSeed = bytes16(
                keccak256(
                    abi.encodePacked(uint256(i), acl, block.chainid, blockhash(block.number - 1), block.timestamp)
                )
            );

            bytes32 expectedResult = keccak256(
                abi.encodePacked(
                    FHEVMExecutorNoEvents.Operators.fheRandBounded,
                    upperBound,
                    FheType(fheType),
                    expectedSeed
                )
            );

            expectedResult = _appendMetadataToPrehandle(
                FheType(fheType),
                expectedResult,
                block.chainid,
                HANDLE_VERSION
            );

            vm.prank(sender);
            if (withEvents) {
                vm.expectEmit(true, true, true, true);
                emit FHEEvents.FheRandBounded(sender, upperBound, FheType(fheType), expectedSeed, expectedResult);
            }
            bytes32 result = fhevmExecutor.fheRandBounded(upperBound, FheType(fheType));
            assertEq(result, expectedResult);
        }
    }

    function test_TrivialEncryptSupportedTypesWorkAsExpected(uint256 pt, uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesTrivialEncrypt));
        address sender = address(123);

        bytes32 expectedResult = keccak256(
            abi.encodePacked(FHEVMExecutorNoEvents.Operators.trivialEncrypt, pt, FheType(fheType), acl, block.chainid)
        );
        expectedResult = _appendMetadataToPrehandle(FheType(fheType), expectedResult, block.chainid, HANDLE_VERSION);

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.TrivialEncrypt(sender, pt, FheType(fheType), expectedResult);
        }
        bytes32 result = fhevmExecutor.trivialEncrypt(pt, FheType(fheType));
        assertEq(result, expectedResult);
    }

    function test_TrivialEncryptWithBytesSupportedTypesWorkAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesTrivialEncryptWithBytes));
        address sender = address(123);

        bytes memory pt = _generateMockBytes(FheType(fheType));
        bytes32 expectedResult = keccak256(
            abi.encodePacked(FHEVMExecutorNoEvents.Operators.trivialEncrypt, pt, FheType(fheType), acl, block.chainid)
        );
        expectedResult = _appendMetadataToPrehandle(FheType(fheType), expectedResult, block.chainid, HANDLE_VERSION);

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.TrivialEncryptBytes(sender, pt, FheType(fheType), expectedResult);
        }
        bytes32 result = fhevmExecutor.trivialEncrypt(pt, FheType(fheType));
        assertEq(result, expectedResult);
    }

    function test_CastWorksAsExpected(uint8 fheInputType, uint8 fheOutputType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheInputType <= uint8(FheType.Int248));
        vm.assume(fheOutputType <= uint8(FheType.Int248));
        vm.assume(
            _isTypeSupported(FheType(fheInputType), supportedTypesInputCast) &&
                _isTypeSupported(FheType(fheOutputType), supportedTypesOutputCast)
        );
        vm.assume(fheInputType != fheOutputType);

        address sender = address(123);
        bytes32 handle = _generateMockHandle(FheType(fheInputType));
        _approveHandleInACL(handle, sender);

        bytes32 expectedResult = keccak256(
            abi.encodePacked(FHEVMExecutorNoEvents.Operators.cast, handle, FheType(fheOutputType), acl, block.chainid)
        );

        expectedResult = _appendMetadataToPrehandle(
            FheType(fheOutputType),
            expectedResult,
            block.chainid,
            HANDLE_VERSION
        );

        vm.prank(sender);
        if (withEvents) {
            vm.expectEmit(true, true, true, true);
            emit FHEEvents.Cast(sender, handle, FheType(fheOutputType), expectedResult);
        }
        bytes32 result = fhevmExecutor.cast(handle, FheType(fheOutputType));
        assertEq(result, expectedResult);
    }

    /**
     * @dev The following tests will verify that only the supported types are allowed for each operation.
     */
    function test_FheAddNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheAdd));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheAdd(lhs, rhs, scalarByte);
    }

    function test_FheSubNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheSub));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheSub(lhs, rhs, scalarByte);
    }

    function test_FheMulNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMul));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheMul(lhs, rhs, scalarByte);
    }

    function test_FheDivNonSupportedTypesRevertAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes1 scalarByte = bytes1(0x01);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheDiv));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheDiv(lhs, rhs, scalarByte);
    }

    function test_FheRemNonSupportedTypesRevertAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes1 scalarByte = bytes1(0x01);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRem));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheRem(lhs, rhs, scalarByte);
    }

    function test_FheBitAndNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitAnd));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheBitAnd(lhs, rhs, scalarByte);
    }

    function test_FheBitOrNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitOr));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheBitOr(lhs, rhs, scalarByte);
    }

    function test_FheBitXorNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheBitXor));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheBitXor(lhs, rhs, scalarByte);
    }

    function test_FheShlNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheShl));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheShl(lhs, rhs, scalarByte);
    }

    function test_FheShrNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheShr));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheShr(lhs, rhs, scalarByte);
    }

    function test_FheRotlNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRotl));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheRotl(lhs, rhs, scalarByte);
    }

    function test_FheRotrNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRotr));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheRotr(lhs, rhs, scalarByte);
    }

    function test_FheEqNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheEq));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheEq(lhs, rhs, scalarByte);
    }

    function test_FheEqWithBytesNonSupportedTypesRevertAsExpected(
        uint8 fheType,
        bytes1 scalarByte,
        bool withEvents
    ) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheEqWithBytes));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));
        bytes memory rhsAdjusted = abi.encode(rhs);

        _approveHandleInACL(lhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheEq(lhs, rhsAdjusted, scalarByte);
    }

    function test_FheNeNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNe));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheNe(lhs, rhs, scalarByte);
    }

    function test_FheNeWithBytesNonSupportedTypesRevertAsExpected(
        uint8 fheType,
        bytes1 scalarByte,
        bool withEvents
    ) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheEqWithBytes));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));
        bytes memory rhsAdjusted = abi.encode(rhs);

        _approveHandleInACL(lhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheNe(lhs, rhsAdjusted, scalarByte);
    }

    function test_FheGeNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheGe));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheGe(lhs, rhs, scalarByte);
    }

    function test_FheGtNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheGt));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheGt(lhs, rhs, scalarByte);
    }

    function test_FheLeNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheLe));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheLe(lhs, rhs, scalarByte);
    }

    function test_FheLtNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheLt));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheLt(lhs, rhs, scalarByte);
    }

    function test_FheMinNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMin));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheMin(lhs, rhs, scalarByte);
    }

    function test_FheMaxNonSupportedTypesRevertAsExpected(uint8 fheType, bytes1 scalarByte, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheMax));
        address sender = address(123);

        bytes32 lhs = _generateMockHandle(FheType(fheType));
        bytes32 rhs = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(lhs, sender);
        _approveHandleInACL(rhs, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheMax(lhs, rhs, scalarByte);
    }

    function test_FheNotNonSupportedTypesRevertAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheNot));
        address sender = address(123);

        bytes32 handle = _generateMockHandle(FheType(fheType));
        _approveHandleInACL(handle, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheNot(handle);
    }

    function test_FheIfThenElseNonSupportedTypesRevertAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheIfThenElse));
        address sender = address(123);

        bytes32 handleControl = _generateMockHandle(FheType(fheType));
        bytes32 handleIfTrue = _generateMockHandle(FheType(fheType));
        bytes32 handleIfFalse = _generateMockHandle(FheType(fheType));

        _approveHandleInACL(handleControl, sender);
        _approveHandleInACL(handleIfTrue, sender);
        _approveHandleInACL(handleIfFalse, sender);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.fheIfThenElse(handleControl, handleIfTrue, handleIfFalse);
    }

    function test_FheRandNonSupportedTypesRevertAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRand));

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        fhevmExecutor.fheRand(FheType(fheType));
    }

    function test_FheRandBoundedNonSupportedTypesRevertAsExpected(
        uint256 upperBound,
        uint8 fheType,
        bool withEvents
    ) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesFheRandBounded));

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        fhevmExecutor.fheRandBounded(upperBound, FheType(fheType));
    }

    function test_CastNonSupportedTypesRevertAsExpected(
        uint8 fheInputType,
        uint8 fheOutputType,
        bool withEvents
    ) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheInputType <= uint8(FheType.Int248));
        vm.assume(fheOutputType <= uint8(FheType.Int248));
        vm.assume(
            !_isTypeSupported(FheType(fheInputType), supportedTypesInputCast) ||
                !_isTypeSupported(FheType(fheOutputType), supportedTypesOutputCast)
        );

        address sender = address(123);
        bytes32 handle = _generateMockHandle(FheType(fheInputType));
        _approveHandleInACL(handle, sender);
        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(sender);
        fhevmExecutor.cast(handle, FheType(fheOutputType));
    }

    function test_CastCannotCastToSameType(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        /// @dev The supported types for the output are more restrictive than the input types.
        vm.assume(_isTypeSupported(FheType(fheType), supportedTypesOutputCast));
        address sender = address(123);
        bytes32 handle = _generateMockHandle(FheType(fheType));
        _approveHandleInACL(handle, sender);
        vm.expectRevert(FHEVMExecutorNoEvents.InvalidType.selector);
        vm.prank(sender);
        fhevmExecutor.cast(handle, FheType(fheType));
    }

    function test_TrivialEncryptNotSupportedTypesRevertAsExpected(uint256 pt, uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesTrivialEncrypt));
        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        fhevmExecutor.trivialEncrypt(pt, FheType(fheType));
    }

    function test_TrivialEncryptWithBytesNotSupportedTypesRevertAsExpected(
        bytes memory pt,
        uint8 fheType,
        bool withEvents
    ) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(!_isTypeSupported(FheType(fheType), supportedTypesTrivialEncryptWithBytes));
        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        fhevmExecutor.trivialEncrypt(pt, FheType(fheType));
    }

    function test_RevertsIfACLNotAllowed_Cast(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.expectPartialRevert(FHEVMExecutorNoEvents.ACLNotAllowed.selector);
        bytes32 handle = _generateMockHandle(FheType.Uint128);
        fhevmExecutor.cast(handle, FheType.Uint64);
    }

    function test_RevertsIfACLNotAllowed_FheEqWithBytes(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.expectPartialRevert(FHEVMExecutorNoEvents.ACLNotAllowed.selector);
        bytes32 lhs = _generateMockHandle(FheType.Uint512);
        bytes memory rhs = _generateMockBytes(FheType.Uint512);
        fhevmExecutor.fheEq(lhs, rhs, 0x01);
    }

    function test_RevertsIfACLNotAllowed_FheNeWithBytes(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.expectPartialRevert(FHEVMExecutorNoEvents.ACLNotAllowed.selector);
        bytes32 lhs = _generateMockHandle(FheType.Uint512);
        bytes memory rhs = _generateMockBytes(FheType.Uint512);
        fhevmExecutor.fheNe(lhs, rhs, 0x01);
    }

    function test_RevertsIfACLNotAllowed_UnaryOp(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 handle = _generateMockHandle(FheType.Uint128);
        vm.expectPartialRevert(FHEVMExecutorNoEvents.ACLNotAllowed.selector);
        /// @dev We use fheNeg as an example of a unary operation.
        fhevmExecutor.fheNeg(handle);
    }

    function test_RevertsIfACLNotAllowed_BinaryOpLHS(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Uint16);
        bytes32 rhs = _generateMockHandle(FheType.Uint16);
        address account = address(123);
        _approveHandleInACL(rhs, account);

        vm.expectPartialRevert(FHEVMExecutorNoEvents.ACLNotAllowed.selector);
        vm.prank(account);
        /// @dev We use fheAdd as an example of a binary operation.
        fhevmExecutor.fheAdd(lhs, rhs, 0x00);
    }

    function test_RevertsIfACLNotAllowed_BinaryOpRHS(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Uint16);
        bytes32 rhs = _generateMockHandle(FheType.Uint16);
        address account = address(123);
        _approveHandleInACL(lhs, account);

        vm.expectPartialRevert(FHEVMExecutorNoEvents.ACLNotAllowed.selector);
        vm.prank(account);
        /// @dev We use fheAdd as an example of a binary operation.
        fhevmExecutor.fheAdd(lhs, rhs, 0x00);
    }

    function test_RevertsIfBinaryOpTypesNotCompatible(uint8 fheTypeLhs, uint8 fheTypeRhs, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheTypeLhs <= uint8(FheType.Int248));
        vm.assume(fheTypeRhs <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheTypeLhs), supportedTypesFheAdd));
        vm.assume(fheTypeLhs != fheTypeRhs);

        bytes32 lhs = _generateMockHandle(FheType(fheTypeLhs));
        bytes32 rhs = _generateMockHandle(FheType(fheTypeRhs));
        address account = address(123);

        _approveHandleInACL(lhs, account);
        _approveHandleInACL(rhs, account);

        vm.expectRevert(FHEVMExecutorNoEvents.IncompatibleTypes.selector);
        vm.prank(account);
        /// @dev We use fheAdd as an example of a binary operation.
        fhevmExecutor.fheAdd(lhs, rhs, 0x00);
    }

    function test_RevertsIfACLNotAllowed_TernaryOpLHS(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Bool);
        bytes32 middle = _generateMockHandle(FheType.Uint16);
        bytes32 rhs = _generateMockHandle(FheType.Uint16);
        address account = address(123);
        _approveHandleInACL(middle, account);
        _approveHandleInACL(rhs, account);

        vm.expectPartialRevert(FHEVMExecutorNoEvents.ACLNotAllowed.selector);
        vm.prank(account);
        /// @dev We use fheIfThenElse as an example of a ternary operation.
        fhevmExecutor.fheIfThenElse(lhs, middle, rhs);
    }

    function test_RevertsIfACLNotAllowed_TernaryOpMiddle(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Bool);
        bytes32 middle = _generateMockHandle(FheType.Uint16);
        bytes32 rhs = _generateMockHandle(FheType.Uint16);
        address account = address(123);
        _approveHandleInACL(lhs, account);
        _approveHandleInACL(rhs, account);

        vm.expectPartialRevert(FHEVMExecutorNoEvents.ACLNotAllowed.selector);
        vm.prank(account);
        /// @dev We use fheIfThenElse as an example of a ternary operation.
        fhevmExecutor.fheIfThenElse(lhs, middle, rhs);
    }

    function test_RevertsIfACLNotAllowed_TernaryOpRHS(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Bool);
        bytes32 middle = _generateMockHandle(FheType.Uint16);
        bytes32 rhs = _generateMockHandle(FheType.Uint16);
        address account = address(123);
        _approveHandleInACL(lhs, account);
        _approveHandleInACL(middle, account);

        vm.expectPartialRevert(FHEVMExecutorNoEvents.ACLNotAllowed.selector);
        vm.prank(account);
        /// @dev We use fheIfThenElse as an example of a ternary operation.
        fhevmExecutor.fheIfThenElse(lhs, middle, rhs);
    }

    function test_RevertsIfTernaryOpLHSIsNotBool(uint8 fheTypeLhs, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheTypeLhs <= uint8(FheType.Int248));
        vm.assume(fheTypeLhs != uint8(FheType.Bool));

        bytes32 lhs = _generateMockHandle(FheType(fheTypeLhs));
        bytes32 middle = _generateMockHandle(FheType.Uint16);
        bytes32 rhs = _generateMockHandle(FheType.Uint16);
        address account = address(123);

        _approveHandleInACL(lhs, account);
        _approveHandleInACL(middle, account);
        _approveHandleInACL(rhs, account);

        vm.expectRevert(FHEVMExecutorNoEvents.UnsupportedType.selector);
        vm.prank(account);
        /// @dev We use fheIfThenElse as an example of a ternary operation.
        fhevmExecutor.fheIfThenElse(lhs, middle, rhs);
    }

    function test_RevertsIfTernaryOpMiddleAndLHSTypesNotCompatible(
        uint8 fheTypeMiddle,
        uint8 fheTypeRhs,
        bool withEvents
    ) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheTypeMiddle <= uint8(FheType.Int248));
        vm.assume(fheTypeRhs <= uint8(FheType.Int248));
        vm.assume(_isTypeSupported(FheType(fheTypeMiddle), supportedTypesFheIfThenElse));
        vm.assume(fheTypeMiddle != fheTypeRhs);

        bytes32 lhs = _generateMockHandle(FheType.Bool);
        bytes32 middle = _generateMockHandle(FheType(fheTypeMiddle));
        bytes32 rhs = _generateMockHandle(FheType(fheTypeRhs));
        address account = address(123);

        _approveHandleInACL(lhs, account);
        _approveHandleInACL(middle, account);
        _approveHandleInACL(rhs, account);

        vm.expectRevert(FHEVMExecutorNoEvents.IncompatibleTypes.selector);
        vm.prank(account);
        /// @dev We use fheIfThenElse as an example of a ternary operation.
        fhevmExecutor.fheIfThenElse(lhs, middle, rhs);
    }

    function test_RevertsIfFheDivTriesDividingByZero(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Uint16);
        bytes32 rhs = 0;
        address account = address(123);
        _approveHandleInACL(lhs, account);

        vm.expectRevert(FHEVMExecutorNoEvents.DivisionByZero.selector);
        vm.prank(account);
        fhevmExecutor.fheDiv(lhs, rhs, 0x01);
    }

    function test_RevertsIfFheRemTriesDividingByZero(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Uint16);
        bytes32 rhs = 0;
        address account = address(123);
        _approveHandleInACL(lhs, account);

        vm.expectRevert(FHEVMExecutorNoEvents.DivisionByZero.selector);
        vm.prank(account);
        fhevmExecutor.fheRem(lhs, rhs, 0x01);
    }

    function test_RevertsIfFheDivRHSIsNotScalar(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Uint16);
        bytes32 rhs = _generateMockHandle(FheType.Uint16);
        address account = address(123);
        _approveHandleInACL(lhs, account);
        _approveHandleInACL(rhs, account);

        vm.expectRevert(FHEVMExecutorNoEvents.IsNotScalar.selector);
        vm.prank(account);
        fhevmExecutor.fheDiv(lhs, rhs, 0x00);
    }

    function test_RevertsIfFheRemRHSIsNotScalar(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Uint16);
        bytes32 rhs = _generateMockHandle(FheType.Uint16);
        address account = address(123);
        _approveHandleInACL(lhs, account);
        _approveHandleInACL(rhs, account);

        vm.expectRevert(FHEVMExecutorNoEvents.IsNotScalar.selector);
        vm.prank(account);
        fhevmExecutor.fheRem(lhs, rhs, 0x00);
    }

    function test_RevertsIfFheEqWithBytesRHSIsNotScalar(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Uint512);
        bytes memory rhs = _generateMockBytes(FheType.Uint512);
        address account = address(123);
        _approveHandleInACL(lhs, account);

        vm.expectRevert(FHEVMExecutorNoEvents.SecondOperandIsNotScalar.selector);
        vm.prank(account);
        fhevmExecutor.fheEq(lhs, rhs, 0x00);
    }

    function test_RevertsIfFheNeWithBytesRHSIsNotScalar(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Uint512);
        bytes memory rhs = _generateMockBytes(FheType.Uint512);
        address account = address(123);
        _approveHandleInACL(lhs, account);

        vm.expectRevert(FHEVMExecutorNoEvents.SecondOperandIsNotScalar.selector);
        vm.prank(account);
        fhevmExecutor.fheNe(lhs, rhs, 0x00);
    }

    function test_RevertsIfFheEqRHSIsScalarForEuint512AndAbove(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Uint512);
        bytes32 rhs = _generateMockHandle(FheType.Uint512);
        address account = address(123);
        _approveHandleInACL(lhs, account);
        _approveHandleInACL(rhs, account);

        vm.expectRevert(FHEVMExecutorNoEvents.IsScalar.selector);
        vm.prank(account);
        fhevmExecutor.fheEq(lhs, rhs, 0x01);
    }

    function test_RevertsIfFheNeRHSIsScalarEuint512AndAbove(bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        bytes32 lhs = _generateMockHandle(FheType.Uint512);
        bytes32 rhs = _generateMockHandle(FheType.Uint512);
        address account = address(123);
        _approveHandleInACL(lhs, account);
        _approveHandleInACL(rhs, account);

        vm.expectRevert(FHEVMExecutorNoEvents.IsScalar.selector);
        vm.prank(account);
        fhevmExecutor.fheNe(lhs, rhs, 0x01);
    }

    function test_RevertsIfUpperBoundIsNotPowerOfTwo(uint256 upperBound, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(upperBound > 0 && ((upperBound & (upperBound - 1)) != 0));
        vm.expectRevert(FHEVMExecutorNoEvents.NotPowerOfTwo.selector);
        fhevmExecutor.fheRandBounded(upperBound, FheType.Uint16);
    }

    function test_VerifyCiphertextWorksIfInputTypeIsAsExpected(uint8 fheType, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        address userAddress = address(123);
        bytes memory mockInputProof = abi.encode("mockProof");
        bytes32 inputHandle = _generateMockHandle(FheType(fheType));
        bytes32 result = fhevmExecutor.verifyCiphertext(inputHandle, userAddress, mockInputProof, FheType(fheType));
        assertEq(result, inputHandle);
    }

    function test_VerifyCiphertextWorksIfInputTypeIsNotAsExpected(
        uint8 fheType,
        uint8 otherFheType,
        bool withEvents
    ) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(fheType <= uint8(FheType.Int248));
        vm.assume(otherFheType <= uint8(FheType.Int248));
        vm.assume(fheType != otherFheType);

        address userAddress = address(123);
        bytes memory mockInputProof = abi.encode("mockProof");
        bytes32 inputHandle = _generateMockHandle(FheType(fheType));
        vm.expectRevert(FHEVMExecutorNoEvents.InvalidType.selector);
        fhevmExecutor.verifyCiphertext(inputHandle, userAddress, mockInputProof, FheType(otherFheType));
    }

    function test_RevertsIfInvalidByteLengthForUint512(uint16 byteLength, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(byteLength != 64);
        bytes memory mockPlainText = new bytes(byteLength);
        vm.expectPartialRevert(FHEVMExecutorNoEvents.InvalidByteLength.selector);
        fhevmExecutor.trivialEncrypt(mockPlainText, FheType.Uint512);
    }

    function test_RevertsIfInvalidByteLengthForUint1024(uint16 byteLength, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(byteLength != 128);
        bytes memory mockPlainText = new bytes(byteLength);
        vm.expectPartialRevert(FHEVMExecutorNoEvents.InvalidByteLength.selector);
        fhevmExecutor.trivialEncrypt(mockPlainText, FheType.Uint1024);
    }

    function test_RevertsIfInvalidByteLengthForUint2048(uint16 byteLength, bool withEvents) public {
        upgradeProxyAndDeployMockContracts(withEvents);
        vm.assume(byteLength != 256);
        bytes memory mockPlainText = new bytes(byteLength);
        vm.expectPartialRevert(FHEVMExecutorNoEvents.InvalidByteLength.selector);
        fhevmExecutor.trivialEncrypt(mockPlainText, FheType.Uint2048);
    }
}
