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
    externalEaddress,
    externalEuint256
} from "@fhevm/solidity/lib/FHE.sol";
import { CoprocessorConfig, Impl } from "@fhevm/solidity/lib/Impl.sol";
import { FheType } from "@fhevm/solidity/lib/FheType.sol";

/// @title A simple FHE Test contract
contract FHETest {
    string public constant CONTRACT_NAME = "FHETestv2";

    mapping(address => mapping(FheType => bytes32)) internal _etypeMap;
    mapping(bytes32 => uint256) internal _db;

    function confidentialProtocolId() public pure returns (uint256) {
        return type(uint256).max - 1;
    }

    function setCoprocessorConfig(CoprocessorConfig memory config) public {
        FHE.setCoprocessor(config);
    }

    /// @notice Returns CoprocessorConfig
    function getCoprocessorConfig() public view returns (CoprocessorConfig memory config) {
        CoprocessorConfig storage $ = Impl.getCoprocessorConfig();

        config.ACLAddress = $.ACLAddress;
        config.CoprocessorAddress = $.CoprocessorAddress;
        config.KMSVerifierAddress = $.KMSVerifierAddress;
    }

    ////////////////////////////////////////////////////////////////////////////
    //
    // ClearText functions
    //
    ////////////////////////////////////////////////////////////////////////////

    function hasClearText(bytes32 handle) public view returns (bool) {
        return Impl.isAllowed(handle, address(this));
    }

    function getClearText(bytes32 handle) public view returns (uint256) {
        require(hasClearText(handle), "Unknown handle");
        return _db[handle];
    }

    ////////////////////////////////////////////////////////////////////////////
    // 
    // Get functions
    // 
    ////////////////////////////////////////////////////////////////////////////

    /// @notice Stores a handle for a given account and FHE type.
    ///         Verifies that the handle's embedded type byte (byte 30) matches the expected FHE type.
    function _setHandleOf(address account, FheType fheType, bytes32 handle, uint256 clearValue, bool makePublic) internal {
        require(_typeOf(handle) == fheType, "FheType mismatch");
        _etypeMap[account][fheType] = handle;
        Impl.allow(handle, address(this));
        Impl.allow(handle, account);
        _db[handle] = clearValue;
        if (makePublic) {
            Impl.makePubliclyDecryptable(handle);
        }
    }

    /// @notice Returns the raw handle for a given account and FHE type
    function hasHandleOf(address account, FheType fheType) public view returns (bool) {
        return _etypeMap[account][fheType] != bytes32(0);
    }

    /// @notice Returns the raw handle for a given account and FHE type
    function getHandleOf(address account, FheType fheType) public view returns (bytes32) {
        bytes32 handle = _etypeMap[account][fheType];
        require(handle != 0, "Handle does not exist");
        return handle;
    }

    /// @notice Returns the raw handle for the caller and a given FHE type
    function getHandle(FheType fheType) public view returns (bytes32) {
        return getHandleOf(msg.sender, fheType);
    }

    /// @notice Returns the ebool for the given address
    function getEboolOf(address account) public view returns (ebool) {
        return ebool.wrap(getHandleOf(account, FheType.Bool));
    }

    /// @notice Returns the caller's ebool
    function getEbool() public view returns (ebool) {
        return getEboolOf(msg.sender);
    }

    /// @notice Returns the euint8 for the given address
    function getEuint8Of(address account) public view returns (euint8) {
        return euint8.wrap(getHandleOf(account, FheType.Uint8));
    }

    /// @notice Returns the caller's euint8
    function getEuint8() public view returns (euint8) {
        return getEuint8Of(msg.sender);
    }

    /// @notice Returns the euint16 for the given address
    function getEuint16Of(address account) public view returns (euint16) {
        return euint16.wrap(getHandleOf(account, FheType.Uint16));
    }

    /// @notice Returns the caller's euint16
    function getEuint16() public view returns (euint16) {
        return getEuint16Of(msg.sender);
    }

    /// @notice Returns the euint32 for the given address
    function getEuint32Of(address account) public view returns (euint32) {
        return euint32.wrap(getHandleOf(account, FheType.Uint32));
    }

    /// @notice Returns the caller's euint32
    function getEuint32() public view returns (euint32) {
        return getEuint32Of(msg.sender);
    }

    /// @notice Returns the euint64 for the given address
    function getEuint64Of(address account) public view returns (euint64) {
        return euint64.wrap(getHandleOf(account, FheType.Uint64));
    }

    /// @notice Returns the caller's euint64
    function getEuint64() public view returns (euint64) {
        return getEuint64Of(msg.sender);
    }

    /// @notice Returns the euint128 for the given address
    function getEuint128Of(address account) public view returns (euint128) {
        return euint128.wrap(getHandleOf(account, FheType.Uint128));
    }

    /// @notice Returns the caller's euint128
    function getEuint128() public view returns (euint128) {
        return getEuint128Of(msg.sender);
    }

    /// @notice Returns the euint256 for the given address
    function getEuint256Of(address account) public view returns (euint256) {
        return euint256.wrap(getHandleOf(account, FheType.Uint256));
    }

    /// @notice Returns the caller's euint256
    function getEuint256() public view returns (euint256) {
        return getEuint256Of(msg.sender);
    }

    /// @notice Returns the eaddress for the given address
    function getEaddressOf(address account) public view returns (eaddress) {
        return eaddress.wrap(getHandleOf(account, FheType.Uint160));
    }

    /// @notice Returns the caller's eaddress
    function getEaddress() public view returns (eaddress) {
        return getEaddressOf(msg.sender);
    }

    ////////////////////////////////////////////////////////////////////////////
    // 
    // makePubliclyDecryptable functions
    // 
    ////////////////////////////////////////////////////////////////////////////

    /// @notice Performs FHE.makePubliclyDecryptable(...)
    function makePubliclyDecryptable(FheType fheType) public {
        bytes32 handle = getHandle(fheType);
        Impl.makePubliclyDecryptable(handle);
    }

    /// @notice Performs FHE.checkSignatures()
    function verify(bytes32[] calldata handlesList, bytes memory cleartexts, bytes memory decryptionProof) public {
        FHE.checkSignatures(handlesList, cleartexts, decryptionProof);
    }

    ////////////////////////////////////////////////////////////////////////////
    // 
    // Init functions
    // 
    ////////////////////////////////////////////////////////////////////////////

    function _zeroEbool() internal returns (ebool) {
        return FHE.asEbool(_zeroEuint8());
    }

    // Shift amount is encrypted to prevent the coprocessor from
    // optimizing away the computation (e.g. detecting a ^ a = 0).
    function _zeroEuint8() internal returns (euint8) {
        euint8 r = FHE.randEuint8();
        euint8 zeroOrOne = FHE.shr(r, 1);
        euint8 a = FHE.shr(FHE.shr(r, zeroOrOne), zeroOrOne);
        euint8 b = FHE.shr(r, FHE.add(zeroOrOne, zeroOrOne));
        return FHE.xor(a, b);
    }

    /// @notice Computes zeroBool | value = value
    function setClearEbool(bool value, bool makePublic) public returns (ebool) {
        ebool v = FHE.or(_zeroEbool(), value);
        _setHandleOf(msg.sender, FheType.Bool, ebool.unwrap(v), uint256(value ? 1 : 0), makePublic);
        return v;
    }

    /// @notice Computes zero8 | value = value
    function setClearEuint8(uint8 value, bool makePublic) public returns (euint8) {
        euint8 v = FHE.or(_zeroEuint8(), value);
        _setHandleOf(msg.sender, FheType.Uint8, euint8.unwrap(v), uint256(value), makePublic);
        return v;
    }

    /// @notice Computes zero16 | value = value
    function setClearEuint16(uint16 value, bool makePublic) public returns (euint16) {
        euint16 v = FHE.or(FHE.asEuint16(_zeroEuint8()), value);
        _setHandleOf(msg.sender, FheType.Uint16, euint16.unwrap(v), uint256(value), makePublic);
        return v;
    }

    /// @notice Computes zero32 | value = value
    function setClearEuint32(uint32 value, bool makePublic) public returns (euint32) {
        euint32 v = FHE.or(FHE.asEuint32(_zeroEuint8()), value);
        _setHandleOf(msg.sender, FheType.Uint32, euint32.unwrap(v), uint256(value), makePublic);
        return v;
    }

    /// @notice Computes zero64 | value = value
    function setClearEuint64(uint64 value, bool makePublic) public returns (euint64) {
        euint64 v = FHE.or(FHE.asEuint64(_zeroEuint8()), value);
        _setHandleOf(msg.sender, FheType.Uint64, euint64.unwrap(v), uint256(value), makePublic);
        return v;
    }

    /// @notice Computes zero128 | value = value
    function setClearEuint128(uint128 value, bool makePublic) public returns (euint128) {
        euint128 v = FHE.or(FHE.asEuint128(_zeroEuint8()), value);
        _setHandleOf(msg.sender, FheType.Uint128, euint128.unwrap(v), uint256(value), makePublic);
        return v;
    }

    /// @notice Computes zero256 | value = value
    function setClearEuint256(uint256 value, bool makePublic) public returns (euint256) {
        euint256 v = FHE.or(FHE.asEuint256(_zeroEuint8()), value);
        _setHandleOf(msg.sender, FheType.Uint256, euint256.unwrap(v), value, makePublic);
        return v;
    }

    /// @notice set eaddress using trivial encryption
    function setClearEaddress(address addr, bool makePublic) public returns (eaddress) {
        eaddress v = FHE.asEaddress(addr);
        _setHandleOf(msg.sender, FheType.Uint160, eaddress.unwrap(v), uint256(uint160(addr)), makePublic);
        return v;
    }

    ////////////////////////////////////////////////////////////////////////////
    // 
    // Operator functions
    // 
    ////////////////////////////////////////////////////////////////////////////

    /// @notice Computes FHE.xor(ebool)
    function xorEbool(externalEbool inputEbool, bytes calldata inputProof, bool clearValue, bool makePublic) public {
        ebool encryptedEbool = FHE.fromExternal(inputEbool, inputProof);
        
        bytes32 leftHandle = getHandle(FheType.Bool);
        bool leftClearValue = getClearText(leftHandle) != 0;
        ebool left = ebool.wrap(leftHandle);

        bool clear = leftClearValue != clearValue;
        ebool enc = FHE.xor(left, encryptedEbool);

        _setHandleOf(msg.sender, FheType.Bool, ebool.unwrap(enc), uint256(clear ? 1 : 0), makePublic);
    }

    /// @notice Computes FHE.add(euint8)
    function addEuint8(externalEuint8 inputEuint8, bytes calldata inputProof, uint8 clearValue, bool makePublic) public {
        euint8 encryptedEuint8 = FHE.fromExternal(inputEuint8, inputProof);

        bytes32 leftHandle = getHandle(FheType.Uint8);
        uint8 leftClearValue = uint8(getClearText(leftHandle));
        euint8 left = euint8.wrap(leftHandle);

        uint8 clear; 
        unchecked { clear = leftClearValue + clearValue; }
        euint8 enc = FHE.add(left, encryptedEuint8);

        _setHandleOf(msg.sender, FheType.Uint8, euint8.unwrap(enc), uint256(clear), makePublic);
    }

    /// @notice Computes FHE.add(euint16)
    function addEuint16(externalEuint16 inputEuint16, bytes calldata inputProof, uint16 clearValue, bool makePublic) public {
        euint16 encryptedEuint16 = FHE.fromExternal(inputEuint16, inputProof);

        bytes32 leftHandle = getHandle(FheType.Uint16);
        uint16 leftClearValue = uint16(getClearText(leftHandle));
        euint16 left = euint16.wrap(leftHandle);

        uint16 clear;
        unchecked { clear = leftClearValue + clearValue; }
        euint16 enc = FHE.add(left, encryptedEuint16);

        _setHandleOf(msg.sender, FheType.Uint16, euint16.unwrap(enc), uint256(clear), makePublic);
    }

    /// @notice Computes FHE.add(euint32)
    function addEuint32(externalEuint32 inputEuint32, bytes calldata inputProof, uint32 clearValue, bool makePublic) public {
        euint32 encryptedEuint32 = FHE.fromExternal(inputEuint32, inputProof);

        bytes32 leftHandle = getHandle(FheType.Uint32);
        uint32 leftClearValue = uint32(getClearText(leftHandle));
        euint32 left = euint32.wrap(leftHandle);

        uint32 clear;
        unchecked { clear = leftClearValue + clearValue; }
        euint32 enc = FHE.add(left, encryptedEuint32);

        _setHandleOf(msg.sender, FheType.Uint32, euint32.unwrap(enc), uint256(clear), makePublic);
    }

    /// @notice Computes FHE.add(euint64)
    function addEuint64(externalEuint64 inputEuint64, bytes calldata inputProof, uint64 clearValue, bool makePublic) public {
        euint64 encryptedEuint64 = FHE.fromExternal(inputEuint64, inputProof);

        bytes32 leftHandle = getHandle(FheType.Uint64);
        uint64 leftClearValue = uint64(getClearText(leftHandle));
        euint64 left = euint64.wrap(leftHandle);

        uint64 clear;
        unchecked { clear = leftClearValue + clearValue; }
        euint64 enc = FHE.add(left, encryptedEuint64);

        _setHandleOf(msg.sender, FheType.Uint64, euint64.unwrap(enc), uint256(clear), makePublic);
    }

    /// @notice Computes FHE.add(euint128)
    function addEuint128(externalEuint128 inputEuint128, bytes calldata inputProof, uint128 clearValue, bool makePublic) public {
        euint128 encryptedEuint128 = FHE.fromExternal(inputEuint128, inputProof);

        bytes32 leftHandle = getHandle(FheType.Uint128);
        uint128 leftClearValue = uint128(getClearText(leftHandle));
        euint128 left = euint128.wrap(leftHandle);

        uint128 clear;
        unchecked { clear = leftClearValue + clearValue; }
        euint128 enc = FHE.add(left, encryptedEuint128);

        _setHandleOf(msg.sender, FheType.Uint128, euint128.unwrap(enc), uint256(clear), makePublic);
    }

    /// @notice Computes FHE.xor(euint256)
    function xorEuint256(externalEuint256 inputEuint256, bytes calldata inputProof, uint256 clearValue, bool makePublic) public {
        euint256 encryptedEuint256 = FHE.fromExternal(inputEuint256, inputProof);

        bytes32 leftHandle = getHandle(FheType.Uint256);
        uint256 leftClearValue = getClearText(leftHandle);
        euint256 left = euint256.wrap(leftHandle);

        uint256 clear = leftClearValue ^ clearValue;
        euint256 enc = FHE.xor(left, encryptedEuint256);

        _setHandleOf(msg.sender, FheType.Uint256, euint256.unwrap(enc), clear, makePublic);
    }

    /// @notice Computes FHE.select(FHE.eq(input, existing), existing, input)
    function eqEaddress(externalEaddress inputEaddress, bytes calldata inputProof, address clearValue, bool makePublic) public {
        eaddress encryptedEaddress = FHE.fromExternal(inputEaddress, inputProof);

        bytes32 leftHandle = getHandle(FheType.Uint160);
        address leftClearValue = address(uint160(getClearText(leftHandle)));
        eaddress left = eaddress.wrap(leftHandle);

        address clear = leftClearValue == clearValue ? leftClearValue : clearValue;
        eaddress enc = FHE.select(FHE.eq(encryptedEaddress, left), left, encryptedEaddress);

        _setHandleOf(msg.sender, FheType.Uint160, eaddress.unwrap(enc), uint256(uint160(clear)), makePublic);
    }

    ////////////////////////////////////////////////////////////////////////////
    // 
    // Set functions
    // 
    ////////////////////////////////////////////////////////////////////////////

    /// @notice sets the current ebool value
    function setEbool(externalEbool inputEbool, bytes calldata inputProof, bool clearValue, bool makePublic) public returns (ebool) {
        ebool encryptedEbool = FHE.fromExternal(inputEbool, inputProof);
        _setHandleOf(msg.sender, FheType.Bool, ebool.unwrap(encryptedEbool), clearValue ? 1 : 0, makePublic);
        return encryptedEbool;
    }

    /// @notice sets the current euint8 value
    function setEuint8(externalEuint8 inputEuint8, bytes calldata inputProof, uint8 clearValue, bool makePublic) public returns (euint8) {
        euint8 enc = FHE.fromExternal(inputEuint8, inputProof);
        _setHandleOf(msg.sender, FheType.Uint8, euint8.unwrap(enc), uint256(clearValue), makePublic);
        return enc;
    }

    /// @notice sets the current euint16 value
    function setEuint16(externalEuint16 inputEuint16, bytes calldata inputProof, uint16 clearValue, bool makePublic) public returns (euint16) {
        euint16 enc = FHE.fromExternal(inputEuint16, inputProof);
        _setHandleOf(msg.sender, FheType.Uint16, euint16.unwrap(enc), uint256(clearValue), makePublic);
        return enc;
    }

    /// @notice sets the current euint32 value
    function setEuint32(externalEuint32 inputEuint32, bytes calldata inputProof, uint32 clearValue, bool makePublic) public returns (euint32) {
        euint32 enc = FHE.fromExternal(inputEuint32, inputProof);
        _setHandleOf(msg.sender, FheType.Uint32, euint32.unwrap(enc), uint256(clearValue), makePublic);
        return enc;
    }

    /// @notice sets the current euint64 value
    function setEuint64(externalEuint64 inputEuint64, bytes calldata inputProof, uint64 clearValue, bool makePublic) public returns (euint64) {
        euint64 enc = FHE.fromExternal(inputEuint64, inputProof);
        _setHandleOf(msg.sender, FheType.Uint64, euint64.unwrap(enc), uint256(clearValue), makePublic);
        return enc;
    }

    /// @notice sets the current euint128 value
    function setEuint128(externalEuint128 inputEuint128, bytes calldata inputProof, uint128 clearValue, bool makePublic) public returns (euint128) {
        euint128 enc = FHE.fromExternal(inputEuint128, inputProof);
        _setHandleOf(msg.sender, FheType.Uint128, euint128.unwrap(enc), uint256(clearValue), makePublic);
        return enc;
    }

    /// @notice sets the current euint256 value
    function setEuint256(externalEuint256 inputEuint256, bytes calldata inputProof, uint256 clearValue, bool makePublic) public returns (euint256) {
        euint256 enc = FHE.fromExternal(inputEuint256, inputProof);
        _setHandleOf(msg.sender, FheType.Uint256, euint256.unwrap(enc), clearValue, makePublic);
        return enc;
    }

    /// @notice sets the current eaddress value
    function setEaddress(externalEaddress inputEaddress, bytes calldata inputProof, address clearValue, bool makePublic) public returns (eaddress) {
        eaddress enc = FHE.fromExternal(inputEaddress, inputProof);
        _setHandleOf(msg.sender, FheType.Uint160, eaddress.unwrap(enc), uint256(uint160(clearValue)), makePublic);
        return enc;
    }

    ////////////////////////////////////////////////////////////////////////////
    // 
    // Others
    // 
    ////////////////////////////////////////////////////////////////////////////

    function _typeOf(bytes32 handle) internal pure virtual returns (FheType typeCt) {
        typeCt = FheType(uint8(handle[30]));
    }

    /// @notice verifies the input handle and make it publicly decryptable
    function createPublicHandle(bytes32 inputHandle, bytes calldata inputProof) public {
        Impl.verify(inputHandle, inputProof, _typeOf(inputHandle));
        Impl.makePubliclyDecryptable(inputHandle);
    }
}
