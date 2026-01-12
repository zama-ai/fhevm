// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { DecryptionRegistry } from "../contracts/DecryptionRegistry.sol";
import { IDecryptionRegistry } from "../contracts/interfaces/IDecryptionRegistry.sol";
import { protocolPaymentAddress } from "../addresses/GatewayAddresses.sol";
import { TestBase } from "./TestUtils.sol";

contract ProtocolPaymentMock {
    function collectInputVerificationFee(address) external {}
    function collectPublicDecryptionFee(address) external {}
    function collectUserDecryptionFee(address) external {}
}

contract DecryptionRegistryTest is TestBase {
    uint256 private constant USER_DECRYPT_COUNTER_BASE =
        0x0200000000000000000000000000000000000000000000000000000000000000;
    uint256 private constant PUBLIC_DECRYPT_COUNTER_BASE =
        0x0100000000000000000000000000000000000000000000000000000000000000;
    bytes32 private constant DECRYPTION_REGISTRY_STORAGE_LOCATION =
        0xb1c7a90bb5b79766e8d1a00bb5b79766e8d1a00bb5b79766e8d1a00bb5b79700;

    DecryptionRegistry private registry;

    event UserDecryptionRequested(
        uint256 indexed requestId,
        bytes32[] handles,
        address[] contractAddresses,
        address indexed userAddress,
        bytes publicKey,
        bytes signature,
        uint256 chainId,
        uint256 timestamp
    );

    event PublicDecryptionRequested(
        uint256 indexed requestId,
        bytes32[] handles,
        address[] contractAddresses,
        uint256 chainId,
        uint256 timestamp
    );

    function setUp() public {
        ProtocolPaymentMock mock = new ProtocolPaymentMock();
        vm.etch(protocolPaymentAddress, address(mock).code);

        registry = new DecryptionRegistry();
        vm.store(address(registry), DECRYPTION_REGISTRY_STORAGE_LOCATION, bytes32(USER_DECRYPT_COUNTER_BASE));
        vm.store(
            address(registry),
            bytes32(uint256(DECRYPTION_REGISTRY_STORAGE_LOCATION) + 1),
            bytes32(PUBLIC_DECRYPT_COUNTER_BASE)
        );
    }

    function test_requestUserDecryption_emitsEvent() public {
        bytes32[] memory handles = new bytes32[](1);
        address[] memory contractAddresses = new address[](1);
        handles[0] = _handleWithChainId(123);
        contractAddresses[0] = address(0xBEEF);

        bytes memory publicKey = hex"0102";
        bytes memory signature = hex"ABCD";
        address user = address(0xA11CE);
        uint256 timestamp = 1_700_000_000;
        vm.warp(timestamp);

        uint256 expectedRequestId = USER_DECRYPT_COUNTER_BASE + 1;

        vm.expectEmit(true, true, false, true);
        emit UserDecryptionRequested(
            expectedRequestId,
            handles,
            contractAddresses,
            user,
            publicKey,
            signature,
            123,
            timestamp
        );

        vm.prank(address(0xB0B));
        uint256 requestId =
            registry.requestUserDecryption(handles, contractAddresses, user, publicKey, signature);
        assertEq(requestId, expectedRequestId);
    }

    function test_requestUserDecryption_revertsOnLengthMismatch() public {
        bytes32[] memory handles = new bytes32[](2);
        address[] memory contractAddresses = new address[](1);

        vm.expectRevert(
            abi.encodeWithSelector(
                IDecryptionRegistry.HandleContractAddressLengthMismatch.selector,
                2,
                1
            )
        );
        registry.requestUserDecryption(handles, contractAddresses, address(0xA11CE), hex"01", hex"02");
    }

    function test_requestPublicDecryption_emitsEvent() public {
        bytes32[] memory handles = new bytes32[](1);
        address[] memory contractAddresses = new address[](1);
        handles[0] = _handleWithChainId(77);
        contractAddresses[0] = address(0xCAFE);

        uint256 timestamp = 1_700_000_100;
        vm.warp(timestamp);

        uint256 expectedRequestId = PUBLIC_DECRYPT_COUNTER_BASE + 1;

        vm.expectEmit(true, false, false, true);
        emit PublicDecryptionRequested(
            expectedRequestId,
            handles,
            contractAddresses,
            77,
            timestamp
        );

        uint256 requestId = registry.requestPublicDecryption(handles, contractAddresses);
        assertEq(requestId, expectedRequestId);
    }

    function _handleWithChainId(uint64 chainId) private pure returns (bytes32) {
        return bytes32(uint256(chainId) << 184);
    }
}
