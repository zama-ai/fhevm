// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {PaddedInputter} from "../host-contracts/contracts/PaddedInputter.sol";
import {KMSVerifier} from "../host-contracts/contracts/KMSVerifier.sol";
import {ACL} from "../host-contracts/contracts/ACL.sol";
import {IProtocolConfig} from "../host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {EXTRA_DATA_V2} from "../host-contracts/contracts/shared/Constants.sol";
import {aclAdd, kmsVerifierAdd, protocolConfigAdd} from "../host-contracts/addresses/FHEVMHostAddresses.sol";
import {LibCleartextPad} from "./shared/LibCleartextPad.sol";
import {LibKmsVerifier, UserDecryptPermitV2} from "./shared/LibKmsVerifier.sol";

// -----------------------------------------------------------------------------------------
// RFC-030 (pad-based input encryption) — pad decryption request/response types.
// Field names, types and order follow the RFC verbatim. The authentication struct is RFC-016's,
// `UserDecryptPermitV2` (LibKmsVerifier), the one `CleartextKMSVerifier.userDecryptV2` checks.
// -----------------------------------------------------------------------------------------

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
    UserDecryptPermitV2 auth; // same as RFC-016
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
 * @notice Everything the KMS nodes of a context would sign and return for a pad decryption request, except the
 *         signatures (cleartext mode only). `signers`, `partyIds` and `digests` are index-aligned: node
 *         `signers[i]` signs `digests[i]`, the EIP-712 digest of its `PadDecryptResponsePayload`
 *         `{packingFactor, extraData, degree, partyIds[i], padShares}`.
 */
struct CleartextPadSharesMaterial {
    bytes32 requestId; // mock-defined request id: keccak256(labels ‖ extraData ‖ signature)
    uint32 packingFactor; // PRF blocks per ring element
    uint32 degree; // sharing degree t of the context (n = 3t + 1)
    bytes extraData; // auth.extraData, verbatim
    PadShare[] padShares; // one entry per witness, in request order; the same for every node
    address[] signers; // KMS signers of the context named by extraData
    uint32[] partyIds; // partyIds[i] is the 1-based party id of signers[i]
    bytes32[] digests; // digests[i] is what signers[i] signs
    uint256 threshold; // user-decryption threshold of that context
}

/**
 * @title CleartextPaddedInputter
 * @notice The RFC-030 padded inputter of a cleartext stack: the markers that identify one, and the cleartext mock
 *         of RFC-030 pad DECRYPTION.
 * @dev The input half is `PaddedInputter`'s, unchanged: this overrides nothing. The cleartext value of a pad-based
 *      input is recovered by `CleartextArithmetic`, from the input package the executor hands it
 *      (`LibCleartextPad.unmaskPart`).
 *
 *      The decryption half, `getCleartextPadSharesMaterial`, stands in for the KMS Connector and Core, which serve
 *      it in production; nothing here does that on a real chain. It lives on this contract because it is the
 *      RFC-030 contract, reached from the executor (`CleartextFHEVMExecutor.getPaddedInputterAddress()`), and
 *      because it must hold a pad to the same witness rules the input half accepted it under: it reads
 *      `PaddedInputter`'s own constants and errors rather than a second copy of them. Ported from fhevm's
 *      sdk/js-sdk/contracts/src/v0.16.0/cleartext/CleartextInputVerifier.sol (`getPadShares`), adapted to this
 *      stack: `IProtocolConfig`, `LibCleartextPad`, `LibKmsVerifier`, the ACL's `isDelegatedForUserDecryption`
 *      (fhevm#4536, backported into the vendored ACL).
 *
 *      A real deployment has no `IS_CLEARTEXT` selector, so a consumer can probe it to tell a cleartext stack from
 *      a production deployment. The generation marker lives on `CleartextACL` alone, the one address every
 *      consumer already holds; `getVersion()` here still reports the base contract.
 *
 *      Constants rather than storage: these run behind proxies, where an initialized state variable is only ever
 *      written to the implementation's own storage and reads back as `false` or zero through the proxy.
 */
