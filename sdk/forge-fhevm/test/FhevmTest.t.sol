// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {FhevmTest} from "../src/FhevmTest.sol";
import {CleartextKMSVerifier} from "@fhevm/host-contracts-cleartext/cleartext/CleartextKMSVerifier.sol";
import {Counter} from "./Counter.sol";
import {euint32, externalEuint32} from "encrypted-types/EncryptedTypes.sol";

/// Exercises the library end to end against a contract that uses the real `FHE` library and
/// `ZamaEthereumConfig` — i.e. the same way a consumer's contract would.
contract FhevmTestTest is FhevmTest {
    Counter internal counter;
    Account internal alice;
    Account internal bob;

    function setUp() public override {
        super.setUp();
        counter = new Counter();
        alice = makeAccount("alice");
        bob = makeAccount("bob");
    }

    function test_stackIsLiveAtTheAddressesZamaConfigPins() public view {
        // If the address patching were wrong, these would be empty accounts and every FHE call a no-op.
        assertGt(address(0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D).code.length, 0, "ACL");
        assertGt(address(0xe3a9105a3a932253A70F126eb1E3b589C643dD24).code.length, 0, "FHEVMExecutor");
        assertGt(address(0x901F8942346f7AB3a01F6D7613119Bca447Bb030).code.length, 0, "KMSVerifier");
    }

    function test_countStartsUninitialized() public view {
        assertEq(euint32.unwrap(counter.getCount()), bytes32(0));
    }

    function test_incrementThenUserDecrypt() public {
        (externalEuint32 five, bytes memory proof) = encryptUint32(5, alice.addr, address(counter));
        vm.prank(alice.addr);
        counter.increment(five, proof);

        bytes32 handle = euint32.unwrap(counter.getCount());
        uint256 clear = userDecrypt(handle, alice.addr, address(counter), signUserDecrypt(alice.key, address(counter)));
        assertEq(clear, 5);
    }

    function test_arithmeticAccumulatesAcrossCalls() public {
        (externalEuint32 a, bytes memory pa) = encryptUint32(7, alice.addr, address(counter));
        vm.prank(alice.addr);
        counter.increment(a, pa);

        (externalEuint32 b, bytes memory pb) = encryptUint32(2, alice.addr, address(counter));
        vm.prank(alice.addr);
        counter.decrement(b, pb);

        bytes32 handle = euint32.unwrap(counter.getCount());
        assertEq(userDecrypt(handle, alice.addr, address(counter), signUserDecrypt(alice.key, address(counter))), 5);
        assertEq(peek(handle), 5); // same value straight from the cleartext DB
    }

    /// The ACL is really enforced: Counter only grants the caller, so bob cannot read alice's result.
    function test_userDecryptRevertsForUnauthorizedUser() public {
        (externalEuint32 five, bytes memory proof) = encryptUint32(5, alice.addr, address(counter));
        vm.prank(alice.addr);
        counter.increment(five, proof);

        bytes32 handle = euint32.unwrap(counter.getCount());
        // Sign first: signUserDecrypt reads the KMS context id on-chain, and that call would otherwise be
        // the one expectRevert latches onto.
        bytes memory sig = signUserDecrypt(bob.key, address(counter));

        vm.expectRevert();
        this.userDecryptExternal(handle, bob.addr, address(counter), sig);
    }

    /// A signature from the wrong key must not authorize a decrypt, even for a handle bob could otherwise read.
    function test_userDecryptRevertsOnWrongSigner() public {
        (externalEuint32 five, bytes memory proof) = encryptUint32(5, alice.addr, address(counter));
        vm.prank(alice.addr);
        counter.increment(five, proof);

        bytes32 handle = euint32.unwrap(counter.getCount());
        bytes memory wrongSig = signUserDecrypt(bob.key, address(counter));

        vm.expectRevert();
        this.userDecryptExternal(handle, alice.addr, address(counter), wrongSig);
    }

    /// The forged KMS proof must satisfy the real on-chain verifier — this is what lets a test drive a
    /// contract's async decryption-callback branch.
    function test_kmsDecryptionProofIsAcceptedByTheVerifier() public {
        (externalEuint32 five, bytes memory proof) = encryptUint32(5, alice.addr, address(counter));
        vm.prank(alice.addr);
        counter.increment(five, proof);

        bytes32[] memory handles = new bytes32[](1);
        handles[0] = euint32.unwrap(counter.getCount());
        bytes memory decryptedResult = abi.encode(uint256(5));

        assertTrue(
            CleartextKMSVerifier(0x901F8942346f7AB3a01F6D7613119Bca447Bb030).verifyDecryptionEIP712KMSSignatures(
                handles, decryptedResult, kmsDecryptionProof(handles, decryptedResult)
            )
        );
    }

    /// A proof signed over a DIFFERENT result must not verify, or the helper would be rubber-stamping.
    function test_kmsDecryptionProofRejectsATamperedResult() public {
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = bytes32(uint256(1));

        bytes memory signedResult = abi.encode(uint256(5));
        bytes memory kmsProof = kmsDecryptionProof(handles, signedResult);

        vm.expectRevert();
        CleartextKMSVerifier(0x901F8942346f7AB3a01F6D7613119Bca447Bb030).verifyDecryptionEIP712KMSSignatures(
            handles, abi.encode(uint256(6)), kmsProof
        );
    }

    /// The depth cap comes off; the per-transaction accounting stays on.
    function test_disableHCUDepthLimitKeepsTheStackWorking() public {
        disableHCUDepthLimit();

        (externalEuint32 one, bytes memory proof) = encryptUint32(1, alice.addr, address(counter));
        vm.prank(alice.addr);
        counter.increment(one, proof);

        bytes32 handle = euint32.unwrap(counter.getCount());
        assertEq(userDecrypt(handle, alice.addr, address(counter), signUserDecrypt(alice.key, address(counter))), 1);
    }

    /// A proof is bound to (user, target). Replaying alice's proof as bob must fail signature recovery.
    function test_inputProofIsBoundToItsUser() public {
        (externalEuint32 five, bytes memory proof) = encryptUint32(5, alice.addr, address(counter));
        vm.prank(bob.addr);
        vm.expectRevert();
        counter.increment(five, proof);
    }
}
