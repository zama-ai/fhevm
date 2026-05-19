// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {KmsNode} from "../../contracts/shared/Structs.sol";

// Test-only mock of the GatewayConfig view surface read by task:assertKmsMigrationSucceeded.
// State is seeded directly; no governance/access control.
contract MockGatewayConfigView {
    uint256 private _currentKmsContextId;
    mapping(uint256 => address[]) private _kmsTxSendersByContext;
    mapping(uint256 => mapping(address => KmsNode)) private _kmsNodeByContext;
    mapping(uint256 => uint256) private _publicDecryptionThresholdByContext;
    mapping(uint256 => uint256) private _userDecryptionThresholdByContext;
    uint256 private _mpcThreshold;
    uint256 private _kmsGenThreshold;

    function seedKmsContext(
        uint256 contextId,
        KmsNode[] calldata nodes,
        uint256 publicDecryptionThreshold,
        uint256 userDecryptionThreshold,
        uint256 mpcThreshold,
        uint256 kmsGenThreshold
    ) external {
        _currentKmsContextId = contextId;
        delete _kmsTxSendersByContext[contextId];
        for (uint256 i = 0; i < nodes.length; i++) {
            _kmsTxSendersByContext[contextId].push(nodes[i].txSenderAddress);
            _kmsNodeByContext[contextId][nodes[i].txSenderAddress] = nodes[i];
        }
        _publicDecryptionThresholdByContext[contextId] = publicDecryptionThreshold;
        _userDecryptionThresholdByContext[contextId] = userDecryptionThreshold;
        _mpcThreshold = mpcThreshold;
        _kmsGenThreshold = kmsGenThreshold;
    }

    function pushPhantomNode(uint256 contextId, KmsNode calldata node) external {
        _kmsTxSendersByContext[contextId].push(node.txSenderAddress);
        _kmsNodeByContext[contextId][node.txSenderAddress] = node;
    }

    function overridePublicDecryptionThreshold(uint256 contextId, uint256 newThreshold) external {
        _publicDecryptionThresholdByContext[contextId] = newThreshold;
    }

    function getCurrentKmsContextId() external view returns (uint256) {
        return _currentKmsContextId;
    }

    function getKmsTxSendersForContext(uint256 contextId) external view returns (address[] memory) {
        return _kmsTxSendersByContext[contextId];
    }

    function getKmsNodeForContext(
        uint256 contextId,
        address kmsTxSenderAddress
    ) external view returns (KmsNode memory) {
        return _kmsNodeByContext[contextId][kmsTxSenderAddress];
    }

    function getPublicDecryptionThresholdForContext(uint256 contextId) external view returns (uint256) {
        return _publicDecryptionThresholdByContext[contextId];
    }

    function getUserDecryptionThresholdForContext(uint256 contextId) external view returns (uint256) {
        return _userDecryptionThresholdByContext[contextId];
    }

    function getMpcThreshold() external view returns (uint256) {
        return _mpcThreshold;
    }

    function getKmsGenThreshold() external view returns (uint256) {
        return _kmsGenThreshold;
    }
}