/// @custom:security-contact https://github.com/zama-ai/fhevm/blob/main/SECURITY.md
contract CleartextPaddedInputter is PaddedInputter {
    /// @notice Marks a cleartext (mock) implementation. Real host contracts have no such selector, so a
    ///         consumer can probe it to tell a cleartext stack from a production deployment.
    bool public constant IS_CLEARTEXT = true;

    // ---- RFC-030 pad decryption constants (the witness limits are PaddedInputter's own) ----

    /// @notice PRF blocks per ring element in the cleartext mock: one 32-byte block each.
    uint32 private constant PAD_DECRYPT_PACKING_FACTOR = 1;

    /// @notice Upper bound on the number of witnesses in one request (implementation-specific).
    uint256 private constant PAD_DECRYPT_MAX_WITNESSES = 32;

    // ---- RFC-030 pad decryption errors (PaddedInputter's own where the meaning is the same) ----

    error PadDecryptInvalidExtraData(bytes extraData);
    error PadDecryptInvalidKmsContextOrEpoch(uint256 contextId, uint256 epochId);
    error PadDecryptInvalidWitnessCount(uint256 count);
    error PadDecryptDuplicateLabel(bytes32 label);
    error PadDecryptTooManyDecryptors(bytes32 label, uint256 count);
    error PadDecryptNullWitnessAddress(bytes32 label);
    error PadDecryptInvalidPadLength(bytes32 label, uint16 padLength);
    error PadDecryptWitnessExpired(bytes32 label, uint64 expirationTimestamp);
    error PadDecryptConsumerNotAllowed(bytes32 label, address consumer);
    error PadDecryptUserNotAuthorized(bytes32 label, address userAddress);

    // -----------------------------------------------------------------------------------------
    // RFC-030 pad decryption ("Request pad decryption")
    // -----------------------------------------------------------------------------------------

    /**
     * @notice Cleartext mock of the KMS pad decryption flow (RFC-030 `verify_pad_decryption_request`): everything
     *         each KMS node of the context would sign and send back, except the signature.
     * @dev Runs the RFC's authentication and authorization checks against host-chain state, releases each
     *      requested pad keyed by label, and builds, for every signer of the context, the EIP-712 digest of the
     *      `PadDecryptResponsePayload` that node signs. The SDK then only signs `digests[i]` with the key of
     *      `signers[i]` and wraps the result. All-or-nothing: any failing witness reverts.
     *
     *      Deviations from the real KMS, all inherent to cleartext mode:
     *      - The pad comes from a keccak-based stand-in PRF (see `LibCleartextPad`), not from
     *        `PRF_{FHE(k)}(label)`. It is deterministic in `(prf, label)` only, as RFC-030 requires.
     *      - Instead of per-node threshold shares encrypted with ML-KEM + AES-GCM, each share is the
     *        whole encrypted pad — ⌈padLength/256⌉ cleartext PRF blocks of 32 bytes, i.e. one ring
     *        element per block (packing factor 1) — XOR-masked with the first 32 bytes of
     *        `auth.publicKey` (repeated), the same mask as `CleartextKMSVerifier` (`LibKmsVerifier.publicKeyMask`). The client unmasks
     *        it with the same 32 bytes and keeps the first `padLength` bits. Every node returns the same
     *        shares; only `partyId`, and so the digest, differs.
     *      - Each share's `sessionId` is a deterministic function of `(prf, label)`: one mock
     *        "session" per label, shared by every node and every requester of that label.
     *      - `degree` is `⌊(n - 1) / 3⌋` for the context's `n` signers (n = 3t + 1), and `requestId` is a
     *        mock-defined hint: keccak256(labels ‖ extraData ‖ signature).
     *
     *      The permit is held to the same limits as `CleartextKMSVerifier.userDecryptV2` (see `_checkPermit`),
     *      including a start not in the future: both are views, and a client must align its clock the same way.
     *
     * @param request The pad decryption request.
     * @return material What every node of the context signs, index-aligned with `material.signers`.
     */
    function getCleartextPadSharesMaterial(PadDecryptRequest calldata request)
        external
        view
        returns (CleartextPadSharesMaterial memory material)
    {
        uint256 contextId;
        (material.padShares, contextId) = _releasePadShares(request);

        // Production: each KMS Core node of the context signs its own PadDecryptResponsePayload.
        IProtocolConfig protocolConfig = IProtocolConfig(protocolConfigAdd);
        material.signers = protocolConfig.getKmsSignersForContext(contextId);
        material.threshold = protocolConfig.getUserDecryptionThresholdForContext(contextId);
        material.extraData = request.auth.extraData;
        material.packingFactor = PAD_DECRYPT_PACKING_FACTOR;
        uint256 n = material.signers.length;
        // forge-lint: disable-next-line(unsafe-typecast)
        material.degree = n == 0 ? 0 : uint32((n - 1) / 3);
        material.requestId = _padRequestId(request);

        // Only `partyId` differs between nodes: the shares are hashed once.
        bytes32 padSharesHash = _padSharesHash(material.padShares);
        bytes32 domainHash = _responseDomainHash();
        material.partyIds = new uint32[](n);
        material.digests = new bytes32[](n);
        for (uint256 i = 0; i < n; ++i) {
            // Party ids are 1-based, matching the deployed KMS node set.
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32 partyId = uint32(i + 1);
            material.partyIds[i] = partyId;
            material.digests[i] = keccak256(
                abi.encodePacked(
                    hex"1901",
                    domainHash,
                    LibCleartextPad.padDecryptResponseStructHash(
                        material.packingFactor, material.extraData, material.degree, partyId, padSharesHash
                    )
                )
            );
        }
    }

    // ---- RFC-030 internals ----

    /// @dev Steps 1 to 7 of the RFC: check and authenticate the permit, resolve the context, check and authorize
    ///      every witness, then release each pad mock-encrypted under the user's public key.
    function _releasePadShares(PadDecryptRequest calldata request)
        private
        view
        returns (PadShare[] memory padShares, uint256 contextId)
    {
        UserDecryptPermitV2 calldata auth = request.auth;
        PadDecryptRequestPayload calldata payload = request.payload;

        // The request's chain first: it is also the chain the permit is signed for.
        if (payload.chainId != block.chainid) revert PadInvalidChainId(payload.chainId);

        // 1.-2. The permit: its shape and window, its signature, its freshness against the ACL.
        // Production: KMS Connector.
        _checkPermit(request);
        ACL acl = ACL(aclAdd);

        // 3. Resolve the context and epoch to serve the request under (v2 extraData only).
        // Production: KMS Connector, against ProtocolConfig.
        contextId = _checkPadExtraData(auth.extraData);

        // 4.-6. Witness structure, label authentication, pad authorization.
        // Production: KMS Connector; it verifies each label, the client (SDK) computed it.
        _checkPadPayload(payload);
        for (uint256 i = 0; i < payload.witnesses.length; ++i) {
            _authorizePadWitness(acl, auth, payload.witnesses[i]);
        }

        // 7. Release each pad, mock-encrypted under the user's public key.
        // Production: KMS Core encrypts each node's payload with ML-KEM + AES-128-GCM under auth.publicKey.
        bytes32 mask = LibKmsVerifier.publicKeyMask(auth.publicKey);
        padShares = new PadShare[](payload.witnesses.length);
        for (uint256 i = 0; i < payload.witnesses.length; ++i) {
            PadWitness calldata w = payload.witnesses[i];
            padShares[i] = PadShare({
                label: w.label,
                // Production: KMS Core, the MPC session that produced this node's share.
                sessionId: LibCleartextPad.sessionId(payload.prf, w.label),
                // Production: coprocessor computes FHE(r) = PRF_{FHE(k)}(label); KMS Core threshold-decrypts it.
                share: LibCleartextPad.xorMask(LibCleartextPad.padBlocks(payload.prf, w.label, w.padLength), mask)
            });
        }
    }

    /// @dev The mock request id over the request's labels, in request order.
    function _padRequestId(PadDecryptRequest calldata request) private pure returns (bytes32) {
        PadWitness[] calldata witnesses = request.payload.witnesses;
        bytes32[] memory labels = new bytes32[](witnesses.length);
        for (uint256 i = 0; i < witnesses.length; ++i) {
            labels[i] = witnesses[i].label;
        }
        return LibCleartextPad.padRequestId(labels, request.auth.extraData, request.signature);
    }

    /// @dev EIP-712 `encodeData` of the `PadShare[] pad_shares` member: the hash of the shares' struct hashes.
    function _padSharesHash(PadShare[] memory padShares) private pure returns (bytes32) {
        bytes32[] memory hashes = new bytes32[](padShares.length);
        for (uint256 i = 0; i < padShares.length; ++i) {
            hashes[i] =
                LibCleartextPad.padShareStructHash(padShares[i].label, padShares[i].sessionId, padShares[i].share);
        }
        return keccak256(abi.encodePacked(hashes));
    }

    /// @dev The domain KMS responses are signed under: the KMSVerifier's own, with the gateway chain id.
    function _responseDomainHash() private view returns (bytes32) {
        (, string memory name, string memory version, uint256 gatewayChainId, address verifyingContract,,) =
            KMSVerifier(kmsVerifierAdd).eip712Domain();
        return LibKmsVerifier.domainSeparatorFrom(name, version, gatewayChainId, verifyingContract);
    }

    /**
     * @dev The RFC-016 permit, held to exactly what `CleartextKMSVerifier.userDecryptV2` holds the same permit to,
     *      in the same order and with the same errors (LibKmsVerifier): at most `MAX_USER_DECRYPT_CONTRACT_ADDRESSES`
     *      allowed contracts, a non-zero window of at most `MAX_USER_DECRYPT_DURATION_SECONDS`, started and not
     *      expired, the user not one of its own allowed contracts, a signature that `ecrecover`s to the user or
     *      passes its gas-capped ERC-1271 check, and a start no earlier than the user's ACL invalidation. One
     *      permit, one rule set: a request `userDecryptV2` refuses is refused here the same way.
     *
     *      The digest is under the KMS `Decryption` domain with the host chain id, already checked to be
     *      `payload.chainId`.
     */
    function _checkPermit(PadDecryptRequest calldata request) private view {
        UserDecryptPermitV2 memory permit = request.auth;
        LibKmsVerifier.requirePermitV2AllowedContractsWithinLimit(permit);
        LibKmsVerifier.requireValidRequestWindowSeconds(permit.startTimestamp, permit.durationSeconds);
        LibKmsVerifier.requirePermitV2UserNotInAllowedContracts(permit);

        (, string memory name, string memory version,, address verifyingContract,,) =
            KMSVerifier(kmsVerifierAdd).eip712Domain();
        LibKmsVerifier.requireUserSignatureV2(
            LibKmsVerifier.domainSeparatorFrom(name, version, block.chainid, verifyingContract),
            permit,
            request.signature
        );
        LibKmsVerifier.requireNotInvalidated(aclAdd, permit.userAddress, permit.startTimestamp);
    }

    /// @dev Requires v2 extraData `[0x02 | contextId | epochId]` naming a valid context and epoch.
    function _checkPadExtraData(bytes calldata extraData) private view returns (uint256 contextId) {
        if (extraData.length != 65 || uint8(extraData[0]) != EXTRA_DATA_V2) {
            revert PadDecryptInvalidExtraData(extraData);
        }
        contextId = uint256(bytes32(extraData[1:33]));
        uint256 epochId = uint256(bytes32(extraData[33:65]));

        IProtocolConfig protocolConfig = IProtocolConfig(protocolConfigAdd);
        if (!protocolConfig.isValidKmsContext(contextId) || !protocolConfig.isValidEpochForContext(contextId, epochId))
        {
            revert PadDecryptInvalidKmsContextOrEpoch(contextId, epochId);
        }
    }

    /// @dev Steps 4 and 5: request-wide fields, per-witness structure and freshness, label authentication.
    function _checkPadPayload(PadDecryptRequestPayload calldata payload) private view {
        if (payload.witnessVersion != PAD_WITNESS_VERSION) revert PadUnsupportedWitnessVersion(payload.witnessVersion);
        // RFC-030: the KMS checks every request against this contract's registry, its single source of truth. A
        // retired instance is refused too: the coprocessor no longer evaluates it.
        if (!isSupportedPrfInstance(payload.prf)) revert PadUnsupportedPrf(payload.prf);

        uint256 n = payload.witnesses.length;
        if (n == 0 || n > PAD_DECRYPT_MAX_WITNESSES) revert PadDecryptInvalidWitnessCount(n);

        for (uint256 i = 0; i < n; ++i) {
            PadWitness calldata w = payload.witnesses[i];

            for (uint256 j = 0; j < i; ++j) {
                if (payload.witnesses[j].label == w.label) revert PadDecryptDuplicateLabel(w.label);
            }

            if (w.decryptors.length > PAD_MAX_DECRYPTORS) {
                revert PadDecryptTooManyDecryptors(w.label, w.decryptors.length);
            }
            if (w.submitter == address(0)) revert PadDecryptNullWitnessAddress(w.label);
            for (uint256 j = 0; j < w.decryptors.length; ++j) {
                if (w.decryptors[j] == address(0)) revert PadDecryptNullWitnessAddress(w.label);
            }
            if (w.padLength == 0 || w.padLength > PAD_MAX_PAD_LENGTH) {
                revert PadDecryptInvalidPadLength(w.label, w.padLength);
            }
            if (w.expirationTimestamp <= block.timestamp) {
                revert PadDecryptWitnessExpired(w.label, w.expirationTimestamp);
            }

            // Production: the client (SDK) computes the label; the Connector, PaddedInputter and coprocessor verify it.
            bytes32 expectedLabel = LibCleartextPad.padLabel(
                payload.witnessVersion,
                payload.prf,
                payload.chainId,
                w.submitter,
                w.consumer,
                w.expirationTimestamp,
                w.decryptors,
                w.nonce
            );
            if (expectedLabel != w.label) revert PadLabelMismatch(w.label, expectedLabel);
        }
    }

    /// @dev Step 6: the consumer must be in `allowedContracts` (unless empty), and the user must be
    ///      the submitter, a listed decryptor, or a live delegate of one of them for the consumer.
    function _authorizePadWitness(ACL acl, UserDecryptPermitV2 calldata auth, PadWitness calldata w) private view {
        if (auth.allowedContracts.length != 0) {
            bool allowed;
            for (uint256 i = 0; i < auth.allowedContracts.length; ++i) {
                if (auth.allowedContracts[i] == w.consumer) {
                    allowed = true;
                    break;
                }
            }
            if (!allowed) revert PadDecryptConsumerNotAllowed(w.label, w.consumer);
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
        revert PadDecryptUserNotAuthorized(w.label, user);
    }
}
