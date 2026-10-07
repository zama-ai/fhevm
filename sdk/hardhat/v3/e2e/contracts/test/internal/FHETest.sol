// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {
    FHE,
    ebool,
    euint8,
    euint16,
    euint32,
    euint64,
    euint128,
    euint256,
    eaddress,
    externalEbool,
    externalEuint8,
    externalEuint16,
    externalEuint32,
    externalEuint64,
    externalEuint128,
    externalEuint256
} from "@fhevm/solidity/lib/FHE.sol";
import {CoprocessorConfig, Impl} from "@fhevm/solidity/lib/Impl.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

/// @title A simple FHE Test contract
contract FHETest is ZamaEthereumConfig {
    string public constant CONTRACT_NAME = "FHETestv1";

    mapping(address => ebool) internal _eboolMap;
    mapping(address => euint8) internal _euint8Map;
    mapping(address => euint16) internal _euint16Map;
    mapping(address => euint32) internal _euint32Map;
    mapping(address => euint64) internal _euint64Map;
    mapping(address => euint128) internal _euint128Map;
    mapping(address => euint256) internal _euint256Map;
    mapping(address => eaddress) internal _eaddressMap;

    mapping(bytes32 => uint256) internal _db;

    /// @notice Returns CoprocessorConfig
    function getCoprocessorConfig() external view returns (CoprocessorConfig memory config) {
        CoprocessorConfig storage $ = Impl.getCoprocessorConfig();

        config.ACLAddress = $.ACLAddress;
        config.CoprocessorAddress = $.CoprocessorAddress;
        config.KMSVerifierAddress = $.KMSVerifierAddress;
    }

    /// @notice Returns the caller's ebool
    function getEbool() external view returns (ebool) {
        return _eboolMap[msg.sender];
    }

    /// @notice Returns the caller's euint8
    function getEuint8() external view returns (euint8) {
        return _euint8Map[msg.sender];
    }

    /// @notice Returns the caller's euint16
    function getEuint16() external view returns (euint16) {
        return _euint16Map[msg.sender];
    }

    /// @notice Returns the caller's euint32
    function getEuint32() external view returns (euint32) {
        return _euint32Map[msg.sender];
    }

    /// @notice Returns the caller's euint64
    function getEuint64() external view returns (euint64) {
        return _euint64Map[msg.sender];
    }

    /// @notice Returns the caller's euint128
    function getEuint128() external view returns (euint128) {
        return _euint128Map[msg.sender];
    }

    /// @notice Returns the caller's euint256
    function getEuint256() external view returns (euint256) {
        return _euint256Map[msg.sender];
    }

    /// @notice Returns the caller's eaddress
    function getEaddress() external view returns (eaddress) {
        return _eaddressMap[msg.sender];
    }

    /// @notice Performs FHE.makePubliclyDecryptable(ebool)
    function makePubliclyDecryptableEbool() external {
        ebool v = _eboolMap[msg.sender];
        require(FHE.isInitialized(v), "ebool not initialized");
        FHE.makePubliclyDecryptable(v);
    }

    /// @notice Performs FHE.makePubliclyDecryptable(euint8)
    function makePubliclyDecryptableEuint8() external {
        euint8 v = _euint8Map[msg.sender];
        require(FHE.isInitialized(v), "euint8 not initialized");
        FHE.makePubliclyDecryptable(v);
    }

    /// @notice Performs FHE.makePubliclyDecryptable(euint16)
    function makePubliclyDecryptableEuint16() external {
        euint16 v = _euint16Map[msg.sender];
        require(FHE.isInitialized(v), "euint16 not initialized");
        FHE.makePubliclyDecryptable(v);
    }

    /// @notice Performs FHE.makePubliclyDecryptable(euint32)
    function makePubliclyDecryptableEuint32() external {
        euint32 v = _euint32Map[msg.sender];
        require(FHE.isInitialized(v), "euint32 not initialized");
        FHE.makePubliclyDecryptable(v);
    }

    /// @notice Performs FHE.makePubliclyDecryptable(euint64)
    function makePubliclyDecryptableEuint64() external {
        euint64 v = _euint64Map[msg.sender];
        require(FHE.isInitialized(v), "euint64 not initialized");
        FHE.makePubliclyDecryptable(v);
    }

    /// @notice Performs FHE.makePubliclyDecryptable(euint128)
    function makePubliclyDecryptableEuint128() external {
        euint128 v = _euint128Map[msg.sender];
        require(FHE.isInitialized(v), "euint128 not initialized");
        FHE.makePubliclyDecryptable(v);
    }

    /// @notice Performs FHE.makePubliclyDecryptable(euint256)
    function makePubliclyDecryptableEuint256() external {
        euint256 v = _euint256Map[msg.sender];
        require(FHE.isInitialized(v), "euint256 not initialized");
        FHE.makePubliclyDecryptable(v);
    }

    /// @notice Performs FHE.makePubliclyDecryptable(eaddress)
    function makePubliclyDecryptableEaddress() external {
        eaddress v = _eaddressMap[msg.sender];
        require(FHE.isInitialized(v), "eaddress not initialized");
        FHE.makePubliclyDecryptable(v);
    }

    /// @notice Performs FHE.checkSignatures()
    function verify(bytes32[] calldata handlesList, bytes memory cleartexts, bytes memory decryptionProof) external {
        FHE.checkSignatures(handlesList, cleartexts, decryptionProof);
    }

    /// @notice Computes FHE.randEbool()
    function randEbool() external {
        ebool v = FHE.randEbool();
        _eboolMap[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.randEuint8()
    function randEuint8() external {
        euint8 v = FHE.randEuint8();
        _euint8Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.randEuint16()
    function randEuint16() external {
        euint16 v = FHE.randEuint16();
        _euint16Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.randEuint32()
    function randEuint32() external {
        euint32 v = FHE.randEuint32();
        _euint32Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.randEuint64()
    function randEuint64() external {
        euint64 v = FHE.randEuint64();
        _euint64Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.randEuint128()
    function randEuint128() external {
        euint128 v = FHE.randEuint128();
        _euint128Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.randEuint256()
    function randEuint256() external {
        euint256 v = FHE.randEuint256();
        _euint256Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.xor(ebool)
    function xorEbool(externalEbool inputEbool, bytes calldata inputProof) external {
        ebool encryptedEbool = FHE.fromExternal(inputEbool, inputProof);
        ebool v = FHE.xor(_eboolMap[msg.sender], encryptedEbool);
        _eboolMap[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.add(euint8)
    function addEuint8(externalEuint8 inputEuint8, bytes calldata inputProof) external {
        euint8 encryptedEuint8 = FHE.fromExternal(inputEuint8, inputProof);
        euint8 v = FHE.add(_euint8Map[msg.sender], encryptedEuint8);
        _euint8Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.add(euint16)
    function addEuint16(externalEuint16 inputEuint16, bytes calldata inputProof) external {
        euint16 encryptedEuint16 = FHE.fromExternal(inputEuint16, inputProof);
        euint16 v = FHE.add(_euint16Map[msg.sender], encryptedEuint16);
        _euint16Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.add(euint32)
    function addEuint32(externalEuint32 inputEuint32, bytes calldata inputProof) external {
        euint32 encryptedEuint32 = FHE.fromExternal(inputEuint32, inputProof);
        euint32 v = FHE.add(_euint32Map[msg.sender], encryptedEuint32);
        _euint32Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.add(euint64)
    function addEuint64(externalEuint64 inputEuint64, bytes calldata inputProof) external {
        euint64 encryptedEuint64 = FHE.fromExternal(inputEuint64, inputProof);
        euint64 v = FHE.add(_euint32Map[msg.sender], encryptedEuint64);
        _euint64Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.add(euint128)
    function addEuint128(externalEuint128 inputEuint128, bytes calldata inputProof) external {
        euint128 encryptedEuint128 = FHE.fromExternal(inputEuint128, inputProof);
        euint128 v = FHE.add(_euint128Map[msg.sender], encryptedEuint128);
        _euint128Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }

    /// @notice Computes FHE.xor(euint256)
    function xorEuint256(externalEuint256 inputEuint256, bytes calldata inputProof) external {
        euint256 encryptedEuint256 = FHE.fromExternal(inputEuint256, inputProof);
        euint256 v = FHE.xor(_euint256Map[msg.sender], encryptedEuint256);
        _euint256Map[msg.sender] = v;
        FHE.allowThis(v);
        FHE.allow(v, msg.sender);
    }
}
