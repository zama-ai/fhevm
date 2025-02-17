// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.28;

import "./interfaces/IHTTPZ.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

/// @title HTTPZ contract
/// @dev See {IHTTPZ}.
contract HTTPZ is IHTTPZ, Ownable2Step, AccessControl {
    /// @notice The protocol's metadata
    ProtocolMetadata public protocolMetadata;

    /// @notice The admin role. Only admins can add KMS nodes, coprocessors and networks, as well
    /// @notice as trigger FHE key, CRS and KSK generations
    bytes32 public constant ADMIN_ROLE = keccak256("ADMIN_ROLE");

    /// @notice The KMS nodes' metadata
    KmsNode[] public kmsNodes;
    /// @notice The KMS nodes' identities (public signature keys)
    bytes[] public kmsNodeIdentities;
    /// @notice The KMS nodes' signed nodes
    mapping(address kmsNodeConnector => bytes signedNodes) public kmsNodeSignedNodes;
    /// @notice The keychain DA addresses (one per KMS node)
    mapping(address kmsNodeConnector => address keychainDaAddress) public keychainDaAddresses;
    /// @notice The number of KMS nodes that are marked as ready
    uint256 private _kmsNodeReadyCounter;
    /// @notice Whether the KMS service is ready (all KMS nodes have been added and marked as ready)
    bool private _kmsServiceReady;
    /// @notice The pending KMS node role. Only pending KMS nodes can mark KMS nodes as ready
    bytes32 public constant PENDING_KMS_NODE_ROLE = keccak256("PENDING_KMS_NODE_ROLE");
    /// @notice The KMS node role. Only KMS nodes can respond to FHE key, CRS and KSK generations
    bytes32 public constant KMS_NODE_ROLE = keccak256("KMS_NODE_ROLE");

    /// @notice The coprocessors' metadata
    Coprocessor[] public coprocessors;
    /// @notice The coprocessors' identities (public signature keys)
    bytes[] public coprocessorIdentities;
    /// @notice The coprocessor DA addresses (one per coprocessor)
    mapping(address coprocessorConnector => address coprocessorDaAddress) public coprocessorDaAddresses;
    /// @notice The number of coprocessors that are marked as ready
    uint256 private _coprocessorReadyCounter;
    /// @notice Whether the the coprocessor service is ready (all coprocessors have been added and marked as ready)
    bool private _coprocessorServiceReady;
    /// @notice The pending coprocessor role. Only pending coprocessors can mark coprocessors as ready
    bytes32 public constant PENDING_COPROCESSOR_ROLE = keccak256("PENDING_COPROCESSOR_ROLE");
    /// @notice The coprocessor role. Only coprocessors can respond to key activation
    bytes32 public constant COPROCESSOR_ROLE = keccak256("COPROCESSOR_ROLE");

    /// @notice The networks' metadata
    Network[] public networks;
    /// @notice The networks' registered status
    mapping(uint256 chainId => bool isRegistered) private _isNetworkRegistered;

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
    string private constant CONTRACT_NAME = "HTTPZ";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    constructor() Ownable(msg.sender) {}

    /// @dev Modifier to check if the KMS service is ready
    modifier kmsServiceReady() {
        if (!_kmsServiceReady) {
            revert KmsNodesNotSet();
        }
        _;
    }
    /// @dev Modifier to check if the coprocessor service is ready
    modifier coprocessorServiceReady() {
        if (!_coprocessorServiceReady) {
            revert CoprocessorsNotSet();
        }
        _;
    }
    /// @dev Modifier to check if the FHE params are initialized
    modifier fheParamsInitialized() {
        if (!_fheParamsInitialized) {
            revert FheParamsNotInitialized();
        }
        _;
    }

    /// @dev See {IHTTPZ-initialize}.
    function initialize(
        ProtocolMetadata calldata initialProtocolMetadata,
        address[] calldata admins
    ) external virtual onlyOwner {
        protocolMetadata = initialProtocolMetadata;

        for (uint256 i = 0; i < admins.length; i++) {
            _grantRole(ADMIN_ROLE, admins[i]);
        }

        emit Initialization(protocolMetadata, admins);
    }

    /// @dev See {IHTTPZ-addKmsNodes}.
    function addKmsNodes(KmsNode[] calldata initialKmsNodes) external virtual onlyRole(ADMIN_ROLE) {
        for (uint256 i = 0; i < initialKmsNodes.length; i++) {
            _grantRole(PENDING_KMS_NODE_ROLE, initialKmsNodes[i].connectorAddress);

            kmsNodes.push(initialKmsNodes[i]);
            kmsNodeIdentities.push(initialKmsNodes[i].identity);
        }

        emit KmsNodesInit(kmsNodeIdentities);
    }

    /// @dev See {IHTTPZ-kmsNodeReady}.
    function kmsNodeReady(
        bytes calldata signedNodes,
        address keychainDaAddress
    ) external virtual onlyRole(PENDING_KMS_NODE_ROLE) {
        _grantRole(KMS_NODE_ROLE, msg.sender);

        /// @dev A KMS node can only be ready once
        _revokeRole(PENDING_KMS_NODE_ROLE, msg.sender);

        kmsNodeSignedNodes[msg.sender] = signedNodes;
        keychainDaAddresses[msg.sender] = keychainDaAddress;
        _kmsNodeReadyCounter++;

        /// @dev Emit the event when all KMS nodes are ready
        if (_kmsNodeReadyCounter == kmsNodes.length) {
            _kmsServiceReady = true;

            emit KmsServiceReady(kmsNodeIdentities);
        }
    }

    /// @dev See {IHTTPZ-addCoprocessors}.
    function addCoprocessors(Coprocessor[] calldata initialCoprocessors) external virtual onlyRole(ADMIN_ROLE) {
        for (uint256 i = 0; i < initialCoprocessors.length; i++) {
            _grantRole(PENDING_COPROCESSOR_ROLE, initialCoprocessors[i].connectorAddress);
            coprocessors.push(initialCoprocessors[i]);
            coprocessorIdentities.push(initialCoprocessors[i].identity);
        }

        emit CoprocessorsInit(coprocessorIdentities);
    }

    /// @dev See {IHTTPZ-coprocessorReady}.
    function coprocessorReady(address coprocessorDaAddress) external virtual onlyRole(PENDING_COPROCESSOR_ROLE) {
        _grantRole(COPROCESSOR_ROLE, msg.sender);

        /// @dev A coprocessor can only be ready once
        _revokeRole(PENDING_COPROCESSOR_ROLE, msg.sender);

        coprocessorDaAddresses[msg.sender] = coprocessorDaAddress;
        _coprocessorReadyCounter++;

        /// @dev Emit the event when all coprocessors are ready
        if (_coprocessorReadyCounter == coprocessors.length) {
            _coprocessorServiceReady = true;

            emit CoprocessorServiceReady(coprocessorIdentities);
        }
    }

    /// @dev See {IHTTPZ-addNetwork}.
    function addNetwork(Network calldata network) external virtual onlyRole(ADMIN_ROLE) {
        networks.push(network);
        _isNetworkRegistered[network.chainId] = true;

        emit AddNetwork(network.chainId);
    }

    /// @dev See {IHTTPZ-preprocessKeygenRequest}.
    function preprocessKeygenRequest() external virtual onlyRole(ADMIN_ROLE) kmsServiceReady fheParamsInitialized {
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

    /// @dev See {IHTTPZ-preprocessKeygenResponse}.
    function preprocessKeygenResponse(
        uint256 preKeyRequestId,
        uint256 preKeyId
    ) external virtual onlyRole(KMS_NODE_ROLE) {
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

    /// @dev See {IHTTPZ-preprocessKskgenRequest}.
    function preprocessKskgenRequest() external virtual onlyRole(ADMIN_ROLE) kmsServiceReady fheParamsInitialized {
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

    /// @dev See {IHTTPZ-preprocessKskgenResponse}.
    function preprocessKskgenResponse(
        uint256 preKskRequestId,
        uint256 preKskId
    ) external virtual onlyRole(KMS_NODE_ROLE) {
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

    /// @dev See {IHTTPZ-keygenRequest}.
    function keygenRequest(uint256 preKeyId) external virtual onlyRole(ADMIN_ROLE) {
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

    /// @dev See {IHTTPZ-keygenResponse}.
    function keygenResponse(uint256 preKeyId, uint256 keyId) external virtual onlyRole(KMS_NODE_ROLE) {
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

    /// @dev See {IHTTPZ-crsgenRequest}.
    function crsgenRequest() external virtual onlyRole(ADMIN_ROLE) kmsServiceReady fheParamsInitialized {
        /// @dev Generate a new preCrsId. This is used to link the FHE params sent in the request
        /// @dev with the ones emitted in the response event
        _preCrsgenCounter++;
        uint256 preCrsId = _preCrsgenCounter;

        /// @dev Store the FHE params to use for the CRS generation
        _crsgenFheParams[preCrsId] = fheParams;

        emit CrsgenRequest(preCrsId, fheParams);
    }

    /// @dev See {IHTTPZ-crsgenResponse}.
    function crsgenResponse(uint256 preCrsId, uint256 crsId) external virtual onlyRole(KMS_NODE_ROLE) {
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

    /// @dev See {IHTTPZ-kskgenRequest}.
    function kskgenRequest(
        uint256 preKskId,
        uint256 sourceKeyId,
        uint256 destKeyId
    ) external virtual onlyRole(ADMIN_ROLE) {
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

    /// @dev See {IHTTPZ-kskgenResponse}.
    function kskgenResponse(uint256 preKskId, uint256 kskId) external virtual onlyRole(KMS_NODE_ROLE) {
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

    /// @dev See {IHTTPZ-activateKeyRequest}.
    function activateKeyRequest(uint256 keyId) external virtual onlyRole(ADMIN_ROLE) coprocessorServiceReady {
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

    /// @dev See {IHTTPZ-activateKeyResponse}.
    function activateKeyResponse(uint256 keyId) external virtual onlyRole(COPROCESSOR_ROLE) {
        /// @dev A coprocessor can only respond once
        if (_activateKeyResponses[keyId][msg.sender]) {
            revert ActivateKeyKmsNodeAlreadyResponded(keyId);
        }

        _activateKeyResponses[keyId][msg.sender] = true;
        _activateKeyResponseCounter[keyId]++;

        // TODO: Check if this threshold is correct
        /// @dev Activate the key once all coprocessors have responded
        if (_activateKeyResponseCounter[keyId] == coprocessors.length) {
            activatedKeyIds.push(keyId);

            /// @dev The current keyId is considered activated (it is unique)
            currentKeyId = keyId;

            emit ActivateKeyResponse(keyId);
        }
    }

    /// @dev See {IHTTPZ-setFheParams}.
    function setFheParams(FheParams memory newFheParams) external virtual onlyOwner {
        if (_fheParamsInitialized) {
            revert FheParamsAlreadyInitialized();
        }

        fheParams = newFheParams;
        _fheParamsInitialized = true;

        emit SetFheParams(newFheParams);
    }

    /// @dev See {IHTTPZ-updateFheParams}.
    function updateFheParams(FheParams memory newFheParams) external virtual onlyOwner fheParamsInitialized {
        fheParams = newFheParams;

        emit UpdateFheParams(newFheParams);
    }

    /// @dev See {IHTTPZ-isKmsNode}.
    function isKmsNode(address kmsNodeAddress) external view virtual returns (bool) {
        return hasRole(KMS_NODE_ROLE, kmsNodeAddress);
    }

    /// @dev See {IHTTPZ-isCoprocessor}.
    function isCoprocessor(address coprocessorAddress) external view virtual returns (bool) {
        return hasRole(COPROCESSOR_ROLE, coprocessorAddress);
    }

    /// @dev See {IHTTPZ-isNetwork}.
    function isNetwork(uint256 chainId) external view virtual returns (bool) {
        return _isNetworkRegistered[chainId];
    }

    /// @dev See {IHTTPZ-isCurrentKeyId}.
    function isCurrentKeyId(uint256 keyId) external view virtual returns (bool) {
        return keyId == currentKeyId;
    }

    /// @dev See {IHTTPZ-getKmsNodesCount}.
    function getKmsNodesCount() public view virtual returns (uint256) {
        return kmsNodes.length;
    }

    /// @dev See {IHTTPZ-getCoprocessorsCount}.
    function getCoprocessorsCount() external view virtual returns (uint256) {
        return coprocessors.length;
    }

    function _isKmsConsensusReached(uint256 verifiedSignaturesCount) internal view virtual returns (bool) {
        // TODO: Change this to use threshold value instead:
        // https://github.com/zama-ai/gateway-l2/issues/63
        uint256 consensusThreshold = (getKmsNodesCount() - 1) / 3 + 1;
        return verifiedSignaturesCount >= consensusThreshold;
    }

    /// @notice Returns the versions of the HTTPZ contract in SemVer format.
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
