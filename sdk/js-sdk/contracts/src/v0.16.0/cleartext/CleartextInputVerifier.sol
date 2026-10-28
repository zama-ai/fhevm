// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {SignatureChecker} from "@openzeppelin/contracts/utils/cryptography/SignatureChecker.sol";
import {InputVerifier} from "../host-contracts/contracts/InputVerifier.sol";
import {KMSVerifier} from "../host-contracts/contracts/KMSVerifier.sol";
import {IProtocolConfigBase} from "../host-contracts/contracts/interfaces/IProtocolConfigBase.sol";
import {EXTRA_DATA_V2} from "../host-contracts/contracts/shared/Constants.sol";
import {aclAdd, kmsVerifierAdd, protocolConfigAdd} from "../host-contracts/addresses/FHEVMHostAddresses.sol";
import {CleartextACL} from "./CleartextACL.sol";
import {CleartextPad} from "./CleartextPad.sol";

// -----------------------------------------------------------------------------------------
// RFC-030 (pad-based input encryption) — pad decryption request/response types.
// Field names, types and order follow the RFC verbatim.
// -----------------------------------------------------------------------------------------

/// @notice RFC-016 signed EIP-712 authentication struct, reused as-is by pad decryption.
struct UserDecryptRequestVerification {
    address userAddress;
    bytes publicKey;
    address[] allowedContracts;
    uint256 startTimestamp;
    uint256 durationSeconds;
    bytes extraData;
}

/// @notice One pad: its witness fields, its label (`hash(witness)`) and the number of bits to release.
struct PadWitness {
    address submitter; // witness field
    address consumer; // witness field
    uint64 expirationTimestamp; // witness field; Unix seconds
    address[] decryptors; // witness field
    bytes16 nonce; // witness field
    bytes32 label; // the pad's label, the hash of the witness
    uint16 padLength; // bits of the pad stream to release; not a witness field
}

/// @notice Unsigned part of a pad decryption request.
struct PadDecryptRequestPayload {
    uint8 witnessVersion; // shared witness field, fixed for the whole request
    uint16 prf; // shared witness field, fixed for the whole request
    uint64 chainId; // shared witness field, fixed for the whole request
    PadWitness[] witnesses; // one entry per pad
}

/// @notice Pad decryption request (`PadDecryptRequest` in RFC-030).
struct PadDecryptRequest {
    UserDecryptRequestVerification auth; // same as RFC-016
    bytes signature; // signature for auth
    PadDecryptRequestPayload payload; // unsigned
}

/// @notice A pad share keyed by label (`PadShare` in RFC-030).
struct PadShare {
    bytes32 label; // which pad this share belongs to
    bytes16 sessionId; // the MPC session that produced this share
    bytes share; // the (mock-encrypted) pad blocks
}

/**
 * @title CleartextInputVerifier
 */