// Test-only mock of the Gateway KMSGeneration view surface. Counters, active IDs and consensus
// digests are read via eth_getStorageAt, so the ERC-7201 layout below must match production.
contract MockGatewayKMSGenerationView {
    struct KeyDigestStorage {
        uint8 keyType;
        bytes digest;
    }

    /// @custom:storage-location erc7201:fhevm_gateway.storage.KMSGeneration
    /// @dev Field offsets must match `KMSGenerationStorage` in
    ///      `gateway-contracts/contracts/KMSGeneration.sol` for the offsets read by
    ///      `task:assertKmsMigrationSucceeded` (3, 4, 5, 6, 8, 9, 12).
    struct KMSGenerationStorage {
        uint256 _gap0; //                                offset 0  (kmsHasSignedForResponse)
        uint256 _gap1; //                                offset 1  (isRequestDone)
        uint256 _gap2; //                                offset 2  (consensusTxSenderAddresses)
        mapping(uint256 => bytes32) consensusDigest; //  offset 3
        uint256 prepKeygenCounter; //                    offset 4
        uint256 keyCounter; //                           offset 5
        mapping(uint256 => uint256) keygenIdPairs; //    offset 6
        uint256 _gap7; //                                offset 7  (keyDigests)
        uint256 activeKeyId; //                          offset 8
        uint256 crsCounter; //                           offset 9
        uint256 _gap10; //                               offset 10 (crsMaxBitLength)
        uint256 _gap11; //                               offset 11 (crsDigests)
        uint256 activeCrsId; //                          offset 12
    }

    // keccak256(abi.encode(uint256(keccak256("fhevm_gateway.storage.KMSGeneration")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant KMS_GENERATION_STORAGE_LOCATION =
        0x0b8fdb1f0a6356dd20a6cbc6f9668fac23b85f96575d10e333e603faa794ac00;

    mapping(uint256 => address[]) private _consensusTxSendersByRequest;
    mapping(uint256 => uint8) private _keyParamsType;
    mapping(uint256 => uint8) private _crsParamsType;
    mapping(uint256 => string[]) private _keyStorageUrls;
    mapping(uint256 => KeyDigestStorage[]) private _keyDigests;
    mapping(uint256 => string[]) private _crsStorageUrls;
    mapping(uint256 => bytes) private _crsDigest;

    function _getStorage() private pure returns (KMSGenerationStorage storage $) {
        bytes32 location = KMS_GENERATION_STORAGE_LOCATION;
        assembly {
            $.slot := location
        }
    }

    function seedKmsGeneration(
        uint256 prepKeygenCounter,
        uint256 keyCounter,
        uint256 crsCounter,
        uint256 activeKeyId,
        uint256 activeCrsId,
        uint256 activePrepKeygenId
    ) external {
        KMSGenerationStorage storage $ = _getStorage();
        $.prepKeygenCounter = prepKeygenCounter;
        $.keyCounter = keyCounter;
        $.crsCounter = crsCounter;
        $.activeKeyId = activeKeyId;
        $.activeCrsId = activeCrsId;
        $.keygenIdPairs[activeKeyId] = activePrepKeygenId;
    }

    function seedConsensusTxSenders(uint256 requestId, address[] calldata txSenders) external {
        _consensusTxSendersByRequest[requestId] = txSenders;
    }

    function seedConsensusDigest(uint256 requestId, bytes32 digest) external {
        _getStorage().consensusDigest[requestId] = digest;
    }

    function seedKeyMaterials(
        uint256 keyId,
        string[] calldata storageUrls,
        KeyDigestStorage[] calldata digests,
        uint8 paramsType
    ) external {
        delete _keyStorageUrls[keyId];
        for (uint256 i = 0; i < storageUrls.length; i++) {
            _keyStorageUrls[keyId].push(storageUrls[i]);
        }
        delete _keyDigests[keyId];
        for (uint256 i = 0; i < digests.length; i++) {
            _keyDigests[keyId].push(digests[i]);
        }
        _keyParamsType[keyId] = paramsType;
    }

    function seedCrsMaterials(
        uint256 crsId,
        string[] calldata storageUrls,
        bytes calldata digest,
        uint8 paramsType
    ) external {
        delete _crsStorageUrls[crsId];
        for (uint256 i = 0; i < storageUrls.length; i++) {
            _crsStorageUrls[crsId].push(storageUrls[i]);
        }
        _crsDigest[crsId] = digest;
        _crsParamsType[crsId] = paramsType;
    }

    function getConsensusTxSenders(uint256 requestId) external view returns (address[] memory) {
        return _consensusTxSendersByRequest[requestId];
    }

    function getKeyParamsType(uint256 keyId) external view returns (uint8) {
        return _keyParamsType[keyId];
    }

    function getCrsParamsType(uint256 crsId) external view returns (uint8) {
        return _crsParamsType[crsId];
    }

    function getKeyMaterials(uint256 keyId) external view returns (string[] memory, KeyDigestStorage[] memory) {
        return (_keyStorageUrls[keyId], _keyDigests[keyId]);
    }

    function getCrsMaterials(uint256 crsId) external view returns (string[] memory, bytes memory) {
        return (_crsStorageUrls[crsId], _crsDigest[crsId]);
    }
}
