// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "../lib/Impl.sol";

import "../lib/FHEVMConfig.sol";

interface IKMSVerifier {
    function verifyDecryptionEIP712KMSSignatures(
        bytes32[] memory handlesList,
        bytes memory decryptedResult,
        bytes[] memory signatures
    ) external returns (bool);
}

interface IDecryptionOracle {
    function requestDecryption(uint256 requestID, bytes32[] calldata ctsHandles, bytes4 callbackSelector) external;
}

struct DecryptionOracleConfigStruct {
    address DecryptionOracleAddress;
}

abstract contract DecryptionOracleCaller {
    error HandlesAlreadySavedForRequestID();
    error NoHandleFoundForRequestID();
    error InvalidKMSSignatures();
    error UnsupportedHandleType();

    uint256 internal counterRequest;
    mapping(uint256 => bytes32[]) private requestedHandles;
    mapping(uint256 => ebool[]) private paramsEBool;
    mapping(uint256 => euint4[]) private paramsEUint4;
    mapping(uint256 => euint8[]) private paramsEUint8;
    mapping(uint256 => euint16[]) private paramsEUint16;
    mapping(uint256 => euint32[]) private paramsEUint32;
    mapping(uint256 => euint64[]) private paramsEUint64;
    mapping(uint256 => eaddress[]) private paramsEAddress;
    mapping(uint256 => address[]) private paramsAddress;
    mapping(uint256 => uint256[]) private paramsUint256;

    event DecryptionFulfilled(uint256 indexed requestID);

    function addParamsEBool(uint256 requestID, ebool _ebool) internal {
        paramsEBool[requestID].push(_ebool);
    }

    function addParamsEUint4(uint256 requestID, euint4 _euint4) internal {
        paramsEUint4[requestID].push(_euint4);
    }

    function addParamsEUint8(uint256 requestID, euint8 _euint8) internal {
        paramsEUint8[requestID].push(_euint8);
    }

    function addParamsEUint16(uint256 requestID, euint16 _euint16) internal {
        paramsEUint16[requestID].push(_euint16);
    }

    function addParamsEUint32(uint256 requestID, euint32 _euint32) internal {
        paramsEUint32[requestID].push(_euint32);
    }

    function addParamsEUint64(uint256 requestID, euint64 _euint64) internal {
        paramsEUint64[requestID].push(_euint64);
    }

    function addParamsEAddress(uint256 requestID, eaddress _eaddress) internal {
        paramsEAddress[requestID].push(_eaddress);
    }

    function addParamsAddress(uint256 requestID, address _address) internal {
        paramsAddress[requestID].push(_address);
    }

    function addParamsUint256(uint256 requestID, uint256 _uint) internal {
        paramsUint256[requestID].push(_uint);
    }

    function saveRequestedHandles(uint256 requestID, bytes32[] memory handlesList) internal {
        if (requestedHandles[requestID].length != 0) {
            revert HandlesAlreadySavedForRequestID();
        }
        requestedHandles[requestID] = handlesList;
    }

    function loadRequestedHandles(uint256 requestID) internal view returns (bytes32[] memory) {
        if (requestedHandles[requestID].length == 0) {
            revert NoHandleFoundForRequestID();
        }
        return requestedHandles[requestID];
    }

    function getParamsEBool(uint256 requestID) internal view returns (ebool[] memory) {
        return paramsEBool[requestID];
    }

    function getParamsEUint4(uint256 requestID) internal view returns (euint4[] memory) {
        return paramsEUint4[requestID];
    }

    function getParamsEUint8(uint256 requestID) internal view returns (euint8[] memory) {
        return paramsEUint8[requestID];
    }

    function getParamsEUint16(uint256 requestID) internal view returns (euint16[] memory) {
        return paramsEUint16[requestID];
    }

    function getParamsEUint32(uint256 requestID) internal view returns (euint32[] memory) {
        return paramsEUint32[requestID];
    }

    function getParamsEUint64(uint256 requestID) internal view returns (euint64[] memory) {
        return paramsEUint64[requestID];
    }

    function getParamsEAddress(uint256 requestID) internal view returns (eaddress[] memory) {
        return paramsEAddress[requestID];
    }

    function getParamsAddress(uint256 requestID) internal view returns (address[] memory) {
        return paramsAddress[requestID];
    }

    function getParamsUint256(uint256 requestID) internal view returns (uint256[] memory) {
        return paramsUint256[requestID];
    }

    // keccak256(abi.encode(uint256(keccak256("fhevm.storage.DecryptionOracleConfig")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant DecryptionOracleLocation =
        0xde725b831312c6c6cbab76ff954da6836e26d78fc467781757c36be9c3d67d00;

    function getDecryptionOracleConfig() internal pure returns (DecryptionOracleConfigStruct storage $) {
        assembly {
            $.slot := DecryptionOracleLocation
        }
    }

    function setDecryptionOracle(address decryptionOracleAddress) internal {
        DecryptionOracleConfigStruct storage $ = getDecryptionOracleConfig();
        $.DecryptionOracleAddress = decryptionOracleAddress;
    }

    function gatewayContractAddress() internal view returns (address) {
        DecryptionOracleConfigStruct storage $ = getDecryptionOracleConfig();
        return $.DecryptionOracleAddress;
    }

    function toBytes32(ebool newCT) internal pure returns (bytes32 ct) {
        ct = ebool.unwrap(newCT);
    }

    function toBytes32(euint4 newCT) internal pure returns (bytes32 ct) {
        ct = euint4.unwrap(newCT);
    }

    function toBytes32(euint8 newCT) internal pure returns (bytes32 ct) {
        ct = euint8.unwrap(newCT);
    }

    function toBytes32(euint16 newCT) internal pure returns (bytes32 ct) {
        ct = euint16.unwrap(newCT);
    }

    function toBytes32(euint32 newCT) internal pure returns (bytes32 ct) {
        ct = euint32.unwrap(newCT);
    }

    function toBytes32(euint64 newCT) internal pure returns (bytes32 ct) {
        ct = euint64.unwrap(newCT);
    }

    function toBytes32(euint128 newCT) internal pure returns (bytes32 ct) {
        ct = euint128.unwrap(newCT);
    }

    function toBytes32(eaddress newCT) internal pure returns (bytes32 ct) {
        ct = eaddress.unwrap(newCT);
    }

    function toBytes32(euint256 newCT) internal pure returns (bytes32 ct) {
        ct = euint256.unwrap(newCT);
    }

    function toBytes32(ebytes64 newCT) internal pure returns (bytes32 ct) {
        ct = ebytes64.unwrap(newCT);
    }

    function toBytes32(ebytes128 newCT) internal pure returns (bytes32 ct) {
        ct = ebytes128.unwrap(newCT);
    }

    function toBytes32(ebytes256 newCT) internal pure returns (bytes32 ct) {
        ct = ebytes256.unwrap(newCT);
    }

    function requestDecryption(
        bytes32[] memory ctsHandles,
        bytes4 callbackSelector
    ) internal returns (uint256 requestID) {
        requestID = counterRequest;
        FHEVMConfigStruct storage $ = Impl.getFHEVMConfig();
        IACL($.ACLAddress).allowForDecryption(ctsHandles);
        DecryptionOracleConfigStruct storage $$ = getDecryptionOracleConfig();
        IDecryptionOracle($$.DecryptionOracleAddress).requestDecryption(requestID, ctsHandles, callbackSelector);
        saveRequestedHandles(requestID, ctsHandles);
        counterRequest++;
    }

    /// @dev this function should be called inside the callback function the dApp contract to verify the signatures
    function verifySignatures(bytes32[] memory handlesList, bytes[] memory signatures) internal returns (bool) {
        uint256 start = 4 + 32; // start position after skipping the selector (4 bytes) and the first argument (index, 32 bytes)
        uint256 length = getSignedDataLength(handlesList);
        bytes memory decryptedResult = new bytes(length);
        assembly {
            calldatacopy(add(decryptedResult, 0x20), start, length) // Copy the relevant part of calldata to decryptedResult memory
        }
        FHEVMConfigStruct storage $ = Impl.getFHEVMConfig();
        return
            IKMSVerifier($.KMSVerifierAddress).verifyDecryptionEIP712KMSSignatures(
                handlesList,
                decryptedResult,
                signatures
            );
    }

    function getSignedDataLength(bytes32[] memory handlesList) private pure returns (uint256) {
        uint256 handlesListlen = handlesList.length;
        uint256 signedDataLength;
        for (uint256 i = 0; i < handlesListlen; i++) {
            uint8 typeCt = uint8(handlesList[i][30]);
            if (typeCt < 9) {
                signedDataLength += 32;
            } else if (typeCt == 9) {
                //ebytes64
                signedDataLength += 128;
            } else if (typeCt == 10) {
                //ebytes128
                signedDataLength += 192;
            } else if (typeCt == 11) {
                //ebytes256
                signedDataLength += 320;
            } else {
                revert UnsupportedHandleType();
            }
        }
        signedDataLength += 32; // add offset of signatures
        return signedDataLength;
    }

    modifier checkSignatures(uint256 requestID, bytes[] memory signatures) {
        bytes32[] memory handlesList = loadRequestedHandles(requestID);
        bool isVerified = verifySignatures(handlesList, signatures);
        if (!isVerified) {
            revert InvalidKMSSignatures();
        }
        _;
        emit DecryptionFulfilled(requestID);
    }
}
