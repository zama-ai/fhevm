// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {TFHEExecutor} from "./TFHEExecutor.sol";

// Importing OpenZeppelin contracts for cryptographic signature verification and access control.
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {Ownable2StepUpgradeable} from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {EIP712UpgradeableCrossChain} from "./EIP712UpgradeableCrossChain.sol";

/**
 * @title    InputVerifier.
 * @notice   This contract allows signature verification of user encrypted inputs.
 *           This contract is called by the HTTPZExecutor inside verifyCiphertext function
 * @dev      The contract uses EIP712UpgradeableCrossChain for cryptographic operations.
 */
contract InputVerifier is UUPSUpgradeable, Ownable2StepUpgradeable, EIP712UpgradeableCrossChain {
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

    /// @notice Returned if the initial signers set is empty.
    error InitialSignersSetIsEmpty();

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
    }

    /// @notice The definition of the CiphertextVerification structure typed data.
    string private constant EIP712_ZKPOK_TYPE =
        "CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId)";

    /// @notice The hash of the EIP712ZKPoK structure typed data definition used for signature validation.
    bytes32 private constant EIP712_ZKPOK_TYPE_HASH = keccak256(bytes(EIP712_ZKPOK_TYPE));

    /// @notice Handle version.
    uint8 public constant HANDLE_VERSION = 0;

    /// @notice Name of the contract.
    string private constant CONTRACT_NAME = "InputVerifier";

    /// @notice Name of the source contract for which original EIP712 was destinated.
    string private constant CONTRACT_NAME_SOURCE = "ZKPoKManager";

    /// @notice Major version of the contract.
    uint256 private constant MAJOR_VERSION = 0;

    /// @notice Minor version of the contract.
    uint256 private constant MINOR_VERSION = 1;

    /// @notice Patch version of the contract.
    uint256 private constant PATCH_VERSION = 0;

    /// @custom:storage-location erc7201:httpz.storage.InputVerifier
    struct InputVerifierStorage {
        mapping(address => bool) isSigner; /// @notice Mapping to keep track of addresses that are signers
        address[] signers; /// @notice Array to keep track of all signers
        uint256 threshold; /// @notice The threshold for the number of signers required for a signature to be valid
    }

    /// keccak256(abi.encode(uint256(keccak256("httpz.storage.InputVerifier")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant InputVerifierStorageLocation =
        0x0f3182ad724e9de82dc79a31e3b0e792f87a844d042e92fa257979351135e300;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /**
     * @notice  Re-initializes the contract.
     */
    function reinitialize(
        address verifyingContractSource,
        uint64 chainIDSource,
        address[] calldata initialSigners
    ) public reinitializer(2) {
        __EIP712_init(CONTRACT_NAME_SOURCE, "1", verifyingContractSource, chainIDSource);
        uint256 initialSignersLen = initialSigners.length;
        if (initialSignersLen == 0) {
            revert InitialSignersSetIsEmpty();
        }
        for (uint256 i = 0; i < initialSignersLen; i++) {
            addSigner(initialSigners[i]);
        }
    }

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
    function verifyCiphertext(
        TFHEExecutor.ContextUserInputs memory context,
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
            ///      and inputProof is len(list_handles) + numSigners + list_handles + signatureCoprocessorSigners (1+1+NUM_HANDLES*32+65*numSigners)

            uint256 inputProofLen = inputProof.length;
            if (inputProofLen == 0) revert EmptyInputProof();
            uint256 numHandles = uint256(uint8(inputProof[0]));
            uint256 numSigners = uint256(uint8(inputProof[1]));

            /// @dev This checks in particular that the list is non-empty.
            if (numHandles <= indexHandle || indexHandle > 254) revert InvalidIndex();
            if (inputProofLen != 2 + 32 * numHandles + 65 * numSigners) revert DeserializingInputProofFail();

            /// @dev Deserialize handle and check that they are from the correct version.
            bytes32[] memory listHandles = new bytes32[](numHandles);
            for (uint256 i = 0; i < numHandles; i++) {
                bytes32 element;
                assembly {
                    element := mload(add(inputProof, add(34, mul(i, 32))))
                }
                /// @dev Check that all handles are from the correct version.
                if (uint8(uint256(element)) != HANDLE_VERSION) revert InvalidHandleVersion();
                listHandles[i] = element;
            }

            bytes[] memory signatures = new bytes[](numSigners);
            for (uint256 j = 0; j < numSigners; j++) {
                signatures[j] = new bytes(65);
                for (uint256 i = 0; i < 65; i++) {
                    signatures[j][i] = inputProof[2 + 32 * numHandles + 65 * j + i];
                }
            }
            CiphertextVerification memory ctVerif;
            ctVerif.ctHandles = listHandles;
            ctVerif.userAddress = context.userAddress;
            ctVerif.contractAddress = context.contractAddress;
            _verifyEIP712(ctVerif, signatures);

            _cacheProof(cacheKey);
            if (result != uint256(listHandles[indexHandle])) revert InvalidInputHandle();
        } else {
            uint8 numHandles = uint8(inputProof[0]);
            /// @dev We know inputProof is non-empty since it has been previously cached.
            if (numHandles <= indexHandle || indexHandle > 254) revert InvalidIndex();
            uint256 element;
            for (uint256 j = 0; j < 32; j++) {
                element |= uint256(uint8(inputProof[2 + indexHandle * 32 + j])) << (8 * (31 - j));
            }
            if (element != result) revert InvalidInputHandle();
        }

        return bytes32(result);
    }

    /**
     * @notice          Adds a new signer.
     * @dev             Only the owner can add a signer.
     * @param signer    The address to be added as a signer.
     */
    function addSigner(address signer) public virtual onlyOwner {
        if (signer == address(0)) {
            revert SignerNull();
        }

        InputVerifierStorage storage $ = _getInputVerifierStorage();
        if ($.isSigner[signer]) {
            revert AlreadySigner();
        }

        $.isSigner[signer] = true;
        $.signers.push(signer);
        _applyThreshold();
        emit SignerAdded(signer);
    }

    /**
     * @notice          Removes an existing signer.
     * @dev             Only the owner can remove a signer.
     * @param signer    The signer address to remove.
     */
    function removeSigner(address signer) public virtual onlyOwner {
        InputVerifierStorage storage $ = _getInputVerifierStorage();
        if (!$.isSigner[signer]) {
            revert NotASigner();
        }

        /// @dev Remove signer from the mapping.
        $.isSigner[signer] = false;

        /// @dev Find the index of the signer and remove it from the array.
        for (uint i = 0; i < $.signers.length; i++) {
            if ($.signers[i] == signer) {
                $.signers[i] = $.signers[$.signers.length - 1]; /// @dev Move the last element into the place to delete.
                $.signers.pop(); /// @dev Remove the last element.
                _applyThreshold();
                emit SignerRemoved(signer);
                return;
            }
        }
    }

    /**
     * @notice          Returns the list of signers.
     * @dev             If there are too many signers, it could be out-of-gas.
     * @return signers  List of signers.
     */
    function getCoprocessorSigners() public view virtual returns (address[] memory) {
        InputVerifierStorage storage $ = _getInputVerifierStorage();
        return $.signers;
    }

    /**
     * @notice              Get the threshold for signature.
     * @return threshold    Threshold for signature verification.
     */
    function getThreshold() public view virtual returns (uint256) {
        InputVerifierStorage storage $ = _getInputVerifierStorage();
        return $.threshold;
    }

    /**
     * @notice              Returns whether the account address is a valid signer.
     * @param account       Account address.
     * @return isSigner     Whether the account is a valid signer.
     */
    function isSigner(address account) public view virtual returns (bool) {
        InputVerifierStorage storage $ = _getInputVerifierStorage();
        return $.isSigner[account];
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

    /// @notice Computes the hash of a given EIP712ZKPoK structured data
    /// @param ctVerification The EIP712ZKPoK structure
    /// @return The hash of the EIP712ZKPoK structure
    function _hashEIP712ZKPoK(CiphertextVerification memory ctVerification) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        EIP712_ZKPOK_TYPE_HASH,
                        keccak256(abi.encodePacked(ctVerification.ctHandles)),
                        ctVerification.userAddress,
                        ctVerification.contractAddress,
                        block.chainid
                    )
                )
            );
    }

    function _verifyEIP712(CiphertextVerification memory ctVerif, bytes[] memory signatures) internal virtual {
        bytes32 digest = _hashEIP712ZKPoK(ctVerif);
        if (!_verifySignaturesDigest(digest, signatures)) revert SignaturesVerificationFailed();
    }

    /**
     * @notice              Verifies multiple signatures for a given message at a certain threshold.
     * @dev                 Calls verifySignature internally.
     * @param digest        The hash of the message that was signed by all signers.
     * @param signatures    An array of signatures to verify.
     * @return isVerified   true if enough provided signatures are valid, false otherwise.
     */
    function _verifySignaturesDigest(bytes32 digest, bytes[] memory signatures) internal virtual returns (bool) {
        uint256 numSignatures = signatures.length;

        if (numSignatures == 0) {
            revert ZeroSignature();
        }

        uint256 threshold = getThreshold();

        if (numSignatures < threshold) {
            revert SignatureThresholdNotReached(numSignatures);
        }

        address[] memory recoveredSigners = new address[](numSignatures);
        uint256 uniqueValidCount;
        for (uint256 i = 0; i < numSignatures; i++) {
            address signerRecovered = _recoverSigner(digest, signatures[i]);
            if (!isSigner(signerRecovered)) {
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
     * @notice Sets the threshold for the number of signers required for a signature to be valid.
     */
    function _applyThreshold() internal virtual {
        InputVerifierStorage storage $ = _getInputVerifierStorage();
        uint256 signerLength = $.signers.length;

        if (signerLength != 0) {
            $.threshold = signerLength / 2 + 1;
        } else {
            /// @dev It is impossible to remove all KMS signers.
            revert AtLeastOneSignerIsRequired();
        }
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
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}
}
