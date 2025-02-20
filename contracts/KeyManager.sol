// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

import "./interfaces/IKeyManager.sol";
import "./interfaces/IHTTPZ.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

/// @title Key Manager contract
/// @dev See {IKeyManager}.
contract KeyManager is IKeyManager, Ownable2Step {
    /// @notice The address of the HTTPZ contract for protocol state calls.
    IHTTPZ internal immutable _HTTPZ;

    /// @notice The key generation preprocessing request counter
    uint256 private _preKeygenRequestCounter;
    /// @notice Whether a key generation preprocessing step is done (required for key generation)
    mapping(uint256 preKeyId => bool isDone) private _isPreKeygenDone;
    /// @notice The FHE params used for a key generation preprocessing step
    mapping(uint256 preKeyRequestId => FheParams fheParams) private _preKeygenRequestFheParams;
    /// @notice The FHE params used for a key generation preprocessing step (under the preprocessed key Id)
    mapping(uint256 preKeyId => FheParams fheParams) private _keygenFheParams;
    /// @notice The KMS responses to a key generation preprocessing step
    mapping(uint256 preKeyId => mapping(address kmsConnector => bool hasResponded)) private _preKeygenResponses;
    /// @notice The KMS response counter for a key generation preprocessing step
    mapping(uint256 preKeyId => uint256 responseCounter) private _preKeygenResponseCounter;

    /// @notice The KSK generation preprocessing request counter
    uint256 private _preKskgenRequestCounter;
    /// @notice Whether a KSK generation preprocessing step is done (required for KSK generation)
    mapping(uint256 preKskId => bool isDone) private _isPreKskgenDone;
    /// @notice The FHE params used for a KSK generation preprocessing step
    mapping(uint256 preKskRequestId => FheParams fheParams) private _preKskgenRequestFheParams;
    /// @notice The FHE params used for a KSK generation preprocessing step (under the preprocessed KSK Id)
    mapping(uint256 preKskId => FheParams fheParams) private _kskgenFheParams;
    /// @notice The KMS responses to a KSK generation preprocessing step
    mapping(uint256 preKskId => mapping(address kmsConnector => bool hasResponded)) private _preKskgenResponses;
    /// @notice The KMS response counter for a KSK generation preprocessing step
    mapping(uint256 preKskId => uint256 responseCounter) private _preKskgenResponseCounter;

    /// @notice Whether a key generation is ongoing
    mapping(uint256 preKeyId => bool isOngoing) private _isKeygenOngoing;
    /// @notice The KMS responses to a key generation
    mapping(uint256 keyId => mapping(address kmsConnector => bool hasResponded)) private _keygenResponses;
    /// @notice The KMS response counter for a key generation
    mapping(uint256 keyId => uint256 responseCounter) private _keygenResponseCounter;
    /// @notice If a keyId has been generated
    mapping(uint256 keyId => bool isGenerated) private _isKeyGenerated;

    /// @notice The CRS generation preprocessing counter
    uint256 private _preCrsgenCounter;
    /// @notice The FHE params used for a CRS generation
    mapping(uint256 preCrsId => FheParams fheParams) private _crsgenFheParams;
    /// @notice The KMS responses to a CRS generation
    mapping(uint256 crsId => mapping(address kmsConnector => bool hasResponded)) private _crsgenResponses;
    /// @notice The KMS response counter for a CRS generation
    mapping(uint256 crsId => uint256 responseCounter) private _crsgenResponseCounter;
    /// @notice If a crsId has been generated
    mapping(uint256 crsId => bool isGenerated) private _isCrsGenerated;

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

    /// @notice The current FHE params
    FheParams public fheParams;
    bool private _fheParamsInitialized;

    /// @notice The contract's metadata
    string private constant CONTRACT_NAME = "KeyManager";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    constructor(IHTTPZ httpz) Ownable(msg.sender) {
        _HTTPZ = httpz;
    }

    /// @dev Modifier to check if the FHE params are initialized
    modifier fheParamsInitialized() {
        if (!_fheParamsInitialized) {
            revert FheParamsNotInitialized();
        }
        _;
    }

    /// @notice Checks if the sender is an administrator.
    modifier onlyAdmin() {
        bool isAdmin = _HTTPZ.isAdmin(msg.sender);
        if (!isAdmin) {
            revert InvalidAdminSender(msg.sender);
        }
        _;
    }

    /// @notice Checks if the sender is a KMS node.
    modifier onlyKmsNode() {
        bool isKmsNode = _HTTPZ.isKmsNode(msg.sender);
        if (!isKmsNode) {
            revert InvalidKmsNodeSender(msg.sender);
        }
        _;
    }

    /// @notice Checks if the sender is a Coprocessor.
    modifier onlyCoprocessor() {
        bool isCoprocessor = _HTTPZ.isCoprocessor(msg.sender);
        if (!isCoprocessor) {
            revert InvalidCoprocessorSender(msg.sender);
        }
        _;
    }

    /// @dev See {IKeyManager-preprocessKeygenRequest}.
    function preprocessKeygenRequest() external virtual onlyAdmin fheParamsInitialized {
        /// @dev TODO: maybe generate a preKeyId here instead of on KMS connectors:
        /// @dev https://github.com/zama-ai/gateway-l2/issues/67
        /// @dev Generate a new preKeyRequestId. This is used to track the key generation preprocessing responses
        /// @dev as well as linking FHE params between the preprocessing and the actual key generation
        /// @dev This is different from the preKeyId generated by the KMS connector
        _preKeygenRequestCounter++;
        uint256 preKeyRequestId = _preKeygenRequestCounter;

        /// @dev Store the FHE params used for the key generation preprocessing step as they will be used
        /// @dev for the actual key generation as well
        _preKeygenRequestFheParams[preKeyRequestId] = fheParams;

        emit PreprocessKeygenRequest(preKeyRequestId, fheParams);
    }

    /// @dev See {IKeyManager-preprocessKeygenResponse}.
    function preprocessKeygenResponse(uint256 preKeyRequestId, uint256 preKeyId) external virtual onlyKmsNode {
        /// @dev A KMS node can only respond once
        if (_preKeygenResponses[preKeyRequestId][msg.sender]) {
            revert PreprocessKeygenKmsNodeAlreadyResponded(preKeyRequestId);
        }

        _preKeygenResponses[preKeyRequestId][msg.sender] = true;
        _preKeygenResponseCounter[preKeyRequestId]++;

        /// @dev Store the FHE params under the preprocessing key Id
        _keygenFheParams[preKeyId] = _preKeygenRequestFheParams[preKeyRequestId];

        /// @dev Only send the event if consensus has not been reached in a previous response call
        /// @dev and the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!_isPreKeygenDone[preKeyRequestId] && _isKmsConsensusReached(_preKeygenResponseCounter[preKeyRequestId])) {
            _isPreKeygenDone[preKeyRequestId] = true;

            emit PreprocessKeygenResponse(preKeyRequestId, preKeyId);
        }
    }

    /// @dev See {IKeyManager-preprocessKskgenRequest}.
    function preprocessKskgenRequest() external virtual onlyAdmin fheParamsInitialized {
        /// @dev TODO: maybe generate a preKeyId here instead of on KMS connectors:
        /// @dev https://github.com/zama-ai/gateway-l2/issues/67
        /// @dev Generate a new preKskRequestId. This is used to track the KSK generation preprocessing responses
        /// @dev as well as linking FHE params between the preprocessing and the actual KSK generation
        /// @dev This is different from the preKeyId generated by the KMS connector
        _preKskgenRequestCounter++;
        uint256 preKskRequestId = _preKskgenRequestCounter;

        /// @dev Store the FHE params used for the KSK generation preprocessing step as they will be used
        /// @dev for the actual KSK generation as well
        _preKskgenRequestFheParams[preKskRequestId] = fheParams;

        emit PreprocessKskgenRequest(preKskRequestId, fheParams);
    }

    /// @dev See {IKeyManager-preprocessKskgenResponse}.
    function preprocessKskgenResponse(uint256 preKskRequestId, uint256 preKskId) external virtual onlyKmsNode {
        /// @dev A KMS node can only respond once
        if (_preKskgenResponses[preKskRequestId][msg.sender]) {
            revert PreprocessKskgenKmsNodeAlreadyResponded(preKskRequestId);
        }

        _preKskgenResponses[preKskRequestId][msg.sender] = true;
        _preKskgenResponseCounter[preKskRequestId]++;

        /// @dev Store the FHE params under the preprocessing KSK Id
        _kskgenFheParams[preKskId] = _preKskgenRequestFheParams[preKskRequestId];

        /// @dev Only send the event if consensus has not been reached in a previous response call
        /// @dev and the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!_isPreKskgenDone[preKskRequestId] && _isKmsConsensusReached(_preKskgenResponseCounter[preKskRequestId])) {
            _isPreKskgenDone[preKskRequestId] = true;

            emit PreprocessKskgenResponse(preKskRequestId, preKskId);
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
        emit KeygenRequest(preKeyId, _keygenFheParams[preKeyId]);
    }

    /// @dev See {IKeyManager-keygenResponse}.
    function keygenResponse(uint256 preKeyId, uint256 keyId) external virtual onlyKmsNode {
        /// @dev A KMS node can only respond once
        if (_keygenResponses[keyId][msg.sender]) {
            revert KeygenKmsNodeAlreadyResponded(keyId);
        }

        _keygenResponses[keyId][msg.sender] = true;
        _keygenResponseCounter[keyId]++;

        /// @dev Only send the event if consensus has not been reached in a previous response call
        /// @dev and the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!_isKeyGenerated[keyId] && _isKmsConsensusReached(_keygenResponseCounter[keyId])) {
            _isKeyGenerated[keyId] = true;

            /// @dev Include the FHE params used for the key generation (and its preprocessing) in
            /// @dev the event for better tracking
            emit KeygenResponse(preKeyId, keyId, _keygenFheParams[preKeyId]);
        }
    }

    /// @dev See {IKeyManager-crsgenRequest}.
    function crsgenRequest() external virtual onlyAdmin fheParamsInitialized {
        /// @dev Generate a new preCrsId. This is used to link the FHE params sent in the request
        /// @dev with the ones emitted in the response event
        _preCrsgenCounter++;
        uint256 preCrsId = _preCrsgenCounter;

        /// @dev Store the FHE params to use for the CRS generation
        _crsgenFheParams[preCrsId] = fheParams;

        emit CrsgenRequest(preCrsId, fheParams);
    }

    /// @dev See {IKeyManager-crsgenResponse}.
    function crsgenResponse(uint256 preCrsId, uint256 crsId) external virtual onlyKmsNode {
        /// @dev A KMS node can only respond once
        if (_crsgenResponses[crsId][msg.sender]) {
            revert CrsgenKmsNodeAlreadyResponded(crsId);
        }

        _crsgenResponses[crsId][msg.sender] = true;
        _crsgenResponseCounter[crsId]++;

        /// @dev Only send the event if consensus has not been reached in a previous response call
        /// @dev and the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!_isCrsGenerated[crsId] && _isKmsConsensusReached(_crsgenResponseCounter[crsId])) {
            _isCrsGenerated[crsId] = true;

            /// @dev Include the FHE params used for the CRS generation in the event for better tracking
            emit CrsgenResponse(preCrsId, crsId, _crsgenFheParams[preCrsId]);
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
        emit KskgenRequest(preKskId, sourceKeyId, destKeyId, _kskgenFheParams[preKskId]);
    }

    /// @dev See {IKeyManager-kskgenResponse}.
    function kskgenResponse(uint256 preKskId, uint256 kskId) external virtual onlyKmsNode {
        /// @dev A KMS node can only respond once
        if (_kskgenResponses[kskId][msg.sender]) {
            revert KskgenKmsNodeAlreadyResponded(kskId);
        }

        _kskgenResponses[kskId][msg.sender] = true;
        _kskgenResponseCounter[kskId]++;

        /// @dev Only send the event if consensus has not been reached in a previous response call
        /// @dev and the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!_isKskGenerated[kskId] && _isKmsConsensusReached(_kskgenResponseCounter[kskId])) {
            _isKskGenerated[kskId] = true;

            /// @dev Store the KSK generation ID as a mapping of the source to the destination key ID
            _kskgenIds[_kskgenSourceKeyIds[preKskId]][_kskgenDestKeyIds[preKskId]] = kskId;

            emit KskgenResponse(preKskId, kskId, _kskgenFheParams[preKskId]);
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
    function setFheParams(FheParams memory newFheParams) external virtual onlyOwner {
        if (_fheParamsInitialized) {
            revert FheParamsAlreadyInitialized();
        }

        fheParams = newFheParams;
        _fheParamsInitialized = true;

        emit SetFheParams(newFheParams);
    }

    /// @dev See {IKeyManager-updateFheParams}.
    function updateFheParams(FheParams memory newFheParams) external virtual onlyOwner fheParamsInitialized {
        fheParams = newFheParams;

        emit UpdateFheParams(newFheParams);
    }

    /// @dev See {IKeyManager-isCurrentKeyId}.
    function isCurrentKeyId(uint256 keyId) external view virtual returns (bool) {
        return keyId == currentKeyId;
    }

    function _isKmsConsensusReached(uint256 verifiedSignaturesCount) internal view virtual returns (bool) {
        // TODO: Change this to use threshold value instead:
        // https://github.com/zama-ai/gateway-l2/issues/63
        uint256 consensusThreshold = (_HTTPZ.getKmsNodesCount() - 1) / 3 + 1;
        return verifiedSignaturesCount >= consensusThreshold;
    }

    function _isCoprocessorConsensusReached(uint256 verifiedSignaturesCount) internal view virtual returns (bool) {
        uint256 consensusThreshold = _HTTPZ.getCoprocessorsCount() / 2 + 1;
        return verifiedSignaturesCount >= consensusThreshold;
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
