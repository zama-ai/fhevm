// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "./LibFheType.sol";
import {IDecryptionPermissions} from "./interfaces/IDecryptionPermissions.sol";
import {IPlaintexts} from "./interfaces/IPlaintexts.sol";
import {LibFhevmHandle} from "./LibFhevmHandle.sol";

/**
 * @notice One handle bound to the contract it was authorised for.
 *
 * @dev A pair rather than a handle list plus one contract, so a request covering several handles can
 *      bind each to the specific contract the user allowed it under.
 *
 * @dev Declared HERE, with the code that works on it, rather than beside any one contract that happens
 *      to take it. It crosses an ABI boundary as the tuple `(bytes32,address)`, so where it is declared
 *      changes no selector and no encoding — only who has to import whom.
 */
struct HandleContractPair {
    bytes32 handle;
    address contractAddress;
}

/**
 * @notice A signature bound to the address that is required to have produced it.
 *
 * @dev The signer here is an EXPECTATION, not a result: it is what the caller asserts, and
 *      `requireSigner` reverts unless `ecrecover` agrees. Pairing the two fields is what stops the
 *      assertion and the evidence from being passed in the wrong order, or one without the other.
 */
struct SignerSignaturePair {
    address signer;
    bytes signature;
}

/**
 * @notice Everything `publicDecryptPayload` needs, except the handles it is asked about.
 *
 * @dev No signer and no request: a public decryption is public, so there is nothing to verify and
 *      nobody to verify it against. The typehash is here and not in the two request bundles because
 *      this one belongs to upstream `KMSVerifier`, which declares it `public constant`.
 */
struct PublicDecryptPayloadArgs {
    address aclAddress;
    address plaintextsAddress;
    bytes32 domainHash;
    bytes32 typeHash;
    bytes extraData;
}

/**
 * @notice Everything `userDecryptPayloadV1` needs, except the pairs it reads from calldata.
 *
 * @dev The delegated twin below carries the reason this shape exists. This one does not yet overflow
 *      the stack — `userDecrypt` has one parameter fewer — but it is one added argument away, and the
 *      two paths are read side by side.
 */
struct UserDecryptPayloadArgsV1 {
    address aclAddress;
    address plaintextsAddress;
    UserDecryptRequestV1 request;
    SignerSignaturePair user;
    bytes32 domainHash;
    bytes extraData;
}

/**
 * @notice Everything `delegatedUserDecryptPayloadV1` needs, except the pairs it reads from calldata.
 *
 * @dev A BUNDLE FOR THE STACK, not for meaning. The eight loose arguments could not be assembled at
 *      the call site: `delegatedUserDecrypt` already holds eight parameters and four returns, and
 *      pushing eight more overflows solc's legacy codegen. Filling one memory struct field by field
 *      touches each value once and keeps a single slot live, which is what makes the single call fit.
 *      Neither consuming project enables `via_ir`, so this is structural, not a style choice.
 */
struct DelegatedUserDecryptPayloadArgsV1 {
    address aclAddress;
    address plaintextsAddress;
    UserDecryptRequestV1 request;
    address delegator;
    SignerSignaturePair delegate;
    bytes32 domainHash;
    bytes extraData;
}

/**
 * @notice The four fields a user-decryption request carries, bundled.
 *
 * @dev A struct rather than four parameters because solc runs out of stack otherwise: the verifier
 *      calls take eight arguments, and unpacking four loose ones alongside them overflows. It is not
 *      part of any ABI — the verifier's own functions still take the fields individually — so this is
 *      purely how callers hold them.
 */
struct UserDecryptRequestV1 {
    bytes transportPublicKey;
    address[] contractAddresses;
    uint256 startTimestamp;
    uint256 durationDays;
}

/// @notice The two calls a CLEARTEXT verifier answers and a production one does not.
/// @dev Declared here rather than imported, for the same reason as `IKmsVerifierView`: this file is
///      copied into two packages and must not depend on either one's generated interfaces.
interface ICleartextKmsVerifierCall {
    function userDecrypt(
        HandleContractPair[] memory pairs,
        address userAddress,
        bytes memory publicKey,
        address[] memory contractAddresses,
        uint256 startTimestamp,
        uint256 durationDays,
        bytes memory userSignature
    ) external view returns (bytes memory, address[] memory, uint256, bytes memory);

    function publicDecrypt(bytes32[] memory handles)
        external
        view
        returns (bytes memory, bytes32, address[] memory, uint256, bytes memory);

    function delegatedUserDecrypt(
        HandleContractPair[] memory pairs,
        address delegator,
        address delegate,
        bytes memory publicKey,
        address[] memory contractAddresses,
        uint256 startTimestamp,
        uint256 durationDays,
        bytes memory delegateSignature
    ) external view returns (bytes memory, address[] memory, uint256, bytes memory);
}

/// @notice The slice of a `KMSVerifier` this library reads. Every one of these is public on the
///         PRODUCTION contract — which is the point: the proof can be rebuilt from outside.
interface IKmsVerifierView {
    function eip712Domain()
        external
        view
        returns (
            bytes1 fields,
            string memory name,
            string memory version,
            uint256 chainId,
            address verifyingContract,
            bytes32 salt,
            uint256[] memory extensions
        );

    function DECRYPTION_RESULT_TYPEHASH() external view returns (bytes32);
    function getCurrentKmsContextId() external view returns (uint256);
    function getKmsSigners() external view returns (address[] memory);
    function getThreshold() external view returns (uint256);
}

/**
 * @notice One handle of a unified (RFC-016) user-decryption request, as the gateway's `HandleEntry`.
 * @dev    `ownerAddress == userAddress` is a direct decryption; anything else is a decryption delegated by
 *         `ownerAddress` to the user, for `contractAddress`.
 */
struct HandleEntry {
    bytes32 handle;
    address contractAddress;
    address ownerAddress;
}

/**
 * @notice A unified (v2, RFC-016) user-decryption permit: the signed fields, as the gateway's
 *         `UserDecryptionRequestPayload` minus the signature (passed beside it).
 * @dev    Fields: the type's fields in the type's order (`userAddress` is signed in v2). A struct so the entry
 *         point stays under the stack limit, the reason the gateway packs it too.
 */
struct UserDecryptPermitV2 {
    address userAddress;
    bytes publicKey;
    address[] allowedContracts;
    uint256 startTimestamp;
    uint256 durationSeconds;
    bytes extraData;
}

/// @dev The ACL views the kms-worker reads for a unified request, beyond `IDecryptionPermissions`.
interface IUserDecryptionPermissionsV2 {
    function isAllowed(bytes32 handle, address account) external view returns (bool);
    function isHandleDelegatedForUserDecryption(
        address delegator,
        address delegate,
        address contractAddress,
        bytes32 handle
    ) external view returns (bool);
    function decryptionSignatureInvalidatedBefore(address account) external view returns (uint256);
}

/// @dev ERC-1271, as the kms-worker calls it.
interface IERC1271 {
    function isValidSignature(bytes32 hash, bytes memory signature) external view returns (bytes4 magicValue);
}

/**
 * @notice Everything `userDecryptPayloadV2` needs, except the handles.
 * @dev    `domainHash` is the host chain id with the gateway `Decryption` contract as verifying contract, the
 *         domain a v2 permit is signed under; only the caller knows it.
 */
struct UserDecryptPayloadArgsV2 {
    address aclAddress;
    address plaintextsAddress;
    bytes32 domainHash;
    UserDecryptPermitV2 permit;
    bytes signature;
}

