// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {FHEVMExecutor} from "./FHEVMExecutor.sol";
import {FheType} from "./shared/FheType.sol";
import {HANDLE_VERSION} from "./shared/Constants.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";
import {aclAdd, fhevmExecutorAdd} from "../addresses/FHEVMHostAddresses.sol";

/**
 * @title PaddedInputter
 * @notice RFC-030 (pad-based input encryption) host-chain verification of `fromExternal` inputs whose
 *         package type byte is `0x00`.
 * @dev The trusted host contract of this flow and its gatekeeper: it emits `Unmask` only after every
 *      consistency and authorization check has passed. Deployed behind an empty proxy and owned by the ACL
 *      owner, like the other host contracts.
 *
 *      The FHEVMExecutor routes a `fromExternal` whose input data starts with `0x00` here, through
 *      `verifyInput`, exactly as it routes a ZKPoK proof to the InputVerifier: the context carries the
 *      claimed submitter (`userAddress`, the app's caller) and the actual consumer (`contractAddress`, the
 *      executor's `msg.sender`). Only the executor may call it, since the consumer binding holds only if
 *      that address is the one the EVM guarantees. For each external handle it:
 *        1. checks the input package (cached per package in transient storage, keyed by `packageDigest`);
 *        2. checks the external handle against `parts`;
 *        3. checks `claimedSubmitter == submitter` and `actualConsumer == consumer`;
 *        4. derives a fresh `computeHandle` from host chain state;
 *        5. emits `Unmask(externalHandle, inputPackage, computeHandle)` for the coprocessor;
 *        6. returns `computeHandle`; the executor marks it minted and grants the transient allowance.
 *      Holds no per-pad state; only the governance-maintained set of PRF instances, the single source of
 *      truth the KMS checks pad decryption requests against. Instance identifiers are never reused.
 *
 *      Input package layout (big-endian integers):
 *        packageType 1 | witnessVersion 1 | prfId 2 | chainId 8 | submitter 20 | consumer 20 |
 *        expirationTimestamp 8 | decryptorCount 1 | decryptors 20×decryptorCount | nonce 16 |
 *        label 32 | partCount 1 | parts 1×partCount | ciphertext N/8
 *      The witness fields are encoded exactly as in the label preimage, so
 *      `label == keccak256("FHE_padl" ‖ inputPackage[1:witnessEnd])`.
 */
