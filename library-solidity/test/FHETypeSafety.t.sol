// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "encrypted-types/EncryptedTypes.sol";
import {FHE} from "../lib/FHE.sol";
import {CoprocessorConfig} from "../lib/Impl.sol";
import {HostContractsDeployerTestUtils} from "@fhevm-foundry/HostContractsDeployerTestUtils.sol";
import {ACL} from "@fhevm-host-contracts/contracts/ACL.sol";
import {FHEVMExecutor} from "@fhevm-host-contracts/contracts/FHEVMExecutor.sol";
import {InputVerifier} from "@fhevm-host-contracts/contracts/InputVerifier.sol";
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

    function addScalar32(bytes32 a, uint32 b) external returns (bytes32) {
        return euint32.unwrap(FHE.add(euint32.wrap(a), b));
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
    uint256 internal constant INPUT_SIGNER_KEY = 0x123;

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

    function test_VerifiedInputAllowsContractButNotCaller() public {
        address[] memory signers = new address[](1);
        signers[0] = vm.addr(INPUT_SIGNER_KEY);
        (InputVerifier verifier, ) = _deployInputVerifier(OWNER, address(0x1234), uint64(block.chainid), signers, 1);

        // One external Uint32 input: index 0, current chain, handle version 0.
        bytes32 handle = bytes32(
            (uint256(keccak256("verified input")) & ~uint256(type(uint88).max)) |
                (uint256(uint64(block.chainid)) << 16) |
                (uint256(uint8(FheType.Uint32)) << 8)
        );
        bytes memory proof = _signInput(verifier, handle);

        assertEq(adapter.importHandle(FheType.Uint32, handle, proof), handle);
        assertTrue(ACL(aclAdd).isAllowed(handle, address(adapter)));
        assertFalse(ACL(aclAdd).isAllowed(handle, address(this)));
        executor.checkHandleType(adapter.addScalar32(handle, 1), FheType.Uint32);

        // The same caller cannot reuse the raw input with an empty proof in this transaction.
        vm.expectRevert(abi.encodeWithSelector(FHE.SenderNotAllowedToUseHandle.selector, handle, address(this)));
        adapter.importHandle(FheType.Uint32, handle, "");
    }

    function _signInput(InputVerifier verifier, bytes32 handle) internal view returns (bytes memory) {
        (, string memory name, string memory version, uint256 chainId, address verifyingContract, , ) = verifier
            .eip712Domain();
        bytes32 domain = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes(name)),
                keccak256(bytes(version)),
                chainId,
                verifyingContract
            )
        );
        bytes32 structHash = keccak256(
            abi.encode(
                verifier.EIP712_INPUT_VERIFICATION_TYPEHASH(),
                keccak256(abi.encodePacked(handle)),
                address(this),
                address(adapter),
                block.chainid,
                keccak256("")
            )
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(
            INPUT_SIGNER_KEY,
            keccak256(abi.encodePacked("\x19\x01", domain, structHash))
        );
        return abi.encodePacked(uint8(1), uint8(1), handle, r, s, v);
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
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.add32(right, wrong);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.addScalar32(wrong, 1);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
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
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
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
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.select32(wrong, right, right);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.select32(condition, wrong, wrong);
        vm.expectRevert(FHEVMExecutor.InvalidType.selector);
        adapter.select32(condition, right, wrong);
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
