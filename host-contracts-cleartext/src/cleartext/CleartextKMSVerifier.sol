// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {CleartextACL, HandleContractPair} from "./CleartextACL.sol";
import {KMSVerifier} from "../host-contracts/contracts/KMSVerifier.sol";
import {FheType} from "../host-contracts/contracts/shared/FheType.sol";
import {aclAdd} from "../host-contracts/addresses/FHEVMHostAddresses.sol";

/**
 * @title CleartextKMSVerifier
 */
contract CleartextKMSVerifier is KMSVerifier {
    /// @notice EIP-712 domain typehash.
    bytes32 private constant EIP712_DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
    /// @notice UserDecryptRequestVerification typehash.
    bytes32 internal constant USER_DECRYPT_REQUEST_TYPEHASH = keccak256(
        "UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 startTimestamp,uint256 durationDays,bytes extraData)"
    );
    bytes32 internal constant DELEGATED_USER_DECRYPT_REQUEST_TYPEHASH = keccak256(
        "DelegatedUserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,address delegatorAddress,uint256 startTimestamp,uint256 durationDays,bytes extraData)"
    );
    error InvalidUserDecryptSignature();
    error ContractAddressNotAuthorized(address contractAddress);

    /// @notice Serializes the v1 extraData layout consumed by the KMS verifier.
    /// @dev Wire format (33 bytes):
    ///        [byte 0     : version = 0x01]
    ///        [bytes 1..32: contextId (uint256, big-endian)]
    ///      Memory layout after `new bytes(33)` (free memory pointer base = `extraData`):
    ///        [extraData .. +31]   length = 33
    ///        [+32]                version byte
    ///        [+33 .. +64]         contextId (32 bytes)
    ///      Inverse of the parse routine that does `mload(add(extraData, 33))`
    ///      after validating the version byte.
    function _buildCurrentExtradata() internal view returns (bytes memory extraData) {
        uint256 contextId = getCurrentKmsContextId();
        extraData = new bytes(33);
        extraData[0] = 0x01;
        assembly {
            mstore(add(extraData, 33), contextId)
        }
    }

    /// @notice Builds the heterogeneous-tuple encoding of cleartext values keyed by handle type.
    /// @dev Equivalent to `abi.encode(v0, v1, ..., v_{n-1})` where each `v_i` is the cleartext
    ///      cast to its FHE-natural Solidity type (bool for `Bool`, address for `Uint160`,
    ///      uint256 otherwise). Each value is right-aligned into a 32-byte word; the result is
    ///      `n * 32` bytes long with no length/offset header. Suitable for `keccak256` over the
    ///      EIP-712 PublicDecryptVerification payload.
    function _encodeTypedCleartexts(bytes32[] memory handles, uint256[] memory cleartexts)
        private
        pure
        returns (bytes memory encoded)
    {
        uint256 n = handles.length;
        encoded = new bytes(32 * n);
        for (uint256 i = 0; i < n; ++i) {
            FheType t = FheType(uint8(handles[i][30]));
            uint256 w = cleartexts[i];
            if (t == FheType.Bool) {
                w = w != 0 ? 1 : 0;
            } else if (t == FheType.Uint160) {
                w = uint256(uint160(w));
            }
            assembly {
                mstore(add(encoded, add(32, mul(i, 32))), w)
            }
        }
    }

    /// @notice EIP-712 domain separator for this KMS verifier, read from `eip712Domain()`.
    function _domainHashWithGatewayChainId() private view returns (bytes32) {
        return _domainHash(0);
    }

    function _domainHashWithHostChainId() private view returns (bytes32) {
        return _domainHash(block.chainid);
    }

    /// @notice Same as `_domainHash()` but lets the caller override the `chainId`
    ///         component of the EIP-712 domain. Pass `0` to use the default
    ///         (`eip712Domain().chainId`, which is the gateway chain id).
    ///         Useful when the caller signed the permit against a chain id that
    ///         differs from what this verifier was initialized with.
    function _domainHash(uint256 overrideChainId) private view returns (bytes32) {
        (, string memory name, string memory version, uint256 gatewayChainId, address verifyingContract,,) =
            eip712Domain();

        uint256 domainChainId = overrideChainId == 0 ? gatewayChainId : overrideChainId;

        return keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH,
                keccak256(bytes(name)),
                keccak256(bytes(version)),
                domainChainId,
                verifyingContract
            )
        );
    }

    /// @notice EIP-712 typed-data hash: `keccak256("\x19\x01" ‖ domainHash ‖ structHash)`.
    function _toTypedDataHash(bytes32 domainHash, bytes32 structHash) private pure returns (bytes32 typedDataHash) {
        assembly ("memory-safe") {
            let ptr := mload(0x40)
            mstore(ptr, hex"1901")
            mstore(add(ptr, 0x02), domainHash)
            mstore(add(ptr, 0x22), structHash)
            typedDataHash := keccak256(ptr, 0x42)
        }
    }

    function _hashPublicDecryptionResult(
        bytes32[] memory ctHandles,
        bytes memory decryptedResult,
        bytes memory extraData
    ) private view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                DECRYPTION_RESULT_TYPEHASH,
                keccak256(abi.encodePacked(ctHandles)),
                keccak256(decryptedResult),
                keccak256(abi.encodePacked(extraData))
            )
        );
        return _toTypedDataHash(_domainHashWithGatewayChainId(), structHash);
    }

    function _hashUserDecryptionResult(
        bytes memory publicKey,
        address[] memory contractAddresses,
        uint256 startTimestamp,
        uint256 durationDays,
        bytes memory extraData
    ) private view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                USER_DECRYPT_REQUEST_TYPEHASH,
                keccak256(publicKey),
                keccak256(abi.encodePacked(contractAddresses)),
                startTimestamp,
                durationDays,
                keccak256(extraData)
            )
        );
        return _toTypedDataHash(_domainHashWithHostChainId(), structHash);
    }

    function _hashDelegatedUserDecryptionResult(
        bytes memory publicKey,
        address[] memory contractAddresses,
        address delegatorAddress,
        uint256 startTimestamp,
        uint256 durationDays,
        bytes memory extraData
    ) private view returns (bytes32) {
        bytes32 structHash = keccak256(
            abi.encode(
                DELEGATED_USER_DECRYPT_REQUEST_TYPEHASH,
                keccak256(publicKey),
                keccak256(abi.encodePacked(contractAddresses)),
                delegatorAddress,
                startTimestamp,
                durationDays,
                keccak256(extraData)
            )
        );
        return _toTypedDataHash(_domainHashWithHostChainId(), structHash);
    }

    error PublicKeyTooShort(uint256 length);

    /// @notice Mock "encrypt-for-user": XORs each cleartext with the first 32 bytes of `publicKey`.
    /// @dev NOT real encryption. A reversible mask used only by the cleartext/debug KMS
    ///      verifier to simulate the user-decryption wire format without implementing
    ///      ECIES / AES. The client reverses it by XORing the received values with the
    ///      same 32 bytes from their own public key.
    ///
    ///      Reverts with `PublicKeyTooShort(publicKey.length)` if `publicKey` is shorter
    ///      than 32 bytes.
    /// @param publicKey The user's public key; only the first 32 bytes are used as mask.
    /// @param cleartexts Values to mask in-place of a copy returned to the caller.
    /// @return masked Masked values, one per input.
    function _xorMaskWithPublicKey(bytes memory publicKey, uint256[] memory cleartexts)
        private
        pure
        returns (uint256[] memory masked)
    {
        if (publicKey.length < 32) revert PublicKeyTooShort(publicKey.length);

        bytes32 mask;
        assembly {
            // mload at publicKey+32 = first 32 bytes of the data region (skip length prefix).
            mask := mload(add(publicKey, 32))
        }

        uint256 n = cleartexts.length;
        masked = new uint256[](n);
        for (uint256 i = 0; i < n; ++i) {
            masked[i] = cleartexts[i] ^ uint256(mask);
        }
    }

    function publicDecrypt(bytes32[] memory handles)
        external
        view
        virtual
        returns (
            bytes memory abiEncodedCleartexts,
            bytes32 digest,
            address[] memory signers,
            uint256 threshold,
            bytes memory extraData
        )
    {
        uint256[] memory cleartexts = CleartextACL(aclAdd).plaintextForPublicDecryption(handles);
        abiEncodedCleartexts = _encodeTypedCleartexts(handles, cleartexts);
        extraData = _buildCurrentExtradata();
        digest = _hashPublicDecryptionResult(handles, abiEncodedCleartexts, extraData);
        signers = getKmsSigners();
        threshold = getThreshold();
    }

    function userDecrypt(
        HandleContractPair[] calldata pairs,
        address userAddress,
        bytes memory publicKey,
        address[] memory contractAddresses,
        uint256 startTimestamp,
        uint256 durationDays,
        bytes memory userSignature
    )
        external
        view
        virtual
        returns (bytes memory payload, address[] memory signers, uint256 threshold, bytes memory extraData)
    {
        if (contractAddresses.length > 10) {
            revert("Too many contracts");
        }

        // Missing startTimestamp / durationDays test
        extraData = _buildCurrentExtradata();
        {
            uint256[] memory rawCleartexts = CleartextACL(aclAdd).plaintextForUserDecryption(pairs, userAddress);
            uint256[] memory cryptedCleartexts = _xorMaskWithPublicKey(publicKey, rawCleartexts);

            payload = abi.encode(cryptedCleartexts, extraData);
        }

        _verifySignature(
            _hashUserDecryptionResult(publicKey, contractAddresses, startTimestamp, durationDays, extraData),
            userSignature,
            userAddress
        );
        _requireAllPairsAuthorized(pairs, contractAddresses);

        signers = getKmsSigners();
        threshold = getThreshold();
    }

    function delegatedUserDecrypt(
        HandleContractPair[] calldata pairs,
        address delegator,
        address delegate,
        bytes memory publicKey,
        address[] memory contractAddresses,
        uint256 startTimestamp,
        uint256 durationDays,
        bytes memory delegateSignature
    )
        external
        view
        virtual
        returns (bytes memory payload, address[] memory signers, uint256 threshold, bytes memory extraData)
    {
        if (contractAddresses.length > 10) {
            revert("Too many contracts");
        }

        // Missing startTimestamp / durationDays test
        extraData = _buildCurrentExtradata();
        {
            uint256[] memory rawCleartexts =
                CleartextACL(aclAdd).plaintextForDelegatedUserDecryption(pairs, delegator, delegate);
            uint256[] memory cryptedCleartexts = _xorMaskWithPublicKey(publicKey, rawCleartexts);

            payload = abi.encode(cryptedCleartexts, extraData);
        }

        _verifySignature(
            _hashDelegatedUserDecryptionResult(
                publicKey, contractAddresses, delegator, startTimestamp, durationDays, extraData
            ),
            delegateSignature,
            delegate
        );
        _requireAllPairsAuthorized(pairs, contractAddresses);

        signers = getKmsSigners();
        threshold = getThreshold();
    }

    /// @notice Recovers a signer from `signature` over `digest` and reverts if the
    ///         result is not `expectedSigner` (or if the recovery fails outright).
    function _verifySignature(bytes32 digest, bytes memory signature, address expectedSigner) private pure {
        (uint8 v, bytes32 r, bytes32 s) = _decodeSignature(signature);
        address recoveredSigner = ecrecover(digest, v, r, s);
        if (recoveredSigner == address(0) || recoveredSigner != expectedSigner) {
            revert InvalidUserDecryptSignature();
        }
    }

    /// @notice Asserts every `pairs[i].contractAddress` appears in `contractAddresses`.
    /// @dev O(N·M) nested scan — acceptable while both dimensions stay small
    ///      (a handful of pairs, ≤10 authorized contracts). If either grows, switch
    ///      to a sorted `contractAddresses` + binary search, or a hash-set.
    function _requireAllPairsAuthorized(HandleContractPair[] calldata pairs, address[] memory contractAddresses)
        private
        pure
    {
        uint256 authCount = contractAddresses.length;
        for (uint256 p = 0; p < pairs.length; ++p) {
            address c = pairs[p].contractAddress;
            bool authorized;
            for (uint256 i = 0; i < authCount; ++i) {
                if (contractAddresses[i] == c) {
                    authorized = true;
                    break;
                }
            }
            if (!authorized) {
                revert ContractAddressNotAuthorized(c);
            }
        }
    }

    function _decodeSignature(bytes memory signature) internal pure returns (uint8 v, bytes32 r, bytes32 s) {
        if (signature.length != 65) {
            revert InvalidUserDecryptSignature();
        }

        assembly {
            r := mload(add(signature, 0x20))
            s := mload(add(signature, 0x40))
            v := byte(0, mload(add(signature, 0x60)))
        }

        if (v < 27) {
            v += 27;
        }
        if (v != 27 && v != 28) {
            revert InvalidUserDecryptSignature();
        }
    }
}
