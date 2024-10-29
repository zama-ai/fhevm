// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./KMSVerifier.sol";
import "./TFHEExecutor.sol";
import "../addresses/KMSVerifierAddress.sol";
import "../addresses/CoprocessorAddress.sol";

// Importing OpenZeppelin contracts for cryptographic signature verification and access control.
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";

/// @title InputVerifier for signature verification of users encrypted inputs
/// @notice This version is only for the Coprocessor version of fhEVM
/// @notice This contract is called by the TFHEExecutor inside verifyCiphertext function, and calls the KMSVerifier to fetch KMS signers addresses
/// @dev The contract uses OpenZeppelin's EIP712Upgradeable for cryptographic operations
contract InputVerifier is UUPSUpgradeable, Ownable2StepUpgradeable, EIP712Upgradeable {
    struct CiphertextVerificationForCopro {
        address aclAddress;
        bytes32 hashOfCiphertext;
        uint256[] handlesList;
        address userAddress;
        address contractAddress;
    }

    /// @notice Handle version
    uint8 public constant HANDLE_VERSION = 0;

    KMSVerifier public constant kmsVerifier = KMSVerifier(kmsVerifierAdd);

    /// @notice Name of the contract
    string private constant CONTRACT_NAME = "InputVerifier";

    /// @notice Version of the contract
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /// @notice Getter function for the KMSVerifier contract address
    function getKMSVerifierAddress() public view virtual returns (address) {
        return address(kmsVerifier);
    }

    address private constant coprocessorAddress = coprocessorAdd;
    string public constant CIPHERTEXTVERIFICATION_COPRO_TYPE =
        "CiphertextVerificationForCopro(address aclAddress,bytes32 hashOfCiphertext,uint256[] handlesList,address userAddress,address contractAddress)";
    bytes32 private constant CIPHERTEXTVERIFICATION_COPRO_TYPE_HASH =
        keccak256(bytes(CIPHERTEXTVERIFICATION_COPRO_TYPE));

    function get_CIPHERTEXTVERIFICATION_COPRO_TYPE() public view virtual returns (string memory) {
        return CIPHERTEXTVERIFICATION_COPRO_TYPE;
    }

    /// @notice Getter function for the Coprocessor account address
    function getCoprocessorAddress() public view virtual returns (address) {
        return coprocessorAddress;
    }

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract setting `initialOwner` as the initial owner
    function initialize(address initialOwner) external initializer {
        __Ownable_init(initialOwner);
        __EIP712_init(CONTRACT_NAME, "1");
    }

    function typeOf(uint256 handle) internal pure virtual returns (uint8) {
        uint8 typeCt = uint8(handle >> 8);
        return typeCt;
    }

    function checkProofCache(
        bytes memory inputProof,
        address userAddress,
        address contractAddress,
        address aclAddress
    ) internal view virtual returns (bool, bytes32) {
        bool isProofCached;
        bytes32 key = keccak256(abi.encodePacked(contractAddress, aclAddress, userAddress, inputProof));
        assembly {
            isProofCached := tload(key)
        }
        return (isProofCached, key);
    }

    function cacheProof(bytes32 proofKey) internal virtual {
        assembly {
            tstore(proofKey, 1)
        }
    }

    function verifyCiphertext(
        TFHEExecutor.ContextUserInputs memory context,
        bytes32 inputHandle,
        bytes memory inputProof
    ) external virtual returns (uint256) {
        (bool isProofCached, bytes32 cacheKey) = checkProofCache(
            inputProof,
            context.userAddress,
            context.contractAddress,
            context.aclAddress
        );
        uint256 result = uint256(inputHandle);
        uint256 indexHandle = (result & 0x0000000000000000000000000000000000000000000000000000000000ff0000) >> 16;

        if (!isProofCached) {
            // bundleCiphertext is compressedPackedCT+ZKPOK
            // inputHandle is keccak256(keccak256(bundleCiphertext)+index)[0:29]+index+type+version
            // and inputProof is len(list_handles) + numSignersKMS + hashCT + list_handles + signatureCopro + signatureKMSSigners (1+1+32+NUM_HANDLES*32+65+65*numSignersKMS)

            uint256 inputProofLen = inputProof.length;
            require(inputProofLen > 0, "Empty inputProof");
            uint256 numHandles = uint256(uint8(inputProof[0]));
            uint256 numSignersKMS = uint256(uint8(inputProof[1]));

            require(numHandles > indexHandle, "Invalid index"); // @note: this checks in particular that the list is non-empty
            require(inputProofLen == 99 + 32 * numHandles + 65 * numSignersKMS, "Error deserializing inputProof");

            bytes32 hashCT;
            assembly {
                hashCT := mload(add(inputProof, 34))
            }

            // deseralize handle and check they are from correct version
            uint256[] memory listHandles = new uint256[](numHandles);
            for (uint256 i = 0; i < numHandles; i++) {
                uint256 element;
                assembly {
                    element := mload(add(inputProof, add(66, mul(i, 32))))
                }
                // check all handles are from correct version
                require(uint8(element) == HANDLE_VERSION, "Wrong handle version");
                listHandles[i] = element;
            }

            {
                bytes memory signatureCoproc = new bytes(65);
                for (uint256 i = 0; i < 65; i++) {
                    signatureCoproc[i] = inputProof[34 + 32 * numHandles + i];
                }
                CiphertextVerificationForCopro memory cvCopro;
                cvCopro.aclAddress = context.aclAddress;
                cvCopro.hashOfCiphertext = hashCT;
                cvCopro.handlesList = listHandles;
                cvCopro.userAddress = context.userAddress;
                cvCopro.contractAddress = context.contractAddress;
                verifyEIP712Copro(cvCopro, signatureCoproc);
            }
            {
                bytes[] memory signaturesKMS = new bytes[](numSignersKMS);
                for (uint256 j = 0; j < numSignersKMS; j++) {
                    signaturesKMS[j] = new bytes(65);
                    for (uint256 i = 0; i < 65; i++) {
                        signaturesKMS[j][i] = inputProof[99 + 32 * numHandles + 65 * j + i];
                    }
                }
                KMSVerifier.CiphertextVerificationForKMS memory cvKMS;
                cvKMS.aclAddress = context.aclAddress;
                cvKMS.hashOfCiphertext = hashCT;
                cvKMS.userAddress = context.userAddress;
                cvKMS.contractAddress = context.contractAddress;
                bool kmsCheck = kmsVerifier.verifyInputEIP712KMSSignatures(cvKMS, signaturesKMS);
                require(kmsCheck, "Not enough unique KMS input signatures");
            }
            cacheProof(cacheKey);
            require(result == listHandles[indexHandle], "Wrong inputHandle");
        } else {
            uint8 numHandles = uint8(inputProof[0]); // @note: we know inputProof is non-empty since it has been previously cached
            require(numHandles > indexHandle, "Invalid index");
            uint256 element;
            for (uint256 j = 0; j < 32; j++) {
                element |= uint256(uint8(inputProof[34 + indexHandle * 32 + j])) << (8 * (31 - j));
            }
            require(element == result, "Wrong inputHandle");
        }
        return result;
    }

    function verifyEIP712Copro(CiphertextVerificationForCopro memory cv, bytes memory signature) internal view virtual {
        bytes32 digest = hashCiphertextVerificationForCopro(cv);
        address signer = ECDSA.recover(digest, signature);
        require(signer == coprocessorAddress, "Coprocessor address mismatch");
    }

    function hashCiphertextVerificationForCopro(
        CiphertextVerificationForCopro memory CVcopro
    ) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        CIPHERTEXTVERIFICATION_COPRO_TYPE_HASH,
                        CVcopro.aclAddress,
                        CVcopro.hashOfCiphertext,
                        keccak256(abi.encodePacked(CVcopro.handlesList)),
                        CVcopro.userAddress,
                        CVcopro.contractAddress
                    )
                )
            );
    }

    /// @notice recovers the signer's address from a `signature` and a `message` digest
    /// @dev Utilizes ECDSA for actual address recovery
    /// @param message The hash of the message that was signed
    /// @param signature The signature to verify
    /// @return signer The address that supposedly signed the message
    function recoverSigner(bytes32 message, bytes memory signature) internal pure virtual returns (address) {
        address signerRecovered = ECDSA.recover(message, signature);
        return signerRecovered;
    }

    /// @notice Getter for the name and version of the contract
    /// @return string representing the name and the version of the contract
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
}
