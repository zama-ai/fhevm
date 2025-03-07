// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./interfaces/IKeyManager.sol";
import "./interfaces/IHTTPZ.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

/// @title Key Manager contract
/// @dev See {IKeyManager}.
contract KeyManager is IKeyManager, Ownable2Step {
    /// @notice The address of the HTTPZ contract for protocol state calls.
    IHTTPZ internal immutable _HTTPZ;

    /// @notice The request counter for all pre-keygen, pre-kskgen and crsgen requests.
    /// @dev The keygen and kskgen requests use the received preKeyId and preKskId respectively.
    uint256 private _requestCounter;
    /// @notice The counter of received KMS responses to a pre-keygen, pre-kskgen or crsgen request.
    mapping(uint256 requestId => uint256 counter) private _responseCounter;

    /// @notice Whether a key generation preprocessing step is done (required for key generation)
    mapping(uint256 preKeyId => bool isDone) private _isPreKeygenDone;
    /// @notice The KMS responses to a key generation preprocessing step
    mapping(uint256 preKeyId => mapping(address kmsConnector => bool hasResponded)) private _preKeygenResponses;
    /// @notice Whether a key generation is ongoing
    mapping(uint256 preKeyId => bool isOngoing) private _isKeygenOngoing;
    /// @notice The KMS responses to a key generation
    mapping(uint256 keyId => mapping(address kmsConnector => bool hasResponded)) private _keygenResponses;
    /// @notice The KMS response counter for a key generation
    mapping(uint256 keyId => uint256 responseCounter) private _keygenResponseCounter;
    /// @notice If a keyId has been generated
    mapping(uint256 keyId => bool isGenerated) private _isKeyGenerated;

    /// @notice Whether a KSK generation preprocessing step is done (required for KSK generation)
    mapping(uint256 preKskId => bool isDone) private _isPreKskgenDone;
    /// @notice The KMS responses to a KSK generation preprocessing step
    mapping(uint256 preKskId => mapping(address kmsConnector => bool hasResponded)) private _preKskgenResponses;
    /// @notice Whether a KSK generation is ongoing
    mapping(uint256 preKskId => bool isOngoing) private _isKskgenOngoing;
    /// @notice The KSK generation source key ID for a KSK generation
    mapping(uint256 preKskId => uint256 sourceKeyId) private _kskgenSourceKeyIds;
    /// @notice The KSK generation destination key ID for a KSK generation
    mapping(uint256 preKskId => uint256 destKeyId) private _kskgenDestKeyIds;
    /// @notice The KMS responses to a KSK generation
    mapping(uint256 kskId => mapping(address kmsConnector => bool hasResponded)) private _kskgenResponses;
    /// @notice The KMS response counter for a KSK generation
    mapping(uint256 kskId => uint256 responseCounter) private _kskgenResponseCounter;
    /// @notice If a kskId has been generated
    mapping(uint256 kskId => bool isGenerated) private _isKskGenerated;
    /// @notice The KSK generation IDs (source keyId => destination keyId => kskId)
    mapping(uint256 sourceKeyId => mapping(uint256 destKeyId => uint256 kskId)) private _kskgenIds;

    /// @notice The KMS responses to a CRS generation
    mapping(uint256 crsId => mapping(address kmsConnector => bool hasResponded)) private _crsgenResponses;
    /// @notice If a crsId has been generated
    mapping(uint256 crsId => bool isGenerated) private _isCrsGenerated;

    /// @notice Whether a key activation is ongoing
    mapping(uint256 keyId => bool isOngoing) private _activateKeyOngoing;
    /// @notice The KMS responses to a key activation
    mapping(uint256 keyId => mapping(address coprocessorConnector => bool hasResponded)) private _activateKeyResponses;
    /// @notice The KMS response counter for a key activation
    mapping(uint256 keyId => uint256 responseCounter) private _activateKeyResponseCounter;

    /// @notice The current (activated) keyId, as there is only one activated key at a time
    uint256 public currentKeyId;
    /// @notice The list of previous activated keyIds, for tracking purposes
    /// @dev The last element should be the current activated keyId
    uint256[] public activatedKeyIds;

    /// @notice The FHE params digest tied to a given FHE params name
    mapping(string fheParamsName => bytes32 paramsDigest) public fheParamsDigests;
    /// @notice The Key ID to its generator FHE params digest
    mapping(uint256 keyId => bytes32 paramsDigest) public keyFheParamsDigests;
    /// @notice The KSK ID to its generator FHE params digest
    mapping(uint256 kskId => bytes32 paramsDigest) public kskFheParamsDigests;
    /// @notice The CRS ID to its generator FHE params digest
    mapping(uint256 crsId => bytes32 paramsDigest) public crsFheParamsDigests;

    /// @notice The preprocessing Key ID to FHE params digest used during keygen
    mapping(uint256 preKeyId => bytes32 paramsDigest) private _preKeyFheParamsDigests;
    /// @notice The keygen request assigned ID to the FHE params digest used during keygen preprocessing
    mapping(uint256 preKeygenRequestId => bytes32 paramsDigest) private _preKeygenFheParamsDigests;
    /// @notice The preprocessing KSK ID to FHE params digest used during kskgen
    mapping(uint256 preKskId => bytes32 paramsDigest) private _preKskFheParamsDigests;
    /// @notice The kskgen request assigned ID to the FHE params digest used during kskgen preprocessing
    mapping(uint256 preKskgenRequestId => bytes32 paramsDigest) private _preKskgenFheParamsDigests;
    /// @notice The crsgen request assigned ID to the FHE params digest used during crsgen
    mapping(uint256 crsgenRequestId => bytes32 paramsDigest) private _crsgenFheParamsDigests;
    /// @notice The fheParamsName mapping for the FHE params initialization status
    mapping(string fheParamsName => bool isInitialized) private _fheParamsInitialized;

    /// @notice The contract's metadata
    string private constant CONTRACT_NAME = "KeyManager";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    constructor(IHTTPZ httpz, string memory fheParamsName, bytes32 fheParamsDigest) Ownable(msg.sender) {
        _HTTPZ = httpz;
        fheParamsDigests[fheParamsName] = fheParamsDigest;
        _fheParamsInitialized[fheParamsName] = true;
    }

    /// @dev Modifier to check if the given FHE params name is initialized
    modifier fheParamsInitialized(string calldata fheParamsName) {
        if (!_fheParamsInitialized[fheParamsName]) {
            revert FheParamsNotInitialized();
        }
        _;
    }

    /// @notice Checks if the sender is an administrator.
    modifier onlyAdmin() {
        _HTTPZ.checkIsAdmin(msg.sender);
        _;
    }

    /// @notice Checks if the sender is a KMS node.
    modifier onlyKmsNode() {
        _HTTPZ.checkIsKmsNode(msg.sender);
        _;
    }

    /// @notice Checks if the sender is a Coprocessor.
    modifier onlyCoprocessor() {
        _HTTPZ.checkIsCoprocessor(msg.sender);
        _;
    }

    /// @dev See {IKeyManager-preprocessKeygenRequest}.
    function preprocessKeygenRequest(
        string calldata fheParamsName
    ) external virtual onlyAdmin fheParamsInitialized(fheParamsName) {
        /// @dev TODO: maybe generate a preKeyId here instead of on KMS connectors:
        /// @dev https://github.com/zama-ai/gateway-l2/issues/67
        /// @dev Generate a new preKeygenRequestId. This is used to track the key generation preprocessing responses
        /// @dev as well as linking FHE params between the preprocessing and the actual key generation
        /// @dev This is different from the preKeyId generated by the KMS connector
        _requestCounter++;
        uint256 preKeygenRequestId = _requestCounter;

        /// @dev Store the FHE params used for the key generation preprocessing step as they will be used
        /// @dev for the actual key generation as well
        bytes32 fheParamsDigest = fheParamsDigests[fheParamsName];
        _preKeygenFheParamsDigests[preKeygenRequestId] = fheParamsDigest;

        emit PreprocessKeygenRequest(preKeygenRequestId, fheParamsDigest);
    }

    /// @dev See {IKeyManager-preprocessKeygenResponse}.
    function preprocessKeygenResponse(uint256 preKeygenRequestId, uint256 preKeyId) external virtual onlyKmsNode {
        /// @dev A KMS node can only respond once
        if (_preKeygenResponses[preKeyId][msg.sender]) {
            revert PreprocessKeygenKmsNodeAlreadyResponded(preKeyId);
        }

        _preKeygenResponses[preKeyId][msg.sender] = true;
        _responseCounter[preKeygenRequestId]++;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!_isPreKeygenDone[preKeyId] && _isKmsConsensusReached(_responseCounter[preKeygenRequestId])) {
            _isPreKeygenDone[preKeyId] = true;

            /// @dev Store the FHE params digest used for the keygen preprocessing
            _preKeyFheParamsDigests[preKeyId] = _preKeygenFheParamsDigests[preKeygenRequestId];

            emit PreprocessKeygenResponse(preKeygenRequestId, preKeyId);
        }
    }

    /// @dev See {IKeyManager-preprocessKskgenRequest}.
    function preprocessKskgenRequest(
        string calldata fheParamsName
    ) external virtual onlyAdmin fheParamsInitialized(fheParamsName) {
        /// @dev TODO: maybe generate a preKeyId here instead of on KMS connectors:
        /// @dev https://github.com/zama-ai/gateway-l2/issues/67
        /// @dev Generate a new preKskRequestId. This is used to track the KSK generation preprocessing responses
        /// @dev as well as linking FHE params between the preprocessing and the actual KSK generation
        /// @dev This is different from the preKeyId generated by the KMS connector
        _requestCounter++;
        uint256 preKskgenRequestId = _requestCounter;

        /// @dev Store the FHE params used for the KSK generation preprocessing step as they will be used
        /// @dev for the actual KSK generation as well
        bytes32 fheParamsDigest = fheParamsDigests[fheParamsName];
        _preKskgenFheParamsDigests[preKskgenRequestId] = fheParamsDigest;

        emit PreprocessKskgenRequest(preKskgenRequestId, fheParamsDigest);
    }

    /// @dev See {IKeyManager-preprocessKskgenResponse}.
    function preprocessKskgenResponse(uint256 preKskgenRequestId, uint256 preKskId) external virtual onlyKmsNode {
        /// @dev A KMS node can only respond once
        if (_preKskgenResponses[preKskId][msg.sender]) {
            revert PreprocessKskgenKmsNodeAlreadyResponded(preKskId);
        }

        _preKskgenResponses[preKskId][msg.sender] = true;
        _responseCounter[preKskgenRequestId]++;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!_isPreKskgenDone[preKskId] && _isKmsConsensusReached(_responseCounter[preKskgenRequestId])) {
            _isPreKskgenDone[preKskId] = true;

            /// @dev Store the FHE params digest used for the kskgen preprocessing
            _preKskFheParamsDigests[preKskId] = _preKskgenFheParamsDigests[preKskgenRequestId];

            emit PreprocessKskgenResponse(preKskgenRequestId, preKskId);
        }
    }

    /// @dev See {IKeyManager-keygenRequest}.
    function keygenRequest(uint256 preKeyId) external virtual onlyAdmin {
        /// @dev A key generation request can only be sent once
        if (_isKeygenOngoing[preKeyId]) {
            revert KeygenRequestAlreadySent(preKeyId);
        }

        /// @dev A key generation requires a key preprocessing step to be completed
        if (!_isPreKeygenDone[preKeyId]) {
            revert KeygenPreprocessingRequired(preKeyId);
        }

        _isKeygenOngoing[preKeyId] = true;

        /// @dev A key generation uses the same FHE params as the one used for the preprocessing step
        emit KeygenRequest(preKeyId, _preKeyFheParamsDigests[preKeyId]);
    }

    /// @dev See {IKeyManager-keygenResponse}.
    function keygenResponse(uint256 preKeyId, uint256 keyId) external virtual onlyKmsNode {
        /// @dev A KMS node can only respond once
        if (_keygenResponses[keyId][msg.sender]) {
            revert KeygenKmsNodeAlreadyResponded(keyId);
        }

        _keygenResponses[keyId][msg.sender] = true;
        _keygenResponseCounter[keyId]++;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!_isKeyGenerated[keyId] && _isKmsConsensusReached(_keygenResponseCounter[keyId])) {
            _isKeyGenerated[keyId] = true;

            /// @dev Store the FHE params digest used during the keygen preprocessing and subsequent keygen
            bytes32 fheParamsDigest = _preKeyFheParamsDigests[preKeyId];
            keyFheParamsDigests[keyId] = fheParamsDigest;

            /// @dev Include the FHE params used for the key generation (and its preprocessing) in
            /// @dev the event for better tracking
            emit KeygenResponse(preKeyId, keyId, fheParamsDigest);
        }
    }

    /// @dev See {IKeyManager-crsgenRequest}.
    function crsgenRequest(
        string calldata fheParamsName
    ) external virtual onlyAdmin fheParamsInitialized(fheParamsName) {
        /// @dev Generate a new crsgenRequestId. This is used to link the FHE params sent in the request
        /// @dev with the ones emitted in the response event
        _requestCounter++;
        uint256 crsgenRequestId = _requestCounter;

        /// @dev Store the FHE params digest to use for the CRS generation
        bytes32 fheParamsDigest = fheParamsDigests[fheParamsName];
        _crsgenFheParamsDigests[crsgenRequestId] = fheParamsDigest;

        emit CrsgenRequest(crsgenRequestId, fheParamsDigest);
    }

    /// @dev See {IKeyManager-crsgenResponse}.
    function crsgenResponse(uint256 crsgenRequestId, uint256 crsId) external virtual onlyKmsNode {
        /// @dev A KMS node can only respond once
        if (_crsgenResponses[crsId][msg.sender]) {
            revert CrsgenKmsNodeAlreadyResponded(crsId);
        }

        _crsgenResponses[crsId][msg.sender] = true;
        _responseCounter[crsgenRequestId]++;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!_isCrsGenerated[crsId] && _isKmsConsensusReached(_responseCounter[crsgenRequestId])) {
            _isCrsGenerated[crsId] = true;

            /// @dev Store the FHE params digest used for the CRS generation
            bytes32 fheParamsDigest = _crsgenFheParamsDigests[crsgenRequestId];
            crsFheParamsDigests[crsId] = fheParamsDigest;

            /// @dev Include the FHE params used for the CRS generation in the event for better tracking
            emit CrsgenResponse(crsgenRequestId, crsId, fheParamsDigest);
        }
    }

    /// @dev See {IKeyManager-kskgenRequest}.
    function kskgenRequest(uint256 preKskId, uint256 sourceKeyId, uint256 destKeyId) external virtual onlyAdmin {
        /// @dev A KSK generation request can only be sent once
        if (_isKskgenOngoing[preKskId]) {
            revert KskgenRequestAlreadySent(preKskId);
        }

        /// @dev A KSK generation requires a KSK preprocessing step to be completed
        if (!_isPreKskgenDone[preKskId]) {
            revert KskgenPreprocessingRequired(preKskId);
        }

        /// @dev The source key must be different from the destination key
        if (sourceKeyId == destKeyId) {
            revert KskgenSameSrcAndDestKeyIds(sourceKeyId);
        }

        /// @dev The source key must be generated
        if (!_isKeyGenerated[sourceKeyId]) {
            revert KskgenSourceKeyNotGenerated(sourceKeyId);
        }

        /// @dev The destination key must be generated
        if (!_isKeyGenerated[destKeyId]) {
            revert KskgenDestKeyNotGenerated(destKeyId);
        }

        _kskgenSourceKeyIds[preKskId] = sourceKeyId;
        _kskgenDestKeyIds[preKskId] = destKeyId;
        _isKskgenOngoing[preKskId] = true;

        /// @dev A KSK generation uses the same FHE params as the one used for the preprocessing step
        emit KskgenRequest(preKskId, sourceKeyId, destKeyId, _preKskFheParamsDigests[preKskId]);
    }

    /// @dev See {IKeyManager-kskgenResponse}.
    function kskgenResponse(uint256 preKskId, uint256 kskId) external virtual onlyKmsNode {
        /// @dev A KMS node can only respond once
        if (_kskgenResponses[kskId][msg.sender]) {
            revert KskgenKmsNodeAlreadyResponded(kskId);
        }

        _kskgenResponses[kskId][msg.sender] = true;
        _kskgenResponseCounter[kskId]++;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!_isKskGenerated[kskId] && _isKmsConsensusReached(_kskgenResponseCounter[kskId])) {
            _isKskGenerated[kskId] = true;

            /// @dev Store the KSK generation ID as a mapping of the source to the destination key ID
            _kskgenIds[_kskgenSourceKeyIds[preKskId]][_kskgenDestKeyIds[preKskId]] = kskId;

            /// @dev Store the FHE params digest used during the kskgen preprocessing and subsequent kskgen
            bytes32 fheParamsDigest = _preKskFheParamsDigests[preKskId];
            kskFheParamsDigests[kskId] = fheParamsDigest;

            emit KskgenResponse(preKskId, kskId, fheParamsDigest);
        }
    }

    /// @dev See {IKeyManager-activateKeyRequest}.
    function activateKeyRequest(uint256 keyId) external virtual onlyAdmin {
        /// @dev A key activation request can only be sent once
        if (_activateKeyOngoing[keyId]) {
            revert ActivateKeyRequestAlreadySent(keyId);
        }

        /// @dev Only a generated key can be activated
        if (!_isKeyGenerated[keyId]) {
            revert ActivateKeyRequiresKeygen(keyId);
        }

        /// @dev Activating a (pending) key requires a KSK from the current (activated) key to this (pending) key
        /// @dev The only exception is for the first key (currentKeyId == 0)
        if (currentKeyId != 0) {
            if (!_isKskGenerated[_kskgenIds[currentKeyId][keyId]]) {
                revert ActivateKeyRequiresKskgen(currentKeyId, keyId);
            }
        }

        _activateKeyOngoing[keyId] = true;

        emit ActivateKeyRequest(keyId);
    }

    /// @dev See {IKeyManager-activateKeyResponse}.
    function activateKeyResponse(uint256 keyId) external virtual onlyCoprocessor {
        /// @dev A coprocessor can only respond once
        if (_activateKeyResponses[keyId][msg.sender]) {
            revert ActivateKeyKmsNodeAlreadyResponded(keyId);
        }

        _activateKeyResponses[keyId][msg.sender] = true;
        _activateKeyResponseCounter[keyId]++;

        /// @dev Only activate the key and send the event if consensus has not been reached in a
        /// @dev previous call (i.e. the key is not the current key yet) and the consensus is
        /// @dev reached in the current call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (currentKeyId != keyId && _isCoprocessorConsensusReached(_activateKeyResponseCounter[keyId])) {
            activatedKeyIds.push(keyId);

            /// @dev The current keyId is considered activated (it is unique)
            currentKeyId = keyId;

            emit ActivateKeyResponse(keyId);
        }
    }

    /// @dev See {IKeyManager-setFheParams}.
    function addFheParams(string calldata fheParamsName, bytes32 fheParamsDigest) external virtual onlyOwner {
        if (_fheParamsInitialized[fheParamsName]) {
            revert FheParamsAlreadyInitialized(fheParamsName);
        }

        fheParamsDigests[fheParamsName] = fheParamsDigest;
        _fheParamsInitialized[fheParamsName] = true;

        emit AddFheParams(fheParamsName, fheParamsDigest);
    }

    /// @dev See {IKeyManager-updateFheParams}.
    function updateFheParams(
        string calldata fheParamsName,
        bytes32 fheParamsDigest
    ) external virtual onlyOwner fheParamsInitialized(fheParamsName) {
        fheParamsDigests[fheParamsName] = fheParamsDigest;

        emit UpdateFheParams(fheParamsName, fheParamsDigest);
    }

    /// @dev See {IKeyManager-isCurrentKeyId}.
    function isCurrentKeyId(uint256 keyId) external view virtual returns (bool) {
        return keyId == currentKeyId;
    }

    /// @notice Checks if the consensus is reached among the KMS nodes.
    /// @dev This function calls the HTTPZ contract to retrieve the consensus threshold.
    /// @param kmsCounter The number of KMS nodes that agreed
    /// @return Whether the consensus is reached
    function _isKmsConsensusReached(uint256 kmsCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = _HTTPZ.getKmsMajorityThreshold();
        return kmsCounter >= consensusThreshold;
    }

    /// @notice Checks if the consensus is reached among the Coprocessors.
    /// @dev This function calls the HTTPZ contract to retrieve the consensus threshold.
    /// @param coprocessorCounter The number of coprocessors that agreed
    /// @return Whether the consensus is reached
    function _isCoprocessorConsensusReached(uint256 coprocessorCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = _HTTPZ.getCoprocessorMajorityThreshold();
        return coprocessorCounter >= consensusThreshold;
    }

    /// @notice Returns the versions of the KeyManager contract in SemVer format.
    /// @dev This is conventionally used for upgrade features.
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
}
