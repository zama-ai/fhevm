// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "./shared/FheType.sol";
import {HANDLE_VERSION} from "./shared/Constants.sol";

/**
 * @title PaddedInputter
 * @notice RFC-030 (pad-based input encryption) host-chain verification of `fromExternal` inputs whose
 *         package type byte is `0x00`.
 * @dev Abstract: the FHEVMExecutor inherits it and routes packages starting with `0x00` to
 *      `_verifyPaddedInput`, passing the claimed submitter (the app's caller) and the actual consumer
 *      (the executor's `msg.sender`). For each external handle it:
 *        1. checks the input package (cached per package in transient storage, keyed by `packageDigest`);
 *        2. checks the external handle against `parts`;
 *        3. checks `claimedSubmitter == submitter` and `actualConsumer == consumer`;
 *        4. derives a fresh `computeHandle` from host chain state;
 *        5. emits `Unmask(externalHandle, inputPackage, computeHandle)` for the coprocessor;
 *        6. returns `computeHandle`; the executor marks it minted and grants the transient allowance.
 *      Holds no per-pad state. The supported PRF instances are governance-maintained: the inheriting
 *      contract implements `_isSupportedPadPrf`.
 *
 *      Input package layout (big-endian integers):
 *        packageType 1 | witnessVersion 1 | prf 2 | chainId 8 | submitter 20 | consumer 20 |
 *        expirationTimestamp 8 | decryptorCount 1 | decryptors 20×decryptorCount | nonce 16 |
 *        label 32 | partCount 1 | parts 1×partCount | ciphertext N/8
 *      The witness fields are encoded exactly as in the label preimage, so
 *      `label == keccak256("FHE_padl" ‖ inputPackage[1:witnessEnd])`.
 */
