// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KMSVerifier} from "../host-contracts/contracts/KMSVerifier.sol";
import {IProtocolConfig} from "../host-contracts/contracts/interfaces/IProtocolConfig.sol";
import {EXTRA_DATA_V2} from "../host-contracts/contracts/shared/Constants.sol";
import {aclAdd, fhevmExecutorAdd, protocolConfigAdd} from "../host-contracts/addresses/FHEVMHostAddresses.sol";
import {
    DelegatedUserDecryptPayloadArgsV1,
    HandleContractPair,
    HandleEntry,
    LibKmsVerifier,
    PublicDecryptPayloadArgs,
    SignerSignaturePair,
    UserDecryptPayloadArgsV1,
    UserDecryptPayloadArgsV2,
    UserDecryptPermitV2,
    UserDecryptRequestV1
} from "./shared/LibKmsVerifier.sol";

/**
 * @notice A legacy (v1) user-decryption permit: every field the user signs (`UserDecryptRequestVerification`), plus
 *         the signer, minus the signature (passed beside it). The v1 type does not sign `userAddress`: the signer
 *         is recovered and must equal it.
 * @dev    Fields: the signer, then the type's fields in the type's order. A struct so the entry point stays
 *         under the stack limit, as `UserDecryptPermitV2`.
 */
struct UserDecryptPermitV1 {
    address userAddress;
    bytes publicKey;
    address[] contractAddresses;
    uint256 startTimestamp;
    uint256 durationDays;
    bytes extraData;
}

/**
 * @notice A legacy (v1) delegated user-decryption permit: every field the delegate signs
 *         (`DelegatedUserDecryptRequestVerification`, which names the delegator), plus the delegate, the signer,
 *         minus the signature (passed beside it).
 * @dev    Fields: the signer (the delegate), then the type's fields in the type's order.
 */
struct DelegatedUserDecryptPermitV1 {
    address delegateAddress;
    bytes publicKey;
    address[] contractAddresses;
    address delegatorAddress;
    uint256 startTimestamp;
    uint256 durationDays;
    bytes extraData;
}

/**
 * @title CleartextKMSVerifier
 */
