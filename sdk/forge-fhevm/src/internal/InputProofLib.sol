// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {FheType} from "@fhevm/host-contracts-cleartext/contracts/shared/FheType.sol";

/**
 * Builds the encrypted-input bundle a contract under test receives as `(externalEuintX, inputProof)`.
 *
 * Two independent things travel in one `inputProof`, and both must be exactly right:
 *
 *  1. The COPROCESSOR SIGNATURE. `InputVerifier.verifyInput` recovers an EIP-712 signature over the
 *     handles, the user, the target contract, the chain id, and `extraData`. A wrong domain or a wrong
 *     byte in `extraData` fails signature recovery.
 *  2. The CLEARTEXT CHANNEL. `CleartextArithmetic._tryReadCleartextFromProof` reads the clear value for
 *     handle `i` as the 32-byte word at `2 + 32*numHandles + 65*numSigners + 32*i` — which is exactly the
 *     `extraData` region. So `extraData` IS the packed clear values, and it is also signed. The same bytes
 *     serve both purposes; they cannot diverge.
 *
 * Layout: [numHandles:1][numSigners:1][handles:32*H][signatures:65*S][extraData:32*H]
 */
library InputProofLib {
    /// @dev Domain separators for the handle hash. Only self-consistency matters — see `computeHandle`.
    bytes8 private constant RAW_CT_HASH_DOMAIN = "ZK-w_rct";
    bytes8 private constant HANDLE_HASH_DOMAIN = "ZK-w_hdl";

    /// @dev `Constants.sol: HANDLE_VERSION`. Checked on-chain (byte 31 of every handle).
    uint8 internal constant HANDLE_VERSION = 0;

    bytes32 private constant EIP712_DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");

    /// @dev Must match `InputVerifier.EIP712_INPUT_VERIFICATION_TYPE` exactly. The adjacent string literals
    ///      concatenate, so the hash is unaffected — they only keep the line inside 120 cols.
    bytes32 private constant CIPHERTEXT_VERIFICATION_TYPEHASH = keccak256(
        "CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,"
        "uint256 contractChainId,bytes extraData)"
    );

    error ValueTooLargeForType(uint256 value, uint8 fheType);

    /**
     * Derives a handle.
     *
     * The chain does NOT re-derive this hash — `InputVerifier.verifyInput` only checks the metadata in
     * bytes 21..31 and trusts the coprocessor signature. So the hashing below merely has to be unique and
     * self-consistent. What IS load-bearing is the trailing metadata:
     *
     *   [0..20]  first 21 bytes of the prehash
     *   [21]     index within the bundle   (read as `indexHandle`)
     *   [22..29] chain id as uint64        (must equal block.chainid, else InvalidChainId)
     *   [30]     FheType
     *   [31]     handle version            (must equal HANDLE_VERSION, else InvalidHandleVersion)
     */
    function computeHandle(bytes memory ciphertext, uint8 index, FheType fheType, address acl, uint64 chainId)
        internal
        pure
        returns (bytes32)
    {
        bytes32 blobHash = keccak256(abi.encodePacked(RAW_CT_HASH_DOMAIN, ciphertext));
        bytes32 prehash = keccak256(abi.encodePacked(HANDLE_HASH_DOMAIN, blobHash, index, acl, uint256(chainId)));

        // Keep the first 21 bytes of the prehash, then overwrite 21..31 with the metadata above.
        uint256 handle = uint256(prehash) & ~uint256(0) << 88; // clear the low 11 bytes
        handle |= uint256(index) << 80;
        handle |= uint256(chainId) << 16;
        handle |= uint256(uint8(fheType)) << 8;
        handle |= uint256(HANDLE_VERSION);
        return bytes32(handle);
    }

    /// @notice The clear values, one 32-byte word per handle. This is both the signed `extraData` and the
    ///         cleartext channel the executor reads.
    function packExtraData(uint256[] memory values) internal pure returns (bytes memory extraData) {
        for (uint256 i; i < values.length; ++i) {
            extraData = abi.encodePacked(extraData, values[i]);
        }
    }

    /// @notice `InputVerifier`'s EIP-712 domain. Built from the values it was INITIALIZED with
    ///         (`verifyingContractSource`, `chainIDSource`) — not from this chain's identity.
    function domainSeparator(address verifyingContractSource, uint256 chainIDSource) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH,
                keccak256(bytes("InputVerification")),
                keccak256(bytes("1")),
                chainIDSource,
                verifyingContractSource
            )
        );
    }

    /// @notice The digest the coprocessor signs, mirroring `InputVerifier._hashEIP712InputVerification`.
    function digest(
        bytes32[] memory handles,
        address user,
        address target,
        uint256 contractChainId,
        bytes memory extraData,
        bytes32 separator
    ) internal pure returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                CIPHERTEXT_VERIFICATION_TYPEHASH,
                keccak256(abi.encodePacked(handles)),
                user,
                target,
                contractChainId,
                keccak256(abi.encodePacked(extraData))
            )
        );
        return keccak256(abi.encodePacked(hex"1901", separator, structHash));
    }

    /// @notice Assembles the final proof. The two count bytes are how the executor locates `extraData`,
    ///         so they must be exact.
    function assemble(bytes32[] memory handles, bytes[] memory signatures, bytes memory extraData)
        internal
        pure
        returns (bytes memory proof)
    {
        proof = abi.encodePacked(uint8(handles.length), uint8(signatures.length));
        for (uint256 i; i < handles.length; ++i) {
            proof = abi.encodePacked(proof, handles[i]);
        }
        for (uint256 i; i < signatures.length; ++i) {
            proof = abi.encodePacked(proof, signatures[i]);
        }
        proof = abi.encodePacked(proof, extraData);
    }

    /// @notice Rejects a clear value that does not fit its FHE type, rather than letting it silently wrap
    ///         when the executor normalizes it.
    function assertFits(uint256 value, FheType fheType) internal pure {
        uint256 bits = bitWidth(fheType);
        if (bits < 256 && value > (uint256(1) << bits) - 1) {
            revert ValueTooLargeForType(value, uint8(fheType));
        }
    }

    function bitWidth(FheType fheType) internal pure returns (uint256) {
        if (fheType == FheType.Bool) return 1;
        if (fheType == FheType.Uint8) return 8;
        if (fheType == FheType.Uint16) return 16;
        if (fheType == FheType.Uint32) return 32;
        if (fheType == FheType.Uint64) return 64;
        if (fheType == FheType.Uint128) return 128;
        if (fheType == FheType.Uint160) return 160;
        return 256;
    }
}