abstract contract PaddedInputter {
    // ---- RFC-030 constants ----

    /// @notice `packageType` byte routing `fromExternal` to this flow.
    uint8 internal constant PAD_PACKAGE_TYPE = 0x00;

    /// @notice The only witness format currently defined.
    uint8 internal constant PAD_WITNESS_VERSION = 0x00;

    /// @notice Upper bound on a package's bit length `N`.
    uint256 internal constant PAD_MAX_PAD_LENGTH = 2048;

    /// @notice Upper bound on the number of entries in the witness's `decryptors`.
    uint256 internal constant PAD_MAX_DECRYPTORS = 16;

    /// @notice Domain tag of the label preimage.
    bytes8 private constant PAD_LABEL_DOMAIN_SEPARATOR = "FHE_padl";

    /// @notice Domain tag of the package digest.
    bytes8 private constant PAD_PACKAGE_DOMAIN_SEPARATOR = "FHE_padp";

    /// @notice Domain tag of the compute handle preimage.
    bytes8 private constant PAD_HANDLE_DOMAIN_SEPARATOR = "FHE_padh";

    // Byte offsets of the fixed-size package header.
    uint256 private constant OFFSET_WITNESS_VERSION = 1;
    uint256 private constant OFFSET_PRF = 2;
    uint256 private constant OFFSET_CHAIN_ID = 4;
    uint256 private constant OFFSET_SUBMITTER = 12;
    uint256 private constant OFFSET_CONSUMER = 32;
    uint256 private constant OFFSET_DECRYPTOR_COUNT = 60;
    uint256 private constant OFFSET_DECRYPTORS = 61;
    /// @dev nonce 16 | label 32 | partCount 1, after the decryptors.
    uint256 private constant TAIL_MIN_LENGTH = 16 + 32 + 1;

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.transient.PaddedInputter")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PADDED_INPUTTER_TRANSIENT_LOCATION =
        0x42ad821e9dedd89becca1cd89b4b7d4157f995675baa2ebc184bfc2edbfecf00;

    // ---- Types ----

    /// @notice A parsed input package; offsets index into the package bytes.
    struct PaddedPackage {
        uint8 witnessVersion;
        uint16 prf;
        uint64 chainId;
        address submitter;
        address consumer;
        uint8 decryptorCount;
        bytes32 label;
        uint8 partCount;
        /// @dev Byte offset of `parts[0]`.
        uint256 partsOffset;
        /// @dev Byte offset of `ciphertext[0]`.
        uint256 ciphertextOffset;
        /// @dev End of the witness fields (exclusive), i.e. the label's offset.
        uint256 witnessEnd;
    }

    // ---- Events ----

    /// @notice Asks the coprocessor to unmask the part `externalHandle` refers to, under `computeHandle`.
    event Unmask(bytes32 externalHandle, bytes inputPackage, bytes32 computeHandle);

    // ---- Errors ----

    error PadMalformedPackage();
    error PadUnsupportedWitnessVersion(uint8 witnessVersion);
    error PadUnsupportedPrf(uint16 prf);
    error PadInvalidChainId(uint64 chainId);
    error PadTooManyDecryptors(uint256 count);
    error PadNullWitnessAddress();
    error PadUnknownPartType(uint256 partIndex, uint8 partType);
    error PadPackageTooLarge(uint256 bits);
    error PadCiphertextLengthMismatch(uint256 ciphertextBytes, uint256 expectedBytes);
    error PadLabelMismatch(bytes32 label, bytes32 expectedLabel);
    error PadInputIndexOutOfBounds(uint8 inputIndex, uint8 partCount);
    error PadInputTypeMismatch(uint8 inputIndex, uint8 handleType, uint8 partType);
    error PadSubmitterMismatch(address claimedSubmitter, address submitter);
    error PadConsumerMismatch(address actualConsumer, address consumer);

    // ---- Hooks ----

    /// @notice Whether `prf` names a PRF instance the coprocessor can still evaluate.
    function _isSupportedPadPrf(uint16 prf) internal view virtual returns (bool);

    // ---- Verification ----

    /// @notice Whether `inputPackage` is routed to this flow (`packageType == 0x00`).
    function _isPaddedPackage(bytes memory inputPackage) internal pure returns (bool) {
        return inputPackage.length != 0 && uint8(inputPackage[0]) == PAD_PACKAGE_TYPE;
    }

    /**
     * @notice RFC-030 steps 1 to 5 for one external handle of `inputPackage`.
     * @param externalHandle The external handle: packageDigest[0..28] | inputIndex | type | version.
     * @param claimedSubmitter The app's `msg.sender`, as passed by the app through `fromExternal`.
     * @param actualConsumer The calling app, i.e. the executor's `msg.sender`.
     * @param inputPackage The whole input package.
     * @return computeHandle The fresh handle the app computes on.
     * @return pkg The parsed package.
     * @return inputIndex The index of the part `externalHandle` refers to.
     */
    function _verifyPaddedInput(
        bytes32 externalHandle,
        address claimedSubmitter,
        address actualConsumer,
        bytes memory inputPackage
    ) internal returns (bytes32 computeHandle, PaddedPackage memory pkg, uint8 inputIndex) {
        bytes32 packageDigest = keccak256(abi.encodePacked(PAD_PACKAGE_DOMAIN_SEPARATOR, inputPackage));

        // 1. Package-level checks, skipped for a package already checked in this transaction.
        pkg = _parsePaddedPackage(inputPackage);
        if (!_isPaddedPackageChecked(packageDigest)) {
            _checkPaddedPackage(inputPackage, pkg);
            _markPaddedPackageChecked(packageDigest);
        }

        // 2. External handle checks.
        inputIndex = uint8(externalHandle[29]);
        if (inputIndex >= pkg.partCount) revert PadInputIndexOutOfBounds(inputIndex, pkg.partCount);
        uint8 partType = uint8(inputPackage[pkg.partsOffset + inputIndex]);
        if (uint8(externalHandle[30]) != partType) {
            revert PadInputTypeMismatch(inputIndex, uint8(externalHandle[30]), partType);
        }

        // 3. Submitter and consumer bindings.
        if (claimedSubmitter != pkg.submitter) revert PadSubmitterMismatch(claimedSubmitter, pkg.submitter);
        if (actualConsumer != pkg.consumer) revert PadConsumerMismatch(actualConsumer, pkg.consumer);

        // 4. Fresh compute handle.
        computeHandle = _computePaddedHandle(packageDigest, inputIndex, FheType(partType));

        // 5. Unmask request for the coprocessor.
        emit Unmask(externalHandle, inputPackage, computeHandle);
    }

    /// @notice Byte offset and byte length of part `inputIndex` in the packed stream (and the ciphertext).
    function _paddedPartRange(
        bytes memory inputPackage,
        PaddedPackage memory pkg,
        uint8 inputIndex
    ) internal pure returns (uint256 byteOffset, uint256 byteLength) {
        for (uint256 i = 0; i < inputIndex; ++i) {
            byteOffset += _paddedPartBytes(uint8(inputPackage[pkg.partsOffset + i]));
        }
        byteLength = _paddedPartBytes(uint8(inputPackage[pkg.partsOffset + inputIndex]));
    }

    // ---- Internals ----

    /// @dev Parses the package header and counters, reverting if they do not fit in the package.
    function _parsePaddedPackage(bytes memory p) private pure returns (PaddedPackage memory pkg) {
        if (p.length < OFFSET_DECRYPTORS + TAIL_MIN_LENGTH) revert PadMalformedPackage();

        pkg.witnessVersion = uint8(p[OFFSET_WITNESS_VERSION]);
        pkg.prf = uint16(_readUint(p, OFFSET_PRF, 2));
        pkg.chainId = uint64(_readUint(p, OFFSET_CHAIN_ID, 8));
        pkg.submitter = address(uint160(_readUint(p, OFFSET_SUBMITTER, 20)));
        pkg.consumer = address(uint160(_readUint(p, OFFSET_CONSUMER, 20)));
        pkg.decryptorCount = uint8(p[OFFSET_DECRYPTOR_COUNT]);

        pkg.witnessEnd = OFFSET_DECRYPTORS + uint256(pkg.decryptorCount) * 20 + 16;
        if (p.length < pkg.witnessEnd + 32 + 1) revert PadMalformedPackage();
        pkg.label = bytes32(_readUint(p, pkg.witnessEnd, 32));
        pkg.partCount = uint8(p[pkg.witnessEnd + 32]);
        pkg.partsOffset = pkg.witnessEnd + 33;
        pkg.ciphertextOffset = pkg.partsOffset + pkg.partCount;
        if (p.length < pkg.ciphertextOffset) revert PadMalformedPackage();
    }

    /// @dev RFC-030 step 1 checks (the counters were checked while parsing).
    function _checkPaddedPackage(bytes memory p, PaddedPackage memory pkg) private view {
        if (pkg.witnessVersion != PAD_WITNESS_VERSION) revert PadUnsupportedWitnessVersion(pkg.witnessVersion);
        if (!_isSupportedPadPrf(pkg.prf)) revert PadUnsupportedPrf(pkg.prf);
        if (pkg.chainId != block.chainid) revert PadInvalidChainId(pkg.chainId);

        if (pkg.decryptorCount > PAD_MAX_DECRYPTORS) revert PadTooManyDecryptors(pkg.decryptorCount);
        if (pkg.submitter == address(0)) revert PadNullWitnessAddress();
        for (uint256 i = 0; i < pkg.decryptorCount; ++i) {
            if (_readUint(p, OFFSET_DECRYPTORS + i * 20, 20) == 0) revert PadNullWitnessAddress();
        }

        // Every part names a known type; N = Σ ⌈w/8⌉·8 bounds the pad and sizes the ciphertext.
        uint256 numBytes;
        for (uint256 i = 0; i < pkg.partCount; ++i) {
            uint8 partType = uint8(p[pkg.partsOffset + i]);
            uint256 partBytes = _paddedPartBytes(partType);
            if (partBytes == 0) revert PadUnknownPartType(i, partType);
            numBytes += partBytes;
        }
        if (numBytes * 8 > PAD_MAX_PAD_LENGTH) revert PadPackageTooLarge(numBytes * 8);
        uint256 ciphertextBytes = p.length - pkg.ciphertextOffset;
        if (ciphertextBytes != numBytes) revert PadCiphertextLengthMismatch(ciphertextBytes, numBytes);

        // label = hash(witness): the witness fields are the package bytes [1, witnessEnd).
        bytes memory witness = new bytes(pkg.witnessEnd - 1);
        for (uint256 i = 0; i < witness.length; ++i) {
            witness[i] = p[1 + i];
        }
        bytes32 expectedLabel = keccak256(abi.encodePacked(PAD_LABEL_DOMAIN_SEPARATOR, witness));
        if (expectedLabel != pkg.label) revert PadLabelMismatch(pkg.label, expectedLabel);
    }

    /**
     * @dev h = keccak256("FHE_padh" ‖ packageDigest ‖ inputIndex ‖ aclAddress ‖ chainid ‖
     *                     blockhash(block.number - 1) ‖ block.timestamp)
     *      with bytes [21..31] rewritten as 0xFF | chainId | type | HANDLE_VERSION.
     */
    function _computePaddedHandle(
        bytes32 packageDigest,
        uint8 inputIndex,
        FheType partType
    ) private view returns (bytes32 handle) {
        handle = keccak256(
            abi.encodePacked(
                PAD_HANDLE_DOMAIN_SEPARATOR,
                packageDigest,
                inputIndex,
                _padAclAddress(),
                block.chainid,
                blockhash(block.number - 1),
                block.timestamp
            )
        );
        handle = handle & 0xffffffffffffffffffffffffffffffffffffffffff0000000000000000000000;
        handle = handle | (bytes32(uint256(0xff)) << 80);
        handle = handle | (bytes32(uint256(uint64(block.chainid))) << 16);
        handle = handle | (bytes32(uint256(uint8(partType))) << 8);
        handle = handle | bytes32(uint256(HANDLE_VERSION));
    }

    /// @notice The host-chain ACL address hashed into compute handles.
    function _padAclAddress() internal view virtual returns (address);

    /// @dev Bytes a part of `partType` occupies in the packed stream (⌈w/8⌉), or 0 for an unknown type.
    function _paddedPartBytes(uint8 partType) internal pure returns (uint256) {
        if (partType == uint8(FheType.Bool)) return 1;
        if (partType == uint8(FheType.Uint8)) return 1;
        if (partType == uint8(FheType.Uint16)) return 2;
        if (partType == uint8(FheType.Uint32)) return 4;
        if (partType == uint8(FheType.Uint64)) return 8;
        if (partType == uint8(FheType.Uint128)) return 16;
        if (partType == uint8(FheType.Uint160)) return 20;
        if (partType == uint8(FheType.Uint256)) return 32;
        return 0;
    }

    /// @dev Big-endian unsigned integer of `length` (≤ 32) bytes at `offset`; the caller checks bounds.
    function _readUint(bytes memory p, uint256 offset, uint256 length) private pure returns (uint256 value) {
        for (uint256 i = 0; i < length; ++i) {
            value = (value << 8) | uint8(p[offset + i]);
        }
    }

    function _paddedPackageSlot(bytes32 packageDigest) private pure returns (bytes32) {
        return keccak256(abi.encode(packageDigest, PADDED_INPUTTER_TRANSIENT_LOCATION));
    }

    function _isPaddedPackageChecked(bytes32 packageDigest) private view returns (bool checked) {
        bytes32 slot = _paddedPackageSlot(packageDigest);
        assembly {
            checked := tload(slot)
        }
    }

    function _markPaddedPackageChecked(bytes32 packageDigest) private {
        bytes32 slot = _paddedPackageSlot(packageDigest);
        assembly {
            tstore(slot, 1)
        }
    }
}
