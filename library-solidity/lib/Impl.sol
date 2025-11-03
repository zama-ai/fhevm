// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FheType} from "./FheType.sol";

/**
 * @title   CoprocessorConfig
 * @notice  This struct contains all addresses of core contracts, which are needed in a typical dApp.
 */
struct CoprocessorConfig {
    address ACLAddress;
    address CoprocessorAddress;
    address KMSVerifierAddress;
}

/**
 * @title   IFHEVMExecutor
 * @notice  This interface contains all functions to conduct FHE operations.
 */
interface IFHEVMExecutor {
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
    function verifyInput(
        bytes32 inputHandle,
        address callerAddress,
        bytes memory inputProof,
        FheType inputType
    ) external returns (bytes32 result);

    /**
     * @notice          Performs the casting to a target type.
     * @param ct        Value to cast.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function cast(bytes32 ct, FheType toType) external returns (bytes32 result);

    /**
     * @notice          Does trivial encryption.
     * @param ct        Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(uint256 ct, FheType toType) external returns (bytes32 result);

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
    function fheRand(FheType randType) external returns (bytes32 result);

    /**
     * @notice              Computes FHERandBounded operation.
     * @param upperBound    Upper bound value.
     * @param randType      Type for the random result.
     * @return result       Result.
     */
    function fheRandBounded(uint256 upperBound, FheType randType) external returns (bytes32 result);

    /**
     * @notice                      Returns the address of the InputVerifier contract used by the coprocessor.
     * @return inputVerifierAddress Address of the InputVerifier.
     */
    function getInputVerifierAddress() external view returns (address);
}

/**
 * @title   IACL.
 * @notice  This interface contains all functions that are used to conduct operations
 *          with the ACL contract.
 */
interface IACL {
    /**
     * @notice              Executes a batch of encoded calls on the ACL contract.
     * @param data          Array containing the ABI-encoded function calls.
     * @return results      Return payloads for each call in `data`.
     */
    function multicall(bytes[] calldata data) external payable returns (bytes[] memory results);

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
     *      Account Abstraction when bundling several UserOps calling the FHEVMExecutor Coprocessor.
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

    /**
     * @notice                  Returns wether a handle is allowed to be publicly decrypted.
     * @param handle            Handle.
     * @return isDecryptable    Whether the handle can be publicly decrypted.
     */
    function isAllowedForDecryption(bytes32 handle) external view returns (bool);

    /**
     * @notice              Returns whether the account is persistently allowed to use the handle.
     * @param handle        Handle.
     * @param account       Address of the account.
     */
    function persistAllowed(bytes32 handle, address account) external view returns (bool);

    /**
     * @notice                  Returns whether the account is on the deny list.
     * @param account           Address of the account.
     * @return isAccountDenied  Whether the account is on the deny list.
     */
    function isAccountDenied(address account) external view returns (bool);

    /**
     * @notice              Delegates user decryption rights to `delegate` for the specified `contractAddress`.
     * @param delegate      The delegate account.
     * @param contractAddress The contract address forming the user decryption context.
     * @param expirationDate UNIX timestamp when the delegation expires.
     */
    function delegateForUserDecryption(address delegate, address contractAddress, uint64 expirationDate) external;

    /**
     * @notice              Revokes previously delegated user decryption rights.
     * @param delegate      The delegate account.
     * @param contractAddress The contract address forming the user decryption context.
     */
    function revokeDelegationForUserDecryption(address delegate, address contractAddress) external;

    /**
     * @notice              Returns the expiration date for delegated user decryption rights.
     * @param delegator     The delegator account.
     * @param delegate      The delegate account.
     * @param contractAddress The contract address forming the user decryption context.
     */
    function getUserDecryptionDelegationExpirationDate(
        address delegator,
        address delegate,
        address contractAddress
    ) external view returns (uint64);

    /**
     * @notice Returns whether an account is delegated to access the handle for user decryption.
     * @param delegator The address of the account that delegates access to its handles.
     * @param delegate The address of the account that receives the delegation.
     * @param contractAddress The contract address to delegate access to.
     * @param handle The handle to check for delegated user decryption.
     * @return isDelegatedForUserDecryption Whether the handle can be accessed for delegated user decryption.
     */
    function isHandleDelegatedForUserDecryption(
        address delegator,
        address delegate,
        address contractAddress,
        bytes32 handle
    ) external view returns (bool);
}

