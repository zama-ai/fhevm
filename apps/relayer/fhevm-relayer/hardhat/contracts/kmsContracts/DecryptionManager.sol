// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;
import {IDecryptionManager} from "./interfaces/IDecryptionManager.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import {EIP712} from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

contract DecryptionManager is Ownable2Step, EIP712 {
    mapping(address => bool) internal isSigner;
    mapping(uint256 publicDecryptionId => uint256[] handlesList) internal requestedPublicHandles;
    mapping(uint256 publicDecryptionId => mapping(address signer => bool alreadySigned)) internal alreadySignedPublic; // there is an edge case which is hard to deal with if we change the signers set in the middle of pending signatures being sent, but maybe we can ignore it for the moment
    mapping(uint256 publicDecryptionId => mapping(bytes32 digest => bytes[] pendingSignatures))
        internal pendingSignaturesPublic;
    address[] internal signers;
    uint256 internal threshold;
    uint256 internal counterPublicDecryption;

    struct PublicDecryptionResult {
        uint256[] handlesList;
        bytes decryptedResult;
    }

    event SignerAdded(address indexed signer);

    event SignerRemoved(address indexed signer);

    event PublicDecryptionRequest(uint256 indexed publicDecryptionId, uint256[] ciphertextHandles);

    event PublicDecryptionResponse(uint256 indexed publicDecryptionId, bytes decryptedResult, bytes[] signatures);

    error KMSAlreadySigner();

    error AtLeastOneSignerIsRequired();

    error KMSInvalidSigner(address invalidSigner);

    error KMSNotASigner();

    error KMSSignerNull();

    error KMSSignerAlreadySigned(uint256 publicDecryptionId, address signer);

    error PublicDecryptionAlreadyDone();

    string public constant PUBLIC_DECRYPTION_RESULT_TYPE =
        "PublicDecryptionResult(uint256[] handlesList,bytes decryptedResult)";

    bytes32 public constant PUBLIC_DECRYPTION_RESULT_TYPEHASH = keccak256(bytes(PUBLIC_DECRYPTION_RESULT_TYPE));

    string private constant CONTRACT_NAME = "DecryptionManager";

    uint256 private constant MAJOR_VERSION = 0;

    uint256 private constant MINOR_VERSION = 1;

    uint256 private constant PATCH_VERSION = 0;

    constructor() Ownable(msg.sender) EIP712(CONTRACT_NAME, "1") {}

    function addSigner(address signer) public virtual onlyOwner {
        if (signer == address(0)) {
            revert KMSSignerNull();
        }
        if (isSigner[signer]) {
            revert KMSAlreadySigner();
        }
        isSigner[signer] = true;
        signers.push(signer);
        _applyThreshold();
        emit SignerAdded(signer);
    }

    function removeSigner(address signer) public virtual onlyOwner {
        if (!isSigner[signer]) {
            revert KMSNotASigner();
        }
        isSigner[signer] = false;

        uint256 signerLength = signers.length;
        for (uint i = 0; i < signerLength; i++) {
            if (signers[i] == signer) {
                signers[i] = signers[signerLength - 1];
                signers.pop();
                _applyThreshold();
                emit SignerRemoved(signer);
                return;
            }
        }
    }

    function publicDecryptionRequest(uint256[] calldata ciphertextHandles) public virtual {
        uint256 publicDecryptionId = counterPublicDecryption;
        emit PublicDecryptionRequest(publicDecryptionId, ciphertextHandles);
        requestedPublicHandles[publicDecryptionId] = ciphertextHandles;
        counterPublicDecryption++;
    }

    function publicDecryptionResponse(
        uint256 publicDecryptionId,
        bytes calldata decryptedResult,
        bytes calldata signature
    ) public virtual {
        PublicDecryptionResult memory decRes;
        decRes.handlesList = requestedPublicHandles[publicDecryptionId];
        decRes.decryptedResult = decryptedResult;
        bytes32 digest = _hashPublicDecryptionResult(decRes);
        bool isDoneBefore = isPublicDecryptionDone(publicDecryptionId, digest);
        address signerRecovered = _recoverSigner(digest, signature);
        if (!isSigner[signerRecovered]) {
            revert KMSInvalidSigner(signerRecovered);
        }
        if (alreadySignedPublic[publicDecryptionId][signerRecovered]) {
            revert KMSSignerAlreadySigned(publicDecryptionId, signerRecovered);
        }
        alreadySignedPublic[publicDecryptionId][signerRecovered] = true;
        pendingSignaturesPublic[publicDecryptionId][digest].push(signature);
        bytes[] memory pendingSignaturesArray = pendingSignaturesPublic[publicDecryptionId][digest];
        bool isDoneAfter = isPublicDecryptionDone(publicDecryptionId, digest);
        if (isDoneAfter && (!isDoneBefore)) {
            emit PublicDecryptionResponse(publicDecryptionId, decryptedResult, pendingSignaturesArray);
        }
    }

    function isPublicDecryptionDone(uint256 publicDecryptionId, bytes32 digest) public virtual returns(bool) {
        bytes[] memory pendingSignaturesArray = pendingSignaturesPublic[publicDecryptionId][digest];
        bool isDone = pendingSignaturesArray.length >= getThreshold();
        return isDone;
    }

    function getSigners() public view virtual returns (address[] memory) {
        return signers;
    }

    function getThreshold() public view virtual returns (uint256) {
        return threshold;
    }

    function getVersion() public pure virtual returns (string memory) {
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

    function _applyThreshold() internal virtual {
        uint256 signerLength = signers.length;

        if (signerLength != 0) {
            threshold = (signerLength - 1) / 3 + 1;
        } else {
            revert AtLeastOneSignerIsRequired();
        }
    }

    function _hashPublicDecryptionResult(PublicDecryptionResult memory decRes) internal view virtual returns (bytes32) {
        return
            _hashTypedDataV4(
                keccak256(
                    abi.encode(
                        PUBLIC_DECRYPTION_RESULT_TYPEHASH,
                        keccak256(abi.encodePacked(decRes.handlesList)),
                        keccak256(decRes.decryptedResult)
                    )
                )
            );
    }

    function _recoverSigner(bytes32 message, bytes memory signature) internal pure virtual returns (address) {
        address signerRecovered = ECDSA.recover(message, signature);
        return signerRecovered;
    }
}
