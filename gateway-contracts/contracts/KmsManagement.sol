// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./interfaces/IKmsManagement.sol";
import "./interfaces/IGatewayConfig.sol";
import { gatewayConfigAddress } from "../addresses/GatewayConfigAddress.sol";
import { Ownable2StepUpgradeable } from "@openzeppelin/contracts-upgradeable/access/Ownable2StepUpgradeable.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";
import "./shared/UUPSUpgradeableEmptyProxy.sol";
import "./shared/GatewayConfigChecks.sol";
import "./shared/Pausable.sol";

/// @title KMS Management contract
/// @dev TODO: This contract is neither used nor up-to-date. It will be reworked in the future.
/// @dev See https://github.com/zama-ai/fhevm-gateway/issues/108
/// @dev See {IKmsManagement}.
contract KmsManagement is
    IKmsManagement,
    Ownable2StepUpgradeable,
    UUPSUpgradeableEmptyProxy,
    GatewayConfigChecks,
    Pausable
{
    /// @notice The address of the GatewayConfig contract for protocol state calls.
    IGatewayConfig private constant GATEWAY_CONFIG = IGatewayConfig(gatewayConfigAddress);

    /// @dev The following constants are used for versioning the contract. They are made private
    /// @dev in order to force derived contracts to consider a different version. Note that
    /// @dev they can still define their own private constants with the same name.
    string private constant CONTRACT_NAME = "KmsManagement";
    uint256 private constant MAJOR_VERSION = 0;
    uint256 private constant MINOR_VERSION = 1;
    uint256 private constant PATCH_VERSION = 0;

    /// @notice The contract's variable storage struct (@dev see ERC-7201)
    /// @custom:storage-location erc7201:fhevm_gateway.storage.KmsManagement
    struct KmsManagementStorage {
        /// @notice The FHE params digest tied to a given FHE params name
        mapping(string fheParamsName => bytes32 paramsDigest) fheParamsDigests;
        /// @notice The number of pre-keygen requests, used to generate the preKeygenRequestIds.
        uint256 _preKeygenRequestCounter;
        /// @notice The counter of received KMS responses to a pre-keygen request.
        mapping(uint256 preKeygenRequestId => uint256 counter) _preKeygenResponseCounter;
        /// @notice Whether a key generation preprocessing step is done (required for key generation)
        mapping(uint256 preKeyId => bool isDone) _isPreKeygenDone;
        /// @notice The KMS responses to a key generation preprocessing step
        mapping(uint256 preKeyId => mapping(address kmsConnector => bool hasResponded)) _preKeygenResponses;
        /// @notice Whether a key generation is ongoing
        mapping(uint256 preKeyId => bool isOngoing) _isKeygenOngoing;
        /// @notice The KMS responses to a key generation
        mapping(uint256 keyId => mapping(address kmsConnector => bool hasResponded)) _keygenResponses;
        /// @notice The KMS response counter for a key generation
        mapping(uint256 keyId => uint256 responseCounter) _keygenResponseCounter;
        /// @notice If a keyId has been generated
        mapping(uint256 keyId => bool isGenerated) _isKeyGenerated;
        /// @notice The number of pre-kskgen requests, used to generate the preKskgenRequestIds.
        uint256 _preKskgenRequestCounter;
        /// @notice The counter of received KMS responses to a pre-kskgen request.
        mapping(uint256 preKskgenRequestId => uint256 counter) _preKskgenResponseCounter;
        /// @notice Whether a KSK generation preprocessing step is done (required for KSK generation)
        mapping(uint256 preKskId => bool isDone) _isPreKskgenDone;
        /// @notice The KMS responses to a KSK generation preprocessing step
        mapping(uint256 preKskId => mapping(address kmsConnector => bool hasResponded)) _preKskgenResponses;
        /// @notice Whether a KSK generation is ongoing
        mapping(uint256 preKskId => bool isOngoing) _isKskgenOngoing;
        /// @notice The KSK generation source key ID for a KSK generation
        mapping(uint256 preKskId => uint256 sourceKeyId) _kskgenSourceKeyIds;
        /// @notice The KSK generation destination key ID for a KSK generation
        mapping(uint256 preKskId => uint256 destKeyId) _kskgenDestKeyIds;
        /// @notice The KMS responses to a KSK generation
        mapping(uint256 kskId => mapping(address kmsConnector => bool hasResponded)) _kskgenResponses;
        /// @notice The KMS response counter for a KSK generation
        mapping(uint256 kskId => uint256 responseCounter) _kskgenResponseCounter;
        /// @notice If a kskId has been generated
        mapping(uint256 kskId => bool isGenerated) _isKskGenerated;
        /// @notice The KSK generation IDs (source keyId => destination keyId => kskId)
        mapping(uint256 sourceKeyId => mapping(uint256 destKeyId => uint256 kskId)) _kskgenIds;
        /// @notice The number of crsgen requests, used to generate the crsgenRequestIds.
        uint256 _crsgenRequestCounter;
        /// @notice The counter of received KMS responses to a crsgen request.
        mapping(uint256 crsgenRequestId => uint256 counter) _crsgenResponseCounter;
        /// @notice The KMS responses to a CRS generation
        mapping(uint256 crsId => mapping(address kmsConnector => bool hasResponded)) _crsgenResponses;
        /// @notice If a crsId has been generated
        mapping(uint256 crsId => bool isGenerated) _isCrsGenerated;
        /// @notice Whether a key activation is ongoing
        mapping(uint256 keyId => bool isOngoing) _activateKeyOngoing;
        /// @notice The KMS responses to a key activation
        mapping(uint256 keyId => mapping(address coprocessorConnector => bool hasResponded)) _activateKeyResponses;
        /// @notice The KMS response counter for a key activation
        mapping(uint256 keyId => uint256 responseCounter) _activateKeyResponseCounter;
        /// @notice The current (activated) keyId, as there is only one activated key at a time
        uint256 currentKeyId;
        /// @notice The list of previous activated keyIds, for tracking purposes
        /// @dev The last element should be the current activated keyId
        uint256[] activatedKeyIds;
        /// @notice The Key ID to its generator FHE params digest
        mapping(uint256 keyId => bytes32 paramsDigest) keyFheParamsDigests;
        /// @notice The KSK ID to its generator FHE params digest
        mapping(uint256 kskId => bytes32 paramsDigest) kskFheParamsDigests;
        /// @notice The CRS ID to its generator FHE params digest
        mapping(uint256 crsId => bytes32 paramsDigest) crsFheParamsDigests;
        /// @notice The preprocessing Key ID to FHE params digest used during keygen
        mapping(uint256 preKeyId => bytes32 paramsDigest) _preKeyFheParamsDigests;
        /// @notice The keygen request assigned ID to the FHE params digest used during keygen preprocessing
        mapping(uint256 preKeygenRequestId => bytes32 paramsDigest) _preKeygenFheParamsDigests;
        /// @notice The preprocessing KSK ID to FHE params digest used during kskgen
        mapping(uint256 preKskId => bytes32 paramsDigest) _preKskFheParamsDigests;
        /// @notice The kskgen request assigned ID to the FHE params digest used during kskgen preprocessing
        mapping(uint256 preKskgenRequestId => bytes32 paramsDigest) _preKskgenFheParamsDigests;
        /// @notice The crsgen request assigned ID to the FHE params digest used during crsgen
        mapping(uint256 crsgenRequestId => bytes32 paramsDigest) _crsgenFheParamsDigests;
        /// @notice The fheParamsName mapping for the FHE params initialization status
        mapping(string fheParamsName => bool isInitialized) _fheParamsInitialized;
    }

    /// @dev Storage location has been computed using the following command:
    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.KmsManagement")) - 1))
    /// @dev & ~bytes32(uint256(0xff))
    bytes32 private constant KMS_MANAGEMENT_STORAGE_LOCATION =
        0xa48b77331ab977c487fc73c0afbe86f1a3a130c068453f497d376ab9fac7e000;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @notice Initializes the contract.
    /// @dev This function needs to be public in order to be called by the UUPS proxy.
    /// @custom:oz-upgrades-validate-as-initializer
    function initializeFromEmptyProxy(
        string memory fheParamsName,
        bytes32 fheParamsDigest
    ) public virtual onlyFromEmptyProxy reinitializer(3) {
        __Ownable_init(owner());
        __Pausable_init();

        KmsManagementStorage storage $ = _getKmsManagementStorage();
        $.fheParamsDigests[fheParamsName] = fheParamsDigest;
        $._fheParamsInitialized[fheParamsName] = true;
    }

    /// @notice Reinitializes the contract.
    function reinitializeV2() external reinitializer(3) {}

    /// @dev Modifier to check if the given FHE params name is initialized
    modifier fheParamsInitialized(string calldata fheParamsName) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();
        if (!$._fheParamsInitialized[fheParamsName]) {
            revert FheParamsNotInitialized();
        }
        _;
    }

    /// @dev See {IKmsManagement-preprocessKeygenRequest}.
    function preprocessKeygenRequest(
        string calldata fheParamsName
    ) external virtual onlyOwner fheParamsInitialized(fheParamsName) whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev TODO: maybe generate a preKeyId here instead of on KMS connectors:
        /// @dev https://github.com/zama-ai/fhevm-gateway/issues/67
        /// @dev Generate a new preKeygenRequestId. This is used to track the key generation preprocessing responses
        /// @dev as well as linking FHE params between the preprocessing and the actual key generation
        /// @dev This is different from the preKeyId generated by the KMS connector
        $._preKeygenRequestCounter++;
        uint256 preKeygenRequestId = $._preKeygenRequestCounter;

        /// @dev Store the FHE params used for the key generation preprocessing step as they will be used
        /// @dev for the actual key generation as well
        bytes32 fheParamsDigest = $.fheParamsDigests[fheParamsName];
        $._preKeygenFheParamsDigests[preKeygenRequestId] = fheParamsDigest;

        emit PreprocessKeygenRequest(preKeygenRequestId, fheParamsDigest);
    }

    /// @dev See {IKmsManagement-preprocessKeygenResponse}.
    function preprocessKeygenResponse(
        uint256 preKeygenRequestId,
        uint256 preKeyId
    ) external virtual onlyKmsTxSender whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev A KMS node can only respond once
        if ($._preKeygenResponses[preKeyId][msg.sender]) {
            revert PreprocessKeygenKmsNodeAlreadyResponded(preKeyId, msg.sender);
        }

        $._preKeygenResponses[preKeyId][msg.sender] = true;
        $._preKeygenResponseCounter[preKeygenRequestId]++;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!$._isPreKeygenDone[preKeyId] && _isKmsConsensusReached($._preKeygenResponseCounter[preKeygenRequestId])) {
            $._isPreKeygenDone[preKeyId] = true;

            /// @dev Store the FHE params digest used for the keygen preprocessing
            $._preKeyFheParamsDigests[preKeyId] = $._preKeygenFheParamsDigests[preKeygenRequestId];

            emit PreprocessKeygenResponse(preKeygenRequestId, preKeyId);
        }
    }

    /// @dev See {IKmsManagement-preprocessKskgenRequest}.
    function preprocessKskgenRequest(
        string calldata fheParamsName
    ) external virtual onlyOwner fheParamsInitialized(fheParamsName) whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev TODO: maybe generate a preKeyId here instead of on KMS connectors:
        /// @dev https://github.com/zama-ai/fhevm-gateway/issues/67
        /// @dev Generate a new preKskRequestId. This is used to track the KSK generation preprocessing responses
        /// @dev as well as linking FHE params between the preprocessing and the actual KSK generation
        /// @dev This is different from the preKeyId generated by the KMS connector
        $._preKskgenRequestCounter++;
        uint256 preKskgenRequestId = $._preKskgenRequestCounter;

        /// @dev Store the FHE params used for the KSK generation preprocessing step as they will be used
        /// @dev for the actual KSK generation as well
        bytes32 fheParamsDigest = $.fheParamsDigests[fheParamsName];
        $._preKskgenFheParamsDigests[preKskgenRequestId] = fheParamsDigest;

        emit PreprocessKskgenRequest(preKskgenRequestId, fheParamsDigest);
    }

    /// @dev See {IKmsManagement-preprocessKskgenResponse}.
    function preprocessKskgenResponse(
        uint256 preKskgenRequestId,
        uint256 preKskId
    ) external virtual onlyKmsTxSender whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev A KMS node can only respond once
        if ($._preKskgenResponses[preKskId][msg.sender]) {
            revert PreprocessKskgenKmsNodeAlreadyResponded(preKskId, msg.sender);
        }

        $._preKskgenResponses[preKskId][msg.sender] = true;
        $._preKskgenResponseCounter[preKskgenRequestId]++;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!$._isPreKskgenDone[preKskId] && _isKmsConsensusReached($._preKskgenResponseCounter[preKskgenRequestId])) {
            $._isPreKskgenDone[preKskId] = true;

            /// @dev Store the FHE params digest used for the kskgen preprocessing
            $._preKskFheParamsDigests[preKskId] = $._preKskgenFheParamsDigests[preKskgenRequestId];

            emit PreprocessKskgenResponse(preKskgenRequestId, preKskId);
        }
    }

    /// @dev See {IKmsManagement-keygenRequest}.
    function keygenRequest(uint256 preKeyId) external virtual onlyOwner whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev A key generation request can only be sent once
        if ($._isKeygenOngoing[preKeyId]) {
            revert KeygenRequestAlreadySent(preKeyId);
        }

        /// @dev A key generation requires a key preprocessing step to be completed
        if (!$._isPreKeygenDone[preKeyId]) {
            revert KeygenPreprocessingRequired(preKeyId);
        }

        $._isKeygenOngoing[preKeyId] = true;

        /// @dev A key generation uses the same FHE params as the one used for the preprocessing step
        emit KeygenRequest(preKeyId, $._preKeyFheParamsDigests[preKeyId]);
    }

    /// @dev See {IKmsManagement-keygenResponse}.
    function keygenResponse(uint256 preKeyId, uint256 keyId) external virtual onlyKmsTxSender whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev A KMS node can only respond once
        if ($._keygenResponses[keyId][msg.sender]) {
            revert KeygenKmsNodeAlreadyResponded(keyId, msg.sender);
        }

        $._keygenResponses[keyId][msg.sender] = true;
        $._keygenResponseCounter[keyId]++;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!$._isKeyGenerated[keyId] && _isKmsConsensusReached($._keygenResponseCounter[keyId])) {
            $._isKeyGenerated[keyId] = true;

            /// @dev Store the FHE params digest used during the keygen preprocessing and subsequent keygen
            bytes32 fheParamsDigest = $._preKeyFheParamsDigests[preKeyId];
            $.keyFheParamsDigests[keyId] = fheParamsDigest;

            /// @dev Include the FHE params used for the key generation (and its preprocessing) in
            /// @dev the event for better tracking
            emit KeygenResponse(preKeyId, keyId, fheParamsDigest);
        }
    }

    /// @dev See {IKmsManagement-crsgenRequest}.
    function crsgenRequest(
        string calldata fheParamsName
    ) external virtual onlyOwner fheParamsInitialized(fheParamsName) whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev Generate a new crsgenRequestId. This is used to link the FHE params sent in the request
        /// @dev with the ones emitted in the response event
        $._crsgenRequestCounter++;
        uint256 crsgenRequestId = $._crsgenRequestCounter;

        /// @dev Store the FHE params digest to use for the CRS generation
        bytes32 fheParamsDigest = $.fheParamsDigests[fheParamsName];
        $._crsgenFheParamsDigests[crsgenRequestId] = fheParamsDigest;

        emit CrsgenRequest(crsgenRequestId, fheParamsDigest);
    }

    /// @dev See {IKmsManagement-crsgenResponse}.
    function crsgenResponse(uint256 crsgenRequestId, uint256 crsId) external virtual onlyKmsTxSender whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev A KMS node can only respond once
        if ($._crsgenResponses[crsId][msg.sender]) {
            revert CrsgenKmsNodeAlreadyResponded(crsId, msg.sender);
        }

        $._crsgenResponses[crsId][msg.sender] = true;
        $._crsgenResponseCounter[crsgenRequestId]++;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!$._isCrsGenerated[crsId] && _isKmsConsensusReached($._crsgenResponseCounter[crsgenRequestId])) {
            $._isCrsGenerated[crsId] = true;

            /// @dev Store the FHE params digest used for the CRS generation
            bytes32 fheParamsDigest = $._crsgenFheParamsDigests[crsgenRequestId];
            $.crsFheParamsDigests[crsId] = fheParamsDigest;

            /// @dev Include the FHE params used for the CRS generation in the event for better tracking
            emit CrsgenResponse(crsgenRequestId, crsId, fheParamsDigest);
        }
    }

    /// @dev See {IKmsManagement-kskgenRequest}.
    function kskgenRequest(
        uint256 preKskId,
        uint256 sourceKeyId,
        uint256 destKeyId
    ) external virtual onlyOwner whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev A KSK generation request can only be sent once
        if ($._isKskgenOngoing[preKskId]) {
            revert KskgenRequestAlreadySent(preKskId);
        }

        /// @dev A KSK generation requires a KSK preprocessing step to be completed
        if (!$._isPreKskgenDone[preKskId]) {
            revert KskgenPreprocessingRequired(preKskId);
        }

        /// @dev The source key must be different from the destination key
        if (sourceKeyId == destKeyId) {
            revert KskgenSameSrcAndDestKeyIds(sourceKeyId);
        }

        /// @dev The source key must be generated
        if (!$._isKeyGenerated[sourceKeyId]) {
            revert KskgenSourceKeyNotGenerated(sourceKeyId);
        }

        /// @dev The destination key must be generated
        if (!$._isKeyGenerated[destKeyId]) {
            revert KskgenDestKeyNotGenerated(destKeyId);
        }

        $._kskgenSourceKeyIds[preKskId] = sourceKeyId;
        $._kskgenDestKeyIds[preKskId] = destKeyId;
        $._isKskgenOngoing[preKskId] = true;

        /// @dev A KSK generation uses the same FHE params as the one used for the preprocessing step
        emit KskgenRequest(preKskId, sourceKeyId, destKeyId, $._preKskFheParamsDigests[preKskId]);
    }

    /// @dev See {IKmsManagement-kskgenResponse}.
    function kskgenResponse(uint256 preKskId, uint256 kskId) external virtual onlyKmsTxSender whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev A KMS node can only respond once
        if ($._kskgenResponses[kskId][msg.sender]) {
            revert KskgenKmsNodeAlreadyResponded(kskId, msg.sender);
        }

        $._kskgenResponses[kskId][msg.sender] = true;
        $._kskgenResponseCounter[kskId]++;

        /// @dev Send the event if and only if the consensus is reached in the current response call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if (!$._isKskGenerated[kskId] && _isKmsConsensusReached($._kskgenResponseCounter[kskId])) {
            $._isKskGenerated[kskId] = true;

            /// @dev Store the KSK generation ID as a mapping of the source to the destination key ID
            $._kskgenIds[$._kskgenSourceKeyIds[preKskId]][$._kskgenDestKeyIds[preKskId]] = kskId;

            /// @dev Store the FHE params digest used during the kskgen preprocessing and subsequent kskgen
            bytes32 fheParamsDigest = $._preKskFheParamsDigests[preKskId];
            $.kskFheParamsDigests[kskId] = fheParamsDigest;

            emit KskgenResponse(preKskId, kskId, fheParamsDigest);
        }
    }

    /// @dev See {IKmsManagement-activateKeyRequest}.
    function activateKeyRequest(uint256 keyId) external virtual onlyOwner whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev A key activation request can only be sent once
        if ($._activateKeyOngoing[keyId]) {
            revert ActivateKeyRequestAlreadySent(keyId);
        }

        /// @dev Only a generated key can be activated
        if (!$._isKeyGenerated[keyId]) {
            revert ActivateKeyRequiresKeygen(keyId);
        }

        /// @dev Activating a (pending) key requires a KSK from the current (activated) key to this (pending) key
        /// @dev The only exception is for the first key (currentKeyId == 0)
        if ($.currentKeyId != 0) {
            if (!$._isKskGenerated[$._kskgenIds[$.currentKeyId][keyId]]) {
                revert ActivateKeyRequiresKskgen($.currentKeyId, keyId);
            }
        }

        $._activateKeyOngoing[keyId] = true;

        emit ActivateKeyRequest(keyId);
    }

    /// @dev See {IKmsManagement-activateKeyResponse}.
    /// @dev TODO: This function should only be called by the coprocessor transaction sender,
    /// @dev update this once integrating keygen in the gateway
    /// @dev See https://github.com/zama-ai/fhevm/issues/33
    function activateKeyResponse(uint256 keyId) external virtual whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();

        /// @dev A coprocessor can only respond once
        if ($._activateKeyResponses[keyId][msg.sender]) {
            revert ActivateKeyCoprocessorAlreadyResponded(keyId, msg.sender);
        }

        $._activateKeyResponses[keyId][msg.sender] = true;
        $._activateKeyResponseCounter[keyId]++;

        /// @dev Only activate the key and send the event if consensus has not been reached in a
        /// @dev previous call (i.e. the key is not the current key yet) and the consensus is
        /// @dev reached in the current call.
        /// @dev This means a "late" response will not be reverted, just ignored
        if ($.currentKeyId != keyId && _isCoprocessorConsensusReached($._activateKeyResponseCounter[keyId])) {
            $.activatedKeyIds.push(keyId);

            /// @dev The current keyId is considered activated (it is unique)
            $.currentKeyId = keyId;

            emit ActivateKeyResponse(keyId);
        }
    }

    /// @dev See {IKmsManagement-setFheParams}.
    function addFheParams(
        string calldata fheParamsName,
        bytes32 fheParamsDigest
    ) external virtual onlyOwner whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();
        if ($._fheParamsInitialized[fheParamsName]) {
            revert FheParamsAlreadyInitialized(fheParamsName);
        }

        $.fheParamsDigests[fheParamsName] = fheParamsDigest;
        $._fheParamsInitialized[fheParamsName] = true;

        emit AddFheParams(fheParamsName, fheParamsDigest);
    }

    /// @dev See {IKmsManagement-updateFheParams}.
    function updateFheParams(
        string calldata fheParamsName,
        bytes32 fheParamsDigest
    ) external virtual onlyOwner fheParamsInitialized(fheParamsName) whenNotPaused {
        KmsManagementStorage storage $ = _getKmsManagementStorage();
        $.fheParamsDigests[fheParamsName] = fheParamsDigest;

        emit UpdateFheParams(fheParamsName, fheParamsDigest);
    }

    /// @dev See {IKmsManagement-fheParamsDigests}.
    function fheParamsDigests(string calldata fheParamsName) external view virtual returns (bytes32) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();
        return $.fheParamsDigests[fheParamsName];
    }

    /// @dev See {IKmsManagement-getCurrentKeyId}.
    function getCurrentKeyId() external view virtual returns (uint256) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();
        return $.currentKeyId;
    }

    /// @dev See {IKmsManagement-activatedKeyIds}.
    function activatedKeyIds(uint256 index) external view virtual returns (uint256) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();
        return $.activatedKeyIds[index];
    }

    /// @dev See {IKmsManagement-keyFheParamsDigests}.
    function keyFheParamsDigests(uint256 keyId) external view virtual returns (bytes32) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();
        return $.keyFheParamsDigests[keyId];
    }

    /// @dev See {IKmsManagement-kskFheParamsDigests}.
    function kskFheParamsDigests(uint256 kskId) external view virtual returns (bytes32) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();
        return $.kskFheParamsDigests[kskId];
    }

    /// @dev See {IKmsManagement-crsFheParamsDigests}.
    function crsFheParamsDigests(uint256 crsId) external view virtual returns (bytes32) {
        KmsManagementStorage storage $ = _getKmsManagementStorage();
        return $.crsFheParamsDigests[crsId];
    }

    /// @dev See {IKmsManagement-getVersion}.
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

    /**
     * @dev Should revert when `msg.sender` is not authorized to upgrade the contract.
     */
    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address _newImplementation) internal virtual override onlyOwner {}

    /// @notice Checks if the consensus is reached among the KMS nodes.
    /// @param kmsCounter The number of KMS nodes that agreed
    /// @return Whether the consensus is reached
    function _isKmsConsensusReached(uint256 kmsCounter) internal view virtual returns (bool) {
        uint256 consensusThreshold = GATEWAY_CONFIG.getPublicDecryptionThreshold();
        return kmsCounter >= consensusThreshold;
    }

    /// @notice Checks if the consensus is reached among the Coprocessors.
    /// @param coprocessorCounter The number of coprocessors that agreed
    /// @return Whether the consensus is reached
    function _isCoprocessorConsensusReached(uint256 coprocessorCounter) internal view virtual returns (bool) {
        // TODO: Get the consensus threshold from the context associated to the keygen
        // See https://github.com/zama-ai/fhevm/issues/33
        // uint256 consensusThreshold = COPROCESSOR_CONTEXTS.getCoprocessorMajorityThresholdFromContext(coprocessorContextId);
        uint256 consensusThreshold = 0;
        return coprocessorCounter >= consensusThreshold;
    }

    /**
     * @dev Returns the KmsManagement storage location.
     * Note that this function is internal but not virtual: derived contracts should be able to
     * access it, but if the underlying storage struct version changes, we force them to define a new
     * getter function and use that one instead in order to avoid overriding the storage location.
     */
    function _getKmsManagementStorage() internal pure returns (KmsManagementStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := KMS_MANAGEMENT_STORAGE_LOCATION
        }
    }
}