/**
 * @title IInputVerifier
 * @notice This interface contains the only function required from InputVerifier.
 */
interface IInputVerifier {
    /**
     * @dev This function removes the transient allowances, which could be useful for integration with
     *      Account Abstraction when bundling several UserOps calling the FHEVMExecutor Coprocessor.
     */
    function cleanTransientStorage() external;
}

/**
 * @title   Impl
 * @notice  This library is the core implementation for computing FHE operations (e.g. add, sub, xor).
 */
library Impl {
    /// keccak256(abi.encode(uint256(keccak256("confidential.storage.config")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant CoprocessorConfigLocation =
        0x9e7b61f58c47dc699ac88507c4f5bb9f121c03808c5676a8078fe583e4649700;

    /**
     * @dev Returns the Coprocessor config.
     */
    function getCoprocessorConfig() internal pure returns (CoprocessorConfig storage $) {
        assembly {
            $.slot := CoprocessorConfigLocation
        }
    }

    /**
     * @notice                  Sets the coprocessor addresses.
     * @param coprocessorConfig Coprocessor config struct that contains contract addresses.
     */
    function setCoprocessor(CoprocessorConfig memory coprocessorConfig) internal {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        $.ACLAddress = coprocessorConfig.ACLAddress;
        $.CoprocessorAddress = coprocessorConfig.CoprocessorAddress;
        $.KMSVerifierAddress = coprocessorConfig.KMSVerifierAddress;
    }

    function add(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheAdd(lhs, rhs, scalarByte);
    }

    function sub(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheSub(lhs, rhs, scalarByte);
    }

    function mul(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheMul(lhs, rhs, scalarByte);
    }

    function div(bytes32 lhs, bytes32 rhs) internal returns (bytes32 result) {
        bytes1 scalarByte = 0x01;
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheDiv(lhs, rhs, scalarByte);
    }

    function rem(bytes32 lhs, bytes32 rhs) internal returns (bytes32 result) {
        bytes1 scalarByte = 0x01;
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheRem(lhs, rhs, scalarByte);
    }

    function and(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheBitAnd(lhs, rhs, scalarByte);
    }

    function or(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheBitOr(lhs, rhs, scalarByte);
    }

    function xor(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheBitXor(lhs, rhs, scalarByte);
    }

    function shl(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheShl(lhs, rhs, scalarByte);
    }

    function shr(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheShr(lhs, rhs, scalarByte);
    }

    function rotl(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheRotl(lhs, rhs, scalarByte);
    }

    function rotr(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheRotr(lhs, rhs, scalarByte);
    }

    function eq(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheEq(lhs, rhs, scalarByte);
    }

    function ne(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheNe(lhs, rhs, scalarByte);
    }

    function ge(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheGe(lhs, rhs, scalarByte);
    }

    function gt(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheGt(lhs, rhs, scalarByte);
    }

    function le(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheLe(lhs, rhs, scalarByte);
    }

    function lt(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheLt(lhs, rhs, scalarByte);
    }

    function min(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheMin(lhs, rhs, scalarByte);
    }

    function max(bytes32 lhs, bytes32 rhs, bool scalar) internal returns (bytes32 result) {
        bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheMax(lhs, rhs, scalarByte);
    }

    function neg(bytes32 ct) internal returns (bytes32 result) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheNeg(ct);
    }

    function not(bytes32 ct) internal returns (bytes32 result) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheNot(ct);
    }

    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(bytes32 control, bytes32 ifTrue, bytes32 ifFalse) internal returns (bytes32 result) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheIfThenElse(control, ifTrue, ifFalse);
    }

    /**
     * @notice              Verifies the ciphertext (FHEVMExecutor) and allows transient (ACL).
     * @param inputHandle   Input handle.
     * @param inputProof    Input proof.
     * @param toType        Input type.
     * @return result       Result.
     */
    function verify(bytes32 inputHandle, bytes memory inputProof, FheType toType) internal returns (bytes32 result) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).verifyInput(inputHandle, msg.sender, inputProof, toType);
        IACL($.ACLAddress).allowTransient(result, msg.sender);
    }

    /**
     * @notice            Performs the casting to a target type.
     * @param ciphertext  Ciphertext to cast.
     * @param toType      Target type.
     * @return result     Result value of the target type.
     */
    function cast(bytes32 ciphertext, FheType toType) internal returns (bytes32 result) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).cast(ciphertext, toType);
    }

    /**
     * @notice          Does trivial encryption.
     * @param value     Value to encrypt.
     * @param toType    Target type.
     * @return result   Result value of the target type.
     */
    function trivialEncrypt(uint256 value, FheType toType) internal returns (bytes32 result) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).trivialEncrypt(value, toType);
    }

    function rand(FheType randType) internal returns (bytes32 result) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheRand(randType);
    }

    function randBounded(uint256 upperBound, FheType randType) internal returns (bytes32 result) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).fheRandBounded(upperBound, randType);
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
        CoprocessorConfig storage $ = getCoprocessorConfig();
        IACL($.ACLAddress).allowTransient(handle, account);
    }

    /**
     * @notice              Allows the use of handle for the address account.
     * @dev                 The caller must be allowed to use handle for allow() to succeed. If not, allow() reverts.
     * @param handle        Handle.
     * @param account       Address of the account.
     */
    function allow(bytes32 handle, address account) internal {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        IACL($.ACLAddress).allow(handle, account);
    }

    /**
     * @notice              Allows the handle to be publicly decryptable.
     * @dev                 The caller must be allowed to use handle for makePubliclyDecryptable() to succeed.
     *                      If not, makePubliclyDecryptable() reverts.
     * @param handle        Handle.
     */
    function makePubliclyDecryptable(bytes32 handle) internal {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        bytes32[] memory handleArray = new bytes32[](1);
        handleArray[0] = handle;
        IACL($.ACLAddress).allowForDecryption(handleArray);
    }

    /**
     * @dev This function removes the transient allowances in the ACL, which could be useful for integration
     *      with Account Abstraction when bundling several UserOps calling the FHEVMExecutor Coprocessor.
     */
    function cleanTransientStorageACL() internal {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        IACL($.ACLAddress).cleanTransientStorage();
    }

    /**
     * @dev This function removes the transient proofs in the InputVerifier, which could be useful for integration
     *      with Account Abstraction when bundling several UserOps calling the FHEVMExecutor Coprocessor.
     */
    function cleanTransientStorageInputVerifier() internal {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        address inputVerifierAddress = IFHEVMExecutor($.CoprocessorAddress).getInputVerifierAddress();
        IInputVerifier(inputVerifierAddress).cleanTransientStorage();
    }

    /**
     * @notice              Returns whether the account is allowed to use the handle, either due to
     *                      allowTransient() or allow().
     * @param handle        Handle.
     * @param account       Address of the account.
     * @return isAllowed    Whether the account can access the handle.
     */
    function isAllowed(bytes32 handle, address account) internal view returns (bool) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        return IACL($.ACLAddress).isAllowed(handle, account);
    }

    /**
     * @notice              Returns whether the handle is allowed to be publicly decrypted.
     * @param handle        Handle.
     * @return isAllowed    Whether the handle can be publicly decrypted.
     */
    function isPubliclyDecryptable(bytes32 handle) internal view returns (bool) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        return IACL($.ACLAddress).isAllowedForDecryption(handle);
    }

    /**
     * @notice              Returns whether the account is persistently allowed to use the handle.
     * @param handle        Handle.
     * @param account       Address of the account.
     * @return isAllowed    Whether the account can access the handle persistently.
     */
    function persistAllowed(bytes32 handle, address account) internal view returns (bool) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        return IACL($.ACLAddress).persistAllowed(handle, account);
    }

    /**
     * @notice                  Returns whether the account is on the deny list.
     * @param account           Address of the account.
     * @return isAccountDenied  Whether the account is on the deny list.
     */
    function isAccountDenied(address account) internal view returns (bool) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        return IACL($.ACLAddress).isAccountDenied(account);
    }

    /**
     * @notice              Delegates user decryption rights to `delegate` for the specified `contractAddress`.
     * @param delegate      The delegate account.
     * @param contractAddress The contract address forming the user decryption context.
     * @param expirationDate UNIX timestamp when the delegation expires.
     */
    function delegateForUserDecryption(address delegate, address contractAddress, uint64 expirationDate) internal {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        IACL($.ACLAddress).delegateForUserDecryption(delegate, contractAddress, expirationDate);
    }

    /**
     * @notice              Delegates user decryption rights in batch leveraging the ACL multicall helper.
     * @param delegate      The delegate account.
     * @param contractAddresses Array of contract addresses forming the user decryption contexts.
     * @param expirationDate UNIX timestamp when the delegation expires.
     */
    function delegateForUserDecryptions(
        address delegate,
        address[] memory contractAddresses,
        uint64 expirationDate
    ) internal {
        uint256 length = contractAddresses.length;
        if (length == 0) {
            return;
        }

        CoprocessorConfig storage $ = getCoprocessorConfig();

        if (length == 1) {
            IACL($.ACLAddress).delegateForUserDecryption(delegate, contractAddresses[0], expirationDate);
            return;
        }

        bytes[] memory calls = new bytes[](length);
        for (uint256 i = 0; i < length; ++i) {
            calls[i] = abi.encodeCall(IACL.delegateForUserDecryption, (delegate, contractAddresses[i], expirationDate));
        }
        IACL($.ACLAddress).multicall(calls);
    }

    /**
     * @notice              Revokes previously delegated user decryption rights.
     * @param delegate      The delegate account.
     * @param contractAddress The contract address forming the user decryption context.
     */
    function revokeDelegationForUserDecryption(address delegate, address contractAddress) internal {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        IACL($.ACLAddress).revokeDelegationForUserDecryption(delegate, contractAddress);
    }

    /**
     * @notice              Revokes delegated user decryption rights in batch leveraging the ACL multicall helper.
     * @param delegate      The delegate account.
     * @param contractAddresses Array of contract addresses forming the user decryption contexts.
     */
    function revokeDelegationsForUserDecryption(address delegate, address[] memory contractAddresses) internal {
        uint256 length = contractAddresses.length;
        if (length == 0) {
            return;
        }

        CoprocessorConfig storage $ = getCoprocessorConfig();

        if (length == 1) {
            IACL($.ACLAddress).revokeDelegationForUserDecryption(delegate, contractAddresses[0]);
            return;
        }

        bytes[] memory calls = new bytes[](length);
        for (uint256 i = 0; i < length; ++i) {
            calls[i] = abi.encodeCall(IACL.revokeDelegationForUserDecryption, (delegate, contractAddresses[i]));
        }
        IACL($.ACLAddress).multicall(calls);
    }

    /**
     * @notice              Returns the expiration date for delegated user decryption rights.
     * @param delegator     The delegator account.
     * @param delegate      The delegate account.
     * @param contractAddress The contract address forming the user decryption context.
     * @return expirationDate The UNIX timestamp when the delegation expires.
     */
    function getUserDecryptionDelegationExpirationDate(
        address delegator,
        address delegate,
        address contractAddress
    ) internal view returns (uint64) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        return IACL($.ACLAddress).getUserDecryptionDelegationExpirationDate(delegator, delegate, contractAddress);
    }

    /**
     * @notice              Returns whether the handle is delegated for user decryption.
     * @param delegator     The delegator account.
     * @param delegate      The delegate account.
     * @param contractAddress The contract address forming the user decryption context.
     * @param handle          The handle.
     * @return isDelegated    Whether the handle is delegated for user decryption.
     */
    function isDelegatedForUserDecryption(
        address delegator,
        address delegate,
        address contractAddress,
        bytes32 handle
    ) internal view returns (bool) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        return IACL($.ACLAddress).isHandleDelegatedForUserDecryption(delegator, delegate, contractAddress, handle);
    }
}
