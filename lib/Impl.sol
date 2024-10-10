// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "./TFHE.sol";
import "./FHEVMConfig.sol";

interface ITFHEExecutor {
    function fheAdd(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheSub(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheMul(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheDiv(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheRem(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheBitAnd(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheBitOr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheBitXor(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheShl(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheShr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheRotl(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheRotr(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheEq(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheNe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheGe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheGt(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheLe(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheLt(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheMin(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheMax(uint256 lhs, uint256 rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheNeg(uint256 ct) external returns (uint256 result);
    function fheNot(uint256 ct) external returns (uint256 result);
    function verifyCiphertext(
        bytes32 inputHandle,
        address callerAddress,
        bytes memory inputProof,
        bytes1 inputType
    ) external returns (uint256 result);
    function cast(uint256 ct, bytes1 toType) external returns (uint256 result);
    function trivialEncrypt(uint256 ct, bytes1 toType) external returns (uint256 result);
    function trivialEncrypt(bytes memory ct, bytes1 toType) external returns (uint256 result);
    function fheEq(uint256 lhs, bytes memory rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheNe(uint256 lhs, bytes memory rhs, bytes1 scalarByte) external returns (uint256 result);
    function fheIfThenElse(uint256 control, uint256 ifTrue, uint256 ifFalse) external returns (uint256 result);
    function fheRand(bytes1 randType) external returns (uint256 result);
    function fheRandBounded(uint256 upperBound, bytes1 randType) external returns (uint256 result);
}

interface IACL {
    function allowTransient(uint256 ciphertext, address account) external;
    function allow(uint256 handle, address account) external;
    function cleanTransientStorage() external;
    function isAllowed(uint256 handle, address account) external view returns (bool);
    function allowForDecryption(uint256[] memory handlesList) external;
}

library Impl {
    // keccak256(abi.encode(uint256(keccak256("fhevm.storage.FHEVMConfig")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant FHEVMConfigLocation = 0xed8d60e34876f751cc8b014c560745351147d9de11b9347c854e881b128ea600;

    function getFHEVMConfig() internal pure returns (FHEVMConfig.FHEVMConfigStruct storage $) {
        assembly {
            $.slot := FHEVMConfigLocation
        }
    }

    function setFHEVM(FHEVMConfig.FHEVMConfigStruct memory fhevmConfig) internal {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        $.ACLAddress = fhevmConfig.ACLAddress;
        $.TFHEExecutorAddress = fhevmConfig.TFHEExecutorAddress;
        $.FHEPaymentAddress = fhevmConfig.FHEPaymentAddress;
        $.KMSVerifierAddress = fhevmConfig.KMSVerifierAddress;
    }

    function add(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheAdd(lhs, rhs, scalarByte);
    }

    function sub(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheSub(lhs, rhs, scalarByte);
    }

    function mul(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheMul(lhs, rhs, scalarByte);
    }

    function div(uint256 lhs, uint256 rhs) internal returns (uint256 result) {
        bytes1 scalarByte = 0x01;
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheDiv(lhs, rhs, scalarByte);
    }

    function rem(uint256 lhs, uint256 rhs) internal returns (uint256 result) {
        bytes1 scalarByte = 0x01;
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheRem(lhs, rhs, scalarByte);
    }

    function and(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheBitAnd(lhs, rhs, scalarByte);
    }

    function or(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheBitOr(lhs, rhs, scalarByte);
    }

    function xor(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheBitXor(lhs, rhs, scalarByte);
    }

    function shl(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheShl(lhs, rhs, scalarByte);
    }

    function shr(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheShr(lhs, rhs, scalarByte);
    }

    function rotl(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheRotl(lhs, rhs, scalarByte);
    }

    function rotr(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheRotr(lhs, rhs, scalarByte);
    }

    function eq(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheEq(lhs, rhs, scalarByte);
    }

    function ne(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheNe(lhs, rhs, scalarByte);
    }

    function ge(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheGe(lhs, rhs, scalarByte);
    }

    function gt(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheGt(lhs, rhs, scalarByte);
    }

    function le(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheLe(lhs, rhs, scalarByte);
    }

    function lt(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheLt(lhs, rhs, scalarByte);
    }

    function min(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheMin(lhs, rhs, scalarByte);
    }

    function max(uint256 lhs, uint256 rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheMax(lhs, rhs, scalarByte);
    }

    function neg(uint256 ct) internal returns (uint256 result) {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheNeg(ct);
    }

    function not(uint256 ct) internal returns (uint256 result) {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheNot(ct);
    }

    // If 'control's value is 'true', the result has the same value as 'ifTrue'.
    // If 'control's value is 'false', the result has the same value as 'ifFalse'.
    function select(uint256 control, uint256 ifTrue, uint256 ifFalse) internal returns (uint256 result) {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheIfThenElse(control, ifTrue, ifFalse);
    }

    function verify(bytes32 inputHandle, bytes memory inputProof, uint8 toType) internal returns (uint256 result) {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).verifyCiphertext(
            inputHandle,
            msg.sender,
            inputProof,
            bytes1(toType)
        );
        IACL($.ACLAddress).allowTransient(result, msg.sender);
    }

    function cast(uint256 ciphertext, uint8 toType) internal returns (uint256 result) {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).cast(ciphertext, bytes1(toType));
    }

    function trivialEncrypt(uint256 value, uint8 toType) internal returns (uint256 result) {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).trivialEncrypt(value, bytes1(toType));
    }

    function trivialEncrypt(bytes memory value, uint8 toType) internal returns (uint256 result) {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).trivialEncrypt(value, bytes1(toType));
    }

    function eq(uint256 lhs, bytes memory rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheEq(lhs, rhs, scalarByte);
    }

    function ne(uint256 lhs, bytes memory rhs, bool scalar) internal returns (uint256 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheNe(lhs, rhs, scalarByte);
    }

    function rand(uint8 randType) internal returns (uint256 result) {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheRand(bytes1(randType));
    }

    function randBounded(uint256 upperBound, uint8 randType) internal returns (uint256 result) {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheRandBounded(upperBound, bytes1(randType));
    }

    function allowTransient(uint256 handle, address account) internal {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        IACL($.ACLAddress).allowTransient(handle, account);
    }

    function allow(uint256 handle, address account) internal {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        IACL($.ACLAddress).allow(handle, account);
    }

    function cleanTransientStorage() internal {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        IACL($.ACLAddress).cleanTransientStorage();
    }

    function isAllowed(uint256 handle, address account) internal view returns (bool) {
        FHEVMConfig.FHEVMConfigStruct storage $ = getFHEVMConfig();
        return IACL($.ACLAddress).isAllowed(handle, account);
    }
}
