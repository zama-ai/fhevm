// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "encrypted-types/EncryptedTypes.sol";
import {FHE} from "../lib/FHE.sol";
import {CoprocessorConfig} from "../lib/Impl.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "@fhevm-host-contracts/contracts/FHEVMExecutor.sol";
import {FheType} from "@fhevm-host-contracts/contracts/shared/FheType.sol";
import {aclAdd, fhevmExecutorAdd, kmsVerifierAdd} from "@fhevm-host-contracts/addresses/FHEVMHostAddresses.sol";

/// @dev Accept raw bytes to exercise Solidity wrapping across an untrusted boundary.
contract TypeSafetyAdapter {
    constructor() {
        FHE.setCoprocessor(CoprocessorConfig(aclAdd, fhevmExecutorAdd, kmsVerifierAdd));
    }

    function mint(FheType fheType, uint256 value) external returns (bytes32 handle) {
        handle = FHEVMExecutor(fhevmExecutorAdd).trivialEncrypt(value, fheType);
        ACL(aclAdd).allow(handle, address(this));
        ACL(aclAdd).allow(handle, msg.sender);
    }

    function isAllowed(FheType expectedType, bytes32 handle, address account) external view returns (bool) {
        if (expectedType == FheType.Bool) return FHE.isAllowed(ebool.wrap(handle), account);
        if (expectedType == FheType.Uint8) return FHE.isAllowed(euint8.wrap(handle), account);
        if (expectedType == FheType.Uint16) return FHE.isAllowed(euint16.wrap(handle), account);
        if (expectedType == FheType.Uint32) return FHE.isAllowed(euint32.wrap(handle), account);
        if (expectedType == FheType.Uint64) return FHE.isAllowed(euint64.wrap(handle), account);
        if (expectedType == FheType.Uint128) return FHE.isAllowed(euint128.wrap(handle), account);
        if (expectedType == FheType.Uint160) return FHE.isAllowed(eaddress.wrap(handle), account);
        if (expectedType == FheType.Uint256) return FHE.isAllowed(euint256.wrap(handle), account);
        revert("Unsupported test type");
    }

    function isSenderAllowed(FheType expectedType, bytes32 handle) external view returns (bool) {
        if (expectedType == FheType.Bool) return FHE.isSenderAllowed(ebool.wrap(handle));
        if (expectedType == FheType.Uint8) return FHE.isSenderAllowed(euint8.wrap(handle));
        if (expectedType == FheType.Uint16) return FHE.isSenderAllowed(euint16.wrap(handle));
        if (expectedType == FheType.Uint32) return FHE.isSenderAllowed(euint32.wrap(handle));
        if (expectedType == FheType.Uint64) return FHE.isSenderAllowed(euint64.wrap(handle));
        if (expectedType == FheType.Uint128) return FHE.isSenderAllowed(euint128.wrap(handle));
        if (expectedType == FheType.Uint160) return FHE.isSenderAllowed(eaddress.wrap(handle));
        if (expectedType == FheType.Uint256) return FHE.isSenderAllowed(euint256.wrap(handle));
        revert("Unsupported test type");
    }

    function isPubliclyDecryptable(FheType expectedType, bytes32 handle) external view returns (bool) {
        if (expectedType == FheType.Bool) return FHE.isPubliclyDecryptable(ebool.wrap(handle));
        if (expectedType == FheType.Uint8) return FHE.isPubliclyDecryptable(euint8.wrap(handle));
        if (expectedType == FheType.Uint16) return FHE.isPubliclyDecryptable(euint16.wrap(handle));
        if (expectedType == FheType.Uint32) return FHE.isPubliclyDecryptable(euint32.wrap(handle));
        if (expectedType == FheType.Uint64) return FHE.isPubliclyDecryptable(euint64.wrap(handle));
        if (expectedType == FheType.Uint128) return FHE.isPubliclyDecryptable(euint128.wrap(handle));
        if (expectedType == FheType.Uint160) return FHE.isPubliclyDecryptable(eaddress.wrap(handle));
        if (expectedType == FheType.Uint256) return FHE.isPubliclyDecryptable(euint256.wrap(handle));
        revert("Unsupported test type");
    }

    function toBytes32(FheType expectedType, bytes32 handle) external view returns (bytes32) {
        if (expectedType == FheType.Bool) return FHE.toBytes32(ebool.wrap(handle));
        if (expectedType == FheType.Uint8) return FHE.toBytes32(euint8.wrap(handle));
        if (expectedType == FheType.Uint16) return FHE.toBytes32(euint16.wrap(handle));
        if (expectedType == FheType.Uint32) return FHE.toBytes32(euint32.wrap(handle));
        if (expectedType == FheType.Uint64) return FHE.toBytes32(euint64.wrap(handle));
        if (expectedType == FheType.Uint128) return FHE.toBytes32(euint128.wrap(handle));
        if (expectedType == FheType.Uint160) return FHE.toBytes32(eaddress.wrap(handle));
        if (expectedType == FheType.Uint256) return FHE.toBytes32(euint256.wrap(handle));
        revert("Unsupported test type");
    }

    function importHandle(FheType expectedType, bytes32 handle, bytes memory proof) external returns (bytes32) {
        if (expectedType == FheType.Bool) return ebool.unwrap(FHE.fromExternal(externalEbool.wrap(handle), proof));
        if (expectedType == FheType.Uint8) return euint8.unwrap(FHE.fromExternal(externalEuint8.wrap(handle), proof));
        if (expectedType == FheType.Uint16)
            return euint16.unwrap(FHE.fromExternal(externalEuint16.wrap(handle), proof));
        if (expectedType == FheType.Uint32)
            return euint32.unwrap(FHE.fromExternal(externalEuint32.wrap(handle), proof));
        if (expectedType == FheType.Uint64)
            return euint64.unwrap(FHE.fromExternal(externalEuint64.wrap(handle), proof));
        if (expectedType == FheType.Uint128)
            return euint128.unwrap(FHE.fromExternal(externalEuint128.wrap(handle), proof));
        if (expectedType == FheType.Uint160)
            return eaddress.unwrap(FHE.fromExternal(externalEaddress.wrap(handle), proof));
        if (expectedType == FheType.Uint256)
            return euint256.unwrap(FHE.fromExternal(externalEuint256.wrap(handle), proof));
        revert("Unsupported test type");
    }

    function add32(bytes32 a, bytes32 b) external returns (bytes32) {
        return euint32.unwrap(FHE.add(euint32.wrap(a), euint32.wrap(b)));
    }

    function andBool(bytes32 a, bytes32 b) external returns (bytes32) {
        return ebool.unwrap(FHE.and(ebool.wrap(a), ebool.wrap(b)));
    }

    function eqAddresses(bytes32 a, bytes32 b) external returns (bytes32) {
        return ebool.unwrap(FHE.eq(eaddress.wrap(a), eaddress.wrap(b)));
    }

    function addScalar32(bytes32 a, uint32 b) external returns (bytes32) {
        return euint32.unwrap(FHE.add(euint32.wrap(a), b));
    }

    function mulDivEncrypted(FheType fheType, bytes32 a, bytes32 b, uint64 divisor) external returns (bytes32) {
        if (fheType == FheType.Uint8) return euint8.unwrap(FHE.mulDiv(euint8.wrap(a), euint8.wrap(b), uint8(divisor)));
        if (fheType == FheType.Uint16)
            return euint16.unwrap(FHE.mulDiv(euint16.wrap(a), euint16.wrap(b), uint16(divisor)));
        if (fheType == FheType.Uint32)
            return euint32.unwrap(FHE.mulDiv(euint32.wrap(a), euint32.wrap(b), uint32(divisor)));
        if (fheType == FheType.Uint64) return euint64.unwrap(FHE.mulDiv(euint64.wrap(a), euint64.wrap(b), divisor));
        revert("Unsupported mulDiv type");
    }

    function mulDivScalar(FheType fheType, bytes32 a, uint64 b, uint64 divisor) external returns (bytes32) {
        if (fheType == FheType.Uint8) return euint8.unwrap(FHE.mulDiv(euint8.wrap(a), uint8(b), uint8(divisor)));
        if (fheType == FheType.Uint16) return euint16.unwrap(FHE.mulDiv(euint16.wrap(a), uint16(b), uint16(divisor)));
        if (fheType == FheType.Uint32) return euint32.unwrap(FHE.mulDiv(euint32.wrap(a), uint32(b), uint32(divisor)));
        if (fheType == FheType.Uint64) return euint64.unwrap(FHE.mulDiv(euint64.wrap(a), b, divisor));
        revert("Unsupported mulDiv type");
    }

    function subScalar32(uint32 a, bytes32 b) external returns (bytes32) {
        return euint32.unwrap(FHE.sub(a, euint32.wrap(b)));
    }

    function eqAddress(bytes32 a, address b) external returns (bytes32) {
        return ebool.unwrap(FHE.eq(eaddress.wrap(a), b));
    }

    function widenTo64(bytes32 a) external returns (bytes32) {
        return euint64.unwrap(FHE.asEuint64(euint32.wrap(a)));
    }

    function mixedAdd(bytes32 a, bytes32 b) external returns (bytes32) {
        return euint32.unwrap(FHE.add(euint16.wrap(a), euint32.wrap(b)));
    }

    function mixedAddReverse(bytes32 a, bytes32 b) external returns (bytes32) {
        return euint32.unwrap(FHE.add(euint32.wrap(a), euint16.wrap(b)));
    }

    function shift8(bytes32 a, bytes32 b) external returns (bytes32) {
        return euint8.unwrap(FHE.shl(euint8.wrap(a), euint8.wrap(b)));
    }

    function shift32(bytes32 a, bytes32 b) external returns (bytes32) {
        return euint32.unwrap(FHE.shl(euint32.wrap(a), euint8.wrap(b)));
    }

    function not32(bytes32 a) external returns (bytes32) {
        return euint32.unwrap(FHE.not(euint32.wrap(a)));
    }

    function toBool(bytes32 a) external returns (bytes32) {
        return ebool.unwrap(FHE.asEbool(euint32.wrap(a)));
    }

    function fromBool(bytes32 a) external returns (bytes32) {
        return euint32.unwrap(FHE.asEuint32(ebool.wrap(a)));
    }

    function select32(bytes32 condition, bytes32 a, bytes32 b) external returns (bytes32) {
        return euint32.unwrap(FHE.select(ebool.wrap(condition), euint32.wrap(a), euint32.wrap(b)));
    }

    function selectBool(bytes32 condition, bytes32 a, bytes32 b) external returns (bytes32) {
        return ebool.unwrap(FHE.select(ebool.wrap(condition), ebool.wrap(a), ebool.wrap(b)));
    }

    function allow32(bytes32 a) external returns (bytes32) {
        return euint32.unwrap(FHE.allowThis(euint32.wrap(a)));
    }

    function sum32(bytes32 a, bytes32 b) external returns (bytes32) {
        euint32[] memory values = new euint32[](2);
        values[0] = euint32.wrap(a);
        values[1] = euint32.wrap(b);
        return euint32.unwrap(FHE.sum(values));
    }

    function isIn32(bytes32 a, bytes32 b) external returns (bytes32) {
        euint32[] memory values = new euint32[](1);
        values[0] = euint32.wrap(b);
        return ebool.unwrap(FHE.isIn(euint32.wrap(a), values));
    }
}

