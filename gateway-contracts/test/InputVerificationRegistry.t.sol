// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { InputVerificationRegistry } from "../contracts/InputVerificationRegistry.sol";
import { IInputVerificationRegistry } from "../contracts/interfaces/IInputVerificationRegistry.sol";
import { protocolPaymentAddress } from "../addresses/GatewayAddresses.sol";
import { TestBase } from "./TestUtils.sol";

contract ProtocolPaymentMock {
    function collectInputVerificationFee(address) external {}
    function collectPublicDecryptionFee(address) external {}
    function collectUserDecryptionFee(address) external {}
}

contract InputVerificationRegistryTest is TestBase {
    uint256 private constant INPUT_VERIFICATION_COUNTER_BASE =
        0x0300000000000000000000000000000000000000000000000000000000000000;
    bytes32 private constant INPUT_VERIFICATION_REGISTRY_STORAGE_LOCATION =
        0xa2c7a90bb5b79766e8d1a00bb5b79766e8d1a00bb5b79766e8d1a00bb5b79700;

    InputVerificationRegistry private registry;

    event InputVerificationRegistered(
        uint256 indexed requestId,
        bytes32 commitment,
        address indexed userAddress,
        uint256 contractChainId,
        address contractAddress,
        bytes userSignature,
        uint256 timestamp
    );

    function setUp() public {
        ProtocolPaymentMock mock = new ProtocolPaymentMock();
        vm.etch(protocolPaymentAddress, address(mock).code);

        registry = new InputVerificationRegistry();
        vm.store(
            address(registry),
            INPUT_VERIFICATION_REGISTRY_STORAGE_LOCATION,
            bytes32(INPUT_VERIFICATION_COUNTER_BASE)
        );
    }

    function test_registerInputVerification_storesRequestAndEmits() public {
        bytes32 commitment = keccak256("ciphertext");
        uint256 chainId = 42161;
        address contractAddress = address(0xB0B);
        address userAddress = address(0xA11CE);
        bytes memory userSignature = hex"DEADBEEF";
        uint256 timestamp = 1_700_000_500;
        vm.warp(timestamp);

        uint256 expectedRequestId = INPUT_VERIFICATION_COUNTER_BASE + 1;

        vm.expectEmit(true, true, false, true);
        emit InputVerificationRegistered(
            expectedRequestId,
            commitment,
            userAddress,
            chainId,
            contractAddress,
            userSignature,
            timestamp
        );

        uint256 requestId = registry.registerInputVerification(
            commitment,
            chainId,
            contractAddress,
            userAddress,
            userSignature
        );
        assertEq(requestId, expectedRequestId);

        (
            bytes32 storedCommitment,
            address storedUser,
            uint256 storedChainId,
            address storedContract,
            uint256 storedFee,
            uint256 storedTimestamp
        ) = registry.getRequest(requestId);

        assertEq(storedCommitment, commitment);
        assertEq(storedUser, userAddress);
        assertEq(storedChainId, chainId);
        assertEq(storedContract, contractAddress);
        assertEq(storedFee, 0);
        assertEq(storedTimestamp, timestamp);
    }

    function test_registerInputVerification_revertsOnEmptyCommitment() public {
        vm.expectRevert(abi.encodeWithSelector(IInputVerificationRegistry.EmptyCommitment.selector));
        registry.registerInputVerification(
            bytes32(0),
            1,
            address(0xB0B),
            address(0xA11CE),
            hex"01"
        );
    }
}