/**
 * @title LibKmsVerifier
 * @notice Reproduces the public-decryption PROOF against a `KMSVerifier` that has no cleartext helper —
 *         i.e. a real one, on a forked chain. The sibling of `LibInputVerifier`, for the other
 *         direction.
 *
 * @dev WHAT IT DOES NOT DO, and why the signature differs from the cleartext helper's.
 *      `CleartextKMSVerifier.publicDecrypt(handles)` returns FIVE things: the proof metadata, and the
 *      CLEARTEXTS themselves, read out of the cleartext database. Four of the five are reproducible
 *      here. The cleartexts are not — a production KMS verifier has none, anywhere, and that absence is
 *      the whole reason the event processor exists.
 *
 *      So they are a PARAMETER. The caller states what it believes the values to be — from a replay, a
 *      seeded store, or its own knowledge — and this library binds that claim into the digest the KMS
 *      signers will be checked against. Inventing them here, or silently reading zero, would produce a
 *      proof that verifies over the wrong numbers.
 *
 * @dev TWO CHAIN IDS, AND THEY DIFFER, exactly as in `LibInputVerifier`. `KMSVerifier` extends
 *      `EIP712UpgradeableCrossChain`, so its domain is pinned to the chain the signatures are MADE on —
 *      Sepolia's reports `"Decryption"`, chainId 10901 and a verifyingContract on the gateway, not its
 *      own address. The domain is therefore always read at runtime, never reconstructed from
 *      `block.chainid`.
 */
