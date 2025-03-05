// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./TFHE.sol";

/**
 * @title   FHEVMConfigStruct
 * @notice  This struct contains all addresses of core contracts, which are needed in a typical dApp.
 */
struct FHEVMConfigStruct {
    address ACLAddress;
    address TFHEExecutorAddress;
    address KMSVerifierAddress;
    address InputVerifierAddress;
}

/**
 * @title   ITFHEExecutor
 * @notice  This interface contains all functions to conduct FHE operations.
 */
interface ITFHEExecutor {
    /**
     * @notice              Computes fheAdd operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheAdd(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheSub operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheSub(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheMul operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMul(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheDiv operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheDiv(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheRem operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRem(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheBitAnd operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitAnd(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheBitOr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitOr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheBitXor operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheBitXor(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheShl operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheShl(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheShr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheShr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheRotl operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRotl(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheRotr operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheRotr(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheEq operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheEq(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheNe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheNe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheGe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheGe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheGt operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheGt(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheLe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheLe(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheLt operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheLt(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheMin operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMin(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheMax operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheMax(bytes32 lhs, bytes32 rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes fheNeg operation.
     * @param ct            Ct
     * @return result       Result.
     */
    function fheNeg(bytes32 ct) external returns (bytes32 result);

    /**
     * @notice              Computes fheNot operation.
     * @param ct            Ct
     * @return result       Result.
     */
    function fheNot(bytes32 ct) external returns (bytes32 result);
    /**
     * @notice                Verifies the ciphertext.
     * @param inputHandle     Input handle.
     * @param callerAddress   Address of the caller.
     * @param inputProof      Input proof.
     * @param inputType       Input type.
     * @return result         Result.
     */
    function verifyCiphertext(
        bytes32 inputHandle,
        address callerAddress,
        bytes memory inputProof,
        bytes1 inputType
    ) external returns (bytes32 result);

    /**
     * @notice          Performs the casting to a target type.
     * @param ct        Value to cast.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function cast(bytes32 ct, bytes1 toType) external returns (bytes32 result);

    /**
     * @notice          Does trivial encryption.
     * @param ct        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(uint256 ct, bytes1 toType) external returns (bytes32 result);

    /**
     * @notice          Does trivial encryption.
     * @param ct        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(bytes memory ct, bytes1 toType) external returns (bytes32 result);

    /**
     * @notice              Computes FHEEq operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheEq(bytes32 lhs, bytes memory rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes FHENe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalarByte    Scalar byte.
     * @return result       Result.
     */
    function fheNe(bytes32 lhs, bytes memory rhs, bytes1 scalarByte) external returns (bytes32 result);

    /**
     * @notice              Computes FHEIfThenElse operation.
     * @param control       Control value.
     * @param ifTrue        If true.
     * @param ifFalse       If false.
     * @return result       Result.
     */
    function fheIfThenElse(bytes32 control, bytes32 ifTrue, bytes32 ifFalse) external returns (bytes32 result);

    /**
     * @notice              Computes FHERand operation.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRand(bytes1 randType) external returns (bytes32 result);

    /**
     * @notice              Computes FHERandBounded operation.
     * @param upperBound    Upper bound value.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRandBounded(uint256 upperBound, bytes1 randType) external returns (bytes32 result);
}

/**
 * @title   IACL.
 * @notice  This interface contains all functions that are used to conduct operations
 *          with the ACL contract.
 */
interface IACL {
    /**
     * @notice              Allows the use of handle by address account for this transaction.
     * @dev                 The caller must be allowed to use handle for allowTransient() to succeed.
     *                      If not, allowTransient() reverts.
     *                      The Coprocessor contract can always allowTransient(), contrarily to allow().
     * @param ciphertext    Ciphertext.
     * @param account       Address of the account.
     */
    function allowTransient(bytes32 ciphertext, address account) external;

    /**
     * @notice              Allows the use of handle for the address account.
     * @dev                 The caller must be allowed to use handle for allow() to succeed. If not, allow() reverts.
     * @param handle        Handle.
     * @param account       Address of the account.
     */
    function allow(bytes32 handle, address account) external;

    /**
     * @dev This function removes the transient allowances, which could be useful for integration with
     *      Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor.
     */
    function cleanTransientStorage() external;

    /**
     * @notice              Returns whether the account is allowed to use the handle, either due to
     *                      allowTransient() or allow().
     * @param handle        Handle.
     * @param account       Address of the account.
     * @return isAllowed    Whether the account can access the handle.
     */
    function isAllowed(bytes32 handle, address account) external view returns (bool);

