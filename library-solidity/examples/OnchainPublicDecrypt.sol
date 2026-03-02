// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/FHE.sol";
import {CoprocessorSetup} from "./CoprocessorSetup.sol";

/// @notice Contract for testing onchain public decryption
contract OnchainPublicDecrypt {
    /// @dev Encrypted state variable
    euint64 xUint64;

    /// @dev Decrypted state variable
    uint64 public yUint64;

    constructor() {
        FHE.setCoprocessor(CoprocessorSetup.defaultConfig());
        xUint64 = FHE.asEuint64(42);
        FHE.allowThis(xUint64);
    }

    /// @notice Request decryption of a 64-bit unsigned integer
    function requestDecryption() public {
        FHE.makePubliclyDecryptable(xUint64);
    }

    /// @notice Callback function for 64-bit unsigned integer decryption
    /// @param abiEncodedCleartexts The abi-encoding of the decrypted 64-bit unsigned integer
    /// @param decryptionProof The KMS public decryption proof, which includes the KMS signatures
    function callbackDecryption(bytes memory abiEncodedCleartexts, bytes memory decryptionProof) public {
        bytes32[] memory handlesList = new bytes32[](1);
        handlesList[0] = FHE.toBytes32(xUint64);
        FHE.checkSignatures(handlesList, abiEncodedCleartexts, decryptionProof);
        yUint64 = abi.decode(abiEncodedCleartexts, (uint64));
    }

    /// @notice View function variant of `callbackUint64`, which will only check decryption signatures validity, not write to storage.
    /// @param abiEncodedCleartexts The abi-encoding of the decrypted 64-bit unsigned integer
    /// @param decryptionProof The KMS public decryption proof, which includes the KMS signatures
    /// @return true only if signatures validation succeeds, might return false or revert otherwise.
    function isPublicDecryptionResultValid(
        bytes memory abiEncodedCleartexts,
        bytes memory decryptionProof
    ) public view returns (bool) {
        bytes32[] memory handlesList = new bytes32[](1);
        handlesList[0] = FHE.toBytes32(xUint64);
        return FHE.isPublicDecryptionResultValid(handlesList, abiEncodedCleartexts, decryptionProof);
    }
}