contract CleartextKMSVerifier is KMSVerifier {
    /// @notice Marks a cleartext (mock) implementation. Real host contracts have no such selector, so a
    ///         consumer can probe it to tell a cleartext stack from a production deployment.
    bool public constant IS_CLEARTEXT = true;

    // ---------------------------------------------------------------------------------------------
    // Private constants
    // ---------------------------------------------------------------------------------------------

    /// @notice EIP-712 domain typehash.
    bytes32 private constant EIP712_DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");

    // ---------------------------------------------------------------------------------------------
    // Errors
    // ---------------------------------------------------------------------------------------------

    /// @notice KMS: the client address must differ from the EIP-712 domain's verifying contract.
    error CleartextErrorUserAddressIsVerifyingContract(address userAddress);

    /// @notice KW `KmsContextInvalid`: a v2 extraData names an epoch that is not active for its context.
    error CleartextErrorInvalidKmsEpoch(uint256 kmsContextId, uint256 epochId);

    /// @notice The extraData a public-decryption request names refers to no valid KMS context.
    error CleartextErrorInvalidKmsContext(uint256 kmsContextId);

    // ---------------------------------------------------------------------------------------------
    // Legacy decrypt functions
    // ---------------------------------------------------------------------------------------------

    /// @notice Returns cleartexts and KMS metadata for public decryption, signed over the CURRENT KMS
    ///         context (extraData v1, `0x01 | contextId`).
    /// @dev    For sdks whose cleartext path does not pass the extraData they will verify the proof against
    ///         (FHEVM SDK 0.13.x calls `publicDecrypt(handles)`). The overload below, which takes the
    ///         extraData, serves any requested version.
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
        // v1, not the v2 (`0x02 | contextId | epochId`) a chain with epochs calls for: the sdk releases that call
        // this overload build v1 and verify the proof against it. An sdk that builds v2 passes it, through the
        // overload below.
        return _publicDecrypt(handles, LibKmsVerifier.extraDataForContext(getCurrentKmsContextId()));
    }

    /// @notice Returns user-decryption payload and KMS metadata after verifying the user's signature.
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
        UserDecryptPayloadArgsV1 memory p = _userDecryptArgsV1(
            userAddress, UserDecryptRequestV1(publicKey, contractAddresses, startTimestamp, durationDays), userSignature
        );
        p.extraData = _userDecryptExtraData(p);
        return _userDecryptV1(pairs, p);
    }

    /// @notice Returns delegated user-decryption payload and KMS metadata after verifying the delegate's signature.
    /// @dev The parameters are assembled into one struct rather than passed loose; see
    ///      `DelegatedUserDecryptPayloadArgsV1` for why the stack requires it.
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
        DelegatedUserDecryptPayloadArgsV1 memory p = _delegatedUserDecryptArgsV1(
            delegator,
            delegate,
            UserDecryptRequestV1(publicKey, contractAddresses, startTimestamp, durationDays),
            delegateSignature
        );
        p.extraData = _delegatedUserDecryptExtraData(p);
        return _delegatedUserDecryptV1(pairs, p);
    }

    // ---------------------------------------------------------------------------------------------
    // New decrypt functions
    // ---------------------------------------------------------------------------------------------

    /// @notice Same, signed over exactly the `extraData` the requester names, as a real relayer has the KMS
    ///         sign over the extraData the request carries.
    /// @dev    Since v14. The sdk decides which KMS committee must sign and verifies the proof against its own
    ///         extraData, ignoring the one returned here. So the digest must be built over the REQUESTED
    ///         extraData: v0 (empty or `0x00`: the current context), v1 (`0x01 | contextId`) or v2
    ///         (`0x02 | contextId | epochId`). Parsed by KMSVerifier's own `_extractKmsContextId`, which rejects
    ///         a malformed or unsupported one; the context it names must be valid. The epoch id of a v2 is not
    ///         checked, as KMSVerifier does not check it when it verifies the proof either.
    function publicDecrypt(bytes32[] memory handles, bytes memory extraData)
        external
        view
        virtual
        returns (
            bytes memory abiEncodedCleartexts,
            bytes32 digest,
            address[] memory signers,
            uint256 threshold,
            bytes memory signedExtraData
        )
    {
        return _publicDecrypt(handles, extraData);
    }

    /// @notice Same as the legacy `userDecrypt`, verified against exactly the `extraData` the user signed their
    ///         permit over.
    /// @dev    Since v14. A permit is reusable until it expires, and its extraData names the KMS context (and, in
    ///         v2, epoch) current WHEN IT WAS SIGNED, which may no longer be the current one; the sdk resolves
    ///         the signers of that context, not of the current one. So the host must not guess it: the caller
    ///         passes the permit's `message.extraData`, verbatim. Parsed by KMSVerifier's own
    ///         `_extractKmsContextId`; the context it names must be valid. Returns that context's signers and
    ///         threshold.
    /// @param pairs     The handles, each with the contract it is held under.
    /// @param permit    The permit's signed fields and its signer, `extraData` verbatim.
    /// @param signature The user's 65-byte EOA signature over the permit.
    function userDecryptV1(
        HandleContractPair[] calldata pairs,
        UserDecryptPermitV1 memory permit,
        bytes memory signature
    )
        external
        view
        virtual
        returns (bytes memory payload, address[] memory signers, uint256 threshold, bytes memory signedExtraData)
    {
        UserDecryptPayloadArgsV1 memory p = _userDecryptArgsV1(
            permit.userAddress,
            UserDecryptRequestV1(
                permit.publicKey, permit.contractAddresses, permit.startTimestamp, permit.durationDays
            ),
            signature
        );
        p.extraData = permit.extraData;
        return _userDecryptV1(pairs, p);
    }

    /// @notice Same as the legacy `delegatedUserDecrypt`, verified against exactly the `extraData` the delegate
    ///         signed their permit over.
    /// @dev    Since v14; see `userDecryptV1`.
    /// @param permit    The permit's signed fields (the delegator included) and its signer, the delegate.
    /// @param signature The delegate's 65-byte EOA signature over the permit.
    function delegatedUserDecryptV1(
        HandleContractPair[] calldata pairs,
        DelegatedUserDecryptPermitV1 memory permit,
        bytes memory signature
    )
        external
        view
        virtual
        returns (bytes memory payload, address[] memory signers, uint256 threshold, bytes memory signedExtraData)
    {
        DelegatedUserDecryptPayloadArgsV1 memory p = _delegatedUserDecryptArgsV1(
            permit.delegatorAddress,
            permit.delegateAddress,
            UserDecryptRequestV1(
                permit.publicKey, permit.contractAddresses, permit.startTimestamp, permit.durationDays
            ),
            signature
        );
        p.extraData = permit.extraData;
        return _delegatedUserDecryptV1(pairs, p);
    }

    // ---------------------------------------------------------------------------------------------
    // Unified (RFC-016) user decryption: permit v2, `durationSeconds` + `allowedContracts`.
    //
    // Since v14 the gateway (`Decryption.userDecryptionRequest(HandleEntry[], ...)`) only checks the
    // request's shape and limits; authorisation moved to the kms-worker
    // (`check_user_decryption_request_v2`). `LibKmsVerifier.userDecryptPayloadV2` mirrors both, in their
    // order; the verifier checks what is its own first (the KMS context, its verifying contract).
    // ---------------------------------------------------------------------------------------------

    /**
     * @notice Unified user decryption: returns the user-decryption payload and KMS metadata once the request
     *         passes every check the gateway and kms-worker make (since v14).
     * @param handles   The handles, each with the contract it is held under and its owner.
     * @param permit    The signed fields of the permit, verbatim (`extraData` included).
     * @param signature A 65-byte EOA signature, an ERC-1271 blob, or empty for a smart account.
     */
    function userDecryptV2(HandleEntry[] calldata handles, UserDecryptPermitV2 memory permit, bytes memory signature)
        external
        view
        virtual
        returns (bytes memory payload, address[] memory signers, uint256 threshold, bytes memory signedExtraData)
    {
        // Its own first: the KMS context (and epoch) the permit names, and that the user is not this verifier.
        uint256 kmsContextId = _validKmsContextId(permit.extraData);
        _requireNotVerifyingContract(permit.userAddress);

        UserDecryptPayloadArgsV2 memory p;
        p.aclAddress = aclAdd;
        p.plaintextsAddress = fhevmExecutorAdd;
        p.domainHash = _domainHashWithHostChainId();
        p.permit = permit;
        p.signature = signature;

        // The gateway's then the kms-worker's checks, in their order, then reads.
        signedExtraData = permit.extraData;
        payload = LibKmsVerifier.userDecryptPayloadV2(handles, p);
        (signers, threshold) = _signersAndThreshold(kmsContextId, true);
    }

    // ---------------------------------------------------------------------------------------------
    // Private helpers
    // ---------------------------------------------------------------------------------------------

    /// @dev The public-decryption answer for `extraData`, with the signers and threshold of the
    ///      KMS context it names.
    function _publicDecrypt(bytes32[] memory handles, bytes memory extraData)
        private
        view
        returns (
            bytes memory abiEncodedCleartexts,
            bytes32 digest,
            address[] memory signers,
            uint256 threshold,
            bytes memory signedExtraData
        )
    {
        uint256 kmsContextId = _validKmsContextId(extraData);

        PublicDecryptPayloadArgs memory p;
        p.aclAddress = aclAdd;
        p.plaintextsAddress = fhevmExecutorAdd;
        p.domainHash = _domainHashWithGatewayChainId();
        p.typeHash = DECRYPTION_RESULT_TYPEHASH;
        p.extraData = _signedExtraData(extraData);

        signedExtraData = p.extraData;
        // The handles' conformance, the ACL walk, then reads.
        (abiEncodedCleartexts, digest) = LibKmsVerifier.publicDecryptPayload(handles, p);

        (signers, threshold) = _signersAndThreshold(kmsContextId, false);
    }

    /// @dev The user-decryption answer for a permit whose signed extraData is `p.extraData`.
    function _userDecryptV1(HandleContractPair[] calldata pairs, UserDecryptPayloadArgsV1 memory p)
        private
        view
        returns (bytes memory payload, address[] memory signers, uint256 threshold, bytes memory signedExtraData)
    {
        uint256 kmsContextId = _validKmsContextId(p.extraData);
        _requireNotVerifyingContract(p.user.signer);
        signedExtraData = p.extraData;
        // The request's contract addresses and window, the pairs' conformance, the user's signature over `p.extraData` (`requireSigner`), the pairs'
        // authorisation, then reads.
        payload = LibKmsVerifier.userDecryptPayloadV1(pairs, p);
        (signers, threshold) = _signersAndThreshold(kmsContextId, true);
    }

    /// @dev The same, for a delegate's permit.
    function _delegatedUserDecryptV1(HandleContractPair[] calldata pairs, DelegatedUserDecryptPayloadArgsV1 memory p)
        private
        view
        returns (bytes memory payload, address[] memory signers, uint256 threshold, bytes memory signedExtraData)
    {
        uint256 kmsContextId = _validKmsContextId(p.extraData);
        _requireNotVerifyingContract(p.delegate.signer);
        signedExtraData = p.extraData;
        // The request's contract addresses and window, the pairs' conformance, the delegate's signature over `p.extraData` (`requireSigner`), the pairs'
        // authorisation, then reads.
        payload = LibKmsVerifier.delegatedUserDecryptPayloadV1(pairs, p);
        (signers, threshold) = _signersAndThreshold(kmsContextId, true);
    }

    // ---------------------------------------------------------------------------------------------

    /// @dev The request and signer of a user decryption, everything but `extraData`.
    function _userDecryptArgsV1(address userAddress, UserDecryptRequestV1 memory request, bytes memory userSignature)
        private
        view
        returns (UserDecryptPayloadArgsV1 memory p)
    {
        p.aclAddress = aclAdd;
        p.plaintextsAddress = fhevmExecutorAdd;
        p.request = request;
        p.user = SignerSignaturePair(userAddress, userSignature);
        p.domainHash = _domainHashWithHostChainId();
    }

    /// @dev The same, for a delegated user decryption.
    function _delegatedUserDecryptArgsV1(
        address delegator,
        address delegate,
        UserDecryptRequestV1 memory request,
        bytes memory delegateSignature
    ) private view returns (DelegatedUserDecryptPayloadArgsV1 memory p) {
        p.aclAddress = aclAdd;
        p.plaintextsAddress = fhevmExecutorAdd;
        p.request = request;
        p.delegator = delegator;
        p.delegate = SignerSignaturePair(delegate, delegateSignature);
        p.domainHash = _domainHashWithHostChainId();
    }

    // ---------------------------------------------------------------------------------------------
    // Private Helpers
    // ---------------------------------------------------------------------------------------------

    /// @dev The KMS context `extraData` names, parsed by KMSVerifier's own `_extractKmsContextId` (which
    ///      rejects a malformed or unsupported extraData); it must be a valid context. For a v2 extraData
    ///      the epoch must also be active for that context, as the kms-worker checks
    ///      (`isValidEpochForContext`) although KMSVerifier does not when it verifies the proof.
    function _validKmsContextId(bytes memory extraData) private view returns (uint256 kmsContextId) {
        kmsContextId = _extractKmsContextId(extraData);
        IProtocolConfig protocolConfig = IProtocolConfig(protocolConfigAdd);
        if (!protocolConfig.isValidKmsContext(kmsContextId)) {
            revert CleartextErrorInvalidKmsContext(kmsContextId);
        }
        // `_extractKmsContextId` has already enforced the exact 65-byte length of a v2.
        if (extraData.length != 0 && uint8(extraData[0]) == EXTRA_DATA_V2) {
            uint256 epochId;
            assembly {
                epochId := mload(add(extraData, 65))
            }
            if (!protocolConfig.isValidEpochForContext(kmsContextId, epochId)) {
                revert CleartextErrorInvalidKmsEpoch(kmsContextId, epochId);
            }
        }
    }

    /// @dev A KMS context's signers and threshold: the public-decryption one, or the user-decryption one
    ///      the gateway applies to user-decryption responses (`getUserDecryptionThresholdForContext`).
    function _signersAndThreshold(uint256 kmsContextId, bool userDecryption)
        private
        view
        returns (address[] memory signers, uint256 threshold)
    {
        IProtocolConfig protocolConfig = IProtocolConfig(protocolConfigAdd);
        signers = protocolConfig.getKmsSignersForContext(kmsContextId);
        threshold = userDecryption
            ? protocolConfig.getUserDecryptionThresholdForContext(kmsContextId)
            : protocolConfig.getPublicDecryptionThresholdForContext(kmsContextId);
    }

    /// @dev KMS `compute_link_checked`: the client address is not the domain's verifying contract.
    function _requireNotVerifyingContract(address userAddress) private view {
        (,,,, address verifyingContract,,) = eip712Domain();
        if (userAddress == verifyingContract) revert CleartextErrorUserAddressIsVerifyingContract(userAddress);
    }

    /// @dev For the overloads WITHOUT `extraData` (published sdk releases do not pass it): the extraData a user
    ///      signed their permit over, guessed as v2 (current context and epoch, what an sdk on protocol API 0.14
    ///      signs) if the signature matches it, otherwise v1 (current context, what an sdk capped at protocol API
    ///      0.13 signs). If neither matches, `LibKmsVerifier.userDecryptPayloadV1` refuses the signature as
    ///      before. LIMITATION: only a permit signed while the current context and epoch were already current
    ///      can match; one signed before a KMS context or epoch change is refused although it is valid. The
    ///      overload that takes `extraData` has no such limitation.
    function _userDecryptExtraData(UserDecryptPayloadArgsV1 memory p) private view returns (bytes memory) {
        bytes memory extraDataV2 = _currentExtraDataV2();
        bytes32 digestV2 = LibKmsVerifier.userDecryptionDigestV1(p.domainHash, p.request, extraDataV2);
        if (_isSignedBy(digestV2, p.user.signature, p.user.signer)) return extraDataV2;
        return LibKmsVerifier.extraDataForContext(getCurrentKmsContextId());
    }

    /// @dev The same, for a delegate's permit.
    function _delegatedUserDecryptExtraData(DelegatedUserDecryptPayloadArgsV1 memory p)
        private
        view
        returns (bytes memory)
    {
        bytes memory extraDataV2 = _currentExtraDataV2();
        bytes32 digestV2 =
            LibKmsVerifier.delegatedUserDecryptionDigestV1(p.domainHash, p.request, p.delegator, extraDataV2);
        if (_isSignedBy(digestV2, p.delegate.signature, p.delegate.signer)) return extraDataV2;
        return LibKmsVerifier.extraDataForContext(getCurrentKmsContextId());
    }

    /// @dev extraData v2 for the current KMS context and epoch: `0x02 | contextId | epochId`, 65 bytes.
    function _currentExtraDataV2() private view returns (bytes memory extraData) {
        (uint256 contextId, uint256 epochId) = IProtocolConfig(protocolConfigAdd).getCurrentKmsContextAndEpoch();
        extraData = new bytes(65);
        extraData[0] = bytes1(EXTRA_DATA_V2);
        assembly {
            mstore(add(extraData, 33), contextId)
            mstore(add(extraData, 65), epochId)
        }
    }

    /// @dev The bytes a v0 extraData is signed as: empty, never `0x00`, as the sdk signs it. v1 and v2 are
    ///      signed as they are.
    function _signedExtraData(bytes memory extraData) private pure returns (bytes memory) {
        if (extraData.length == 0 || uint8(extraData[0]) == 0x00) return new bytes(0);
        return extraData;
    }

    /// @dev Whether `signature` over `digest` recovers to `signer`. Never reverts: a malformed signature is
    ///      simply not a match (`LibKmsVerifier.tryRecover`), so the extraData guess cannot report it ahead of the request
    ///      checks that come first. `LibKmsVerifier.requireSigner` refuses it later, with `MalformedSignature`.
    function _isSignedBy(bytes32 digest, bytes memory signature, address signer) private pure returns (bool) {
        address recovered = LibKmsVerifier.tryRecover(digest, signature);
        return recovered != address(0) && recovered == signer;
    }

    /// @notice EIP-712 domain separator for this KMS verifier, read from `eip712Domain()`.
    function _domainHashWithGatewayChainId() private view returns (bytes32) {
        return _domainHash(0);
    }

    /// @notice EIP-712 domain separator using the current host chain id.
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

        return LibKmsVerifier.domainSeparatorFrom(name, version, domainChainId, verifyingContract);
    }
}
