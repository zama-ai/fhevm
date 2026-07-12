// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {FhevmTest} from "../../src/testing/FhevmTest.sol";

import {FHE, euint64, externalEuint64} from "@fhevm/solidity/lib/FHE.sol";
import {CoprocessorConfig} from "@fhevm/solidity/lib/Impl.sol";
import {FheType} from "@fhevm/host-contracts/contracts/shared/FheType.sol";
import {aclAdd, fhevmExecutorAdd, kmsVerifierAdd} from "@fhevm/host-contracts/addresses/FHEVMHostAddresses.sol";

/// @dev A minimal contract-under-test that consumes two encrypted inputs and adds them on-chain,
///      wired to the harness's canonical cleartext addresses.
contract Adder {
    euint64 public result;

    constructor() {
        FHE.setCoprocessor(
            CoprocessorConfig({
                ACLAddress: aclAdd,
                CoprocessorAddress: fhevmExecutorAdd,
                KMSVerifierAddress: kmsVerifierAdd
            })
        );
    }

    function add(externalEuint64 a, externalEuint64 b, bytes calldata inputProof) external returns (euint64 r) {
        euint64 ea = FHE.fromExternal(a, inputProof);
        euint64 eb = FHE.fromExternal(b, inputProof);
        r = FHE.add(ea, eb);
        FHE.allowThis(r);
        FHE.allow(r, msg.sender);
        result = r;
    }

    function makeResultPublic() external {
        FHE.makePubliclyDecryptable(result);
    }

    /// @dev Persistently authorize `who` to user-decrypt the current result handle.
    function allowResult(address who) external {
        FHE.allow(result, who);
    }
}

/// @notice Proves the package-hosted Foundry harness works in-process, driving THIS package's
///         on-chain cleartext executor (no off-chain replay), under a plain `forge test`
///         (no --disable-code-size-limit — the 27 KB executor is placed via deployCodeTo).
contract FhevmHarnessSmoke is FhevmTest {
    Adder internal adder;

    function setUp() public override {
        super.setUp();
        adder = new Adder();
    }

    function test_encryptedAddDecrypts() public {
        uint256[] memory values = new uint256[](2);
        values[0] = 40;
        values[1] = 2;
        FheType[] memory types = new FheType[](2);
        types[0] = FheType.Uint64;
        types[1] = FheType.Uint64;

        (bytes32[] memory handles, bytes memory inputProof) = encrypt(values, types, address(adder));

        euint64 r = adder.add(externalEuint64.wrap(handles[0]), externalEuint64.wrap(handles[1]), inputProof);

        // Cleartext computed on-chain by CleartextFHEVMExecutor, read back from CleartextACL.
        assertEq(decrypt(r), 42, "40 + 2 should decrypt to 42");
    }

    function test_publicDecryptWithProof() public {
        uint256[] memory values = new uint256[](2);
        values[0] = 100;
        values[1] = 23;
        FheType[] memory types = new FheType[](2);
        types[0] = FheType.Uint64;
        types[1] = FheType.Uint64;

        (bytes32[] memory handles, bytes memory inputProof) = encrypt(values, types, address(adder));
        euint64 r = adder.add(externalEuint64.wrap(handles[0]), externalEuint64.wrap(handles[1]), inputProof);

        // Mark publicly decryptable (via the CUT, which holds the coprocessor config), then read
        // back via the KMS-proof path.
        adder.makeResultPublic();

        bytes32[] memory toDecrypt = new bytes32[](1);
        toDecrypt[0] = euint64.unwrap(r);
        (uint256[] memory cleartexts,) = publicDecrypt(toDecrypt);
        assertEq(cleartexts[0], 123, "100 + 23 should publicly decrypt to 123");
    }

    /// @dev Exercises the EIP-712 user-decryption path: a user authorized on the handle signs a
    ///      decrypt request; the harness verifies the signature and returns the on-chain cleartext.
    function test_userDecryptWithSignature() public {
        uint256 userPk = 0xA11CE;
        address user = vm.addr(userPk);

        uint256[] memory values = new uint256[](2);
        values[0] = 7;
        values[1] = 35;
        FheType[] memory types = new FheType[](2);
        types[0] = FheType.Uint64;
        types[1] = FheType.Uint64;

        (bytes32[] memory handles, bytes memory inputProof) = encrypt(values, types, address(adder));
        euint64 r = adder.add(externalEuint64.wrap(handles[0]), externalEuint64.wrap(handles[1]), inputProof);

        // Authorize the user on the result, then user-decrypt with their EIP-712 signature.
        adder.allowResult(user);
        bytes memory sig = signUserDecrypt(userPk, address(adder));
        uint256 clear = userDecrypt(euint64.unwrap(r), user, address(adder), sig);

        assertEq(clear, 42, "7 + 35 should user-decrypt to 42");
    }
}