    /**
     * @notice              Allows a list of handles to be decrypted.
     * @param handlesList   List of handles.
     */
    function allowForDecryption(bytes32[] memory handlesList) external;
}

/**
 * @title IInputVerifier
 * @notice This interface contains the only function required from InputVerifier.
 */
interface IInputVerifier {
    /**
     * @dev This function removes the transient allowances, which could be useful for integration with
     *      Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor.
     */
    function cleanTransientStorage() external;
}

/**
 * @title   Impl
 * @notice  This library is the core implementation for computing FHE operations (e.g. add, sub, xor).
 */
library Impl {
    /// @dev keccak256(abi.encode(uint256(keccak256("fhevm.storage.FHEVMConfig")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant FHEVMConfigLocation = 0xed8d60e34876f751cc8b014c560745351147d9de11b9347c854e881b128ea600;

    /**
     * @dev Returns the FHEVM config.
     */
    function getFHEVMConfig() internal pure returns (FHEVMConfigStruct storage $) {
        assembly {
            $.slot := FHEVMConfigLocation
        }
    }

    /**
     * @notice            Sets the FHEVM addresses.
     * @param fhevmConfig FHEVM config struct that contains contract addresses.
     */
    function setFHEVM(FHEVMConfigStruct memory fhevmConfig) internal {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        $.ACLAddress = fhevmConfig.ACLAddress;
        $.TFHEExecutorAddress = fhevmConfig.TFHEExecutorAddress;
        $.KMSVerifierAddress = fhevmConfig.KMSVerifierAddress;
        $.InputVerifierAddress = fhevmConfig.InputVerifierAddress;
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function add(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheAdd(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function sub(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheSub(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function mul(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheMul(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function div(bytes32 lhs, bytes32 rhs) internal returns (bytes32 result) {
        bytes1 scalarByte = 0x01;
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheDiv(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function rem(bytes32 lhs, bytes32 rhs) internal returns (bytes32 result) {
        bytes1 scalarByte = 0x01;
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheRem(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function and(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheBitAnd(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function or(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheBitOr(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function xor(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheBitXor(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function shl(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheShl(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function shr(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheShr(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function rotl(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheRotl(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function rotr(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheRotr(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function eq(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheEq(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function ne(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheNe(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function ge(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheGe(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function gt(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheGt(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function le(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheLe(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function lt(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheLt(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function min(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheMin(lhs, rhs, scalarByte);
    }

    /**
     * @dev Returns the FHEVM config.
     */
    function max(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheMax(lhs, rhs, scalarByte);
    }

    function neg(bytes32 ct) internal returns (bytes32 result) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheNeg(ct);
    }

    function not(bytes32 ct) internal returns (bytes32 result) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheNot(ct);
    }

    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *         If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(bytes32 control, bytes32 ifTrue, bytes32 ifFalse) internal returns (bytes32 result) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheIfThenElse(control, ifTrue, ifFalse);
    }

    /**
     * @notice              Verifies the ciphertext (TFHEExecutor) and allows transient (ACL).
     * @param inputHandle   Input handle.
     * @param inputProof    Input proof.
     * @param toType        Input type.
     * @return result       Result.
     */
    function verify(bytes32 inputHandle, bytes memory inputProof, uint8 toType) internal returns (bytes32 result) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).verifyCiphertext(
            inputHandle,
            msg.sender,
            inputProof,
            bytes1(toType)
        );
        IACL($.ACLAddress).allowTransient(result, msg.sender);
    }

    /**
     * @notice            Performs the casting to a target type.
     * @param ciphertext  Ciphertext to cast.
     * @param toType      Target type.
     * @return result     Result value of the target type.
     */
    function cast(bytes32 ciphertext, uint8 toType) internal returns (bytes32 result) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).cast(ciphertext, bytes1(toType));
    }

    /**
     * @notice          Does trivial encryption.
     * @param value     Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(uint256 value, uint8 toType) internal returns (bytes32 result) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).trivialEncrypt(value, bytes1(toType));
    }

    /**
     * @notice          Does trivial encryption.
     * @param value     Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(bytes memory value, uint8 toType) internal returns (bytes32 result) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).trivialEncrypt(value, bytes1(toType));
    }

    /**
     * @notice              Computes FHEEq operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalar        Scalar byte.
     * @return result       Result.
     */
    function eq(bytes32 lhs, bytes memory rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheEq(lhs, rhs, scalarByte);
    }

    /**
     * @notice              Computes FHENe operation.
     * @param lhs           LHS.
     * @param rhs           RHS.
     * @param scalar        Scalar byte.
     * @return result       Result.
     */
    function ne(bytes32 lhs, bytes memory rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheNe(lhs, rhs, scalarByte);
    }

    function rand(uint8 randType) internal returns (bytes32 result) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheRand(bytes1(randType));
    }

    function randBounded(uint256 upperBound, uint8 randType) internal returns (bytes32 result) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        result = ITFHEExecutor($.TFHEExecutorAddress).fheRandBounded(upperBound, bytes1(randType));
    }

    /**
     * @notice              Allows the use of handle by address account for this transaction.
     * @dev                 The caller must be allowed to use handle for allowTransient() to succeed.
     *                      If not, allowTransient() reverts.
     *                      The Coprocessor contract can always allowTransient(), contrarily to allow().
     * @param handle        Handle.
     * @param account       Address of the account.
     */
    function allowTransient(bytes32 handle, address account) internal {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        IACL($.ACLAddress).allowTransient(handle, account);
    }

    /**
     * @notice              Allows the use of handle for the address account.
     * @dev                 The caller must be allowed to use handle for allow() to succeed. If not, allow() reverts.
     * @param handle        Handle.
     * @param account       Address of the account.
     */
    function allow(bytes32 handle, address account) internal {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        IACL($.ACLAddress).allow(handle, account);
    }

    /**
     * @dev This function removes the transient allowances in the ACL, which could be useful for integration
     *      with Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor.
     */
    function cleanTransientStorageACL() internal {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        IACL($.ACLAddress).cleanTransientStorage();
    }

    /**
     * @dev This function removes the transient proofs in the InputVerifier, which could be useful for integration
     *      with Account Abstraction when bundling several UserOps calling the TFHEExecutorCoprocessor.
     */
    function cleanTransientStorageInputVerifier() internal {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        IInputVerifier($.InputVerifierAddress).cleanTransientStorage();
    }

    /**
     * @notice              Returns whether the account is allowed to use the handle, either due to
     *                      allowTransient() or allow().
     * @param handle        Handle.
     * @param account       Address of the account.
     * @return isAllowed    Whether the account can access the handle.
     */
    function isAllowed(bytes32 handle, address account) internal view returns (bool) {
        FHEVMConfigStruct storage $ = getFHEVMConfig();
        return IACL($.ACLAddress).isAllowed(handle, account);
    }
}
