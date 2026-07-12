// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "../host-contracts/contracts/shared/FheType.sol";
import {HANDLE_VERSION} from "../host-contracts/contracts/shared/Constants.sol";

library InputProofHelper {
    bytes32 internal constant EIP712_DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
    bytes32 internal constant EIP712_INPUT_VERIFICATION_TYPEHASH = keccak256(
        "CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,bytes extraData)"
    );

    /// @notice Computes the deterministic handle for an encrypted input.
    /// @param mockCiphertext Arbitrary ciphertext bytes used for handle derivation.
    /// @param index The position of this handle within a multi-handle input proof.
    /// @param fheType The FHE type (Bool, Uint8, ..., Uint256) encoded into the handle.
    /// @param aclAddress The ACL contract address embedded in the handle hash.
    /// @param chainId The chain id embedded in the handle.
    /// @return handle The computed 32-byte encrypted handle.
    function computeInputHandle(
        bytes memory mockCiphertext,
        uint8 index,
        FheType fheType,
        address aclAddress,
        uint64 chainId
    ) internal pure returns (bytes32 handle) {
        bytes32 blobHash = keccak256(abi.encodePacked("ZK-w_rct", mockCiphertext));
        bytes32 handleHash = keccak256(abi.encodePacked("ZK-w_hdl", blobHash, index, aclAddress, uint256(chainId)));

        handle = handleHash & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        handle |= bytes32(uint256(index) << 80);
        handle |= bytes32(uint256(chainId) << 16);
        handle |= bytes32(uint256(uint8(fheType)) << 8);
        handle |= bytes32(uint256(HANDLE_VERSION));
    }

    /// @notice Computes the EIP-712 domain separator for the input verifier.
    /// @param verifyingContract The input verifier contract address.
    /// @param chainId The chain id for the domain.
    /// @return The computed EIP-712 domain separator.
    function computeInputVerifierDomainSeparator(address verifyingContract, uint256 chainId)
        internal
        pure
        returns (bytes32)
    {
        return keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH,
                keccak256(bytes("InputVerification")),
                keccak256(bytes("1")),
                chainId,
                verifyingContract
            )
        );
    }

    /// @notice Computes the EIP-712 typed-data digest for input verification.
    /// @param handles The ciphertext handles being verified.
    /// @param userAddress The user who encrypted the input.
    /// @param contractAddress The contract authorized to consume the input.
    /// @param contractChainId The chain id included in the signed payload.
    /// @param extraData Additional bytes included in the signature.
    /// @param domainSeparator The EIP-712 domain separator from `computeInputVerifierDomainSeparator`.
    /// @return The EIP-712 digest to be signed by the input signer.
    function computeInputVerificationDigest(
        bytes32[] memory handles,
        address userAddress,
        address contractAddress,
        uint256 contractChainId,
        bytes memory extraData,
        bytes32 domainSeparator
    ) internal pure returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                EIP712_INPUT_VERIFICATION_TYPEHASH,
                keccak256(abi.encodePacked(handles)),
                userAddress,
                contractAddress,
                contractChainId,
                keccak256(abi.encodePacked(extraData))
            )
        );
        return keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
    }

    /// @notice Assembles handles, signatures, and extra data into the input proof wire format.
    /// @param handles The ciphertext handles included in the proof.
    /// @param signatures The ECDSA signatures from input signers.
    /// @param extraData Extra bytes appended after signatures.
    /// @return proof The serialized proof bytes consumed by `InputVerifier`.
    function assembleInputProof(bytes32[] memory handles, bytes[] memory signatures, bytes memory extraData)
        internal
        pure
        returns (bytes memory proof)
    {
        proof = abi.encodePacked(uint8(handles.length), uint8(signatures.length));
        for (uint256 i = 0; i < handles.length; i++) {
            proof = abi.encodePacked(proof, handles[i]);
        }
        for (uint256 i = 0; i < signatures.length; i++) {
            proof = abi.encodePacked(proof, signatures[i]);
        }
        proof = abi.encodePacked(proof, extraData);
    }
}
