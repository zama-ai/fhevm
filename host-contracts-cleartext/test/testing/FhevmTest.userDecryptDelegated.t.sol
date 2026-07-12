// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {FhevmTest} from "../../src/testing/FhevmTest.sol";
import {FheType} from "@fhevm/host-contracts/contracts/shared/FheType.sol";

import {externalEuint64} from "encrypted-types/EncryptedTypes.sol";

/// @notice Exercises the delegated user-decryption flow mirrored from the js-sdk cleartext
///         KMS verifier: user A delegates handle access (scoped to contract A) to delegate X,
///         and X decrypts by signing the delegated request itself.
contract FhevmTestUserDecryptDelegatedTest is FhevmTest {
    uint256 internal constant DELEGATOR_PK = 0xA11CE; // user A
    uint256 internal constant DELEGATE_PK = 0xB0B; // delegate X
    address internal constant CONTRACT_A = address(0xCA11AB1E);

    address internal delegator;
    address internal delegate;

    function setUp() public override {
        super.setUp();
        delegator = vm.addr(DELEGATOR_PK);
        delegate = vm.addr(DELEGATE_PK);
    }

    /// @dev Encrypts `value`, persists it for both the delegator and contract A, then records
    ///      an A→X delegation scoped to contract A.
    function _setupDelegatedHandle(uint64 value, uint64 expiry) internal returns (bytes32 handle) {
        (externalEuint64 h, bytes memory proof) = encryptUint64(value, address(this));
        handle = externalEuint64.unwrap(h);

        _executor.verifyInput(handle, address(this), proof, FheType.Uint64);
        _acl.allow(handle, delegator);
        _acl.allow(handle, CONTRACT_A);

        vm.prank(delegator);
        _acl.delegateForUserDecryption(delegate, CONTRACT_A, expiry);
    }

    function test_userDecryptDelegated_returnsCorrectPlaintext() public {
        bytes32 handle = _setupDelegatedHandle(4242, uint64(block.timestamp + 1 days));

        bytes memory signature = signDelegatedUserDecrypt(DELEGATE_PK, delegator, CONTRACT_A);
        uint256 cleartext = userDecryptDelegated(handle, delegator, delegate, CONTRACT_A, signature);

        assertEq(cleartext, 4242);
    }

    function test_userDecryptDelegated_revertsWhenNotDelegated() public {
        (externalEuint64 h, bytes memory proof) = encryptUint64(1, address(this));
        bytes32 handle = externalEuint64.unwrap(h);
        _executor.verifyInput(handle, address(this), proof, FheType.Uint64);
        _acl.allow(handle, delegator);
        _acl.allow(handle, CONTRACT_A);
        // No delegateForUserDecryption call.

        bytes memory signature = signDelegatedUserDecrypt(DELEGATE_PK, delegator, CONTRACT_A);

        vm.expectRevert(
            abi.encodeWithSelector(HandleNotDelegatedForDecrypt.selector, handle, delegator, delegate, CONTRACT_A)
        );
        this.callUserDecryptDelegated(handle, delegator, delegate, CONTRACT_A, signature);
    }

    function test_userDecryptDelegated_revertsWhenSignedByWrongKey() public {
        bytes32 handle = _setupDelegatedHandle(7, uint64(block.timestamp + 1 days));

        // Signed by the delegator instead of the delegate.
        bytes memory badSignature = signDelegatedUserDecrypt(DELEGATOR_PK, delegator, CONTRACT_A);

        vm.expectRevert(InvalidUserDecryptSignature.selector);
        this.callUserDecryptDelegated(handle, delegator, delegate, CONTRACT_A, badSignature);
    }

    function test_userDecryptDelegated_revertsWhenDelegationExpired() public {
        bytes32 handle = _setupDelegatedHandle(9, uint64(block.timestamp + 1 hours));

        vm.warp(block.timestamp + 2 hours);

        bytes memory signature = signDelegatedUserDecrypt(DELEGATE_PK, delegator, CONTRACT_A);

        vm.expectRevert(
            abi.encodeWithSelector(HandleNotDelegatedForDecrypt.selector, handle, delegator, delegate, CONTRACT_A)
        );
        this.callUserDecryptDelegated(handle, delegator, delegate, CONTRACT_A, signature);
    }

    function callUserDecryptDelegated(
        bytes32 handle,
        address delegator_,
        address delegate_,
        address contractAddress,
        bytes memory signature
    ) external returns (uint256) {
        return userDecryptDelegated(handle, delegator_, delegate_, contractAddress, signature);
    }
}