contract PaddedInputter is UUPSUpgradeableEmptyProxy, ACLOwnable {
    // ---- Contract information ----

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "PaddedInputter";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 1;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 2;

    /// @notice The only caller allowed to verify inputs.
    address private constant FHEVM_EXECUTOR_ADDRESS = fhevmExecutorAdd;

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

    // ---- Storage ----

    /// @custom:storage-location erc7201:fhevm.storage.PaddedInputter
    struct PaddedInputterStorage {
        mapping(uint16 prfId => bool isSupported) isSupportedPrf; /// @notice PRF instances packages may name
        mapping(uint16 prfId => bool isRetired) isRetiredPrf; /// @notice Retired instances, never admitted again
    }

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.PaddedInputter")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PADDED_INPUTTER_STORAGE_LOCATION =
        0xfe971c578e4861345fc59b10278c9b2b421c12ec10ab8775017ec233d3d21900;

    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.transient.PaddedInputter")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant PADDED_INPUTTER_TRANSIENT_LOCATION =
        0x42ad821e9dedd89becca1cd89b4b7d4157f995675baa2ebc184bfc2edbfecf00;

    // ---- Types ----

    /// @notice A parsed input package; offsets index into the package bytes.
    struct PaddedPackage {
        uint8 witnessVersion;
        uint16 prfId;
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

    /// @notice Emitted when a PRF instance is admitted: packages naming it are now accepted.
    event PrfInstanceAdded(uint16 prfId);

    /// @notice Emitted when a PRF instance is retired: packages naming it are now refused, for good.
    event PrfInstanceRetired(uint16 prfId);

    // ---- Errors ----

    /// @notice Returned if the caller is not the FHEVMExecutor.
    error CallerMustBeFHEVMExecutorContract();

    /// @notice Returned when adding a PRF instance that is already supported.
    error PadPrfAlreadySupported(uint16 prfId);

    /// @notice Returned when adding a PRF instance that was retired: identifiers are never reused.
    error PadPrfRetired(uint16 prfId);

    /// @notice Returned when adding PRF instance 0, which is never an instance: an unset `prfId` reads as 0, and
    ///         must be refused (`PadUnsupportedPrf(0)`) rather than silently taken as a live instance.
    error PadZeroPrfInstance();

    error PadMalformedPackage();
    error PadUnsupportedWitnessVersion(uint8 witnessVersion);
    error PadUnsupportedPrf(uint16 prfId);
    error PadInvalidChainId(uint64 chainId);
    error PadTooManyDecryptors(uint256 count);
    error PadNullWitnessAddress();
    error PadUnknownPartType(uint256 partIndex, uint8 partType);
    error PadPackageTooLarge(uint256 bits);
    error PadCiphertextLengthMismatch(uint256 ciphertextBytes, uint256 expectedBytes);
    error PadLabelMismatch(bytes32 label, bytes32 expectedLabel);
    error PadHandleDigestMismatch(bytes32 externalHandle, bytes32 packageDigest);
    error PadHandleVersionMismatch(uint8 handleVersion);
    error PadInputIndexOutOfBounds(uint8 inputIndex, uint8 partCount);
    error PadInputTypeMismatch(uint8 inputIndex, uint8 handleType, uint8 partType);
    error PadSubmitterMismatch(address claimedSubmitter, address submitter);
    error PadConsumerMismatch(address actualConsumer, address consumer);

    // ---- Initialization ----

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice Initializes the contract.
     * @param initialPrfInstances The PRF instances supported from the start; the coprocessor must already
     *        evaluate each of them.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(uint16[] calldata initialPrfInstances)
        public
        virtual
        onlyFromEmptyProxy
        reinitializer(REINITIALIZER_VERSION)
    {
        for (uint256 i = 0; i < initialPrfInstances.length; ++i) {
            _addPrfInstance(initialPrfInstances[i]);
        }
    }

    // ---- Governance ----

    /**
     * @notice Admits a PRF instance. The coprocessor must evaluate it before it is admitted here.
     * @param prfId The PRF instance identifier.
     */
    function addPrfInstance(uint16 prfId) public virtual onlyACLOwner {
        _addPrfInstance(prfId);
    }

    /**
     * @notice Retires a PRF instance for good: packages naming it are refused from now on, and the
     *         identifier can never be admitted again, since a stale package would otherwise pass the
     *         on-chain check and be unmasked under the wrong instance.
     * @dev The coprocessor drops the instance only once every `Unmask` emitted before retirement is processed.
     * @param prfId The PRF instance identifier.
     */
    function retirePrfInstance(uint16 prfId) public virtual onlyACLOwner {
        PaddedInputterStorage storage $ = _getPaddedInputterStorage();
        if (!$.isSupportedPrf[prfId]) revert PadUnsupportedPrf(prfId);
        $.isSupportedPrf[prfId] = false;
        $.isRetiredPrf[prfId] = true;
        emit PrfInstanceRetired(prfId);
    }

    // ---- Verification ----

    /**
     * @notice RFC-030 steps 1 to 6 for one external handle of `inputPackage`.
     * @param context The claimed submitter (`userAddress`) and the actual consumer (`contractAddress`).
     * @param externalHandle The external handle: packageDigest[0..28] | inputIndex | type | version.
     * @param inputPackage The whole input package.
     * @return computeHandle The fresh handle the app computes on.
     */
    function verifyInput(
        FHEVMExecutor.ContextUserInputs memory context,
        bytes32 externalHandle,
        bytes memory inputPackage
    ) public virtual returns (bytes32 computeHandle) {
        if (msg.sender != FHEVM_EXECUTOR_ADDRESS) revert CallerMustBeFHEVMExecutorContract();

        bytes32 packageDigest = keccak256(abi.encodePacked(PAD_PACKAGE_DOMAIN_SEPARATOR, inputPackage));

        // 1. Package-level checks, skipped for a package already checked in this transaction.
        PaddedPackage memory pkg = _parsePaddedPackage(inputPackage);
        if (!_isPaddedPackageChecked(packageDigest)) {
            _checkPaddedPackage(inputPackage, pkg);
            _markPaddedPackageChecked(packageDigest);
        }

        // 2. External handle checks.

        // 2.1 The handle must name this very package, not one that merely shares an index and type.
        if (bytes29(externalHandle) != bytes29(packageDigest)) {
            revert PadHandleDigestMismatch(externalHandle, packageDigest);
        }

        // 2.2 The handle must be of the current handle layout.
        if (uint8(externalHandle[31]) != HANDLE_VERSION) revert PadHandleVersionMismatch(uint8(externalHandle[31]));

        uint8 inputIndex = uint8(externalHandle[29]);

        // 2.3 The handle's part index must exist in this package.
        if (inputIndex >= pkg.partCount) revert PadInputIndexOutOfBounds(inputIndex, pkg.partCount);

        uint8 partType = uint8(inputPackage[pkg.partsOffset + inputIndex]);

        // 2.4 The handle's type must match the type of the part it points to.
        if (uint8(externalHandle[30]) != partType) {
            revert PadInputTypeMismatch(inputIndex, uint8(externalHandle[30]), partType);
        }

        // 3. Submitter and consumer bindings.
        if (context.userAddress != pkg.submitter) revert PadSubmitterMismatch(context.userAddress, pkg.submitter);
        if (context.contractAddress != pkg.consumer) {
            revert PadConsumerMismatch(context.contractAddress, pkg.consumer);
        }

        // 4. Fresh compute handle.
        computeHandle = _computePaddedHandle(packageDigest, inputIndex, FheType(partType));

        // 5. Unmask request for the coprocessor.
        emit Unmask(externalHandle, inputPackage, computeHandle);

        // 6. Returned to the executor, which marks it minted and grants the transient allowance.
    }

    // ---- Getters ----

    /**
     * @notice Whether `prfId` names a PRF instance packages may currently use.
     * @param prfId The PRF instance identifier.
     */
    function isSupportedPrfInstance(uint16 prfId) public view virtual returns (bool) {
        return _getPaddedInputterStorage().isSupportedPrf[prfId];
    }

    /**
     * @notice Whether `prfId` names a retired PRF instance, which can never be admitted again.
     * @param prfId The PRF instance identifier.
     */
    function isRetiredPrfInstance(uint16 prfId) public view virtual returns (bool) {
        return _getPaddedInputterStorage().isRetiredPrf[prfId];
    }

    /**
     * @notice        Getter for the name and version of the contract.
     * @return string Name and the version of the contract.
     */
    function getVersion() external pure virtual returns (string memory) {
        return string(
            abi.encodePacked(
                CONTRACT_NAME,
                " v",
                Strings.toString(MAJOR_VERSION),
                ".",
                Strings.toString(MINOR_VERSION),
                ".",
                Strings.toString(PATCH_VERSION)
            )
        );
    }

    // ---- Internals ----

    function _addPrfInstance(uint16 prfId) internal virtual {
        if (prfId == 0) revert PadZeroPrfInstance();
        PaddedInputterStorage storage $ = _getPaddedInputterStorage();
        if ($.isRetiredPrf[prfId]) revert PadPrfRetired(prfId);
        if ($.isSupportedPrf[prfId]) revert PadPrfAlreadySupported(prfId);
        $.isSupportedPrf[prfId] = true;
        emit PrfInstanceAdded(prfId);
    }

    /// @dev Parses the package header and counters, reverting if they do not fit in the package.
    function _parsePaddedPackage(bytes memory p) private pure returns (PaddedPackage memory pkg) {
        if (p.length < OFFSET_DECRYPTORS + TAIL_MIN_LENGTH) revert PadMalformedPackage();
        // The executor routes on this byte already; checked again so this contract stands on its own.
        if (uint8(p[0]) != PAD_PACKAGE_TYPE) revert PadMalformedPackage();

        pkg.witnessVersion = uint8(p[OFFSET_WITNESS_VERSION]);
        pkg.prfId = uint16(_extractBytes2(_loadWord(p, OFFSET_PRF)));
        pkg.chainId = uint64(_extractBytes8(_loadWord(p, OFFSET_CHAIN_ID)));
        pkg.submitter = address(_extractBytes20(_loadWord(p, OFFSET_SUBMITTER)));
        pkg.consumer = address(_extractBytes20(_loadWord(p, OFFSET_CONSUMER)));
        pkg.decryptorCount = uint8(p[OFFSET_DECRYPTOR_COUNT]);

        pkg.witnessEnd = OFFSET_DECRYPTORS + uint256(pkg.decryptorCount) * 20 + 16;
        if (p.length < pkg.witnessEnd + 32 + 1) revert PadMalformedPackage();
        pkg.label = _loadWord(p, pkg.witnessEnd);
        pkg.partCount = uint8(p[pkg.witnessEnd + 32]);
        pkg.partsOffset = pkg.witnessEnd + 33;
        pkg.ciphertextOffset = pkg.partsOffset + pkg.partCount;
        if (p.length < pkg.ciphertextOffset) revert PadMalformedPackage();
    }

    /// @dev RFC-030 step 1 checks (the counters were checked while parsing).
    function _checkPaddedPackage(bytes memory p, PaddedPackage memory pkg) private view {
        if (pkg.witnessVersion != PAD_WITNESS_VERSION) revert PadUnsupportedWitnessVersion(pkg.witnessVersion);
        if (!isSupportedPrfInstance(pkg.prfId)) revert PadUnsupportedPrf(pkg.prfId);
        if (pkg.chainId != block.chainid) revert PadInvalidChainId(pkg.chainId);

        if (pkg.decryptorCount > PAD_MAX_DECRYPTORS) revert PadTooManyDecryptors(pkg.decryptorCount);
        if (pkg.submitter == address(0)) revert PadNullWitnessAddress();
        for (uint256 i = 0; i < pkg.decryptorCount; ++i) {
            if (address(_extractBytes20(_loadWord(p, OFFSET_DECRYPTORS + i * 20))) == address(0)) {
                revert PadNullWitnessAddress();
            }
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
        // Previous implementation, O(length) byte-by-byte instead of a single copy:
        //
        // for (uint256 i = 0; i < witness.length; ++i) {
        //     witness[i] = p[1 + i];
        // }
        assembly ("memory-safe") {
            mcopy(add(witness, 0x20), add(p, 0x21), mload(witness))
        }
        bytes32 expectedLabel = keccak256(abi.encodePacked(PAD_LABEL_DOMAIN_SEPARATOR, witness));
        if (expectedLabel != pkg.label) revert PadLabelMismatch(pkg.label, expectedLabel);
    }

    /**
     * @dev h = keccak256("FHE_padh" ‖ packageDigest ‖ inputIndex ‖ aclAddress ‖ chainid ‖
     *                     blockhash(block.number - 1) ‖ block.timestamp)
     *      with bytes [21..31] rewritten as 0xFF | chainId | type | HANDLE_VERSION.
     */
    function _computePaddedHandle(bytes32 packageDigest, uint8 inputIndex, FheType partType)
        private
        view
        returns (bytes32 handle)
    {
        handle = keccak256(
            abi.encodePacked(
                PAD_HANDLE_DOMAIN_SEPARATOR,
                packageDigest,
                inputIndex,
                aclAdd,
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

    // Previous implementation, an 8-comparison chain in the worst case instead of a single lookup:
    //
    // /// @dev Bytes a part of `partType` occupies in the packed stream (⌈w/8⌉), or 0 for an unknown type.
    // function _paddedPartBytes(uint8 partType) private pure returns (uint256) {
    //     if (partType == uint8(FheType.Bool)) return 1;
    //     if (partType == uint8(FheType.Uint8)) return 1;
    //     if (partType == uint8(FheType.Uint16)) return 2;
    //     if (partType == uint8(FheType.Uint32)) return 4;
    //     if (partType == uint8(FheType.Uint64)) return 8;
    //     if (partType == uint8(FheType.Uint128)) return 16;
    //     if (partType == uint8(FheType.Uint160)) return 20;
    //     if (partType == uint8(FheType.Uint256)) return 32;
    //     return 0;
    // }

    /**
     * @dev Byte `i` (big-endian) of this table is the size, in bytes, of a part of `FheType` value `i`
     *      (⌈w/8⌉): Bool=1, Uint8=1, Uint16=2, Uint32=4, Uint64=8, Uint128=16, Uint160=20, Uint256=32; every
     *      other byte, including every `FheType` not supported by this flow, is 0.
     */
    bytes32 private constant PADDED_PART_BYTES = 0x0100010204081014200000000000000000000000000000000000000000000000;

    /// @dev Bytes a part of `partType` occupies in the packed stream, or 0 for an unsupported type. The `BYTE`
    ///      opcode returns 0 for `partType >= 32`, covering every `FheType` value beyond `PADDED_PART_BYTES`
    ///      without an explicit bounds check.
    function _paddedPartBytes(uint8 partType) private pure returns (uint256 size) {
        bytes32 table = PADDED_PART_BYTES;
        assembly ("memory-safe") {
            size := byte(partType, table)
        }
    }

    // Previous implementation, O(length) per field instead of O(1):
    //
    // /// @dev Big-endian unsigned integer of `length` (≤ 32) bytes at `offset`; the caller checks bounds.
    // function _readUint(bytes memory p, uint256 offset, uint256 length) private pure returns (uint256 value) {
    //     for (uint256 i = 0; i < length; ++i) {
    //         value = (value << 8) | uint8(p[offset + i]);
    //     }
    // }

    /// @dev Loads the 32-byte word starting at `offset` in `p`'s data; caller checks bounds.
    function _loadWord(bytes memory p, uint256 offset) private pure returns (bytes32 word) {
        assembly ("memory-safe") {
            word := mload(add(add(p, 0x20), offset))
        }
    }

    // The three extractors below are inlined, specialized copies of OpenZeppelin Contracts'
    // Packing.extract_32_2 / extract_32_8 / extract_32_20 (utils/Packing.sol, MIT License,
    // Copyright (c) 2016-2025 zOS Global Limited and contributors), reproduced here to avoid
    // importing the whole library for three fixed-offset extractions.

    /// @dev Top 2 bytes of `word`, right-aligned.
    function _extractBytes2(bytes32 word) private pure returns (bytes2 result) {
        assembly ("memory-safe") {
            result := and(word, shl(240, not(0)))
        }
    }

    /// @dev Top 8 bytes of `word`, right-aligned.
    function _extractBytes8(bytes32 word) private pure returns (bytes8 result) {
        assembly ("memory-safe") {
            result := and(word, shl(192, not(0)))
        }
    }

    /// @dev Top 20 bytes of `word`, right-aligned.
    function _extractBytes20(bytes32 word) private pure returns (bytes20 result) {
        assembly ("memory-safe") {
            result := and(word, shl(96, not(0)))
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

    /**
     * @dev Returns the PaddedInputter storage location.
     */
    function _getPaddedInputterStorage() internal pure returns (PaddedInputterStorage storage $) {
        assembly {
            $.slot := PADDED_INPUTTER_STORAGE_LOCATION
        }
    }

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
