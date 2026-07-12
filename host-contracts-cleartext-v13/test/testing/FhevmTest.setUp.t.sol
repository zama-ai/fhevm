// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {FHE} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {
    fhevmExecutorAdd,
    aclAdd,
    inputVerifierAdd,
    kmsVerifierAdd
} from "@fhevm/host-contracts/addresses/FHEVMHostAddresses.sol";
import {FhevmTest} from "../../src/testing/FhevmTest.sol";
import {FheType} from "@fhevm/host-contracts/contracts/shared/FheType.sol";

import {externalEuint64, euint64} from "encrypted-types/EncryptedTypes.sol";

contract FhevmSetUpRoutingHarness is ZamaEthereumConfig {
    function addFromExternal(externalEuint64 lhs, bytes calldata lhsProof, externalEuint64 rhs, bytes calldata rhsProof)
        external
        returns (euint64)
    {
        euint64 left = FHE.fromExternal(lhs, lhsProof);
        euint64 right = FHE.fromExternal(rhs, rhsProof);
        euint64 sum = FHE.add(left, right);
        FHE.allowThis(sum);
        FHE.allow(sum, msg.sender);
        return sum;
    }
}

contract FhevmTestSetUpTest is FhevmTest {
    function test_setUp_deploysExecutorAtExpectedAddress() public view {
        assertEq(address(_executor), fhevmExecutorAdd);
        assertGt(fhevmExecutorAdd.code.length, 0);
    }

    function test_setUp_deploysACLAtExpectedAddress() public view {
        assertEq(address(_acl), aclAdd);
        assertGt(aclAdd.code.length, 0);
    }

    function test_setUp_deploysKMSVerifierAtExpectedAddress() public view {
        assertEq(address(_kmsVerifier), kmsVerifierAdd);
        assertGt(kmsVerifierAdd.code.length, 0);
    }

    function test_setUp_deploysInputVerifierAtExpectedAddress() public view {
        assertEq(address(_inputVerifier), inputVerifierAdd);
        assertGt(inputVerifierAdd.code.length, 0);
    }

    function test_setUp_kmsVerifierHasMockSigner() public view {
        assertTrue(_kmsVerifier.isSigner(mockKmsSigner));
        assertEq(_kmsVerifier.getThreshold(), 1);
    }

    function test_setUp_inputVerifierHasMockSigner() public view {
        assertTrue(_inputVerifier.isSigner(mockInputSigner));
        assertEq(_inputVerifier.getThreshold(), 1);
    }

    function test_setUp_usesChainId31337() public view {
        assertEq(block.chainid, 31337);
    }

    function test_setUp_fheLibraryCallsRouteToExecutor() public {
        FhevmSetUpRoutingHarness harness = new FhevmSetUpRoutingHarness();

        (externalEuint64 lhs, bytes memory lhsProof) = encryptUint64(11, address(harness));
        (externalEuint64 rhs, bytes memory rhsProof) = encryptUint64(19, address(harness));

        euint64 result = harness.addFromExternal(lhs, lhsProof, rhs, rhsProof);

        bytes32 resultHandle = euint64.unwrap(result);
        assertEq(uint8(resultHandle[30]), uint8(FheType.Uint64));
        assertEq(decrypt(resultHandle), 30);
    }
}