contract TypeSafetySmartAccount {
    function forward(
        TypeSafetyAdapter adapter,
        FheType fheType
    ) external returns (bytes32 original, bytes32 forwarded) {
        original = adapter.mint(fheType, 1);
        forwarded = adapter.importHandle(fheType, original, "");
    }
}

contract FHETypeSafetyTest is HostContractsDeployerTestUtils {
    TypeSafetyAdapter internal adapter;
    FHEVMExecutor internal executor;
    address internal constant OWNER = address(0xAA11);

    function setUp() public {
        vm.warp(1_000_000);
        // Exercise the real executor, ACL, and HCU contracts behind their proxies.
        _deployACL(OWNER);
        _deployFHEVMExecutor(OWNER);
        _deployHCULimit(OWNER);
        executor = FHEVMExecutor(fhevmExecutorAdd);
        adapter = new TypeSafetyAdapter();
    }

    function _types() internal pure returns (FheType[8] memory) {
        return [
            FheType.Bool,
            FheType.Uint8,
            FheType.Uint16,
            FheType.Uint32,
            FheType.Uint64,
            FheType.Uint128,
            FheType.Uint160,
            FheType.Uint256
        ];
    }

    function test_FromExternalRejectsEveryMismatchedTypePair() public {
        FheType[8] memory types = _types();
        for (uint256 actual; actual < types.length; ++actual) {
            bytes32 handle = adapter.mint(types[actual], 1);
            for (uint256 expected; expected < types.length; ++expected) {
                if (actual != expected) vm.expectRevert(FHEVMExecutor.InvalidType.selector);
                bytes32 imported = adapter.importHandle(types[expected], handle, "");
                if (actual == expected) assertEq(imported, handle);
            }
        }
    }

    function test_ACLViewsRejectEveryMismatchedTypePair() public {
        FheType[8] memory types = _types();
        bytes32[] memory handles = new bytes32[](1);
        for (uint256 actual; actual < types.length; ++actual) {
            bytes32 handle = adapter.mint(types[actual], 1);
            handles[0] = handle;
            ACL(aclAdd).allowForDecryption(handles);
            for (uint256 expected; expected < types.length; ++expected) {
                if (actual != expected) vm.expectRevert(FHEVMExecutor.InvalidType.selector);
                bool allowed = adapter.isAllowed(types[expected], handle, address(this));
                if (actual == expected) assertTrue(allowed);

                if (actual != expected) vm.expectRevert(FHEVMExecutor.InvalidType.selector);
                bool senderAllowed = adapter.isSenderAllowed(types[expected], handle);
                if (actual == expected) assertTrue(senderAllowed);

                if (actual != expected) vm.expectRevert(FHEVMExecutor.InvalidType.selector);
                bool publiclyDecryptable = adapter.isPubliclyDecryptable(types[expected], handle);
                if (actual == expected) assertTrue(publiclyDecryptable);
            }
        }
    }

    function test_ToBytes32RejectsEveryMismatchedTypePair() public {
        FheType[8] memory types = _types();
        for (uint256 actual; actual < types.length; ++actual) {
            bytes32 handle = adapter.mint(types[actual], 1);
            for (uint256 expected; expected < types.length; ++expected) {
                if (actual != expected) vm.expectRevert(FHEVMExecutor.InvalidType.selector);
                bytes32 converted = adapter.toBytes32(types[expected], handle);
                if (actual == expected) assertEq(converted, handle);
            }
        }
    }

    function test_TypedViewsPreserveZeroSentinelForEveryType() public view {
        FheType[8] memory types = _types();
        for (uint256 i; i < types.length; ++i) {
            assertFalse(adapter.isAllowed(types[i], bytes32(0), address(this)));
            assertFalse(adapter.isSenderAllowed(types[i], bytes32(0)));
            assertFalse(adapter.isPubliclyDecryptable(types[i], bytes32(0)));
            assertEq(adapter.toBytes32(types[i], bytes32(0)), bytes32(0));
        }
    }

    function test_TypedViewsDoNotGrantPermissions() public {
        FheType[8] memory types = _types();
        address outsider = address(0xBAD);
        for (uint256 i; i < types.length; ++i) {
            bytes32 handle = adapter.mint(types[i], 1);
            assertFalse(adapter.isAllowed(types[i], handle, outsider));
            vm.prank(outsider);
            assertFalse(adapter.isSenderAllowed(types[i], handle));
            assertFalse(adapter.isPubliclyDecryptable(types[i], handle));
            vm.prank(outsider);
            assertEq(adapter.toBytes32(types[i], handle), handle);
            assertFalse(ACL(aclAdd).isAllowed(handle, outsider));
        }
    }

    function test_FromExternalPreservesZeroSentinelForEveryType() public {
        FheType[8] memory types = _types();
        for (uint256 i; i < types.length; ++i) {
            bytes32 imported = adapter.importHandle(types[i], bytes32(0), "");
            assertNotEq(imported, bytes32(0));
            executor.checkHandleType(imported, types[i]);
        }
    }

    function test_SmartAccountCanForwardEveryMatchingType() public {
        TypeSafetySmartAccount account = new TypeSafetySmartAccount();
        FheType[8] memory types = _types();
        for (uint256 i; i < types.length; ++i) {
            (bytes32 original, bytes32 forwarded) = account.forward(adapter, types[i]);
            assertEq(original, forwarded);
        }
    }

    function test_TypeValidationDoesNotReplaceSenderAuthorization() public {
        bytes32 handle = adapter.mint(FheType.Uint32, 1);
        address outsider = address(0xBAD);
        vm.expectRevert(abi.encodeWithSelector(FHE.SenderNotAllowedToUseHandle.selector, handle, outsider));
        vm.prank(outsider);
        adapter.importHandle(FheType.Uint32, handle, "");
    }

    function test_BoolCannotBeImportedOrComparedAsAddress() public {
        bytes32 poison = adapter.mint(FheType.Bool, 1);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.importHandle(FheType.Uint160, poison, "");
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.eqAddress(poison, address(0xBEEF));
        // Even a scalar that fits Bool cannot bypass the library's type assertion.
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.eqAddress(poison, address(1));
    }

    function test_BinaryOperationsRejectMatchingButWrongUnderlyingTypes() public {
        bytes32 wrong = adapter.mint(FheType.Uint16, 1);
        bytes32 right = adapter.mint(FheType.Uint32, 2);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.add32(wrong, wrong);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.add32(wrong, right);
        vm.expectRevert(FHEVMExecutor.IncompatibleTypes.selector);
        adapter.add32(right, wrong);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.addScalar32(wrong, 1);
        vm.expectRevert(FHEVMExecutor.IncompatibleTypes.selector);
        adapter.subScalar32(1, wrong);
    }

    function test_CastsAndWideningValidateOriginalSourceTypes() public {
        bytes32 u8 = adapter.mint(FheType.Uint8, 1);
        bytes32 u16 = adapter.mint(FheType.Uint16, 1);
        bytes32 u32 = adapter.mint(FheType.Uint32, 1);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.widenTo64(u16);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.mixedAdd(u8, u32);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.shift32(u32, u16);
        vm.expectRevert(FHEVMExecutor.IncompatibleTypes.selector);
        adapter.shift32(u16, u8);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.toBool(u16);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.fromBool(u8);
    }

    function test_UnarySelectAndAllowanceValidateTheirArguments() public {
        bytes32 condition = adapter.mint(FheType.Bool, 1);
        bytes32 wrong = adapter.mint(FheType.Uint16, 1);
        bytes32 right = adapter.mint(FheType.Uint32, 1);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.not32(wrong);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.allow32(wrong);
        vm.expectRevert(FHEVMExecutor.UnsupportedType.selector);
        adapter.select32(wrong, right, right);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.select32(condition, wrong, wrong);
        vm.expectRevert(FHEVMExecutor.IncompatibleTypes.selector);
        adapter.select32(condition, right, wrong);
    }

    function _assertCallType(bytes memory data, bool shouldSucceed, FheType expectedType) internal {
        (bool success, bytes memory result) = address(adapter).call(data);
        assertEq(success, shouldSucceed);
        if (success) executor.checkHandleType(abi.decode(result, (bytes32)), expectedType);
    }

    function test_BinaryOperationsEnforceBothDeclaredTypesForEveryTypePair() public {
        FheType[8] memory types = _types();
        bytes32[8] memory handles;
        for (uint256 i; i < types.length; ++i) handles[i] = adapter.mint(types[i], 1);

        for (uint256 i; i < types.length; ++i) {
            for (uint256 j; j < types.length; ++j) {
                bytes32 a = handles[i];
                bytes32 b = handles[j];
                _assertCallType(
                    abi.encodeCall(adapter.andBool, (a, b)),
                    types[i] == FheType.Bool && types[j] == FheType.Bool,
                    FheType.Bool
                );
                _assertCallType(
                    abi.encodeCall(adapter.eqAddresses, (a, b)),
                    types[i] == FheType.Uint160 && types[j] == FheType.Uint160,
                    FheType.Bool
                );
                _assertCallType(
                    abi.encodeCall(adapter.add32, (a, b)),
                    types[i] == FheType.Uint32 && types[j] == FheType.Uint32,
                    FheType.Uint32
                );
                _assertCallType(
                    abi.encodeCall(adapter.mixedAdd, (a, b)),
                    types[i] == FheType.Uint16 && types[j] == FheType.Uint32,
                    FheType.Uint32
                );
                _assertCallType(
                    abi.encodeCall(adapter.mixedAddReverse, (a, b)),
                    types[i] == FheType.Uint32 && types[j] == FheType.Uint16,
                    FheType.Uint32
                );
                _assertCallType(
                    abi.encodeCall(adapter.shift8, (a, b)),
                    types[i] == FheType.Uint8 && types[j] == FheType.Uint8,
                    FheType.Uint8
                );
                _assertCallType(
                    abi.encodeCall(adapter.shift32, (a, b)),
                    types[i] == FheType.Uint32 && types[j] == FheType.Uint8,
                    FheType.Uint32
                );
            }
        }
    }

    function test_SelectAndScalarLeftEnforceDeclaredTypes() public {
        FheType[8] memory types = _types();
        bytes32 boolean = adapter.mint(FheType.Bool, 1);
        bytes32 u32 = adapter.mint(FheType.Uint32, 1);
        for (uint256 i; i < types.length; ++i) {
            bytes32 value = adapter.mint(types[i], 1);
            bool isBool = types[i] == FheType.Bool;
            bool isUint32 = types[i] == FheType.Uint32;
            _assertCallType(abi.encodeCall(adapter.subScalar32, (1, value)), isUint32, FheType.Uint32);
            _assertCallType(abi.encodeCall(adapter.select32, (value, u32, u32)), isBool, FheType.Uint32);
            _assertCallType(abi.encodeCall(adapter.select32, (boolean, value, value)), isUint32, FheType.Uint32);
            _assertCallType(abi.encodeCall(adapter.select32, (boolean, value, u32)), isUint32, FheType.Uint32);
            _assertCallType(abi.encodeCall(adapter.select32, (boolean, u32, value)), isUint32, FheType.Uint32);
            _assertCallType(abi.encodeCall(adapter.selectBool, (value, boolean, boolean)), isBool, FheType.Bool);
            _assertCallType(abi.encodeCall(adapter.selectBool, (boolean, value, value)), isBool, FheType.Bool);
            _assertCallType(abi.encodeCall(adapter.selectBool, (boolean, value, boolean)), isBool, FheType.Bool);
            _assertCallType(abi.encodeCall(adapter.selectBool, (boolean, boolean, value)), isBool, FheType.Bool);
        }
    }

    function test_EachOperandCanBeUninitialized() public {
        bytes32 boolean = adapter.mint(FheType.Bool, 1);
        bytes32 u8 = adapter.mint(FheType.Uint8, 1);
        bytes32 u16 = adapter.mint(FheType.Uint16, 1);
        bytes32 u32 = adapter.mint(FheType.Uint32, 1);
        bytes32 account = adapter.mint(FheType.Uint160, 1);
        executor.checkHandleType(adapter.andBool(0, boolean), FheType.Bool);
        executor.checkHandleType(adapter.andBool(boolean, 0), FheType.Bool);
        executor.checkHandleType(adapter.eqAddresses(0, account), FheType.Bool);
        executor.checkHandleType(adapter.eqAddresses(account, 0), FheType.Bool);
        executor.checkHandleType(adapter.add32(0, u32), FheType.Uint32);
        executor.checkHandleType(adapter.add32(u32, 0), FheType.Uint32);
        executor.checkHandleType(adapter.mixedAdd(0, u32), FheType.Uint32);
        executor.checkHandleType(adapter.mixedAdd(u16, 0), FheType.Uint32);
        executor.checkHandleType(adapter.mixedAddReverse(0, u16), FheType.Uint32);
        executor.checkHandleType(adapter.mixedAddReverse(u32, 0), FheType.Uint32);
        executor.checkHandleType(adapter.shift8(0, u8), FheType.Uint8);
        executor.checkHandleType(adapter.shift8(u8, 0), FheType.Uint8);
        executor.checkHandleType(adapter.shift32(0, u8), FheType.Uint32);
        executor.checkHandleType(adapter.shift32(u32, 0), FheType.Uint32);
        executor.checkHandleType(adapter.addScalar32(0, 1), FheType.Uint32);
        executor.checkHandleType(adapter.subScalar32(1, 0), FheType.Uint32);
        executor.checkHandleType(adapter.select32(0, u32, u32), FheType.Uint32);
        executor.checkHandleType(adapter.select32(boolean, 0, u32), FheType.Uint32);
        executor.checkHandleType(adapter.select32(boolean, u32, 0), FheType.Uint32);
        executor.checkHandleType(adapter.selectBool(0, boolean, boolean), FheType.Bool);
        executor.checkHandleType(adapter.selectBool(boolean, 0, boolean), FheType.Bool);
        executor.checkHandleType(adapter.selectBool(boolean, boolean, 0), FheType.Bool);
    }

    function test_CollectionsUseExistingExpectedTypeChecks() public {
        bytes32 wrong = adapter.mint(FheType.Uint16, 1);
        bytes32 right = adapter.mint(FheType.Uint32, 1);
        vm.expectRevert(FHEVMExecutor.IncompatibleTypes.selector);
        adapter.sum32(wrong, wrong);
        vm.expectRevert(FHEVMExecutor.IncompatibleTypes.selector);
        adapter.sum32(right, wrong);
        vm.expectRevert(FHEVMExecutor.IncompatibleTypes.selector);
        adapter.isIn32(wrong, wrong);
        vm.expectRevert(FHEVMExecutor.IncompatibleTypes.selector);
        adapter.isIn32(right, wrong);
    }

    function test_MulDivEnforcesDeclaredTypesForEveryTypePair() public {
        FheType[4] memory mulDivTypes = [FheType.Uint8, FheType.Uint16, FheType.Uint32, FheType.Uint64];
        FheType[8] memory types = _types();
        bytes32[8] memory handles;
        for (uint256 i; i < types.length; ++i) handles[i] = adapter.mint(types[i], 1);

        for (uint256 t; t < mulDivTypes.length; ++t) {
            FheType expected = mulDivTypes[t];
            for (uint256 i; i < types.length; ++i) {
                // Small scalars also fit narrower underlying types, so the type assertion is essential.
                _assertCallType(
                    abi.encodeCall(adapter.mulDivScalar, (expected, handles[i], 1, 1)),
                    types[i] == expected,
                    expected
                );
                for (uint256 j; j < types.length; ++j) {
                    _assertCallType(
                        abi.encodeCall(adapter.mulDivEncrypted, (expected, handles[i], handles[j], 1)),
                        types[i] == expected && types[j] == expected,
                        expected
                    );
                }
            }
        }
    }

    function test_MulDivUint8PreservesUninitializedOperandsAndScalarBoundaries() public {
        _checkMulDivUninitializedOperandsAndScalarBoundaries(FheType.Uint8, type(uint8).max);
    }

    function test_MulDivUint16PreservesUninitializedOperandsAndScalarBoundaries() public {
        _checkMulDivUninitializedOperandsAndScalarBoundaries(FheType.Uint16, type(uint16).max);
    }

    function test_MulDivUint32PreservesUninitializedOperandsAndScalarBoundaries() public {
        _checkMulDivUninitializedOperandsAndScalarBoundaries(FheType.Uint32, type(uint32).max);
    }

    function test_MulDivUint64PreservesUninitializedOperandsAndScalarBoundaries() public {
        _checkMulDivUninitializedOperandsAndScalarBoundaries(FheType.Uint64, type(uint64).max);
    }

    function _checkMulDivUninitializedOperandsAndScalarBoundaries(FheType fheType, uint64 maximum) internal {
        // Each width runs in its own transaction to stay within the real HCU budget.
        bytes32 handle = adapter.mint(fheType, 1);
        executor.checkHandleType(adapter.mulDivEncrypted(fheType, 0, handle, 1), fheType);
        executor.checkHandleType(adapter.mulDivEncrypted(fheType, handle, 0, 1), fheType);
        executor.checkHandleType(adapter.mulDivEncrypted(fheType, 0, 0, 1), fheType);
        executor.checkHandleType(adapter.mulDivScalar(fheType, 0, 1, 1), fheType);
        executor.checkHandleType(adapter.mulDivScalar(fheType, handle, 0, 1), fheType);
        executor.checkHandleType(adapter.mulDivScalar(fheType, handle, maximum, maximum), fheType);
        vm.expectRevert(FHEVMExecutor.DivisionByZero.selector);
        adapter.mulDivEncrypted(fheType, handle, handle, 0);
        vm.expectRevert(FHEVMExecutor.DivisionByZero.selector);
        adapter.mulDivScalar(fheType, handle, 1, 0);
    }

    function test_ValidOperationsAndUninitializedArgumentsRemainSupported() public {
        bytes32 u8 = adapter.mint(FheType.Uint8, 70);
        bytes32 u16 = adapter.mint(FheType.Uint16, 7);
        bytes32 u32 = adapter.mint(FheType.Uint32, 42);
        bytes32 condition = adapter.mint(FheType.Bool, 1);
        executor.checkHandleType(adapter.add32(u32, u32), FheType.Uint32);
        executor.checkHandleType(adapter.addScalar32(u32, 1), FheType.Uint32);
        executor.checkHandleType(adapter.subScalar32(100, u32), FheType.Uint32);
        executor.checkHandleType(adapter.widenTo64(u32), FheType.Uint64);
        executor.checkHandleType(adapter.mixedAdd(u16, u32), FheType.Uint32);
        executor.checkHandleType(adapter.shift32(u32, u8), FheType.Uint32);
        executor.checkHandleType(adapter.not32(u32), FheType.Uint32);
        executor.checkHandleType(adapter.toBool(u32), FheType.Bool);
        executor.checkHandleType(adapter.fromBool(condition), FheType.Uint32);
        executor.checkHandleType(adapter.select32(condition, u32, u32), FheType.Uint32);
        executor.checkHandleType(adapter.allow32(u32), FheType.Uint32);
        executor.checkHandleType(adapter.sum32(u32, u32), FheType.Uint32);
        executor.checkHandleType(adapter.isIn32(u32, u32), FheType.Bool);
        executor.checkHandleType(adapter.add32(0, 0), FheType.Uint32);
        executor.checkHandleType(adapter.mixedAdd(0, 0), FheType.Uint32);
        executor.checkHandleType(adapter.widenTo64(0), FheType.Uint64);
        executor.checkHandleType(adapter.shift32(0, 0), FheType.Uint32);
        executor.checkHandleType(adapter.select32(0, 0, 0), FheType.Uint32);
        executor.checkHandleType(adapter.sum32(0, 0), FheType.Uint32);
        executor.checkHandleType(adapter.isIn32(0, 0), FheType.Bool);
    }
}