contract CleartextInputVerifier is InputVerifier {
    // ---- RFC-030 constants ----

    /// @notice Domain tag of the label preimage.
    bytes8 private constant PAD_LABEL_DOMAIN_SEPARATOR = "FHE_padl";

    /// @notice Domain tag of the cleartext session id (stand-in for the MPC session, cleartext mode only).
    bytes8 private constant CLEARTEXT_SESSION_DOMAIN_SEPARATOR = "FHE_csid";

    /// @notice Bits per block of the cleartext PRF (one keccak256 output).
    uint256 public constant CLEARTEXT_PAD_BLOCK_BITS = CleartextPad.BLOCK_BITS;

    /// @notice The only witness format currently defined.
    uint8 public constant PAD_WITNESS_VERSION = 0x00;

    /// @notice Upper bound on the requested pad length, in bits.
    uint16 public constant MAX_PAD_LENGTH = 2048;

    /// @notice Upper bound on the number of entries in `PadWitness.decryptors`.
    uint256 public constant MAX_DECRYPTORS = 16;

    /// @notice Upper bound on the number of witnesses in one request (implementation-specific).
    uint256 public constant MAX_WITNESSES = 32;

    /// @notice EIP-712 domain typehash.
    bytes32 private constant EIP712_DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");

    /// @notice RFC-016 `UserDecryptRequestVerification` typehash.
    bytes32 private constant USER_DECRYPT_REQUEST_VERIFICATION_TYPEHASH = keccak256(
        "UserDecryptRequestVerification(address userAddress,bytes publicKey,address[] allowedContracts,uint256 startTimestamp,uint256 durationSeconds,bytes extraData)"
    );

    // ---- RFC-030 errors ----

    error PadInvalidUserAddress();
    error PadInvalidSignature(address userAddress);
    error PadRequestExpired(uint256 startTimestamp, uint256 durationSeconds);
    error PadSignatureInvalidated(address userAddress, uint256 startTimestamp, uint256 invalidatedBefore);
    error PadInvalidExtraData(bytes extraData);
    error PadInvalidKmsContextOrEpoch(uint256 contextId, uint256 epochId);
    error PadUnsupportedWitnessVersion(uint8 witnessVersion);
    error PadInvalidChainId(uint64 chainId);
    error PadInvalidWitnessCount(uint256 count);
    error PadDuplicateLabel(bytes32 label);
    error PadTooManyDecryptors(bytes32 label, uint256 count);
    error PadNullWitnessAddress(bytes32 label);
    error PadInvalidPadLength(bytes32 label, uint16 padLength);
    error PadWitnessExpired(bytes32 label, uint64 expirationTimestamp);
    error PadLabelMismatch(bytes32 label, bytes32 expectedLabel);
    error PadConsumerNotAllowed(bytes32 label, address consumer);
    error PadUserNotAuthorized(bytes32 label, address userAddress);
    error PadPublicKeyTooShort(uint256 length);

    function inputProof(
        bytes32[] calldata ctHandles,
        address userAddress,
        address contractAddress,
        bytes calldata extraData
    ) public view returns (bytes32 digest, address[] memory signers, uint256 threshold) {
        CleartextACL(aclAdd).requireNotPaused();

        CiphertextVerification memory ctVerif;
        ctVerif.ctHandles = ctHandles;
        ctVerif.userAddress = userAddress;
        ctVerif.contractAddress = contractAddress;
        ctVerif.contractChainId = block.chainid;
        ctVerif.extraData = extraData;

        digest = _hashEIP712InputVerification(ctVerif);

        InputVerifierStorage storage $ = _getInputVerifierStorage();
        signers = $.signers;
        threshold = $.threshold;
    }

    // -----------------------------------------------------------------------------------------
    // RFC-030 pad decryption ("Request pad decryption")
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Cleartext mock of the KMS pad decryption flow (RFC-030 `verify_pad_decryption_request`).
     * @dev Runs the RFC's authentication and authorization checks against host-chain state, then
     *      returns each requested pad keyed by label. All-or-nothing: any failing witness reverts.
     *
     *      Deviations from the real KMS, all inherent to cleartext mode:
     *      - The pad comes from a keccak-based stand-in PRF (see `CleartextPad`), not from
     *        `PRF_{FHE(k)}(label)`. It is deterministic in `(prf, label)` only, as RFC-030 requires.
     *      - Every PRF instance id is accepted (there is no `PaddedInputter` registry yet); `prf`
     *        is still bound into both the label and the cleartext pad.
     *      - Instead of per-node threshold shares encrypted with ML-KEM + AES-GCM, each share is the
     *        whole encrypted pad — ⌈padLength/256⌉ cleartext PRF blocks of 32 bytes, i.e. one ring
     *        element per block (packing factor 1) — XOR-masked with the first 32 bytes of
     *        `auth.publicKey` (repeated), the same mock as `CleartextKMSVerifier`. The client unmasks
     *        it with the same 32 bytes and keeps the first `padLength` bits. The context signers,
     *        user-decryption threshold and `extraData` are returned so the SDK can sign
     *        `PadDecryptResponsePayload` with its local KMS keys.
     *      - Each share's `sessionId` is a deterministic function of `(prf, label)`: one mock
     *        "session" per label, shared by every node and every requester of that label.
     *      - `auth.startTimestamp <= block.timestamp` is not enforced: a view call on anvil can
     *        observe a block timestamp older than the client's wall clock. Expiry is enforced.
     *
     * @param request The pad decryption request.
     * @return padShares One entry per witness, in request order, keyed by label.
     * @return signers KMS signers of the context named by `auth.extraData`.
     * @return threshold User-decryption threshold of that context.
     * @return extraData `auth.extraData`, verbatim.
     */
    function getPadShares(
        PadDecryptRequest calldata request
    )
        external
        view
        returns (PadShare[] memory padShares, address[] memory signers, uint256 threshold, bytes memory extraData)
    {
        UserDecryptRequestVerification calldata auth = request.auth;
        PadDecryptRequestPayload calldata payload = request.payload;

        // 1. Authenticate that the request is from the user (ecrecover, then ERC-1271 fallback).
        _authenticatePadRequest(auth, request.signature, payload.chainId);

        // 2. Check freshness of the request.
        CleartextACL acl = CleartextACL(aclAdd);
        if (block.timestamp > auth.startTimestamp + auth.durationSeconds) {
            revert PadRequestExpired(auth.startTimestamp, auth.durationSeconds);
        }
        uint256 invalidatedBefore = acl.decryptionSignatureInvalidatedBefore(auth.userAddress);
        if (auth.startTimestamp < invalidatedBefore) {
            revert PadSignatureInvalidated(auth.userAddress, auth.startTimestamp, invalidatedBefore);
        }

        // 3. Resolve the context and epoch to serve the request under (v2 extraData only).
        uint256 contextId = _checkPadExtraData(auth.extraData);

        // 4.-6. Witness structure, label authentication, pad authorization.
        _checkPadPayload(payload);
        for (uint256 i = 0; i < payload.witnesses.length; ++i) {
            _authorizePadWitness(acl, auth, payload.witnesses[i]);
        }

        // 7. Release each pad, mock-encrypted under the user's public key.
        bytes32 mask = _publicKeyMask(auth.publicKey);
        padShares = new PadShare[](payload.witnesses.length);
        for (uint256 i = 0; i < payload.witnesses.length; ++i) {
            PadWitness calldata w = payload.witnesses[i];
            padShares[i] = PadShare({
                label: w.label,
                sessionId: bytes16(keccak256(abi.encodePacked(CLEARTEXT_SESSION_DOMAIN_SEPARATOR, payload.prf, w.label))),
                share: _xorMask(CleartextPad.padBlocks(payload.prf, w.label, w.padLength), mask)
            });
        }

        IProtocolConfigBase protocolConfig = IProtocolConfigBase(protocolConfigAdd);
        signers = protocolConfig.getKmsSignersForContext(contextId);
        threshold = protocolConfig.getUserDecryptionThresholdForContext(contextId);
        extraData = auth.extraData;
    }

    /**
     * @notice Computes an RFC-030 label:
     *         keccak256("FHE_padl" ‖ witnessVersion ‖ prf ‖ chainId ‖ submitter ‖ consumer ‖
     *                   expirationTimestamp ‖ decryptorCount ‖ decryptors ‖ nonce)
     * @dev Fixed-width big-endian fields; each decryptor is 20 bytes (not ABI-padded).
     *      `witness.label` and `witness.padLength` are ignored.
     */
    function computePadLabel(
        uint8 witnessVersion,
        uint16 prf,
        uint64 chainId,
        PadWitness calldata witness
    ) public pure returns (bytes32) {
        bytes memory preimage = abi.encodePacked(
            PAD_LABEL_DOMAIN_SEPARATOR,
            witnessVersion,
            prf,
            chainId,
            witness.submitter,
            witness.consumer,
            witness.expirationTimestamp,
            uint8(witness.decryptors.length)
        );
        for (uint256 i = 0; i < witness.decryptors.length; ++i) {
            preimage = abi.encodePacked(preimage, witness.decryptors[i]);
        }
        return keccak256(abi.encodePacked(preimage, witness.nonce));
    }

    // ---- RFC-030 internals ----

    /// @dev RFC-016 EIP-712 digest under the KMS `Decryption` domain, with the host chain id
    ///      (`payload.chainId`) as domain chain id, then ecrecover with ERC-1271 fallback.
    function _authenticatePadRequest(
        UserDecryptRequestVerification calldata auth,
        bytes calldata signature,
        uint64 chainId
    ) private view {
        if (auth.userAddress == address(0)) revert PadInvalidUserAddress();

        bytes32 structHash = keccak256(
            abi.encode(
                USER_DECRYPT_REQUEST_VERIFICATION_TYPEHASH,
                auth.userAddress,
                keccak256(auth.publicKey),
                keccak256(abi.encodePacked(auth.allowedContracts)),
                auth.startTimestamp,
                auth.durationSeconds,
                keccak256(auth.extraData)
            )
        );
        bytes32 digest = keccak256(abi.encodePacked(hex"1901", _decryptionDomainHash(chainId), structHash));

        if (signature.length != 0) {
            (address recovered, ECDSA.RecoverError err, ) = ECDSA.tryRecover(digest, signature);
            if (err == ECDSA.RecoverError.NoError && recovered == auth.userAddress) {
                return;
            }
        }
        if (!SignatureChecker.isValidERC1271SignatureNow(auth.userAddress, digest, signature)) {
            revert PadInvalidSignature(auth.userAddress);
        }
    }

    /// @dev KMS `Decryption` EIP-712 domain (name, version, verifyingContract read from the KMSVerifier).
    function _decryptionDomainHash(uint64 chainId) private view returns (bytes32) {
        (, string memory name, string memory version, , address verifyingContract, , ) = KMSVerifier(kmsVerifierAdd)
            .eip712Domain();
        return
            keccak256(
                abi.encode(
                    EIP712_DOMAIN_TYPEHASH,
                    keccak256(bytes(name)),
                    keccak256(bytes(version)),
                    uint256(chainId),
                    verifyingContract
                )
            );
    }

    /// @dev Requires v2 extraData `[0x02 | contextId | epochId]` naming a valid context and epoch.
    function _checkPadExtraData(bytes calldata extraData) private view returns (uint256 contextId) {
        if (extraData.length != 65 || uint8(extraData[0]) != EXTRA_DATA_V2) {
            revert PadInvalidExtraData(extraData);
        }
        contextId = uint256(bytes32(extraData[1:33]));
        uint256 epochId = uint256(bytes32(extraData[33:65]));

        IProtocolConfigBase protocolConfig = IProtocolConfigBase(protocolConfigAdd);
        if (!protocolConfig.isValidKmsContext(contextId) || !protocolConfig.isValidEpochForContext(contextId, epochId)) {
            revert PadInvalidKmsContextOrEpoch(contextId, epochId);
        }
    }

    /// @dev Steps 4 and 5: request-wide fields, per-witness structure and freshness, label authentication.
    function _checkPadPayload(PadDecryptRequestPayload calldata payload) private view {
        if (payload.witnessVersion != PAD_WITNESS_VERSION) revert PadUnsupportedWitnessVersion(payload.witnessVersion);
        if (payload.chainId != block.chainid) revert PadInvalidChainId(payload.chainId);

        uint256 n = payload.witnesses.length;
        if (n == 0 || n > MAX_WITNESSES) revert PadInvalidWitnessCount(n);

        for (uint256 i = 0; i < n; ++i) {
            PadWitness calldata w = payload.witnesses[i];

            for (uint256 j = 0; j < i; ++j) {
                if (payload.witnesses[j].label == w.label) revert PadDuplicateLabel(w.label);
            }

            if (w.decryptors.length > MAX_DECRYPTORS) revert PadTooManyDecryptors(w.label, w.decryptors.length);
            if (w.submitter == address(0)) revert PadNullWitnessAddress(w.label);
            for (uint256 j = 0; j < w.decryptors.length; ++j) {
                if (w.decryptors[j] == address(0)) revert PadNullWitnessAddress(w.label);
            }
            if (w.padLength == 0 || w.padLength > MAX_PAD_LENGTH) revert PadInvalidPadLength(w.label, w.padLength);
            if (w.expirationTimestamp <= block.timestamp) revert PadWitnessExpired(w.label, w.expirationTimestamp);

            bytes32 expectedLabel = computePadLabel(payload.witnessVersion, payload.prf, payload.chainId, w);
            if (expectedLabel != w.label) revert PadLabelMismatch(w.label, expectedLabel);
        }
    }

    /// @dev Step 6: the consumer must be in `allowedContracts` (unless empty), and the user must be
    ///      the submitter, a listed decryptor, or a live delegate of one of them for the consumer.
    function _authorizePadWitness(
        CleartextACL acl,
        UserDecryptRequestVerification calldata auth,
        PadWitness calldata w
    ) private view {
        if (auth.allowedContracts.length != 0) {
            bool allowed;
            for (uint256 i = 0; i < auth.allowedContracts.length; ++i) {
                if (auth.allowedContracts[i] == w.consumer) {
                    allowed = true;
                    break;
                }
            }
            if (!allowed) revert PadConsumerNotAllowed(w.label, w.consumer);
        }

        address user = auth.userAddress;
        if (user == w.submitter) return;
        for (uint256 i = 0; i < w.decryptors.length; ++i) {
            if (user == w.decryptors[i]) return;
        }

        // Delegation fallback (RFC-030: the ACL's handle-free `isDelegatedForUserDecryption` view).
        if (acl.isDelegatedForUserDecryption(w.submitter, user, w.consumer)) return;
        for (uint256 i = 0; i < w.decryptors.length; ++i) {
            if (acl.isDelegatedForUserDecryption(w.decryptors[i], user, w.consumer)) return;
        }
        revert PadUserNotAuthorized(w.label, user);
    }

    /// @dev First 32 bytes of the user's public key, the mock encryption mask.
    function _publicKeyMask(bytes calldata publicKey) private pure returns (bytes32) {
        if (publicKey.length < 32) revert PadPublicKeyTooShort(publicKey.length);
        return bytes32(publicKey[0:32]);
    }

    /// @dev XORs `data` with `mask` repeated every 32 bytes (NOT real encryption).
    function _xorMask(bytes memory data, bytes32 mask) private pure returns (bytes memory) {
        for (uint256 i = 0; i < data.length; ++i) {
            data[i] = data[i] ^ mask[i % 32];
        }
        return data;
    }
}
