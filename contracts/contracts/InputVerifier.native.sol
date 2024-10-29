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

/// @title InputVerifier for signature verification of users encrypted inputs
/// @notice This version is only for the Native version of fhEVM
/// @notice This contract is called by the TFHEExecutor inside verifyCiphertext function, and calls the KMSVerifier to fetch KMS signers addresses
contract InputVerifier is UUPSUpgradeable, Ownable2StepUpgradeable {
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

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract setting `initialOwner` as the initial owner
    function initialize(address initialOwner) external initializer {
        __Ownable_init(initialOwner);
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
            // and inputProof is len(list_handles) + numSignersKMS + list_handles + signatureKMSSigners + bundleCiphertext (1+1+NUM_HANDLES*32+65*numSignersKMS+bundleCiphertext.length)

            uint256 inputProofLen = inputProof.length;
            require(inputProofLen > 0, "Empty inputProof");
            uint256 numHandles = uint256(uint8(inputProof[0]));
            uint256 numSignersKMS = uint256(uint8(inputProof[1]));

            require(numHandles > indexHandle, "Invalid index"); // @note: this checks in particular that the list is non-empty
            // @note: on native if an invalid indexHandle above the "real" numHandles is passed, it will be mapped to a trivialEncrypt(0) by backend

            require(inputProofLen > 2 + 32 * numHandles + 65 * numSignersKMS, "Error deserializing inputProof");

            bytes32 hashCT;
            {
                uint256 prefixLength = 2 + 32 * numHandles + 65 * numSignersKMS;
                uint256 bundleCiphertextLength = inputProofLen - prefixLength;
                bytes memory bundleCiphertext = new bytes(bundleCiphertextLength);
                for (uint256 i = 0; i < bundleCiphertextLength; i++) {
                    bundleCiphertext[i] = inputProof[prefixLength + i];
                }
                hashCT = keccak256(bundleCiphertext);
            }

            {
                bytes[] memory signaturesKMS = new bytes[](numSignersKMS);
                for (uint256 j = 0; j < numSignersKMS; j++) {
                    signaturesKMS[j] = new bytes(65);
                    for (uint256 i = 0; i < 65; i++) {
                        signaturesKMS[j][i] = inputProof[2 + 32 * numHandles + 65 * j + i];
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

            // deseralize handle and check they are from correct version and correct values (handles are recomputed onchain in native case)
            for (uint256 i = 0; i < numHandles; i++) {
                uint256 element;
                assembly {
                    element := mload(add(inputProof, add(34, mul(i, 32))))
                }
                // check all handles are from correct version
                require(uint8(element) == HANDLE_VERSION, "Wrong handle version");
                uint256 indexElement = (element & 0x0000000000000000000000000000000000000000000000000000000000ff0000) >>
                    16;
                require(indexElement == i, "Wrong index for serialized handle");

                uint256 recomputedHandle = uint256(keccak256(abi.encodePacked(hashCT, uint8(i))));
                require(
                    (recomputedHandle & 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000) ==
                        (element & 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000),
                    "Wrong handle in inputProof"
                ); // @note only the before last byte corresponding to type, ie element[30] could not be checked, i.e on native type is malleable, this means it will be casted accordingly by the backend (or trivialEncrypt(0) if index is invalid)
                if (i == indexHandle) {
                    require(result == element, "Wrong inputHandle");
                }
            }

            cacheProof(cacheKey);
        } else {
            uint8 numHandles = uint8(inputProof[0]); // @note: we know inputProof is non-empty since it has been previously cached
            require(numHandles > indexHandle, "Invalid index");
            uint256 element;
            for (uint256 j = 0; j < 32; j++) {
                element |= uint256(uint8(inputProof[2 + indexHandle * 32 + j])) << (8 * (31 - j));
            }
            require(element == result, "Wrong inputHandle");
        }
        return result;
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
