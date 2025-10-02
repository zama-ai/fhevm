// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEVMExecutor} from "./FHEVMExecutor.sol";

// Importing OpenZeppelin contracts for cryptographic signature verification and access control.
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {UUPSUpgradeableEmptyProxy} from "./shared/UUPSUpgradeableEmptyProxy.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {EIP712UpgradeableCrossChain} from "./shared/EIP712UpgradeableCrossChain.sol";
import {HANDLE_VERSION} from "./shared/Constants.sol";
import {ACLOwnable} from "./shared/ACLOwnable.sol";

/**
 * @title    InputVerifier.
 * @notice   This contract allows signature verification of user encrypted inputs.
 *           This contract is called by the FHEVMExecutor inside verifyInput function
 * @dev      The contract uses EIP712UpgradeableCrossChain for cryptographic operations.
 */
contract InputVerifier is UUPSUpgradeableEmptyProxy, EIP712UpgradeableCrossChain, ACLOwnable {
    /// @notice         Emitted when a signer is added.
    /// @param signer   The address of the signer that was added.
    event SignerAdded(address indexed signer);

    /// @notice         Emitted when a signer is removed.
    /// @param signer   The address of the signer that was removed.
    event SignerRemoved(address indexed signer);

    /// @notice Returned if the deserializing of the input proof fails.
    error DeserializingInputProofFail();

    /// @notice Returned if the input proof is empty.
    error EmptyInputProof();

    /// @notice Returned if the chain id from the input handle is invalid.
    error InvalidChainId();

    /// @notice Returned if the index is invalid.
    error InvalidIndex();

    /// @notice Returned if the input handle is wrong.
    error InvalidInputHandle();

    /// @notice Returned if the handle version is not the correct one.
    error InvalidHandleVersion();

    /// @notice Returned if signer is null.
    error SignerNull();

    /// @notice Returned if signer is already registered.
    error AlreadySigner();

    /// @notice Returned if no signer is already registered.
    error AtLeastOneSignerIsRequired();

    ///  @notice Returned if not a registered signer.
    error NotASigner();

    /// @notice Returned in case signerRecovered is an invalid signer.
    error InvalidSigner(address signerRecovered);

    /// @notice Returned if number of unique signers is not reached.
    error SignatureThresholdNotReached(uint256 numSignatures);

    /// @notice Returned if signature is null.
    error ZeroSignature();

    /// @notice Returned when signatures verification fails.
    error SignaturesVerificationFailed();

    /// @notice Returned when the set of coprocessor signers is empty.
    error EmptyCoprocessorSignerAddresses(uint256 contextId);

    /// @notice Returned when the context ID is null.
    error InvalidNullContextId();

    /// @notice Returned when the context ID is not marked as active or suspended.
    error InvalidContextId(uint256 contextId);

    /// @notice Returned when the context ID has already been initialized (not in the NotInitialized state).
    error ContextAlreadyInitialized(uint256 contextId);

    /// @notice The state of a coprocessor context ID.
    enum CoprocessorContextState {
        NotInitialized,
        Active,
        Suspended,
        Deactivated
    }

    /// @param handles      List of handles.
    /// @param userAddress      Address of the user.
    /// @param contractAddress  Contract address.
    /// @param contractChainId  ChainID of contract.
    struct CiphertextVerification {
        /// @notice The Coprocessor's computed ciphertext handles.
        bytes32[] ctHandles;
        /// @notice The address of the user that has provided the input in the ZK Proof verification request.
        address userAddress;
        /// @notice The address of the dapp requiring the ZK Proof verification.
        address contractAddress;
        /// @notice The chainId of the contract requiring the ZK Proof verification.
        uint256 contractChainId;
        /// @notice The coprocessor context ID used for the ZK Proof verification.
        uint256 coprocessorContextId;
        /// @notice Generic bytes metadata for versioned payloads. First byte is for the version.
        bytes extraData;
    }

    /// @notice The definition of the CiphertextVerification structure typed data.
    string public constant EIP712_INPUT_VERIFICATION_TYPE =
        "CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,uint256 coprocessorContextId,bytes extraData)";

    /// @notice The hash of the CiphertextVerification structure typed data definition used for signature validation.
    bytes32 public constant EIP712_INPUT_VERIFICATION_TYPEHASH = keccak256(bytes(EIP712_INPUT_VERIFICATION_TYPE));

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "InputVerifier";

    /// @notice Name of the source contract for which original EIP712 was destinated.
    string private constant CONTRACT_NAME_SOURCE = "InputVerification";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 1;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @custom:storage-location erc7201:fhevm.storage.InputVerifier
    struct InputVerifierStorage {
        // TODO: remove this for mainnet.
        mapping(address => bool) isSigner; /// @notice Mapping to keep track of addresses that are signers
        // TODO: remove this for mainnet.
        address[] signers; /// @notice Array to keep track of all signers
        // TODO: remove this for mainnet.
        uint256 threshold; /// @notice The threshold for the number of signers required for a signature to be valid
        /// @notice Current active coprocessor context ID.
        uint256 activeCoprocessorContextId;
        /// @notice Suspended coprocessor context ID.
        uint256 suspendedCoprocessorContextId;
        /// @notice Mapping to keep track of coprocessor context states.
        mapping(uint256 contextId => CoprocessorContextState contextState) coprocessorContextStates;
        /// @notice Mapping to keep track of coprocessor context signers.
        mapping(uint256 contextId => address[] signers) coprocessorContextSigners;
    }

    /// Constant used for making sure the version number used in the `reinitializer` modifier is
    /// identical between `initializeFromEmptyProxy` and the `reinitializeVX` method
    uint64 private constant REINITIALIZER_VERSION = 2;

    /// keccak256(abi.encode(uint256(keccak256("fhevm.storage.InputVerifier")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant InputVerifierStorageLocation =
        0x3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace00;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Initializes the contract.
     * @param verifyingContractSource InputVerification contract address from Gateway chain.
     * @param chainIDSource chainID of Gateway chain.
     * @param initialContextId Initial active context ID.
     * @param initialContextSigners Initial list of signers for the active context ID.
     */
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        address verifyingContractSource,
        uint64 chainIDSource,
        uint256 initialCoprocessorContextId,
        address[] calldata initialCoprocessorSigners
    ) public virtual onlyFromEmptyProxy reinitializer(REINITIALIZER_VERSION) {
        __EIP712_init(CONTRACT_NAME_SOURCE, "1", verifyingContractSource, chainIDSource);

        // Check for valid initial context ID and non-empty initial signers set.
        if (initialCoprocessorContextId == 0) {
            revert InvalidNullContextId();
        }
        if (initialCoprocessorSigners.length == 0) {
            revert EmptyCoprocessorSignerAddresses(initialCoprocessorContextId);
        }

        InputVerifierStorage storage $ = _getInputVerifierStorage();

        // Activate the initial context.
        $.coprocessorContextStates[initialCoprocessorContextId] = CoprocessorContextState.Active;
        $.coprocessorContextSigners[initialCoprocessorContextId] = initialCoprocessorSigners;
        $.activeCoprocessorContextId = initialCoprocessorContextId;
    }

    /**
     * @notice Re-initializes the contract from V1.
     * @dev Define a `reinitializeVX` function once the contract needs to be upgraded.
     */
    /// @custom:oz-upgrades-unsafe-allow missing-initializer-call
    /// @custom:oz-upgrades-validate-as-initializer
    // function reinitializeV2() public virtual reinitializer(REINITIALIZER_VERSION) {}

    /**
     * @dev This function removes the transient allowances, which could be useful for
            integration with Account Abstraction when bundling several UserOps calling InputVerifier.
     */
    function cleanTransientStorage() public virtual {
        assembly {
            let length := tload(0)
            tstore(0, 0)
            let lengthPlusOne := add(length, 1)
            for {
                let i := 1
            } lt(i, lengthPlusOne) {
                i := add(i, 1)
            } {
                let handle := tload(i)
                tstore(i, 0)
                tstore(handle, 0)
            }
        }
    }

    /**
     * @notice              Verifies the ciphertext.
     * @param context       Context user inputs.
     * @param inputHandle   Input handle.
     * @param inputProof    Input proof.
     * @return result       Result.
     */
    function verifyInput(
        FHEVMExecutor.ContextUserInputs memory context,
        bytes32 inputHandle,
        bytes memory inputProof
    ) public virtual returns (bytes32) {
        (bool isProofCached, bytes32 cacheKey) = _checkProofCache(
            inputProof,
            context.userAddress,
            context.contractAddress
        );

        uint64 recoveredChainId = uint64(
            uint256((inputHandle & 0x00000000000000000000000000000000000000000000ffffffffffffffffffff) >> 16)
        );

        if (recoveredChainId != block.chainid) revert InvalidChainId();

        uint256 result = uint256(inputHandle);
        uint256 indexHandle = (result & 0x0000000000000000000000000000000000000000ff00000000000000000000) >> 80;

        if (!isProofCached) {
            /// @dev bundleCiphertext is compressedPackedCT+ZKPOK
            ///      inputHandle is keccak256(keccak256(bundleCiphertext)+index)[0:20] + index[21] + chainId[22:29] + type[30] + version[31]
            ///      and inputProof is numHandles + numSigners + coprocessorContextId + handles + coprocessorSignatures (1 + 1 + 1 + 32*numHandles + 65*numSigners + extraData bytes)

            // uint256 inputProofLen = inputProof.length;
            if (inputProof.length == 0) revert EmptyInputProof();
            uint256 numHandles = uint256(uint8(inputProof[0]));
            uint256 numSigners = uint256(uint8(inputProof[1]));

            // Extract the coprocessor context ID from inputProof.
            uint256 coprocessorContextId;
            assembly {
                coprocessorContextId := mload(add(inputProof, 0x22)) // 0x20 offset for array prefix + 2 bytes header
            }

            /// @dev This checks in particular that the list is non-empty.
            if (numHandles <= indexHandle || indexHandle > 254) revert InvalidIndex();

            /// @dev The extraData is the rest of the inputProof bytes after:
            ///     + numHandles (1 byte)
            ///     + numSigners (1 byte)
            ///     + coprocesorContextId (32 bytes)
            ///     + handles (32 bytes each)
            ///     + coprocessorSignatures (65 bytes each)
            uint256 extraDataOffset = 34 + 32 * numHandles + 65 * numSigners;

            /// @dev Check that the inputProof is long enough to contain at least the numHandles + numSigners + handles + coprocessorSignatures
            if (inputProof.length < extraDataOffset) revert DeserializingInputProofFail();

            /// @dev Deserialize handle and check that they are from the correct version.
            bytes32[] memory listHandles = new bytes32[](numHandles);
            for (uint256 i = 0; i < numHandles; i++) {
                bytes32 element;
                assembly {
                    // 32 (array length) + 2 (numSigners and numHandles) + 32 (coprocessorContextId) + 32*i
                    element := mload(add(inputProof, add(66, mul(i, 32))))
                }
                /// @dev Check that all handles are from the correct version.
                if (uint8(uint256(element)) != HANDLE_VERSION) revert InvalidHandleVersion();
                listHandles[i] = element;
            }

            bytes[] memory signatures = new bytes[](numSigners);
            for (uint256 j = 0; j < numSigners; j++) {
                signatures[j] = new bytes(65);
                for (uint256 i = 0; i < 65; i++) {
                    signatures[j][i] = inputProof[34 + 32 * numHandles + 65 * j + i];
                }
            }

            CiphertextVerification memory ctVerif;
            ctVerif.ctHandles = listHandles;
            ctVerif.userAddress = context.userAddress;
            ctVerif.contractAddress = context.contractAddress;
            ctVerif.contractChainId = block.chainid;
            ctVerif.coprocessorContextId = coprocessorContextId;

            /// @dev Extract the extraData from the inputProof.
            uint256 extraDataSize = inputProof.length - extraDataOffset;
            ctVerif.extraData = new bytes(extraDataSize);

            for (uint i = 0; i < extraDataSize; i++) {
                ctVerif.extraData[i] = inputProof[extraDataOffset + i];
            }

            _verifyEIP712(ctVerif, signatures);

            _cacheProof(cacheKey);
            if (result != uint256(listHandles[indexHandle])) revert InvalidInputHandle();
        } else {
            uint8 numHandles = uint8(inputProof[0]);
            /// @dev We know inputProof is non-empty since it has been previously cached.
            if (numHandles <= indexHandle || indexHandle > 254) revert InvalidIndex();
            uint256 element;
            for (uint256 j = 0; j < 32; j++) {
                element |= uint256(uint8(inputProof[34 + indexHandle * 32 + j])) << (8 * (31 - j));
            }
            if (element != result) revert InvalidInputHandle();
        }

        return bytes32(result);
    }

    /**
     * @notice Returns the list of signers of a specific context ID.
     * @dev If there are too many signers, it could be out-of-gas.
     * @param coprocessorContextId The coprocessor context ID of the signer addresses to return.
     * @return signers  List of signers.
     */
    function getCoprocessorSigners(uint256 coprocessorContextId) public view virtual returns (address[] memory) {
        InputVerifierStorage storage $ = _getInputVerifierStorage();

        return $.coprocessorContextSigners[coprocessorContextId];
    }

    /**
     * @notice Get the threshold for signature.
     * @param coprocessorContextId The coprocessor context ID of the signer addresses to get the threshold from.
     * @return threshold Threshold for signature verification.
     */
    function getThreshold(uint256 coprocessorContextId) public view virtual returns (uint256) {
        address[] memory coprocessorSigners = getCoprocessorSigners(coprocessorContextId);

        // The majority threshold is the number of coprocessors that is required to validate consensus.
        // It is currently defined as a strict majority within the coprocessor context (50% + 1).
        return coprocessorSigners.length / 2 + 1;
    }

    /**
     * @notice Returns whether the account address is a valid signer.
     * @param coprocessorContextId The coprocessor context ID of the signer addresses to check the signer against.
     * @param account Account address.
     * @return isSigner Whether the account is a valid signer.
     */
    function isSigner(uint256 coprocessorContextId, address account) public view virtual returns (bool) {
        address[] memory coprocessorSigners = getCoprocessorSigners(coprocessorContextId);
        for (uint256 i = 0; i < coprocessorSigners.length; i++) {
            if (coprocessorSigners[i] == account) {
                return true;
            }
        }
        return false;
    }

    /**
     * @notice Returns whether the coprocessor context ID is active or suspended.
     * @param coprocessorContextId The coprocessor context ID to check.
     * @return isActiveOrSuspended Whether the coprocessor context ID is active or suspended.
     */
    function isCoprocessorContextActiveOrSuspended(uint256 coprocessorContextId) public view virtual returns (bool) {
        InputVerifierStorage storage $ = _getInputVerifierStorage();
        if (
            $.coprocessorContextStates[coprocessorContextId] == CoprocessorContextState.Suspended ||
            $.coprocessorContextStates[coprocessorContextId] == CoprocessorContextState.Active
        ) {
            return true;
        }
        revert InvalidContextId(coprocessorContextId);
    }

    function addNewContextAndSuspendOldOne(
        uint256 newContextId,
        address[] calldata newContextSigners
    ) public virtual onlyACLOwner {
        if (newContextId == 0) {
            revert InvalidNullContextId();
        }
        if (newContextSigners.length == 0) {
            revert EmptyCoprocessorSignerAddresses(newContextId);
        }

        InputVerifierStorage storage $ = _getInputVerifierStorage();

        // Check that the new context ID is not already used.
        if ($.coprocessorContextStates[newContextId] != CoprocessorContextState.NotInitialized) {
            revert ContextAlreadyInitialized(newContextId);
        }

        // Suspend the current active context.
        $.coprocessorContextStates[$.activeCoprocessorContextId] = CoprocessorContextState.Suspended;
        $.suspendedCoprocessorContextId = $.activeCoprocessorContextId;

        // Activate the new context.
        $.coprocessorContextStates[newContextId] = CoprocessorContextState.Active;
        $.coprocessorContextSigners[newContextId] = newContextSigners;
        $.activeCoprocessorContextId = newContextId;
    }

    function removeSuspendedCoprocessorContext() public virtual onlyACLOwner {
        InputVerifierStorage storage $ = _getInputVerifierStorage();

        // Mark the suspended context as deactivated.
        $.coprocessorContextStates[$.suspendedCoprocessorContextId] = CoprocessorContextState.Deactivated;
    }

    /**
     * @notice        Getter for the handle version.
     * @return uint8 The current version for new handles.
     */
    function getHandleVersion() external pure virtual returns (uint8) {
        return HANDLE_VERSION;
    }

    /**
     * @notice        Getter for the name and version of the contract.
     * @return string Name and the version of the contract.
     */
    function getVersion() external pure virtual returns (string memory) {
        return
            string(
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

    function _cacheProof(bytes32 proofKey) internal virtual {
        assembly {
            tstore(proofKey, 1)
            let length := tload(0)
            let lengthPlusOne := add(length, 1)
            tstore(lengthPlusOne, proofKey)
            tstore(0, lengthPlusOne)
        }
    }

    function _checkProofCache(
        bytes memory inputProof,
        address userAddress,
        address contractAddress
    ) internal view virtual returns (bool, bytes32) {
        bool isProofCached;
        bytes32 key = keccak256(abi.encodePacked(contractAddress, userAddress, inputProof));
        assembly {
            isProofCached := tload(key)
        }
        return (isProofCached, key);
    }

    /// @notice Computes the hash of a given CiphertextVerification structured data
    /// @param ctVerification The CiphertextVerification structure
    /// @return The hash of the CiphertextVerification structure
    function _hashEIP712InputVerification(
        CiphertextVerification memory ctVerification
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_INPUT_VERIFICATION_TYPEHASH,
                        keccak256(abi.encodePacked(ctVerification.ctHandles)),
                        ctVerification.userAddress,
                        ctVerification.contractAddress,
                        ctVerification.contractChainId,
                        ctVerification.coprocessorContextId,
                        keccak256(abi.encodePacked(ctVerification.extraData))
                    )
                )
            );
    }

    function _verifyEIP712(CiphertextVerification memory ctVerif, bytes[] memory signatures) internal virtual {
        // Ensure the coprocessorContextId is a valid active or suspended.
        isCoprocessorContextActiveOrSuspended(ctVerif.coprocessorContextId);

        // Verify the signatures for the given coprocessorContextId.
        bytes32 digest = _hashEIP712InputVerification(ctVerif);
        if (!_verifySignaturesDigest(ctVerif.coprocessorContextId, digest, signatures))
            revert SignaturesVerificationFailed();
    }

    /**
     * @notice              Verifies multiple signatures for a given message at a certain threshold.
     * @dev                 Calls verifySignature internally.
     * @param coprocessorContextId The coprocessor context ID of the signer addresses to use for verification.
     * @param digest        The hash of the message that was signed by all signers.
     * @param signatures    An array of signatures to verify.
     * @return isVerified   true if enough provided signatures are valid, false otherwise.
     */
    function _verifySignaturesDigest(
        uint256 coprocessorContextId,
        bytes32 digest,
        bytes[] memory signatures
    ) internal virtual returns (bool) {
        uint256 numSignatures = signatures.length;

        if (numSignatures == 0) {
            revert ZeroSignature();
        }

        uint256 threshold = getThreshold(coprocessorContextId);

        if (numSignatures < threshold) {
            revert SignatureThresholdNotReached(numSignatures);
        }

        address[] memory recoveredSigners = new address[](numSignatures);
        uint256 uniqueValidCount;
        for (uint256 i = 0; i < numSignatures; i++) {
            address signerRecovered = _recoverSigner(digest, signatures[i]);
            if (!isSigner(coprocessorContextId, signerRecovered)) {
                revert InvalidSigner(signerRecovered);
            }
            if (!_tload(signerRecovered)) {
                recoveredSigners[uniqueValidCount] = signerRecovered;
                uniqueValidCount++;
                _tstore(signerRecovered, 1);
            }
            if (uniqueValidCount >= threshold) {
                _cleanTransientHashMap(recoveredSigners, uniqueValidCount);
                return true;
            }
        }
        _cleanTransientHashMap(recoveredSigners, uniqueValidCount);
        return false;
    }

    /**
     * @notice          Cleans a hashmap in transient storage.
     * @dev             This is important to keep composability in the context of account abstraction.
     * @param keys      An array of keys to cleanup from transient storage.
     * @param maxIndex  The biggest index to take into account from the array - assumed to be less or equal to keys.length.
     */
    function _cleanTransientHashMap(address[] memory keys, uint256 maxIndex) internal virtual {
        for (uint256 j = 0; j < maxIndex; j++) {
            _tstore(keys[j], 0);
        }
    }

    /**
     * @notice           Reads transient storage.
     * @dev              Uses inline assembly to access the Transient Storage's tload operation.
     * @param location   The address used as key where transient storage of the contract is read at.
     * @return value     true if value stored at the given location is non-null, false otherwise.
     */
    function _tload(address location) internal view virtual returns (bool value) {
        assembly {
            value := tload(location)
        }
    }

    /**
     * @notice          Writes to transient storage.
     * @dev             Uses inline assembly to access the Transient Storage's _tstore operation.
     * @param location  The address used as key where transient storage of the contract is written at.
     * @param value     An uint256 stored at location key in transient storage of the contract.
     */
    function _tstore(address location, uint256 value) internal virtual {
        assembly {
            tstore(location, value)
        }
    }

    /**
     * @notice          Recovers the signer's address from a `signature` and a `message` digest.
     * @dev             It utilizes ECDSA for actual address recovery. It does not support contract signature (EIP-1271).
     * @param message   The hash of the message that was signed.
     * @param signature The signature to verify.
     * @return signer   The address that supposedly signed the message.
     */
    function _recoverSigner(bytes32 message, bytes memory signature) internal pure virtual returns (address) {
        address signerRecovered = ECDSA.recover(message, signature);
        return signerRecovered;
    }

    /**
     * @dev Returns the InputVerifier storage location.
     */
    function _getInputVerifierStorage() internal pure returns (InputVerifierStorage storage $) {
        assembly {
            $.slot := InputVerifierStorageLocation
        }
    }

    /**
     * @dev Should revert when msg.sender is not authorized to upgrade the contract.
     */
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyACLOwner {}
}
