// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../../lib/Impl.sol";

interface IKMSVerifier {
    function verifyDecryptionEIP712KMSSignatures(
        address aclAddress,
        uint256[] memory handlesList,
        bytes memory decryptedResult,
        bytes[] memory signatures
    ) external returns (bool);
}

interface IGatewayContract {
    function requestDecryption(
        uint256[] calldata ctsHandles,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp,
        bool passSignaturesToCaller
    ) external returns (uint256);
}

struct GatewayConfigStruct {
    address GatewayContractAddress;
}

library Gateway {
    // keccak256(abi.encode(uint256(keccak256("fhevm.storage.GatewayConfig")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant GatewayLocation = 0x93ab6e17f2c461cce6ea5d4ec117e51dda77a64affc2b2c05f8cd440def0e700;

    function getGetwayConfig() internal pure returns (GatewayConfigStruct storage $) {
        assembly {
            $.slot := GatewayLocation
        }
    }

    function setGateway(address gatewayAddress) internal {
        GatewayConfigStruct storage $ = getGetwayConfig();
        $.GatewayContractAddress = gatewayAddress;
    }

    function gatewayContractAddress() internal view returns (address) {
        GatewayConfigStruct storage $ = getGetwayConfig();
        return $.GatewayContractAddress;
    }

    function toUint256(ebool newCT) internal pure returns (uint256 ct) {
        ct = ebool.unwrap(newCT);
    }

    function toUint256(euint4 newCT) internal pure returns (uint256 ct) {
        ct = euint4.unwrap(newCT);
    }

    function toUint256(euint8 newCT) internal pure returns (uint256 ct) {
        ct = euint8.unwrap(newCT);
    }

    function toUint256(euint16 newCT) internal pure returns (uint256 ct) {
        ct = euint16.unwrap(newCT);
    }

    function toUint256(euint32 newCT) internal pure returns (uint256 ct) {
        ct = euint32.unwrap(newCT);
    }

    function toUint256(euint64 newCT) internal pure returns (uint256 ct) {
        ct = euint64.unwrap(newCT);
    }

    function toUint256(euint128 newCT) internal pure returns (uint256 ct) {
        ct = euint128.unwrap(newCT);
    }

    function toUint256(eaddress newCT) internal pure returns (uint256 ct) {
        ct = eaddress.unwrap(newCT);
    }

    function toUint256(euint256 newCT) internal pure returns (uint256 ct) {
        ct = euint256.unwrap(newCT);
    }

    function toUint256(ebytes64 newCT) internal pure returns (uint256 ct) {
        ct = ebytes64.unwrap(newCT);
    }

    function toUint256(ebytes128 newCT) internal pure returns (uint256 ct) {
        ct = ebytes128.unwrap(newCT);
    }

    function toUint256(ebytes256 newCT) internal pure returns (uint256 ct) {
        ct = ebytes256.unwrap(newCT);
    }

    function requestDecryption(
        uint256[] memory ctsHandles,
        bytes4 callbackSelector,
        uint256 msgValue,
        uint256 maxTimestamp,
        bool passSignaturesToCaller
    ) internal returns (uint256 requestID) {
        FHEVMConfigStruct storage $ = Impl.getFHEVMConfig();
        IACL($.ACLAddress).allowForDecryption(ctsHandles);
        GatewayConfigStruct storage $$ = getGetwayConfig();
        requestID = IGatewayContract($$.GatewayContractAddress).requestDecryption(
            ctsHandles,
            callbackSelector,
            msgValue,
            maxTimestamp,
            passSignaturesToCaller
        );
    }

    /// @dev this function is supposed to be called inside the callback function if the dev wants the dApp contract to verify the signatures
    /// @dev this is useful to give dev the choice not to rely on trusting the GatewayContract.
    /// @notice this could be used only when signatures are made available to the callback, i.e when `passSignaturesToCaller` is set to true during request
    function verifySignatures(uint256[] memory handlesList, bytes[] memory signatures) internal returns (bool) {
        uint256 start = 4 + 32; // start position after skipping the selector (4 bytes) and the first argument (index, 32 bytes)
        uint256 length = getSignedDataLength(handlesList);
        bytes memory decryptedResult = new bytes(length);
        assembly {
            calldatacopy(add(decryptedResult, 0x20), start, length) // Copy the relevant part of calldata to decryptedResult memory
        }
        decryptedResult = shiftOffsets(decryptedResult, handlesList);
        FHEVMConfigStruct storage $ = Impl.getFHEVMConfig();
        return
            IKMSVerifier($.KMSVerifierAddress).verifyDecryptionEIP712KMSSignatures(
                $.ACLAddress,
                handlesList,
                decryptedResult,
                signatures
            );
    }

    function getSignedDataLength(uint256[] memory handlesList) private pure returns (uint256) {
        uint256 handlesListlen = handlesList.length;
        uint256 signedDataLength;
        for (uint256 i = 0; i < handlesListlen; i++) {
            uint8 typeCt = uint8(handlesList[i] >> 8);
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
                revert("Unsupported handle type");
            }
        }
        signedDataLength += 32; // add offset of signatures
        return signedDataLength;
    }

    function shiftOffsets(bytes memory input, uint256[] memory handlesList) private pure returns (bytes memory) {
        uint256 numArgs = handlesList.length;
        for (uint256 i = 0; i < numArgs; i++) {
            uint8 typeCt = uint8(handlesList[i] >> 8);
            if (typeCt >= 9) {
                input = subToBytes32Slice(input, 32 * i); // because we append the signatures, all bytes offsets are shifted by 0x20
            }
        }
        input = remove32Slice(input, 32 * numArgs);
        return input;
    }

    function subToBytes32Slice(bytes memory data, uint256 offset) private pure returns (bytes memory) {
        // @note: data is assumed to be more than 32+offset bytes long
        assembly {
            let ptr := add(add(data, 0x20), offset)
            let val := mload(ptr)
            val := sub(val, 0x20)
            mstore(ptr, val)
        }
        return data;
    }

    function remove32Slice(bytes memory input, uint256 start) private pure returns (bytes memory) {
        // @note we assume start+32 is less than input.length
        bytes memory result = new bytes(input.length - 32);

        for (uint256 i = 0; i < start; i++) {
            result[i] = input[i];
        }

        for (uint256 i = start + 32; i < input.length; i++) {
            result[i - 32] = input[i];
        }

        return result;
    }
}