library LibKmsVerifier {
    // ---------------------------------------------------------------------------------------------
    // Errors
    // ---------------------------------------------------------------------------------------------

    /// @notice The verifier's EIP-712 domain is not the plain four-field one this library can rebuild.
    error UnsupportedEip712Domain(bytes1 fields);

    /// @notice The handle was never made publicly decryptable.
    error CleartextErrorHandleNotAllowedForPublicDecryption(bytes32 handle);

    /// @notice The delegation the request relies on was never granted.
    error CleartextErrorHandleNotDelegatedForUserDecryption(
        bytes32 handle, address contractAddress, address delegator, address delegate
    );

    /// @notice A pair naming the user as its own contract, which no grant can cover.
    error CleartextErrorUserAddressEqualsContractAddress();

    /// @notice The user was never granted persistent access to the handle.
    error CleartextErrorUserNotAuthorizedForDecrypt(bytes32 handle, address userAddress);

    /// @notice The contract was never granted persistent access to the handle.
    error CleartextErrorContractNotAuthorizedForDecrypt(bytes32 handle, address contractAddress);

    /// @notice A pair naming a contract the request does not cover.
    error CleartextErrorContractAddressNotAuthorized(address contractAddress);

    /// @notice The signature does not recover to the address it is claimed for.
    error CleartextErrorInvalidUserDecryptSignature();

    /// @notice A signature that is not 65 bytes, or whose `v` is neither 27 nor 28.
    error MalformedSignature();

    /// @notice The transport public key was too short to take a 32-byte mask from.
    error PublicKeyTooShort(uint256 length);

    /// @notice GW `EmptyCtHandles` / KW: a request must name at least one handle.
    error CleartextErrorEmptyCtHandles();

    /// @notice GW `InvalidFHEType` / `UnsupportedFHEType`: only Bool and Uint8..Uint256 decrypt.
    error CleartextErrorUnsupportedFheType(bytes32 handle, uint8 fheType);

    /// @notice GW `MaxDecryptionRequestBitSizeExceeded`.
    error CleartextErrorMaxDecryptionRequestBitSizeExceeded(uint256 maxBitSize, uint256 totalBitSize);

    /// @notice GW `EmptyContractAddresses`.
    error CleartextErrorEmptyContractAddresses();

    /// @notice GW `ContractAddressesMaxLengthExceeded`.
    error CleartextErrorContractAddressesMaxLengthExceeded(uint256 maxLength, uint256 length);

    /// @notice GW `UserAddressInContractAddresses` / `DelegatorAddressInContractAddresses`.
    error CleartextErrorAccountInContractAddresses(address account);

    /// @notice GW `InvalidNullDurationDays`.
    error CleartextErrorInvalidNullDurationDays();

    /// @notice GW `MaxDurationDaysExceeded`.
    error CleartextErrorMaxDurationDaysExceeded(uint256 maxDurationDays, uint256 durationDays);

    /// @notice GW `StartTimestampInFuture`.
    error CleartextErrorStartTimestampInFuture(uint256 currentTimestamp, uint256 startTimestamp);

    /// @notice GW `UserDecryptionRequestExpired`.
    error CleartextErrorUserDecryptionRequestExpired(
        uint256 currentTimestamp, uint256 startTimestamp, uint256 durationDays
    );

    /// @notice GW `InvalidNullDurationSeconds`.
    error CleartextErrorInvalidNullDurationSeconds();

    /// @notice GW `MaxDurationSecondsExceeded`.
    error CleartextErrorMaxDurationSecondsExceeded(uint256 maxDurationSeconds, uint256 durationSeconds);

    /// @notice GW `UserDecryptionRequestExpiredSeconds`.
    error CleartextErrorUserDecryptionRequestExpiredSeconds(
        uint256 currentTimestamp, uint256 startTimestamp, uint256 durationSeconds
    );

    /// @notice KW: the user may not list itself in `allowedContracts`.
    error CleartextErrorUserAddressInAllowedContracts(address userAddress);

    /// @notice KW `verify_signature`: neither `ecrecover` nor ERC-1271 accepted the signature.
    error CleartextErrorInvalidUserDecryptSignatureV2(address userAddress);

    /// @notice KW: the user invalidated every permit that starts before `invalidatedBefore`.
    error CleartextErrorUserDecryptSignatureInvalidated(
        address userAddress, uint256 startTimestamp, uint256 invalidatedBefore
    );

    /// @notice KW direct ownership: `isAllowed(handle, user)` is false.
    error CleartextErrorUserNotAllowed(bytes32 handle, address userAddress);

    /// @notice KW delegated ownership: `isHandleDelegatedForUserDecryption` is false.
    error CleartextErrorUserNotDelegated(
        bytes32 handle, address ownerAddress, address userAddress, address contractAddress
    );

    /// @notice KW: `allowedContracts` is non-empty and none of them is allowed on the handle.
    error CleartextErrorNoAllowedContractForHandle(bytes32 handle);

    // ---------------------------------------------------------------------------------------------
    // Constants
    // ---------------------------------------------------------------------------------------------

    /// @dev `keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)")`
    bytes32 private constant EIP712_DOMAIN_TYPEHASH =
        0x8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f;

    /// @dev ERC-5267 field mask for name + version + chainId + verifyingContract, and nothing else.
    bytes1 private constant EXPECTED_DOMAIN_FIELDS = 0x0f;

    /**
     * @notice The two request typehashes, HERE rather than on any verifier.
     *
     * @dev They are not deployment-specific and not upstream's: they describe the two request structs
     *      this library hashes, so a copy on a verifier is a copy that can fall behind. `internal` so a
     *      consumer rebuilding a digest by hand — a forge helper, say — names the same constant the
     *      chain-side check uses. `DECRYPTION_RESULT_TYPEHASH` stays where it is: that one is upstream
     *      `KMSVerifier`'s own public constant, and this library reads it through the interface.
     */
    bytes32 internal constant USER_DECRYPT_REQUEST_TYPEHASH = keccak256(
        "UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 startTimestamp,uint256 durationDays,bytes extraData)"
    );

    bytes32 internal constant DELEGATED_USER_DECRYPT_REQUEST_TYPEHASH = keccak256(
        "DelegatedUserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,address delegatorAddress,uint256 startTimestamp,uint256 durationDays,bytes extraData)"
    );

    /// @dev GW `MAX_DECRYPTION_REQUEST_BITS`, summed over `FHETypeBitSizes` (a Bool counts 2 bits).
    uint256 internal constant MAX_DECRYPTION_REQUEST_BITS = 2048;

    /// @dev GW `MAX_USER_DECRYPT_CONTRACT_ADDRESSES`.
    uint256 internal constant MAX_USER_DECRYPT_CONTRACT_ADDRESSES = 10;

    /// @dev GW `MAX_USER_DECRYPT_DURATION_DAYS`.
    uint256 internal constant MAX_USER_DECRYPT_DURATION_DAYS = 365;

    /// @dev The unified permit's EIP-712 type, `shared/user-decryption-signature` in the connector.
    bytes32 internal constant USER_DECRYPT_REQUEST_V2_TYPEHASH = keccak256(
        "UserDecryptRequestVerification(address userAddress,bytes publicKey,address[] allowedContracts,uint256 startTimestamp,uint256 durationSeconds,bytes extraData)"
    );

    /// @dev GW `MAX_USER_DECRYPT_DURATION_SECONDS`.
    uint256 internal constant MAX_USER_DECRYPT_DURATION_SECONDS = MAX_USER_DECRYPT_DURATION_DAYS * 1 days;

    /// @dev KW `erc1271_gas_limit` default.
    uint256 private constant ERC1271_GAS_LIMIT = 250_000;

    /// @dev `bytes4(keccak256("isValidSignature(bytes32,bytes)"))`.
    bytes4 private constant ERC1271_MAGIC_VALUE = 0x1626ba7e;

    // ---------------------------------------------------------------------------------------------
    // Public functions (`internal`: what an importer calls)
    // ---------------------------------------------------------------------------------------------

    /**
     * @notice What `CleartextKMSVerifier.publicDecrypt` would have returned, minus the values.
     * @param  kmsVerifier            The verifier the proof will be presented to.
     * @param  handles                The handles this decryption covers, in order.
     * @param  abiEncodedCleartexts   What the caller asserts those handles decrypt to, in the layout
     *                                `_encodeTypedCleartexts` produces: one 32-byte word per handle,
     *                                each value right-aligned at its FHE-natural type.
     */
    function publicDecryptProofDigest(address kmsVerifier, bytes32[] memory handles, bytes memory abiEncodedCleartexts)
        internal
        view
        returns (bytes32 digest, address[] memory signers, uint256 threshold, bytes memory extraData)
    {
        IKmsVerifierView v = IKmsVerifierView(kmsVerifier);

        extraData = currentExtraData(kmsVerifier);

        bytes32 structHash = keccak256(
            abi.encode(
                v.DECRYPTION_RESULT_TYPEHASH(),
                keccak256(abi.encodePacked(handles)),
                keccak256(abiEncodedCleartexts),
                keccak256(abi.encodePacked(extraData))
            )
        );

        digest = toTypedDataHash(domainSeparator(kmsVerifier), structHash);
        signers = v.getKmsSigners();
        threshold = v.getThreshold();
    }

    /**
     * @notice What `CleartextKMSVerifier.userDecrypt` would have returned, minus the values.
     *
     * @dev Same division as `publicDecryptProofDigest`, for the same reason: the payload is
     *      `abi.encode(cleartexts XOR mask, extraData)`, and the cleartexts come from the database. A
     *      production verifier has none, so the caller supplies them and this builds the rest.
     *
     * @dev WHAT IT DOES NOT CHECK, and the cleartext verifier does: the user's EIP-712 signature over
     *      the request, and that every pair is ACL-authorised for that user. Those are the real KMS's
     *      job, off chain, and nothing on a forked host can stand in for them — so a caller using this
     *      path is asserting authorisation as well as values.
     */
    function userDecryptPayload(address kmsVerifier, bytes memory transportPublicKey, uint256[] memory cleartexts)
        internal
        view
        returns (bytes memory payload, address[] memory signers, uint256 threshold, bytes memory extraData)
    {
        IKmsVerifierView v = IKmsVerifierView(kmsVerifier);

        extraData = currentExtraData(kmsVerifier);
        payload = abi.encode(maskWithPublicKey(transportPublicKey, cleartexts), extraData);
        signers = v.getKmsSigners();
        threshold = v.getThreshold();
    }

    /// @notice The XOR mask the transport key applies, mirroring `_xorMaskWithPublicKey`.
    /// @dev    The mask is the FIRST 32 BYTES of the key — for an uncompressed secp256k1 point that is
    ///         `0x04` followed by the first 31 bytes of X, NOT X. Getting that wrong yields shares that
    ///         reconstruct to garbage rather than failing.
    function maskWithPublicKey(bytes memory transportPublicKey, uint256[] memory cleartexts)
        internal
        pure
        returns (uint256[] memory masked)
    {
        bytes32 mask = publicKeyMask(transportPublicKey);

        masked = new uint256[](cleartexts.length);
        for (uint256 i = 0; i < cleartexts.length; i++) {
            masked[i] = cleartexts[i] ^ uint256(mask);
        }
    }

    /// @notice The 32-byte XOR mask of a transport public key: its FIRST 32 bytes (see `maskWithPublicKey`). The one
    ///         rule every cleartext mock masks with, user decryption and RFC-030 pad decryption alike.
    function publicKeyMask(bytes memory transportPublicKey) internal pure returns (bytes32 mask) {
        if (transportPublicKey.length < 32) revert PublicKeyTooShort(transportPublicKey.length);
        assembly {
            mask := mload(add(transportPublicKey, 32))
        }
    }

    /**
     * @notice The heterogeneous-tuple encoding of cleartexts keyed by handle type.
     *
     * @dev One 32-byte word per handle, each value right-aligned at its FHE-natural type — `bool` for
     *      `Bool`, `address` for `Uint160`, `uint256` otherwise — with no length or offset header. This
     *      is what the decryption digest is taken over, so the NARROWING matters: a `Bool` word holding
     *      2 would hash differently from the 1 the protocol would have produced.
     */
    function encodeTypedCleartexts(bytes32[] memory handles, uint256[] memory cleartexts)
        internal
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

    /**
     * @notice The three EIP-712 struct hashes, each taken to a typed-data hash under `domainHash`.
     *
     * @dev THE DOMAIN IS A PARAMETER, which is what lets these live here at all. A verifier's separator
     *      comes from its own `eip712Domain()` and the caller already holds it — the public-decrypt one
     *      under the GATEWAY chain id, the two request hashes under the HOST chain id, a distinction
     *      that belongs to the caller and not to the hashing.
     *
     *      `publicDecryptionDigest` also takes its typehash, because that one is upstream
     *      `KMSVerifier`'s `public constant` and passing it keeps one definition rather than a copy
     *      here that could fall behind an upstream change. The two REQUEST typehashes are this
     *      library's own constants, so those digests do not take one.
     */
    function publicDecryptionDigest(
        bytes32 domainHash,
        bytes32 typeHash,
        bytes32[] memory ctHandles,
        bytes memory decryptedResult,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        return toTypedDataHash(
            domainHash,
            keccak256(
                abi.encode(
                    typeHash,
                    keccak256(abi.encodePacked(ctHandles)),
                    keccak256(decryptedResult),
                    keccak256(abi.encodePacked(extraData))
                )
            )
        );
    }

    /// @notice The user-decrypt request digest, over the request as the caller holds it.
    function userDecryptionDigestV1(bytes32 domainHash, UserDecryptRequestV1 memory request, bytes memory extraData)
        internal
        pure
        returns (bytes32)
    {
        return toTypedDataHash(
            domainHash,
            keccak256(
                abi.encode(
                    USER_DECRYPT_REQUEST_TYPEHASH,
                    keccak256(request.transportPublicKey),
                    keccak256(abi.encodePacked(request.contractAddresses)),
                    request.startTimestamp,
                    request.durationDays,
                    keccak256(extraData)
                )
            )
        );
    }

    /**
     * @notice The digest a user signs to request a decryption from `kmsVerifier`, and the `extraData`
     *         that digest commits to.
     *
     * @dev The two travel together because they must: the extraData names the KMS context, and a digest
     *      built against one context with the extraData of another verifies nowhere. Callers used to
     *      compose these three calls themselves, and could pair them wrongly.
     */
    function userDecryptDigestV1OnStack(address kmsVerifier, UserDecryptRequestV1 memory request)
        internal
        view
        returns (bytes32 digest, bytes memory extraData)
    {
        extraData = currentExtraData(kmsVerifier);
        digest = userDecryptionDigestV1(hostDomainSeparator(kmsVerifier), request, extraData);
    }

    /// @notice The delegated twin.
    function delegatedUserDecryptDigestV1OnStack(
        address kmsVerifier,
        UserDecryptRequestV1 memory request,
        address delegatorAddress
    ) internal view returns (bytes32 digest, bytes memory extraData) {
        extraData = currentExtraData(kmsVerifier);
        digest = delegatedUserDecryptionDigestV1(hostDomainSeparator(kmsVerifier), request, delegatorAddress, extraData);
    }

    /// @notice The delegated user-decrypt request digest — the same, plus the delegator.
    function delegatedUserDecryptionDigestV1(
        bytes32 domainHash,
        UserDecryptRequestV1 memory request,
        address delegatorAddress,
        bytes memory extraData
    ) internal pure returns (bytes32) {
        return toTypedDataHash(
            domainHash,
            keccak256(
                abi.encode(
                    DELEGATED_USER_DECRYPT_REQUEST_TYPEHASH,
                    keccak256(request.transportPublicKey),
                    keccak256(abi.encodePacked(request.contractAddresses)),
                    delegatorAddress,
                    request.startTimestamp,
                    request.durationDays,
                    keccak256(extraData)
                )
            )
        );
    }

    /**
     * @notice The cleartexts behind a set of handles, once the ACL has been asked whether the caller
     *         may have them. One function per decryption path.
     *
     * @dev BOTH ADDRESSES ARE PARAMETERS, which is what lets these live here. The verifier reaches the
     *      ACL and the plaintext store through compile-time constants, correct for exactly one
     *      deployment; passing them makes the same checks usable against a stack deployed anywhere —
     *      and makes what each path requires explicit rather than ambient.
     *
     * @dev THE ORDER MATTERS. Every permission is checked for every handle BEFORE any value is read,
     *      so a request that is partly unauthorised reveals nothing at all. Interleaving the two loops
     *      would leak the values preceding the first refusal through gas and, on a revert with partial
     *      state, through traces.
     */
    function plaintextsForPublicDecryption(address aclAddress, address plaintextsAddress, bytes32[] memory handles)
        internal
        view
        returns (uint256[] memory cleartexts)
    {
        uint256 n = handles.length;

        for (uint256 i = 0; i < n; ++i) {
            if (!IDecryptionPermissions(aclAddress).isAllowedForDecryption(handles[i])) {
                revert CleartextErrorHandleNotAllowedForPublicDecryption(handles[i]);
            }
        }

        cleartexts = new uint256[](n);
        for (uint256 i = 0; i < n; ++i) {
            cleartexts[i] = IPlaintexts(plaintextsAddress).plaintexts(handles[i]);
        }
    }

    /// @notice Same, for a user decrypting handles allowed to them AND to the contract holding them.
    function plaintextsForUserDecryption(
        address aclAddress,
        address plaintextsAddress,
        HandleContractPair[] memory pairs,
        address userAddress
    ) internal view returns (uint256[] memory cleartexts) {
        uint256 n = pairs.length;

        for (uint256 i = 0; i < n; ++i) {
            bytes32 handle = pairs[i].handle;
            address contractAddress = pairs[i].contractAddress;

            if (userAddress == contractAddress) revert CleartextErrorUserAddressEqualsContractAddress();
            if (!IDecryptionPermissions(aclAddress).persistAllowed(handle, userAddress)) {
                revert CleartextErrorUserNotAuthorizedForDecrypt(handle, userAddress);
            }
            if (!IDecryptionPermissions(aclAddress).persistAllowed(handle, contractAddress)) {
                revert CleartextErrorContractNotAuthorizedForDecrypt(handle, contractAddress);
            }
        }

        cleartexts = new uint256[](n);
        for (uint256 i = 0; i < n; ++i) {
            cleartexts[i] = IPlaintexts(plaintextsAddress).plaintexts(pairs[i].handle);
        }
    }

    /// @notice Same, for a delegate acting on a delegator's behalf.
    function plaintextsForDelegatedUserDecryption(
        address aclAddress,
        address plaintextsAddress,
        HandleContractPair[] memory pairs,
        address delegator,
        address delegate
    ) internal view returns (uint256[] memory cleartexts) {
        uint256 n = pairs.length;

        for (uint256 i = 0; i < n; ++i) {
            bytes32 handle = pairs[i].handle;
            address contractAddress = pairs[i].contractAddress;

            if (!IDecryptionPermissions(aclAddress)
                    .isHandleDelegatedForUserDecryption(delegator, delegate, contractAddress, handle)) {
                revert CleartextErrorHandleNotDelegatedForUserDecryption(handle, contractAddress, delegator, delegate);
            }
        }

        cleartexts = new uint256[](n);
        for (uint256 i = 0; i < n; ++i) {
            cleartexts[i] = IPlaintexts(plaintextsAddress).plaintexts(pairs[i].handle);
        }
    }

    // --- Request conformance: the gateway's checks on the handles of a decryption request ---

    /// @notice GW: 1 to `MAX_USER_DECRYPT_CONTRACT_ADDRESSES` contract addresses, none of which is `account`
    ///         (the user, or the delegator of a delegated request). Stricter than the per-pair
    ///         `user != contract` check below, which only looks at the contracts the pairs name.
    function requireValidContractAddresses(address[] memory contractAddresses, address account) internal pure {
        uint256 n = contractAddresses.length;
        if (n == 0) revert CleartextErrorEmptyContractAddresses();
        if (n > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
            revert CleartextErrorContractAddressesMaxLengthExceeded(MAX_USER_DECRYPT_CONTRACT_ADDRESSES, n);
        }
        for (uint256 i = 0; i < n; ++i) {
            if (contractAddresses[i] == account) revert CleartextErrorAccountInContractAddresses(account);
        }
    }

    /// @notice GW `_checkUserDecryptionRequestValidity`, in the same order: a non-zero duration of at most
    ///         `MAX_USER_DECRYPT_DURATION_DAYS`, started, not expired.
    function requireValidRequestWindow(uint256 startTimestamp, uint256 durationDays) internal view {
        if (durationDays == 0) revert CleartextErrorInvalidNullDurationDays();
        if (durationDays > MAX_USER_DECRYPT_DURATION_DAYS) {
            revert CleartextErrorMaxDurationDaysExceeded(MAX_USER_DECRYPT_DURATION_DAYS, durationDays);
        }
        if (startTimestamp > block.timestamp) {
            revert CleartextErrorStartTimestampInFuture(block.timestamp, startTimestamp);
        }
        if (startTimestamp + durationDays * 1 days < block.timestamp) {
            revert CleartextErrorUserDecryptionRequestExpired(block.timestamp, startTimestamp, durationDays);
        }
    }

    /// @notice GW `_extractCtHandlesCheckConformance`, for a public decryption: at
    ///         least one handle, every handle conformant, at most `MAX_DECRYPTION_REQUEST_BITS` in total.
    function requireConformantHandles(bytes32[] memory handles) internal view {
        uint256 n = handles.length;
        if (n == 0) revert CleartextErrorEmptyCtHandles();
        uint256 totalBitSize;
        for (uint256 i = 0; i < n; ++i) {
            totalBitSize += requireConformantHandle(handles[i]);
        }
        requireTotalBitSize(totalBitSize);
    }

    /// @notice GW `_extractCtHandlesCheckConformanceUser`: at least one pair,
    ///         every handle conformant, at most `MAX_DECRYPTION_REQUEST_BITS` in total.
    function requireConformantPairs(HandleContractPair[] memory pairs) internal view {
        uint256 n = pairs.length;
        if (n == 0) revert CleartextErrorEmptyCtHandles();
        uint256 totalBitSize;
        for (uint256 i = 0; i < n; ++i) {
            totalBitSize += requireConformantHandle(pairs[i].handle);
        }
        requireTotalBitSize(totalBitSize);
    }

    /// @notice GW `MaxDecryptionRequestBitSizeExceeded`: a request decrypts at most `MAX_DECRYPTION_REQUEST_BITS`.
    function requireTotalBitSize(uint256 totalBitSize) internal pure {
        if (totalBitSize > MAX_DECRYPTION_REQUEST_BITS) {
            revert CleartextErrorMaxDecryptionRequestBitSizeExceeded(MAX_DECRYPTION_REQUEST_BITS, totalBitSize);
        }
    }

    /// @notice One handle of a request: non-zero and on this chain (GW chain-id check, as
    ///         `LibFhevmHandle.checkChainId`), and of a decryptable type. Returns the bits it counts.
    /// @dev    NOT CHECKED: that the handle exists (GW `CiphertextMaterialNotFound`). "Not in the cleartext store"
    ///         does not mean "never minted" on a fork, where handles older than the cleartext layer exist but the
    ///         fresh store holds none of them. A handle that was truly never minted is refused anyway, by the ACL
    ///         walk: nothing can be allowed on it.
    function requireConformantHandle(bytes32 handle) internal view returns (uint256 bitSize) {
        LibFhevmHandle.checkChainId(handle);
        bitSize = decryptableBitSize(handle);
    }

    /// @notice The bits `handle` counts in a decryption request (GW `FHETypeBitSizes.getBitSize`); reverts for
    ///         a type that does not decrypt. A Bool counts 2 here, 1 in `LibFheType.bitWidthForType`.
    /// @dev    Read from the raw byte, so an out-of-range type reverts with a named error rather than an
    ///         enum-conversion panic.
    function decryptableBitSize(bytes32 handle) internal pure returns (uint256) {
        uint8 t = uint8(handle[30]);
        if (t == uint8(FheType.Bool)) return 2;
        if (t == uint8(FheType.Uint8)) return 8;
        if (t == uint8(FheType.Uint16)) return 16;
        if (t == uint8(FheType.Uint32)) return 32;
        if (t == uint8(FheType.Uint64)) return 64;
        if (t == uint8(FheType.Uint128)) return 128;
        if (t == uint8(FheType.Uint160)) return 160;
        if (t == uint8(FheType.Uint256)) return 256;
        revert CleartextErrorUnsupportedFheType(handle, t);
    }

    /**
     * @notice The public-decryption payload: read every cleartext, encode it, and hash what the KMS
     *         nodes will sign. The third path, beside the two payload builders below.
     *
     * @dev NOT the verifier's `publicDecrypt`, which also answers with the signer set and threshold.
     *      This is only the half that depends on the handles.
     *
     * @dev The handles' conformance first (the gateway's rules: `requireConformantHandles`). Nothing else is
     *      checked and nothing can be: a public decryption is public, so the ACL walk inside
     *      `plaintextsForPublicDecryption` is the whole of the authorisation.
     *
     * @dev `extraData` is carried in rather than derived, because it comes from the verifier's own
     *      KMS context id — an internal read the caller makes far more cheaply than a staticcall back
     *      into itself. The domain and typehash are parameters for the reasons above: the gateway chain
     *      id is the caller's to know, and the typehash is upstream's public constant.
     */
    function publicDecryptPayload(bytes32[] memory handles, PublicDecryptPayloadArgs memory p)
        internal
        view
        returns (bytes memory abiEncodedCleartexts, bytes32 digest)
    {
        requireConformantHandles(handles);
        abiEncodedCleartexts =
            encodeTypedCleartexts(handles, plaintextsForPublicDecryption(p.aclAddress, p.plaintextsAddress, handles));
        digest = publicDecryptionDigest(p.domainHash, p.typeHash, handles, abiEncodedCleartexts, p.extraData);
    }

    /**
     * @notice The delegated user-decryption payload: check, read, mask, encode.
     *
     * @dev THE WHOLE VALUE HALF IN ONE PLACE. Permission check, plaintext read, transport mask and
     *      encoding were four steps spread across the verifier; they are one operation and only ever
     *      appear together. Keeping them here also keeps the ORDER — every pair authorised before any
     *      value is read — which is a property of the sequence, not of any one step.
     *
     *      EVERY CHECK IS HERE TOO — the request's contract addresses and window, the pairs' conformance,
     *      the delegate's signature and the pair/contract coverage — so the order cannot be got wrong by a caller: nothing is read until the request is
     *      proven. The caller supplies the domain and typehash because only it knows which chain id its
     *      verifier signs under.
     */
    function delegatedUserDecryptPayloadV1(
        HandleContractPair[] memory pairs,
        DelegatedUserDecryptPayloadArgsV1 memory p
    ) internal view returns (bytes memory payload) {
        // GW checks the DELEGATOR against the contract list; the KMS checks the client, the delegate.
        requireValidContractAddresses(p.request.contractAddresses, p.delegator);
        requireValidRequestWindow(p.request.startTimestamp, p.request.durationDays);
        requireConformantPairs(pairs);
        requireSigner(
            delegatedUserDecryptionDigestV1(p.domainHash, p.request, p.delegator, p.extraData),
            p.delegate.signature,
            p.delegate.signer
        );
        requireAllPairsAuthorized(pairs, p.request.contractAddresses);

        uint256[] memory raw = plaintextsForDelegatedUserDecryption(
            p.aclAddress, p.plaintextsAddress, pairs, p.delegator, p.delegate.signer
        );

        return abi.encode(maskWithPublicKey(p.request.transportPublicKey, raw), p.extraData);
    }

    /// @notice The same, for a user decrypting on their own behalf.
    function userDecryptPayloadV1(HandleContractPair[] memory pairs, UserDecryptPayloadArgsV1 memory p)
        internal
        view
        returns (bytes memory payload)
    {
        requireValidContractAddresses(p.request.contractAddresses, p.user.signer);
        requireValidRequestWindow(p.request.startTimestamp, p.request.durationDays);
        requireConformantPairs(pairs);
        requireSigner(userDecryptionDigestV1(p.domainHash, p.request, p.extraData), p.user.signature, p.user.signer);
        requireAllPairsAuthorized(pairs, p.request.contractAddresses);

        uint256[] memory raw = plaintextsForUserDecryption(p.aclAddress, p.plaintextsAddress, pairs, p.user.signer);

        return abi.encode(maskWithPublicKey(p.request.transportPublicKey, raw), p.extraData);
    }

    // --- Against a verifier that cannot answer for itself ---

    /**
     * @notice The public-decryption answer from whichever stack `kmsVerifier` belongs to.
     *
     * @dev The third `OnStack` entry point, and the same shape as the two below: a cleartext verifier
     *      answers for itself; anything else is served from the named ACL and plaintext source. Without
     *      this, public decryption was the one path with no way to reach a fork — it could only ask a
     *      verifier that holds cleartexts, or be told the values outright.
     *
     * @dev The GATEWAY chain id here, not the host one: a decryption RESULT is signed under the
     *      verifier's own domain, unlike a user-decryption REQUEST.
     */
    function publicDecryptV1OnForkStack(
        address kmsVerifier,
        address aclAddress,
        address plaintextsAddress,
        bytes32[] memory handles
    )
        internal
        view
        returns (bytes memory abiEncodedCleartexts, bytes32 digest, address[] memory signers, bytes memory extraData)
    {
        PublicDecryptPayloadArgs memory p;
        p.aclAddress = aclAddress;
        p.plaintextsAddress = plaintextsAddress;
        p.domainHash = domainSeparator(kmsVerifier);
        p.typeHash = IKmsVerifierView(kmsVerifier).DECRYPTION_RESULT_TYPEHASH();
        p.extraData = currentExtraData(kmsVerifier);

        (abiEncodedCleartexts, digest) = publicDecryptPayload(handles, p);
        extraData = p.extraData;
        signers = IKmsVerifierView(kmsVerifier).getKmsSigners();
    }

    /**
     * @notice What `CleartextKMSVerifier.userDecrypt` returns, computed for a verifier that has no
     *         such function — a production one, on a fork.
     *
     * @dev THE POINT. A production `KMSVerifier` cannot do this: it holds no cleartexts, and its
     *      `userDecrypt` does not exist. Everything it WOULD have supplied is taken from elsewhere —
     *      the permission walk from the forked ACL, the values from whatever implements `IPlaintexts`
     *      (on a fork, the event processor), the domain and the signer set from the verifier's own
     *      public surface. The checks are not skipped: `userDecryptPayloadV1` still checks the request's
     *      contract addresses and window and the pairs' conformance (the gateway's rules), verifies the user's
     *      signature and requires every pair to be authorised, against the REAL ACL.
     *
     * @dev THE GATEWAY'S LIMITS APPLY HERE TOO, the cap of `MAX_USER_DECRYPT_CONTRACT_ADDRESSES` contract
     *      addresses included: the gateway's `Decryption` contract refuses a request beyond it, so a request
     *      the fork accepted, the chain would refuse.
     */
    function userDecryptV1OnForkStack(
        address kmsVerifier,
        address aclAddress,
        address plaintextsAddress,
        HandleContractPair[] memory pairs,
        UserDecryptRequestV1 memory request,
        SignerSignaturePair memory user
    )
        internal
        view
        returns (bytes memory payload, address[] memory signers, uint256 threshold, bytes memory extraData)
    {
        UserDecryptPayloadArgsV1 memory p;
        p.aclAddress = aclAddress;
        p.plaintextsAddress = plaintextsAddress;
        p.request = request;
        p.user = user;
        p.domainHash = hostDomainSeparator(kmsVerifier);
        p.extraData = currentExtraData(kmsVerifier);

        extraData = p.extraData;
        payload = userDecryptPayloadV1(pairs, p);
        signers = IKmsVerifierView(kmsVerifier).getKmsSigners();
        threshold = IKmsVerifierView(kmsVerifier).getThreshold();
    }

    /// @notice The delegated twin, and the reason the delegated path can reach a fork at all.
    /// @dev The delegation itself is checked, against the forked ACL, by
    ///      `plaintextsForDelegatedUserDecryption` inside the payload builder — so a test that asserts
    ///      "the delegate may read" is asserting something, rather than signcrypting its own claim.
    function delegatedUserDecryptV1OnForkStack(
        address kmsVerifier,
        address aclAddress,
        address plaintextsAddress,
        HandleContractPair[] memory pairs,
        UserDecryptRequestV1 memory request,
        address delegator,
        SignerSignaturePair memory delegate
    )
        internal
        view
        returns (bytes memory payload, address[] memory signers, uint256 threshold, bytes memory extraData)
    {
        DelegatedUserDecryptPayloadArgsV1 memory p;
        p.aclAddress = aclAddress;
        p.plaintextsAddress = plaintextsAddress;
        p.request = request;
        p.delegator = delegator;
        p.delegate = delegate;
        p.domainHash = hostDomainSeparator(kmsVerifier);
        p.extraData = currentExtraData(kmsVerifier);

        extraData = p.extraData;
        payload = delegatedUserDecryptPayloadV1(pairs, p);
        signers = IKmsVerifierView(kmsVerifier).getKmsSigners();
        threshold = IKmsVerifierView(kmsVerifier).getThreshold();
    }

    /**
     * @notice The same three operations asked of a CLEARTEXT verifier, which answers them itself.
     *
     * @dev NO ACL AND NO PLAINTEXT SOURCE, deliberately. A cleartext verifier reaches both through its
     *      own compile-time constants and exposes no getter for either, so a caller that passed them
     *      could not be told they were being ignored. Two entry points rather than one silent branch:
     *      the caller probes with `LibCleartextProbe.isCleartext` and picks, and whichever it picks,
     *      every argument it passes is used.
     */
    function userDecryptV1OnCleartextStack(
        address kmsVerifier,
        HandleContractPair[] memory pairs,
        UserDecryptRequestV1 memory request,
        SignerSignaturePair memory user
    ) internal view returns (bytes memory, address[] memory, uint256, bytes memory) {
        return _cleartextUserDecrypt(kmsVerifier, pairs, request, user);
    }

    /// @notice The delegated twin.
    function delegatedUserDecryptV1OnCleartextStack(
        address kmsVerifier,
        HandleContractPair[] memory pairs,
        UserDecryptRequestV1 memory request,
        address delegator,
        SignerSignaturePair memory delegate
    ) internal view returns (bytes memory, address[] memory, uint256, bytes memory) {
        return _cleartextDelegatedUserDecrypt(kmsVerifier, pairs, request, delegator, delegate);
    }

    /// @notice The public twin.
    /// @dev The verifier's `threshold` is dropped: every caller of this signs with the FULL signer set.
    function publicDecryptV1OnCleartextStack(address kmsVerifier, bytes32[] memory handles)
        internal
        view
        returns (bytes memory abiEncodedCleartexts, bytes32 digest, address[] memory signers, bytes memory extraData)
    {
        (abiEncodedCleartexts, digest, signers,, extraData) =
            ICleartextKmsVerifierCall(kmsVerifier).publicDecrypt(handles);
    }

    /// @notice Splits a 65-byte `r ‖ s ‖ v`, normalising a `v` of 0/1 to 27/28.
    function decodeSignature(bytes memory signature) internal pure returns (uint8 v, bytes32 r, bytes32 s) {
        if (signature.length != 65) revert MalformedSignature();

        assembly {
            r := mload(add(signature, 0x20))
            s := mload(add(signature, 0x40))
            v := byte(0, mload(add(signature, 0x60)))
        }

        if (v < 27) v += 27;
        if (v != 27 && v != 28) revert MalformedSignature();
    }

    /// @notice Recovers `digest` and refuses anything but `expectedSigner`.
    /// @dev    `ecrecover` returns the zero address on failure, which would otherwise match an
    ///         `expectedSigner` of zero.
    function requireSigner(bytes32 digest, bytes memory signature, address expectedSigner) internal pure {
        (uint8 v, bytes32 r, bytes32 s) = decodeSignature(signature);

        address recovered = ecrecover(digest, v, r, s);
        if (recovered == address(0) || recovered != expectedSigner) {
            revert CleartextErrorInvalidUserDecryptSignature();
        }
    }

    /// @notice Every pair's contract must be one the request covers.
    /// @dev    A handle is authorised for a CONTRACT, so a pair naming one outside the request's list
    ///         would be read under a permission the signature never covered.
    function requireAllPairsAuthorized(HandleContractPair[] memory pairs, address[] memory contractAddresses)
        internal
        pure
    {
        for (uint256 i = 0; i < pairs.length; ++i) {
            address c = pairs[i].contractAddress;

            bool authorized;
            for (uint256 j = 0; j < contractAddresses.length; ++j) {
                if (contractAddresses[j] == c) {
                    authorized = true;
                    break;
                }
            }
            if (!authorized) revert CleartextErrorContractAddressNotAuthorized(c);
        }
    }

    /// @notice The `extraData` the verifier expects today: format byte `0x01` then the KMS context id.
    /// @dev    Mirrors `CleartextKMSVerifier._buildCurrentExtradata`. The context id is read from the
    ///         verifier rather than assumed, because a new context is minted whenever the KMS signer
    ///         set changes and a proof carrying a stale one is checked against the wrong signers.
    function currentExtraData(address kmsVerifier) internal view returns (bytes memory) {
        return extraDataForContext(IKmsVerifierView(kmsVerifier).getCurrentKmsContextId());
    }

    /**
     * @notice The same encoding, for a caller that already holds the context id.
     *
     * @dev SPLIT SO THE VERIFIER DOES NOT CALL ITSELF. `currentExtraData` fetches the id over
     *      `STATICCALL`, which is right for a caller looking at some OTHER contract — and wrong for the
     *      verifier itself, which reaches `getCurrentKmsContextId()` with a jump. Routing that through
     *      an external call to `address(this)` would add a call frame, ABI encoding and return-data
     *      copying to what was a direct read, and change the gas a test measures.
     *
     *      Same division as `domainSeparator` / `domainSeparatorFrom`: the library fetches for a
     *      stranger, and computes for whoever already has the inputs.
     */
    function extraDataForContext(uint256 contextId) internal pure returns (bytes memory extraData) {
        extraData = new bytes(33);
        extraData[0] = 0x01;
        assembly {
            mstore(add(extraData, 33), contextId)
        }
    }

    /// @notice The verifier's EIP-712 domain separator, rebuilt from what it reports about itself.
    function domainSeparator(address kmsVerifier) internal view returns (bytes32) {
        (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract,,) =
            IKmsVerifierView(kmsVerifier).eip712Domain();

        if (fields != EXPECTED_DOMAIN_FIELDS) revert UnsupportedEip712Domain(fields);

        return domainSeparatorFrom(name, version, chainId, verifyingContract);
    }

    /**
     * @notice The separator a USER-DECRYPT REQUEST is signed under, rebuilt from a verifier's own
     *         `eip712Domain()`.
     *
     * @dev TWO CHAIN IDS, BOTH CORRECT. A verifier's `eip712Domain()` reports the GATEWAY chain id,
     *      because that is where its source contract lives, and that is the domain a public-decryption
     *      RESULT is signed under. A user-decryption REQUEST is signed under the same name, version and
     *      verifying contract but the HOST chain id — the chain the request is made on. That is what
     *      `CleartextKMSVerifier._domainHashWithHostChainId` does with its own domain, and this is the
     *      same thing done to somebody else's.
     */
    function hostDomainSeparator(address kmsVerifier) internal view returns (bytes32) {
        (bytes1 fields, string memory name, string memory version,, address verifyingContract,,) =
            IKmsVerifierView(kmsVerifier).eip712Domain();

        if (fields != EXPECTED_DOMAIN_FIELDS) revert UnsupportedEip712Domain(fields);

        return domainSeparatorFrom(name, version, block.chainid, verifyingContract);
    }

    /// @notice The same separator from fields the caller already holds.
    /// @dev    THE one implementation of the domain hash. `CleartextKMSVerifier` reads its own
    ///         `eip712Domain()` and comes here, rather than keeping a second copy of this keccak — so a
    ///         digest built on a fork and one built by the mock cannot disagree about the domain.
    function domainSeparatorFrom(string memory name, string memory version, uint256 chainId, address verifyingContract)
        internal
        pure
        returns (bytes32)
    {
        return keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH, keccak256(bytes(name)), keccak256(bytes(version)), chainId, verifyingContract
            )
        );
    }

    /// @notice EIP-712 typed-data hash: `keccak256("\x19\x01" ‖ domainSeparator ‖ structHash)`.
    function toTypedDataHash(bytes32 domainHash, bytes32 structHash) internal pure returns (bytes32 typedDataHash) {
        assembly ("memory-safe") {
            let ptr := mload(0x40)
            mstore(ptr, hex"1901")
            mstore(add(ptr, 0x02), domainHash)
            mstore(add(ptr, 0x22), structHash)
            typedDataHash := keccak256(ptr, 0x42)
        }
    }

    // --- Unified (RFC-016) user decryption: permit v2 ---

    /**
     * @notice The unified (v2, RFC-016) user-decryption payload: every check the gateway and kms-worker make (since v14), in
     *         their order, then read, mask, encode.
     *
     * @dev Since v14 the gateway (`Decryption.userDecryptionRequest(HandleEntry[], ...)`) only checks the request's
     *      shape and limits; authorisation moved to the kms-worker (`check_user_decryption_request_v2`). This
     *      mirrors both, so a client that signs a v2 permit is held to exactly what the connector enforces.
     *
     * @dev THE CALLER KEEPS WHAT IS ITS OWN, checked before this call: the KMS context (and epoch) the permit's
     *      extraData names (KMSVerifier), and that the user is not its own verifying contract (KMS core).
     */
    function userDecryptPayloadV2(HandleEntry[] memory handles, UserDecryptPayloadArgsV2 memory p)
        internal
        view
        returns (bytes memory payload)
    {
        UserDecryptPermitV2 memory permit = p.permit;

        // --- Gateway `userDecryptionRequest(HandleEntry[], ...)`, in its order ---
        if (handles.length == 0) revert CleartextErrorEmptyCtHandles();
        requirePermitV2AllowedContractsWithinLimit(permit);
        requireValidRequestWindowSeconds(permit.startTimestamp, permit.durationSeconds);
        requireConformantHandleEntries(handles);

        // --- kms-worker `check_user_decryption_request_v2` (the window is the gateway's, enforced above) ---
        requirePermitV2UserNotInAllowedContracts(permit);
        requireUserSignatureV2(p.domainHash, permit, p.signature);
        requireNotInvalidated(p.aclAddress, permit.userAddress, permit.startTimestamp);
        requireHandleEntriesAuthorized(p.aclAddress, handles, permit.userAddress, permit.allowedContracts);

        // --- Every check passed: read, mask, encode, as the v1 paths do ---
        uint256 n = handles.length;
        uint256[] memory cleartexts = new uint256[](n);
        for (uint256 i = 0; i < n; ++i) {
            cleartexts[i] = IPlaintexts(p.plaintextsAddress).plaintexts(handles[i].handle);
        }
        return abi.encode(maskWithPublicKey(permit.publicKey, cleartexts), permit.extraData);
    }

    /// @dev GW: at most `MAX_USER_DECRYPT_CONTRACT_ADDRESSES` allowed contracts. Empty is permissive mode, so only
    ///      the upper bound is enforced.
    function requirePermitV2AllowedContractsWithinLimit(UserDecryptPermitV2 memory permit) internal pure {
        if (permit.allowedContracts.length > MAX_USER_DECRYPT_CONTRACT_ADDRESSES) {
            revert CleartextErrorContractAddressesMaxLengthExceeded(
                MAX_USER_DECRYPT_CONTRACT_ADDRESSES, permit.allowedContracts.length
            );
        }
    }

    /// @dev KW `check_user_decryption_request_v2`: the user is not one of its own allowed contracts.
    function requirePermitV2UserNotInAllowedContracts(UserDecryptPermitV2 memory permit) internal pure {
        for (uint256 i = 0; i < permit.allowedContracts.length; ++i) {
            if (permit.allowedContracts[i] == permit.userAddress) {
                revert CleartextErrorUserAddressInAllowedContracts(permit.userAddress);
            }
        }
    }

    /// @dev GW `_checkUserDecryptionRequestValiditySeconds`, in the same order.
    function requireValidRequestWindowSeconds(uint256 startTimestamp, uint256 durationSeconds) internal view {
        if (durationSeconds == 0) revert CleartextErrorInvalidNullDurationSeconds();
        if (durationSeconds > MAX_USER_DECRYPT_DURATION_SECONDS) {
            revert CleartextErrorMaxDurationSecondsExceeded(MAX_USER_DECRYPT_DURATION_SECONDS, durationSeconds);
        }
        if (startTimestamp > block.timestamp) {
            revert CleartextErrorStartTimestampInFuture(block.timestamp, startTimestamp);
        }
        if (startTimestamp + durationSeconds < block.timestamp) {
            revert CleartextErrorUserDecryptionRequestExpiredSeconds(block.timestamp, startTimestamp, durationSeconds);
        }
    }

    /// @dev GW `_extractCtHandlesCheckConformanceHandleEntry`: one chain (this
    ///      one), decryptable types, at most `MAX_DECRYPTION_REQUEST_BITS`.
    function requireConformantHandleEntries(HandleEntry[] memory handles) internal view {
        uint256 totalBitSize;
        for (uint256 i = 0; i < handles.length; ++i) {
            totalBitSize += requireConformantHandle(handles[i].handle);
        }
        requireTotalBitSize(totalBitSize);
    }

    /// @dev KW `verify_signature` (RFC-012): a non-empty signature that `ecrecover`s to the user is accepted
    ///      with no call; anything else (empty, wrong length, unparsable, other signer) falls through to
    ///      `IERC1271(user).isValidSignature(digest, signature)`, gas-capped, which must return at least 32
    ///      bytes starting with the magic value. An EOA answers with empty returndata, and is refused.
    ///      `domainHash`: the host chain id with the gateway `Decryption` contract as verifying contract.
    function requireUserSignatureV2(bytes32 domainHash, UserDecryptPermitV2 memory permit, bytes memory signature)
        internal
        view
    {
        bytes32 digest = toTypedDataHash(
            domainHash,
            keccak256(
                abi.encode(
                    USER_DECRYPT_REQUEST_V2_TYPEHASH,
                    permit.userAddress,
                    keccak256(permit.publicKey),
                    keccak256(abi.encodePacked(permit.allowedContracts)),
                    permit.startTimestamp,
                    permit.durationSeconds,
                    keccak256(permit.extraData)
                )
            )
        );

        // `tryRecover` answers zero for an empty or bad signature: zero must never match, even a zero user.
        address recovered = tryRecover(digest, signature);
        if (recovered != address(0) && recovered == permit.userAddress) return;

        (bool ok, bytes memory ret) = permit.userAddress.staticcall{gas: ERC1271_GAS_LIMIT}(
            abi.encodeCall(IERC1271.isValidSignature, (digest, signature))
        );
        if (!ok || ret.length < 32 || bytes4(ret) != ERC1271_MAGIC_VALUE) {
            revert CleartextErrorInvalidUserDecryptSignatureV2(permit.userAddress);
        }
    }

    /// @dev `ecrecover` that never reverts: zero for anything but a well-formed 65-byte `r ‖ s ‖ v`
    ///      (`v` of 0/1 or 27/28), so a bad signature reaches the ERC-1271 fallback as in the connector.
    function tryRecover(bytes32 digest, bytes memory signature) internal pure returns (address) {
        if (signature.length != 65) return address(0);
        bytes32 r;
        bytes32 s;
        uint8 v;
        assembly {
            r := mload(add(signature, 0x20))
            s := mload(add(signature, 0x40))
            v := byte(0, mload(add(signature, 0x60)))
        }
        if (v < 27) v += 27;
        if (v != 27 && v != 28) return address(0);
        return ecrecover(digest, v, r, s);
    }

    /// @dev KW: `startTimestamp >= ACL.decryptionSignatureInvalidatedBefore(user)`.
    function requireNotInvalidated(address aclAddress, address userAddress, uint256 startTimestamp) internal view {
        uint256 invalidatedBefore =
            IUserDecryptionPermissionsV2(aclAddress).decryptionSignatureInvalidatedBefore(userAddress);
        if (startTimestamp < invalidatedBefore) {
            revert CleartextErrorUserDecryptSignatureInvalidated(userAddress, startTimestamp, invalidatedBefore);
        }
    }

    /// @dev KW per-handle ownership and `allowedContracts` checks. NOTE, as in the connector: a direct
    ///      handle checks `isAllowed(handle, user)` only, NOT `isAllowed(handle, entry.contractAddress)`;
    ///      the contract side is the `allowedContracts` check, a no-op when the list is empty.
    function requireHandleEntriesAuthorized(
        address aclAddress,
        HandleEntry[] memory handles,
        address userAddress,
        address[] memory allowedContracts
    ) internal view {
        IUserDecryptionPermissionsV2 acl = IUserDecryptionPermissionsV2(aclAddress);
        for (uint256 i = 0; i < handles.length; ++i) {
            HandleEntry memory e = handles[i];
            if (e.ownerAddress == userAddress) {
                if (!acl.isAllowed(e.handle, userAddress)) revert CleartextErrorUserNotAllowed(e.handle, userAddress);
            } else if (!acl.isHandleDelegatedForUserDecryption(
                    e.ownerAddress, userAddress, e.contractAddress, e.handle
                )) {
                revert CleartextErrorUserNotDelegated(e.handle, e.ownerAddress, userAddress, e.contractAddress);
            }

            if (allowedContracts.length != 0) {
                bool anyAllowed;
                for (uint256 j = 0; j < allowedContracts.length; ++j) {
                    if (acl.isAllowed(e.handle, allowedContracts[j])) {
                        anyAllowed = true;
                        break;
                    }
                }
                if (!anyAllowed) revert CleartextErrorNoAllowedContractForHandle(e.handle);
            }
        }
    }

    // ---------------------------------------------------------------------------------------------
    // Private functions
    // ---------------------------------------------------------------------------------------------

    /// @dev FRAMES. Unpacking the request into the verifier's seven and eight arguments alongside the
    ///      caller's own parameters and four returns overflows solc's legacy codegen.
    function _cleartextUserDecrypt(
        address kmsVerifier,
        HandleContractPair[] memory pairs,
        UserDecryptRequestV1 memory request,
        SignerSignaturePair memory user
    ) private view returns (bytes memory, address[] memory, uint256, bytes memory) {
        return ICleartextKmsVerifierCall(kmsVerifier)
            .userDecrypt(
                pairs,
                user.signer,
                request.transportPublicKey,
                request.contractAddresses,
                request.startTimestamp,
                request.durationDays,
                user.signature
            );
    }

    function _cleartextDelegatedUserDecrypt(
        address kmsVerifier,
        HandleContractPair[] memory pairs,
        UserDecryptRequestV1 memory request,
        address delegator,
        SignerSignaturePair memory delegate
    ) private view returns (bytes memory, address[] memory, uint256, bytes memory) {
        return ICleartextKmsVerifierCall(kmsVerifier)
            .delegatedUserDecrypt(
                pairs,
                delegator,
                delegate.signer,
                request.transportPublicKey,
                request.contractAddresses,
                request.startTimestamp,
                request.durationDays,
                delegate.signature
            );
    }
}
