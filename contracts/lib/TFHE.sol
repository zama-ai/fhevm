// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import "./Impl.sol";
import {FheType} from "../contracts/FheType.sol";

type ebool is bytes32;
type euint8 is bytes32;
type euint16 is bytes32;
type euint32 is bytes32;
type euint64 is bytes32;
type euint128 is bytes32;
type euint256 is bytes32;
type ebytes64 is bytes32;
type ebytes128 is bytes32;
type ebytes256 is bytes32;
type eaddress is bytes32;
type einput is bytes32;

/**
 * @title   TFHE
 * @notice  This library is the interaction point for all smart contract developers
 *          that interact with TFHE.
 */
library TFHE {
    /// @notice Returned if the input's length is greater than 64 bytes.
    error InputLengthAbove64Bytes(uint256 inputLength);

    /// @notice Returned if the input's length is greater than 128 bytes.
    error InputLengthAbove128Bytes(uint256 inputLength);

    /// @notice Returned if the input's length is greater than 256 bytes.
    error InputLengthAbove256Bytes(uint256 inputLength);

    /**
     * @notice            Sets the FHEVM addresses.
     * @param fhevmConfig FHEVM config struct that contains contract addresses.
     */
    function setFHEVM(FHEVMConfigStruct memory fhevmConfig) internal {
        Impl.setFHEVM(fhevmConfig);
    }

    /**
     * @dev Returns true if the encrypted integer is initialized and false otherwise.
     */
    function isInitialized(ebool v) internal pure returns (bool) {
        return ebool.unwrap(v) != 0;
    }

    /**
     * @dev Returns true if the encrypted integer is initialized and false otherwise.
     */
    function isInitialized(euint8 v) internal pure returns (bool) {
        return euint8.unwrap(v) != 0;
    }

    /**
     * @dev Returns true if the encrypted integer is initialized and false otherwise.
     */
    function isInitialized(euint16 v) internal pure returns (bool) {
        return euint16.unwrap(v) != 0;
    }

    /**
     * @dev Returns true if the encrypted integer is initialized and false otherwise.
     */
    function isInitialized(euint32 v) internal pure returns (bool) {
        return euint32.unwrap(v) != 0;
    }

    /**
     * @dev Returns true if the encrypted integer is initialized and false otherwise.
     */
    function isInitialized(euint64 v) internal pure returns (bool) {
        return euint64.unwrap(v) != 0;
    }

    /**
     * @dev Returns true if the encrypted integer is initialized and false otherwise.
     */
    function isInitialized(euint128 v) internal pure returns (bool) {
        return euint128.unwrap(v) != 0;
    }

    /**
     * @dev Returns true if the encrypted integer is initialized and false otherwise.
     */
    function isInitialized(euint256 v) internal pure returns (bool) {
        return euint256.unwrap(v) != 0;
    }

    /**
     * @dev Returns true if the encrypted integer is initialized and false otherwise.
     */
    function isInitialized(ebytes64 v) internal pure returns (bool) {
        return ebytes64.unwrap(v) != 0;
    }

    /**
     * @dev Returns true if the encrypted integer is initialized and false otherwise.
     */
    function isInitialized(ebytes128 v) internal pure returns (bool) {
        return ebytes128.unwrap(v) != 0;
    }

    /**
     * @dev Returns true if the encrypted integer is initialized and false otherwise.
     */
    function isInitialized(ebytes256 v) internal pure returns (bool) {
        return ebytes256.unwrap(v) != 0;
    }

    /**
     * @dev Returns true if the encrypted integer is initialized and false otherwise.
     */
    function isInitialized(eaddress v) internal pure returns (bool) {
        return eaddress.unwrap(v) != 0;
    }

    /**
     * @dev Evaluates and(ebool a, ebool b) and returns the result.
     */
    function and(ebool a, ebool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.and(ebool.unwrap(a), ebool.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(ebool a, ebool b) and returns the result.
     */
    function or(ebool a, ebool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.or(ebool.unwrap(a), ebool.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(ebool a, ebool b) and returns the result.
     */
    function xor(ebool a, ebool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.xor(ebool.unwrap(a), ebool.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(ebool a, ebool b) and returns the result.
     */
    function eq(ebool a, ebool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.eq(ebool.unwrap(a), ebool.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(ebool a, ebool b) and returns the result.
     */
    function ne(ebool a, ebool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.ne(ebool.unwrap(a), ebool.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint8 a, euint8 b)  and returns the result.
     */
    function add(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.add(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint8 a, euint8 b)  and returns the result.
     */
    function sub(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.sub(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint8 a, euint8 b)  and returns the result.
     */
    function mul(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.mul(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint8 a, euint8 b)  and returns the result.
     */
    function and(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.and(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint8 a, euint8 b)  and returns the result.
     */
    function or(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.or(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint8 a, euint8 b)  and returns the result.
     */
    function xor(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.xor(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint8 a, euint8 b)  and returns the result.
     */
    function eq(euint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint8 a, euint8 b)  and returns the result.
     */
    function ne(euint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint8 a, euint8 b)  and returns the result.
     */
    function ge(euint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint8 a, euint8 b)  and returns the result.
     */
    function gt(euint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint8 a, euint8 b)  and returns the result.
     */
    function le(euint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint8 a, euint8 b)  and returns the result.
     */
    function lt(euint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint8 a, euint8 b)  and returns the result.
     */
    function min(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.min(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint8 a, euint8 b)  and returns the result.
     */
    function max(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.max(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint8 a, euint16 b)  and returns the result.
     */
    function add(euint8 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint8 a, euint16 b)  and returns the result.
     */
    function sub(euint8 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint8 a, euint16 b)  and returns the result.
     */
    function mul(euint8 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint8 a, euint16 b)  and returns the result.
     */
    function and(euint8 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.and(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint8 a, euint16 b)  and returns the result.
     */
    function or(euint8 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.or(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint8 a, euint16 b)  and returns the result.
     */
    function xor(euint8 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.xor(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint8 a, euint16 b)  and returns the result.
     */
    function eq(euint8 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint8 a, euint16 b)  and returns the result.
     */
    function ne(euint8 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint8 a, euint16 b)  and returns the result.
     */
    function ge(euint8 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint8 a, euint16 b)  and returns the result.
     */
    function gt(euint8 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint8 a, euint16 b)  and returns the result.
     */
    function le(euint8 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint8 a, euint16 b)  and returns the result.
     */
    function lt(euint8 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint8 a, euint16 b)  and returns the result.
     */
    function min(euint8 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint8 a, euint16 b)  and returns the result.
     */
    function max(euint8 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(asEuint16(a)), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint8 a, euint32 b)  and returns the result.
     */
    function add(euint8 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint8 a, euint32 b)  and returns the result.
     */
    function sub(euint8 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint8 a, euint32 b)  and returns the result.
     */
    function mul(euint8 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint8 a, euint32 b)  and returns the result.
     */
    function and(euint8 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint8 a, euint32 b)  and returns the result.
     */
    function or(euint8 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint8 a, euint32 b)  and returns the result.
     */
    function xor(euint8 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint8 a, euint32 b)  and returns the result.
     */
    function eq(euint8 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint8 a, euint32 b)  and returns the result.
     */
    function ne(euint8 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint8 a, euint32 b)  and returns the result.
     */
    function ge(euint8 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint8 a, euint32 b)  and returns the result.
     */
    function gt(euint8 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint8 a, euint32 b)  and returns the result.
     */
    function le(euint8 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint8 a, euint32 b)  and returns the result.
     */
    function lt(euint8 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint8 a, euint32 b)  and returns the result.
     */
    function min(euint8 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint8 a, euint32 b)  and returns the result.
     */
    function max(euint8 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint8 a, euint64 b)  and returns the result.
     */
    function add(euint8 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint8 a, euint64 b)  and returns the result.
     */
    function sub(euint8 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint8 a, euint64 b)  and returns the result.
     */
    function mul(euint8 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint8 a, euint64 b)  and returns the result.
     */
    function and(euint8 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint8 a, euint64 b)  and returns the result.
     */
    function or(euint8 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint8 a, euint64 b)  and returns the result.
     */
    function xor(euint8 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint8 a, euint64 b)  and returns the result.
     */
    function eq(euint8 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint8 a, euint64 b)  and returns the result.
     */
    function ne(euint8 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint8 a, euint64 b)  and returns the result.
     */
    function ge(euint8 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint8 a, euint64 b)  and returns the result.
     */
    function gt(euint8 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint8 a, euint64 b)  and returns the result.
     */
    function le(euint8 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint8 a, euint64 b)  and returns the result.
     */
    function lt(euint8 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint8 a, euint64 b)  and returns the result.
     */
    function min(euint8 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint8 a, euint64 b)  and returns the result.
     */
    function max(euint8 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint8 a, euint128 b)  and returns the result.
     */
    function add(euint8 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.add(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint8 a, euint128 b)  and returns the result.
     */
    function sub(euint8 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.sub(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint8 a, euint128 b)  and returns the result.
     */
    function mul(euint8 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.mul(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint8 a, euint128 b)  and returns the result.
     */
    function and(euint8 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.and(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint8 a, euint128 b)  and returns the result.
     */
    function or(euint8 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.or(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint8 a, euint128 b)  and returns the result.
     */
    function xor(euint8 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.xor(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint8 a, euint128 b)  and returns the result.
     */
    function eq(euint8 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.eq(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint8 a, euint128 b)  and returns the result.
     */
    function ne(euint8 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ne(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint8 a, euint128 b)  and returns the result.
     */
    function ge(euint8 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ge(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint8 a, euint128 b)  and returns the result.
     */
    function gt(euint8 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.gt(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint8 a, euint128 b)  and returns the result.
     */
    function le(euint8 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.le(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint8 a, euint128 b)  and returns the result.
     */
    function lt(euint8 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.lt(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint8 a, euint128 b)  and returns the result.
     */
    function min(euint8 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.min(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint8 a, euint128 b)  and returns the result.
     */
    function max(euint8 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.max(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint8 a, euint256 b)  and returns the result.
     */
    function and(euint8 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint8 a, euint256 b)  and returns the result.
     */
    function or(euint8 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint8 a, euint256 b)  and returns the result.
     */
    function xor(euint8 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint8 a, euint256 b)  and returns the result.
     */
    function eq(euint8 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint8 a, euint256 b)  and returns the result.
     */
    function ne(euint8 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint16 a, euint8 b)  and returns the result.
     */
    function add(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates sub(euint16 a, euint8 b)  and returns the result.
     */
    function sub(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates mul(euint16 a, euint8 b)  and returns the result.
     */
    function mul(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates and(euint16 a, euint8 b)  and returns the result.
     */
    function and(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.and(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates or(euint16 a, euint8 b)  and returns the result.
     */
    function or(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.or(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates xor(euint16 a, euint8 b)  and returns the result.
     */
    function xor(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.xor(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates eq(euint16 a, euint8 b)  and returns the result.
     */
    function eq(euint16 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates ne(euint16 a, euint8 b)  and returns the result.
     */
    function ne(euint16 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates ge(euint16 a, euint8 b)  and returns the result.
     */
    function ge(euint16 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates gt(euint16 a, euint8 b)  and returns the result.
     */
    function gt(euint16 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates le(euint16 a, euint8 b)  and returns the result.
     */
    function le(euint16 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates lt(euint16 a, euint8 b)  and returns the result.
     */
    function lt(euint16 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates min(euint16 a, euint8 b)  and returns the result.
     */
    function min(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates max(euint16 a, euint8 b)  and returns the result.
     */
    function max(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates add(euint16 a, euint16 b)  and returns the result.
     */
    function add(euint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint16 a, euint16 b)  and returns the result.
     */
    function sub(euint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint16 a, euint16 b)  and returns the result.
     */
    function mul(euint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint16 a, euint16 b)  and returns the result.
     */
    function and(euint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.and(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint16 a, euint16 b)  and returns the result.
     */
    function or(euint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.or(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint16 a, euint16 b)  and returns the result.
     */
    function xor(euint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.xor(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint16 a, euint16 b)  and returns the result.
     */
    function eq(euint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint16 a, euint16 b)  and returns the result.
     */
    function ne(euint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint16 a, euint16 b)  and returns the result.
     */
    function ge(euint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint16 a, euint16 b)  and returns the result.
     */
    function gt(euint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint16 a, euint16 b)  and returns the result.
     */
    function le(euint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint16 a, euint16 b)  and returns the result.
     */
    function lt(euint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint16 a, euint16 b)  and returns the result.
     */
    function min(euint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint16 a, euint16 b)  and returns the result.
     */
    function max(euint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(a), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint16 a, euint32 b)  and returns the result.
     */
    function add(euint16 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint16 a, euint32 b)  and returns the result.
     */
    function sub(euint16 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint16 a, euint32 b)  and returns the result.
     */
    function mul(euint16 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint16 a, euint32 b)  and returns the result.
     */
    function and(euint16 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint16 a, euint32 b)  and returns the result.
     */
    function or(euint16 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint16 a, euint32 b)  and returns the result.
     */
    function xor(euint16 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint16 a, euint32 b)  and returns the result.
     */
    function eq(euint16 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint16 a, euint32 b)  and returns the result.
     */
    function ne(euint16 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint16 a, euint32 b)  and returns the result.
     */
    function ge(euint16 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint16 a, euint32 b)  and returns the result.
     */
    function gt(euint16 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint16 a, euint32 b)  and returns the result.
     */
    function le(euint16 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint16 a, euint32 b)  and returns the result.
     */
    function lt(euint16 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint16 a, euint32 b)  and returns the result.
     */
    function min(euint16 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint16 a, euint32 b)  and returns the result.
     */
    function max(euint16 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(asEuint32(a)), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint16 a, euint64 b)  and returns the result.
     */
    function add(euint16 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint16 a, euint64 b)  and returns the result.
     */
    function sub(euint16 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint16 a, euint64 b)  and returns the result.
     */
    function mul(euint16 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint16 a, euint64 b)  and returns the result.
     */
    function and(euint16 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint16 a, euint64 b)  and returns the result.
     */
    function or(euint16 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint16 a, euint64 b)  and returns the result.
     */
    function xor(euint16 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint16 a, euint64 b)  and returns the result.
     */
    function eq(euint16 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint16 a, euint64 b)  and returns the result.
     */
    function ne(euint16 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint16 a, euint64 b)  and returns the result.
     */
    function ge(euint16 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint16 a, euint64 b)  and returns the result.
     */
    function gt(euint16 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint16 a, euint64 b)  and returns the result.
     */
    function le(euint16 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint16 a, euint64 b)  and returns the result.
     */
    function lt(euint16 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint16 a, euint64 b)  and returns the result.
     */
    function min(euint16 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint16 a, euint64 b)  and returns the result.
     */
    function max(euint16 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint16 a, euint128 b)  and returns the result.
     */
    function add(euint16 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.add(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint16 a, euint128 b)  and returns the result.
     */
    function sub(euint16 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.sub(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint16 a, euint128 b)  and returns the result.
     */
    function mul(euint16 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.mul(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint16 a, euint128 b)  and returns the result.
     */
    function and(euint16 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.and(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint16 a, euint128 b)  and returns the result.
     */
    function or(euint16 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.or(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint16 a, euint128 b)  and returns the result.
     */
    function xor(euint16 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.xor(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint16 a, euint128 b)  and returns the result.
     */
    function eq(euint16 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.eq(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint16 a, euint128 b)  and returns the result.
     */
    function ne(euint16 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ne(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint16 a, euint128 b)  and returns the result.
     */
    function ge(euint16 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ge(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint16 a, euint128 b)  and returns the result.
     */
    function gt(euint16 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.gt(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint16 a, euint128 b)  and returns the result.
     */
    function le(euint16 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.le(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint16 a, euint128 b)  and returns the result.
     */
    function lt(euint16 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.lt(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint16 a, euint128 b)  and returns the result.
     */
    function min(euint16 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.min(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint16 a, euint128 b)  and returns the result.
     */
    function max(euint16 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.max(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint16 a, euint256 b)  and returns the result.
     */
    function and(euint16 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint16 a, euint256 b)  and returns the result.
     */
    function or(euint16 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint16 a, euint256 b)  and returns the result.
     */
    function xor(euint16 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint16 a, euint256 b)  and returns the result.
     */
    function eq(euint16 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint16 a, euint256 b)  and returns the result.
     */
    function ne(euint16 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint32 a, euint8 b)  and returns the result.
     */
    function add(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates sub(euint32 a, euint8 b)  and returns the result.
     */
    function sub(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates mul(euint32 a, euint8 b)  and returns the result.
     */
    function mul(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates and(euint32 a, euint8 b)  and returns the result.
     */
    function and(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates or(euint32 a, euint8 b)  and returns the result.
     */
    function or(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates xor(euint32 a, euint8 b)  and returns the result.
     */
    function xor(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates eq(euint32 a, euint8 b)  and returns the result.
     */
    function eq(euint32 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates ne(euint32 a, euint8 b)  and returns the result.
     */
    function ne(euint32 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates ge(euint32 a, euint8 b)  and returns the result.
     */
    function ge(euint32 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates gt(euint32 a, euint8 b)  and returns the result.
     */
    function gt(euint32 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates le(euint32 a, euint8 b)  and returns the result.
     */
    function le(euint32 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates lt(euint32 a, euint8 b)  and returns the result.
     */
    function lt(euint32 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates min(euint32 a, euint8 b)  and returns the result.
     */
    function min(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates max(euint32 a, euint8 b)  and returns the result.
     */
    function max(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates add(euint32 a, euint16 b)  and returns the result.
     */
    function add(euint32 a, euint16 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates sub(euint32 a, euint16 b)  and returns the result.
     */
    function sub(euint32 a, euint16 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates mul(euint32 a, euint16 b)  and returns the result.
     */
    function mul(euint32 a, euint16 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates and(euint32 a, euint16 b)  and returns the result.
     */
    function and(euint32 a, euint16 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates or(euint32 a, euint16 b)  and returns the result.
     */
    function or(euint32 a, euint16 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates xor(euint32 a, euint16 b)  and returns the result.
     */
    function xor(euint32 a, euint16 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates eq(euint32 a, euint16 b)  and returns the result.
     */
    function eq(euint32 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates ne(euint32 a, euint16 b)  and returns the result.
     */
    function ne(euint32 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates ge(euint32 a, euint16 b)  and returns the result.
     */
    function ge(euint32 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates gt(euint32 a, euint16 b)  and returns the result.
     */
    function gt(euint32 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates le(euint32 a, euint16 b)  and returns the result.
     */
    function le(euint32 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates lt(euint32 a, euint16 b)  and returns the result.
     */
    function lt(euint32 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates min(euint32 a, euint16 b)  and returns the result.
     */
    function min(euint32 a, euint16 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates max(euint32 a, euint16 b)  and returns the result.
     */
    function max(euint32 a, euint16 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates add(euint32 a, euint32 b)  and returns the result.
     */
    function add(euint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint32 a, euint32 b)  and returns the result.
     */
    function sub(euint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint32 a, euint32 b)  and returns the result.
     */
    function mul(euint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint32 a, euint32 b)  and returns the result.
     */
    function and(euint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint32 a, euint32 b)  and returns the result.
     */
    function or(euint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint32 a, euint32 b)  and returns the result.
     */
    function xor(euint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint32 a, euint32 b)  and returns the result.
     */
    function eq(euint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint32 a, euint32 b)  and returns the result.
     */
    function ne(euint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint32 a, euint32 b)  and returns the result.
     */
    function ge(euint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint32 a, euint32 b)  and returns the result.
     */
    function gt(euint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint32 a, euint32 b)  and returns the result.
     */
    function le(euint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint32 a, euint32 b)  and returns the result.
     */
    function lt(euint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint32 a, euint32 b)  and returns the result.
     */
    function min(euint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint32 a, euint32 b)  and returns the result.
     */
    function max(euint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(a), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint32 a, euint64 b)  and returns the result.
     */
    function add(euint32 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint32 a, euint64 b)  and returns the result.
     */
    function sub(euint32 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint32 a, euint64 b)  and returns the result.
     */
    function mul(euint32 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint32 a, euint64 b)  and returns the result.
     */
    function and(euint32 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint32 a, euint64 b)  and returns the result.
     */
    function or(euint32 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint32 a, euint64 b)  and returns the result.
     */
    function xor(euint32 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint32 a, euint64 b)  and returns the result.
     */
    function eq(euint32 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint32 a, euint64 b)  and returns the result.
     */
    function ne(euint32 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint32 a, euint64 b)  and returns the result.
     */
    function ge(euint32 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint32 a, euint64 b)  and returns the result.
     */
    function gt(euint32 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint32 a, euint64 b)  and returns the result.
     */
    function le(euint32 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint32 a, euint64 b)  and returns the result.
     */
    function lt(euint32 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint32 a, euint64 b)  and returns the result.
     */
    function min(euint32 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint32 a, euint64 b)  and returns the result.
     */
    function max(euint32 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(asEuint64(a)), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint32 a, euint128 b)  and returns the result.
     */
    function add(euint32 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.add(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint32 a, euint128 b)  and returns the result.
     */
    function sub(euint32 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.sub(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint32 a, euint128 b)  and returns the result.
     */
    function mul(euint32 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.mul(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint32 a, euint128 b)  and returns the result.
     */
    function and(euint32 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.and(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint32 a, euint128 b)  and returns the result.
     */
    function or(euint32 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.or(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint32 a, euint128 b)  and returns the result.
     */
    function xor(euint32 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.xor(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint32 a, euint128 b)  and returns the result.
     */
    function eq(euint32 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.eq(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint32 a, euint128 b)  and returns the result.
     */
    function ne(euint32 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ne(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint32 a, euint128 b)  and returns the result.
     */
    function ge(euint32 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ge(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint32 a, euint128 b)  and returns the result.
     */
    function gt(euint32 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.gt(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint32 a, euint128 b)  and returns the result.
     */
    function le(euint32 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.le(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint32 a, euint128 b)  and returns the result.
     */
    function lt(euint32 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.lt(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint32 a, euint128 b)  and returns the result.
     */
    function min(euint32 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.min(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint32 a, euint128 b)  and returns the result.
     */
    function max(euint32 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.max(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint32 a, euint256 b)  and returns the result.
     */
    function and(euint32 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint32 a, euint256 b)  and returns the result.
     */
    function or(euint32 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint32 a, euint256 b)  and returns the result.
     */
    function xor(euint32 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint32 a, euint256 b)  and returns the result.
     */
    function eq(euint32 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint32 a, euint256 b)  and returns the result.
     */
    function ne(euint32 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint64 a, euint8 b)  and returns the result.
     */
    function add(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates sub(euint64 a, euint8 b)  and returns the result.
     */
    function sub(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates mul(euint64 a, euint8 b)  and returns the result.
     */
    function mul(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates and(euint64 a, euint8 b)  and returns the result.
     */
    function and(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates or(euint64 a, euint8 b)  and returns the result.
     */
    function or(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates xor(euint64 a, euint8 b)  and returns the result.
     */
    function xor(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates eq(euint64 a, euint8 b)  and returns the result.
     */
    function eq(euint64 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates ne(euint64 a, euint8 b)  and returns the result.
     */
    function ne(euint64 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates ge(euint64 a, euint8 b)  and returns the result.
     */
    function ge(euint64 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates gt(euint64 a, euint8 b)  and returns the result.
     */
    function gt(euint64 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates le(euint64 a, euint8 b)  and returns the result.
     */
    function le(euint64 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates lt(euint64 a, euint8 b)  and returns the result.
     */
    function lt(euint64 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates min(euint64 a, euint8 b)  and returns the result.
     */
    function min(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates max(euint64 a, euint8 b)  and returns the result.
     */
    function max(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates add(euint64 a, euint16 b)  and returns the result.
     */
    function add(euint64 a, euint16 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates sub(euint64 a, euint16 b)  and returns the result.
     */
    function sub(euint64 a, euint16 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates mul(euint64 a, euint16 b)  and returns the result.
     */
    function mul(euint64 a, euint16 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates and(euint64 a, euint16 b)  and returns the result.
     */
    function and(euint64 a, euint16 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates or(euint64 a, euint16 b)  and returns the result.
     */
    function or(euint64 a, euint16 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates xor(euint64 a, euint16 b)  and returns the result.
     */
    function xor(euint64 a, euint16 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates eq(euint64 a, euint16 b)  and returns the result.
     */
    function eq(euint64 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates ne(euint64 a, euint16 b)  and returns the result.
     */
    function ne(euint64 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates ge(euint64 a, euint16 b)  and returns the result.
     */
    function ge(euint64 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates gt(euint64 a, euint16 b)  and returns the result.
     */
    function gt(euint64 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates le(euint64 a, euint16 b)  and returns the result.
     */
    function le(euint64 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates lt(euint64 a, euint16 b)  and returns the result.
     */
    function lt(euint64 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates min(euint64 a, euint16 b)  and returns the result.
     */
    function min(euint64 a, euint16 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates max(euint64 a, euint16 b)  and returns the result.
     */
    function max(euint64 a, euint16 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates add(euint64 a, euint32 b)  and returns the result.
     */
    function add(euint64 a, euint32 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates sub(euint64 a, euint32 b)  and returns the result.
     */
    function sub(euint64 a, euint32 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates mul(euint64 a, euint32 b)  and returns the result.
     */
    function mul(euint64 a, euint32 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates and(euint64 a, euint32 b)  and returns the result.
     */
    function and(euint64 a, euint32 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates or(euint64 a, euint32 b)  and returns the result.
     */
    function or(euint64 a, euint32 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates xor(euint64 a, euint32 b)  and returns the result.
     */
    function xor(euint64 a, euint32 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates eq(euint64 a, euint32 b)  and returns the result.
     */
    function eq(euint64 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates ne(euint64 a, euint32 b)  and returns the result.
     */
    function ne(euint64 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates ge(euint64 a, euint32 b)  and returns the result.
     */
    function ge(euint64 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates gt(euint64 a, euint32 b)  and returns the result.
     */
    function gt(euint64 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates le(euint64 a, euint32 b)  and returns the result.
     */
    function le(euint64 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates lt(euint64 a, euint32 b)  and returns the result.
     */
    function lt(euint64 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates min(euint64 a, euint32 b)  and returns the result.
     */
    function min(euint64 a, euint32 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates max(euint64 a, euint32 b)  and returns the result.
     */
    function max(euint64 a, euint32 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates add(euint64 a, euint64 b)  and returns the result.
     */
    function add(euint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint64 a, euint64 b)  and returns the result.
     */
    function sub(euint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint64 a, euint64 b)  and returns the result.
     */
    function mul(euint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint64 a, euint64 b)  and returns the result.
     */
    function and(euint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint64 a, euint64 b)  and returns the result.
     */
    function or(euint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint64 a, euint64 b)  and returns the result.
     */
    function xor(euint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint64 a, euint64 b)  and returns the result.
     */
    function eq(euint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint64 a, euint64 b)  and returns the result.
     */
    function ne(euint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint64 a, euint64 b)  and returns the result.
     */
    function ge(euint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint64 a, euint64 b)  and returns the result.
     */
    function gt(euint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint64 a, euint64 b)  and returns the result.
     */
    function le(euint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint64 a, euint64 b)  and returns the result.
     */
    function lt(euint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint64 a, euint64 b)  and returns the result.
     */
    function min(euint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint64 a, euint64 b)  and returns the result.
     */
    function max(euint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(a), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint64 a, euint128 b)  and returns the result.
     */
    function add(euint64 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.add(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint64 a, euint128 b)  and returns the result.
     */
    function sub(euint64 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.sub(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint64 a, euint128 b)  and returns the result.
     */
    function mul(euint64 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.mul(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint64 a, euint128 b)  and returns the result.
     */
    function and(euint64 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.and(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint64 a, euint128 b)  and returns the result.
     */
    function or(euint64 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.or(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint64 a, euint128 b)  and returns the result.
     */
    function xor(euint64 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.xor(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint64 a, euint128 b)  and returns the result.
     */
    function eq(euint64 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.eq(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint64 a, euint128 b)  and returns the result.
     */
    function ne(euint64 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ne(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint64 a, euint128 b)  and returns the result.
     */
    function ge(euint64 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ge(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint64 a, euint128 b)  and returns the result.
     */
    function gt(euint64 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.gt(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint64 a, euint128 b)  and returns the result.
     */
    function le(euint64 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.le(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint64 a, euint128 b)  and returns the result.
     */
    function lt(euint64 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.lt(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint64 a, euint128 b)  and returns the result.
     */
    function min(euint64 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.min(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint64 a, euint128 b)  and returns the result.
     */
    function max(euint64 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.max(euint128.unwrap(asEuint128(a)), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint64 a, euint256 b)  and returns the result.
     */
    function and(euint64 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint64 a, euint256 b)  and returns the result.
     */
    function or(euint64 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint64 a, euint256 b)  and returns the result.
     */
    function xor(euint64 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint64 a, euint256 b)  and returns the result.
     */
    function eq(euint64 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint64 a, euint256 b)  and returns the result.
     */
    function ne(euint64 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates add(euint128 a, euint8 b)  and returns the result.
     */
    function add(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.add(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates sub(euint128 a, euint8 b)  and returns the result.
     */
    function sub(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.sub(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates mul(euint128 a, euint8 b)  and returns the result.
     */
    function mul(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.mul(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates and(euint128 a, euint8 b)  and returns the result.
     */
    function and(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.and(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates or(euint128 a, euint8 b)  and returns the result.
     */
    function or(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.or(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates xor(euint128 a, euint8 b)  and returns the result.
     */
    function xor(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.xor(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates eq(euint128 a, euint8 b)  and returns the result.
     */
    function eq(euint128 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates ne(euint128 a, euint8 b)  and returns the result.
     */
    function ne(euint128 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates ge(euint128 a, euint8 b)  and returns the result.
     */
    function ge(euint128 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates gt(euint128 a, euint8 b)  and returns the result.
     */
    function gt(euint128 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates le(euint128 a, euint8 b)  and returns the result.
     */
    function le(euint128 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates lt(euint128 a, euint8 b)  and returns the result.
     */
    function lt(euint128 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates min(euint128 a, euint8 b)  and returns the result.
     */
    function min(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.min(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates max(euint128 a, euint8 b)  and returns the result.
     */
    function max(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.max(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates add(euint128 a, euint16 b)  and returns the result.
     */
    function add(euint128 a, euint16 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint128.wrap(Impl.add(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates sub(euint128 a, euint16 b)  and returns the result.
     */
    function sub(euint128 a, euint16 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint128.wrap(Impl.sub(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates mul(euint128 a, euint16 b)  and returns the result.
     */
    function mul(euint128 a, euint16 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint128.wrap(Impl.mul(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates and(euint128 a, euint16 b)  and returns the result.
     */
    function and(euint128 a, euint16 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint128.wrap(Impl.and(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates or(euint128 a, euint16 b)  and returns the result.
     */
    function or(euint128 a, euint16 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint128.wrap(Impl.or(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates xor(euint128 a, euint16 b)  and returns the result.
     */
    function xor(euint128 a, euint16 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint128.wrap(Impl.xor(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates eq(euint128 a, euint16 b)  and returns the result.
     */
    function eq(euint128 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates ne(euint128 a, euint16 b)  and returns the result.
     */
    function ne(euint128 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates ge(euint128 a, euint16 b)  and returns the result.
     */
    function ge(euint128 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates gt(euint128 a, euint16 b)  and returns the result.
     */
    function gt(euint128 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates le(euint128 a, euint16 b)  and returns the result.
     */
    function le(euint128 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates lt(euint128 a, euint16 b)  and returns the result.
     */
    function lt(euint128 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates min(euint128 a, euint16 b)  and returns the result.
     */
    function min(euint128 a, euint16 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint128.wrap(Impl.min(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates max(euint128 a, euint16 b)  and returns the result.
     */
    function max(euint128 a, euint16 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint128.wrap(Impl.max(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates add(euint128 a, euint32 b)  and returns the result.
     */
    function add(euint128 a, euint32 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint128.wrap(Impl.add(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates sub(euint128 a, euint32 b)  and returns the result.
     */
    function sub(euint128 a, euint32 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint128.wrap(Impl.sub(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates mul(euint128 a, euint32 b)  and returns the result.
     */
    function mul(euint128 a, euint32 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint128.wrap(Impl.mul(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates and(euint128 a, euint32 b)  and returns the result.
     */
    function and(euint128 a, euint32 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint128.wrap(Impl.and(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates or(euint128 a, euint32 b)  and returns the result.
     */
    function or(euint128 a, euint32 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint128.wrap(Impl.or(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates xor(euint128 a, euint32 b)  and returns the result.
     */
    function xor(euint128 a, euint32 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint128.wrap(Impl.xor(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates eq(euint128 a, euint32 b)  and returns the result.
     */
    function eq(euint128 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates ne(euint128 a, euint32 b)  and returns the result.
     */
    function ne(euint128 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates ge(euint128 a, euint32 b)  and returns the result.
     */
    function ge(euint128 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates gt(euint128 a, euint32 b)  and returns the result.
     */
    function gt(euint128 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates le(euint128 a, euint32 b)  and returns the result.
     */
    function le(euint128 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates lt(euint128 a, euint32 b)  and returns the result.
     */
    function lt(euint128 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates min(euint128 a, euint32 b)  and returns the result.
     */
    function min(euint128 a, euint32 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint128.wrap(Impl.min(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates max(euint128 a, euint32 b)  and returns the result.
     */
    function max(euint128 a, euint32 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint128.wrap(Impl.max(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates add(euint128 a, euint64 b)  and returns the result.
     */
    function add(euint128 a, euint64 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint128.wrap(Impl.add(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates sub(euint128 a, euint64 b)  and returns the result.
     */
    function sub(euint128 a, euint64 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint128.wrap(Impl.sub(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates mul(euint128 a, euint64 b)  and returns the result.
     */
    function mul(euint128 a, euint64 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint128.wrap(Impl.mul(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates and(euint128 a, euint64 b)  and returns the result.
     */
    function and(euint128 a, euint64 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint128.wrap(Impl.and(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates or(euint128 a, euint64 b)  and returns the result.
     */
    function or(euint128 a, euint64 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint128.wrap(Impl.or(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates xor(euint128 a, euint64 b)  and returns the result.
     */
    function xor(euint128 a, euint64 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint128.wrap(Impl.xor(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates eq(euint128 a, euint64 b)  and returns the result.
     */
    function eq(euint128 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates ne(euint128 a, euint64 b)  and returns the result.
     */
    function ne(euint128 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates ge(euint128 a, euint64 b)  and returns the result.
     */
    function ge(euint128 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates gt(euint128 a, euint64 b)  and returns the result.
     */
    function gt(euint128 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates le(euint128 a, euint64 b)  and returns the result.
     */
    function le(euint128 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates lt(euint128 a, euint64 b)  and returns the result.
     */
    function lt(euint128 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates min(euint128 a, euint64 b)  and returns the result.
     */
    function min(euint128 a, euint64 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint128.wrap(Impl.min(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates max(euint128 a, euint64 b)  and returns the result.
     */
    function max(euint128 a, euint64 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint128.wrap(Impl.max(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates add(euint128 a, euint128 b)  and returns the result.
     */
    function add(euint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.add(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates sub(euint128 a, euint128 b)  and returns the result.
     */
    function sub(euint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.sub(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint128 a, euint128 b)  and returns the result.
     */
    function mul(euint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.mul(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint128 a, euint128 b)  and returns the result.
     */
    function and(euint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.and(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint128 a, euint128 b)  and returns the result.
     */
    function or(euint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.or(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint128 a, euint128 b)  and returns the result.
     */
    function xor(euint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.xor(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint128 a, euint128 b)  and returns the result.
     */
    function eq(euint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.eq(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint128 a, euint128 b)  and returns the result.
     */
    function ne(euint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ne(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates ge(euint128 a, euint128 b)  and returns the result.
     */
    function ge(euint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ge(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates gt(euint128 a, euint128 b)  and returns the result.
     */
    function gt(euint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.gt(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates le(euint128 a, euint128 b)  and returns the result.
     */
    function le(euint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.le(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates lt(euint128 a, euint128 b)  and returns the result.
     */
    function lt(euint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.lt(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates min(euint128 a, euint128 b)  and returns the result.
     */
    function min(euint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.min(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates max(euint128 a, euint128 b)  and returns the result.
     */
    function max(euint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.max(euint128.unwrap(a), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint128 a, euint256 b)  and returns the result.
     */
    function and(euint128 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint128 a, euint256 b)  and returns the result.
     */
    function or(euint128 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint128 a, euint256 b)  and returns the result.
     */
    function xor(euint128 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint128 a, euint256 b)  and returns the result.
     */
    function eq(euint128 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint128 a, euint256 b)  and returns the result.
     */
    function ne(euint128 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(asEuint256(a)), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(euint256 a, euint8 b)  and returns the result.
     */
    function and(euint256 a, euint8 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates or(euint256 a, euint8 b)  and returns the result.
     */
    function or(euint256 a, euint8 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates xor(euint256 a, euint8 b)  and returns the result.
     */
    function xor(euint256 a, euint8 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates eq(euint256 a, euint8 b)  and returns the result.
     */
    function eq(euint256 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates ne(euint256 a, euint8 b)  and returns the result.
     */
    function ne(euint256 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates and(euint256 a, euint16 b)  and returns the result.
     */
    function and(euint256 a, euint16 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates or(euint256 a, euint16 b)  and returns the result.
     */
    function or(euint256 a, euint16 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates xor(euint256 a, euint16 b)  and returns the result.
     */
    function xor(euint256 a, euint16 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates eq(euint256 a, euint16 b)  and returns the result.
     */
    function eq(euint256 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates ne(euint256 a, euint16 b)  and returns the result.
     */
    function ne(euint256 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates and(euint256 a, euint32 b)  and returns the result.
     */
    function and(euint256 a, euint32 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates or(euint256 a, euint32 b)  and returns the result.
     */
    function or(euint256 a, euint32 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates xor(euint256 a, euint32 b)  and returns the result.
     */
    function xor(euint256 a, euint32 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates eq(euint256 a, euint32 b)  and returns the result.
     */
    function eq(euint256 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates ne(euint256 a, euint32 b)  and returns the result.
     */
    function ne(euint256 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates and(euint256 a, euint64 b)  and returns the result.
     */
    function and(euint256 a, euint64 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates or(euint256 a, euint64 b)  and returns the result.
     */
    function or(euint256 a, euint64 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates xor(euint256 a, euint64 b)  and returns the result.
     */
    function xor(euint256 a, euint64 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates eq(euint256 a, euint64 b)  and returns the result.
     */
    function eq(euint256 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates ne(euint256 a, euint64 b)  and returns the result.
     */
    function ne(euint256 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates and(euint256 a, euint128 b)  and returns the result.
     */
    function and(euint256 a, euint128 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates or(euint256 a, euint128 b)  and returns the result.
     */
    function or(euint256 a, euint128 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates xor(euint256 a, euint128 b)  and returns the result.
     */
    function xor(euint256 a, euint128 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates eq(euint256 a, euint128 b)  and returns the result.
     */
    function eq(euint256 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates ne(euint256 a, euint128 b)  and returns the result.
     */
    function ne(euint256 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates and(euint256 a, euint256 b)  and returns the result.
     */
    function and(euint256 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(a), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates or(euint256 a, euint256 b)  and returns the result.
     */
    function or(euint256 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(a), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates xor(euint256 a, euint256 b)  and returns the result.
     */
    function xor(euint256 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(a), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(euint256 a, euint256 b)  and returns the result.
     */
    function eq(euint256 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(a), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(euint256 a, euint256 b)  and returns the result.
     */
    function ne(euint256 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(a), euint256.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(ebytes64 a, ebytes64 b) and returns the result.
     */
    function eq(ebytes64 a, ebytes64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes64(padToBytes64(hex""));
        }
        if (!isInitialized(b)) {
            b = asEbytes64(padToBytes64(hex""));
        }
        return ebool.wrap(Impl.eq(ebytes64.unwrap(a), ebytes64.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(ebytes64 a, ebytes64 b) and returns the result.
     */
    function ne(ebytes64 a, ebytes64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes64(padToBytes64(hex""));
        }
        if (!isInitialized(b)) {
            b = asEbytes64(padToBytes64(hex""));
        }
        return ebool.wrap(Impl.ne(ebytes64.unwrap(a), ebytes64.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(ebytes128 a, ebytes128 b) and returns the result.
     */
    function eq(ebytes128 a, ebytes128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes128(padToBytes128(hex""));
        }
        if (!isInitialized(b)) {
            b = asEbytes128(padToBytes128(hex""));
        }
        return ebool.wrap(Impl.eq(ebytes128.unwrap(a), ebytes128.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(ebytes128 a, ebytes128 b) and returns the result.
     */
    function ne(ebytes128 a, ebytes128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes128(padToBytes128(hex""));
        }
        if (!isInitialized(b)) {
            b = asEbytes128(padToBytes128(hex""));
        }
        return ebool.wrap(Impl.ne(ebytes128.unwrap(a), ebytes128.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(ebytes256 a, ebytes256 b) and returns the result.
     */
    function eq(ebytes256 a, ebytes256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes256(padToBytes256(hex""));
        }
        if (!isInitialized(b)) {
            b = asEbytes256(padToBytes256(hex""));
        }
        return ebool.wrap(Impl.eq(ebytes256.unwrap(a), ebytes256.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(ebytes256 a, ebytes256 b) and returns the result.
     */
    function ne(ebytes256 a, ebytes256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes256(padToBytes256(hex""));
        }
        if (!isInitialized(b)) {
            b = asEbytes256(padToBytes256(hex""));
        }
        return ebool.wrap(Impl.ne(ebytes256.unwrap(a), ebytes256.unwrap(b), false));
    }

    /**
     * @dev Evaluates eq(eaddress a, eaddress b) and returns the result.
     */
    function eq(eaddress a, eaddress b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), eaddress.unwrap(b), false));
    }

    /**
     * @dev Evaluates ne(eaddress a, eaddress b) and returns the result.
     */
    function ne(eaddress a, eaddress b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), eaddress.unwrap(b), false));
    }

    /**
     * @dev Evaluates and(ebool a, bool b) and returns the result.
     */
    function and(ebool a, bool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        return ebool.wrap(Impl.and(ebool.unwrap(a), bytes32(uint256(b ? 1 : 0)), true));
    }

    /**
     * @dev Evaluates and(bool a, ebool b) and returns the result.
     */
    function and(bool a, ebool b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.and(ebool.unwrap(b), bytes32(uint256(a ? 1 : 0)), true));
    }

    /**
     * @dev Evaluates or(ebool a, bool b) and returns the result.
     */
    function or(ebool a, bool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        return ebool.wrap(Impl.or(ebool.unwrap(a), bytes32(uint256(b ? 1 : 0)), true));
    }

    /**
     * @dev Evaluates or(bool a, ebool b) and returns the result.
     */
    function or(bool a, ebool b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.or(ebool.unwrap(b), bytes32(uint256(a ? 1 : 0)), true));
    }

    /**
     * @dev Evaluates xor(ebool a, bool b) and returns the result.
     */
    function xor(ebool a, bool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        return ebool.wrap(Impl.xor(ebool.unwrap(a), bytes32(uint256(b ? 1 : 0)), true));
    }

    /**
     * @dev Evaluates xor(bool a, ebool b) and returns the result.
     */
    function xor(bool a, ebool b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.xor(ebool.unwrap(b), bytes32(uint256(a ? 1 : 0)), true));
    }

    /**
     * @dev Evaluates eq(ebool a, bool b) and returns the result.
     */
    function eq(ebool a, bool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        return ebool.wrap(Impl.eq(ebool.unwrap(a), bytes32(uint256(b ? 1 : 0)), true));
    }

    /**
     * @dev Evaluates eq(bool a, ebool b) and returns the result.
     */
    function eq(bool a, ebool b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.eq(ebool.unwrap(b), bytes32(uint256(a ? 1 : 0)), true));
    }

    /**
     * @dev Evaluates ne(ebool a, bool b) and returns the result.
     */
    function ne(ebool a, bool b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbool(false);
        }
        return ebool.wrap(Impl.ne(ebool.unwrap(a), bytes32(uint256(b ? 1 : 0)), true));
    }

    /**
     * @dev Evaluates ne(bool a, ebool b) and returns the result.
     */
    function ne(bool a, ebool b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbool(false);
        }
        return ebool.wrap(Impl.ne(ebool.unwrap(b), bytes32(uint256(a ? 1 : 0)), true));
    }

    /**
     * @dev Evaluates add(euint8 a, uint8 b) and returns the result.
     */
    function add(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.add(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates add(uint8 a, euint8 b) and returns the result.
     */
    function add(uint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.add(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates sub(euint8 a, uint8 b) and returns the result.
     */
    function sub(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.sub(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates sub(uint8 a, euint8 b) and returns the result.
     */
    function sub(uint8 a, euint8 b) internal returns (euint8) {
        euint8 aEnc = asEuint8(a);
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.sub(euint8.unwrap(aEnc), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint8 a, uint8 b) and returns the result.
     */
    function mul(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.mul(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates mul(uint8 a, euint8 b) and returns the result.
     */
    function mul(uint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.mul(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates div(euint8 a, uint8 b) and returns the result.
     */
    function div(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.div(euint8.unwrap(a), bytes32(uint256(b))));
    }

    /**
     * @dev Evaluates rem(euint8 a, uint8 b) and returns the result.
     */
    function rem(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.rem(euint8.unwrap(a), bytes32(uint256(b))));
    }

    /**
     * @dev Evaluates and(euint8 a, uint8 b) and returns the result.
     */
    function and(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.and(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates and(uint8 a, euint8 b) and returns the result.
     */
    function and(uint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.and(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates or(euint8 a, uint8 b) and returns the result.
     */
    function or(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.or(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates or(uint8 a, euint8 b) and returns the result.
     */
    function or(uint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.or(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates xor(euint8 a, uint8 b) and returns the result.
     */
    function xor(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.xor(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates xor(uint8 a, euint8 b) and returns the result.
     */
    function xor(uint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.xor(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates eq(euint8 a, uint8 b) and returns the result.
     */
    function eq(euint8 a, uint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates eq(uint8 a, euint8 b) and returns the result.
     */
    function eq(uint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.eq(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates ne(euint8 a, uint8 b) and returns the result.
     */
    function ne(euint8 a, uint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates ne(uint8 a, euint8 b) and returns the result.
     */
    function ne(uint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ne(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates ge(euint8 a, uint8 b) and returns the result.
     */
    function ge(euint8 a, uint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates ge(uint8 a, euint8 b) and returns the result.
     */
    function ge(uint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates gt(euint8 a, uint8 b) and returns the result.
     */
    function gt(euint8 a, uint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates gt(uint8 a, euint8 b) and returns the result.
     */
    function gt(uint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates le(euint8 a, uint8 b) and returns the result.
     */
    function le(euint8 a, uint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.le(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates le(uint8 a, euint8 b) and returns the result.
     */
    function le(uint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.ge(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates lt(euint8 a, uint8 b) and returns the result.
     */
    function lt(euint8 a, uint8 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return ebool.wrap(Impl.lt(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates lt(uint8 a, euint8 b) and returns the result.
     */
    function lt(uint8 a, euint8 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return ebool.wrap(Impl.gt(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates min(euint8 a, uint8 b) and returns the result.
     */
    function min(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.min(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates min(uint8 a, euint8 b) and returns the result.
     */
    function min(uint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.min(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates max(euint8 a, uint8 b) and returns the result.
     */
    function max(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.max(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates max(uint8 a, euint8 b) and returns the result.
     */
    function max(uint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.max(euint8.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates add(euint16 a, uint16 b) and returns the result.
     */
    function add(euint16 a, uint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates add(uint16 a, euint16 b) and returns the result.
     */
    function add(uint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.add(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates sub(euint16 a, uint16 b) and returns the result.
     */
    function sub(euint16 a, uint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates sub(uint16 a, euint16 b) and returns the result.
     */
    function sub(uint16 a, euint16 b) internal returns (euint16) {
        euint16 aEnc = asEuint16(a);
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.sub(euint16.unwrap(aEnc), euint16.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint16 a, uint16 b) and returns the result.
     */
    function mul(euint16 a, uint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates mul(uint16 a, euint16 b) and returns the result.
     */
    function mul(uint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.mul(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates div(euint16 a, uint16 b) and returns the result.
     */
    function div(euint16 a, uint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.div(euint16.unwrap(a), bytes32(uint256(b))));
    }

    /**
     * @dev Evaluates rem(euint16 a, uint16 b) and returns the result.
     */
    function rem(euint16 a, uint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.rem(euint16.unwrap(a), bytes32(uint256(b))));
    }

    /**
     * @dev Evaluates and(euint16 a, uint16 b) and returns the result.
     */
    function and(euint16 a, uint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.and(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates and(uint16 a, euint16 b) and returns the result.
     */
    function and(uint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.and(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates or(euint16 a, uint16 b) and returns the result.
     */
    function or(euint16 a, uint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.or(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates or(uint16 a, euint16 b) and returns the result.
     */
    function or(uint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.or(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates xor(euint16 a, uint16 b) and returns the result.
     */
    function xor(euint16 a, uint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.xor(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates xor(uint16 a, euint16 b) and returns the result.
     */
    function xor(uint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.xor(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates eq(euint16 a, uint16 b) and returns the result.
     */
    function eq(euint16 a, uint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates eq(uint16 a, euint16 b) and returns the result.
     */
    function eq(uint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.eq(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates ne(euint16 a, uint16 b) and returns the result.
     */
    function ne(euint16 a, uint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates ne(uint16 a, euint16 b) and returns the result.
     */
    function ne(uint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ne(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates ge(euint16 a, uint16 b) and returns the result.
     */
    function ge(euint16 a, uint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates ge(uint16 a, euint16 b) and returns the result.
     */
    function ge(uint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates gt(euint16 a, uint16 b) and returns the result.
     */
    function gt(euint16 a, uint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates gt(uint16 a, euint16 b) and returns the result.
     */
    function gt(uint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates le(euint16 a, uint16 b) and returns the result.
     */
    function le(euint16 a, uint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.le(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates le(uint16 a, euint16 b) and returns the result.
     */
    function le(uint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.ge(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates lt(euint16 a, uint16 b) and returns the result.
     */
    function lt(euint16 a, uint16 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return ebool.wrap(Impl.lt(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates lt(uint16 a, euint16 b) and returns the result.
     */
    function lt(uint16 a, euint16 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return ebool.wrap(Impl.gt(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates min(euint16 a, uint16 b) and returns the result.
     */
    function min(euint16 a, uint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates min(uint16 a, euint16 b) and returns the result.
     */
    function min(uint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.min(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates max(euint16 a, uint16 b) and returns the result.
     */
    function max(euint16 a, uint16 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates max(uint16 a, euint16 b) and returns the result.
     */
    function max(uint16 a, euint16 b) internal returns (euint16) {
        if (!isInitialized(b)) {
            b = asEuint16(0);
        }
        return euint16.wrap(Impl.max(euint16.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates add(euint32 a, uint32 b) and returns the result.
     */
    function add(euint32 a, uint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates add(uint32 a, euint32 b) and returns the result.
     */
    function add(uint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.add(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates sub(euint32 a, uint32 b) and returns the result.
     */
    function sub(euint32 a, uint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates sub(uint32 a, euint32 b) and returns the result.
     */
    function sub(uint32 a, euint32 b) internal returns (euint32) {
        euint32 aEnc = asEuint32(a);
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.sub(euint32.unwrap(aEnc), euint32.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint32 a, uint32 b) and returns the result.
     */
    function mul(euint32 a, uint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates mul(uint32 a, euint32 b) and returns the result.
     */
    function mul(uint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.mul(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates div(euint32 a, uint32 b) and returns the result.
     */
    function div(euint32 a, uint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.div(euint32.unwrap(a), bytes32(uint256(b))));
    }

    /**
     * @dev Evaluates rem(euint32 a, uint32 b) and returns the result.
     */
    function rem(euint32 a, uint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.rem(euint32.unwrap(a), bytes32(uint256(b))));
    }

    /**
     * @dev Evaluates and(euint32 a, uint32 b) and returns the result.
     */
    function and(euint32 a, uint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates and(uint32 a, euint32 b) and returns the result.
     */
    function and(uint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.and(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates or(euint32 a, uint32 b) and returns the result.
     */
    function or(euint32 a, uint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates or(uint32 a, euint32 b) and returns the result.
     */
    function or(uint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.or(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates xor(euint32 a, uint32 b) and returns the result.
     */
    function xor(euint32 a, uint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates xor(uint32 a, euint32 b) and returns the result.
     */
    function xor(uint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.xor(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates eq(euint32 a, uint32 b) and returns the result.
     */
    function eq(euint32 a, uint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates eq(uint32 a, euint32 b) and returns the result.
     */
    function eq(uint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.eq(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates ne(euint32 a, uint32 b) and returns the result.
     */
    function ne(euint32 a, uint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates ne(uint32 a, euint32 b) and returns the result.
     */
    function ne(uint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ne(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates ge(euint32 a, uint32 b) and returns the result.
     */
    function ge(euint32 a, uint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates ge(uint32 a, euint32 b) and returns the result.
     */
    function ge(uint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates gt(euint32 a, uint32 b) and returns the result.
     */
    function gt(euint32 a, uint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates gt(uint32 a, euint32 b) and returns the result.
     */
    function gt(uint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates le(euint32 a, uint32 b) and returns the result.
     */
    function le(euint32 a, uint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.le(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates le(uint32 a, euint32 b) and returns the result.
     */
    function le(uint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.ge(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates lt(euint32 a, uint32 b) and returns the result.
     */
    function lt(euint32 a, uint32 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return ebool.wrap(Impl.lt(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates lt(uint32 a, euint32 b) and returns the result.
     */
    function lt(uint32 a, euint32 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return ebool.wrap(Impl.gt(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates min(euint32 a, uint32 b) and returns the result.
     */
    function min(euint32 a, uint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates min(uint32 a, euint32 b) and returns the result.
     */
    function min(uint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.min(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates max(euint32 a, uint32 b) and returns the result.
     */
    function max(euint32 a, uint32 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates max(uint32 a, euint32 b) and returns the result.
     */
    function max(uint32 a, euint32 b) internal returns (euint32) {
        if (!isInitialized(b)) {
            b = asEuint32(0);
        }
        return euint32.wrap(Impl.max(euint32.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates add(euint64 a, uint64 b) and returns the result.
     */
    function add(euint64 a, uint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates add(uint64 a, euint64 b) and returns the result.
     */
    function add(uint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.add(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates sub(euint64 a, uint64 b) and returns the result.
     */
    function sub(euint64 a, uint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates sub(uint64 a, euint64 b) and returns the result.
     */
    function sub(uint64 a, euint64 b) internal returns (euint64) {
        euint64 aEnc = asEuint64(a);
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.sub(euint64.unwrap(aEnc), euint64.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint64 a, uint64 b) and returns the result.
     */
    function mul(euint64 a, uint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates mul(uint64 a, euint64 b) and returns the result.
     */
    function mul(uint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.mul(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates div(euint64 a, uint64 b) and returns the result.
     */
    function div(euint64 a, uint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.div(euint64.unwrap(a), bytes32(uint256(b))));
    }

    /**
     * @dev Evaluates rem(euint64 a, uint64 b) and returns the result.
     */
    function rem(euint64 a, uint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.rem(euint64.unwrap(a), bytes32(uint256(b))));
    }

    /**
     * @dev Evaluates and(euint64 a, uint64 b) and returns the result.
     */
    function and(euint64 a, uint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates and(uint64 a, euint64 b) and returns the result.
     */
    function and(uint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.and(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates or(euint64 a, uint64 b) and returns the result.
     */
    function or(euint64 a, uint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates or(uint64 a, euint64 b) and returns the result.
     */
    function or(uint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.or(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates xor(euint64 a, uint64 b) and returns the result.
     */
    function xor(euint64 a, uint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates xor(uint64 a, euint64 b) and returns the result.
     */
    function xor(uint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.xor(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates eq(euint64 a, uint64 b) and returns the result.
     */
    function eq(euint64 a, uint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates eq(uint64 a, euint64 b) and returns the result.
     */
    function eq(uint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.eq(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates ne(euint64 a, uint64 b) and returns the result.
     */
    function ne(euint64 a, uint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates ne(uint64 a, euint64 b) and returns the result.
     */
    function ne(uint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ne(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates ge(euint64 a, uint64 b) and returns the result.
     */
    function ge(euint64 a, uint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates ge(uint64 a, euint64 b) and returns the result.
     */
    function ge(uint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates gt(euint64 a, uint64 b) and returns the result.
     */
    function gt(euint64 a, uint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates gt(uint64 a, euint64 b) and returns the result.
     */
    function gt(uint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates le(euint64 a, uint64 b) and returns the result.
     */
    function le(euint64 a, uint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.le(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates le(uint64 a, euint64 b) and returns the result.
     */
    function le(uint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.ge(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates lt(euint64 a, uint64 b) and returns the result.
     */
    function lt(euint64 a, uint64 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return ebool.wrap(Impl.lt(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates lt(uint64 a, euint64 b) and returns the result.
     */
    function lt(uint64 a, euint64 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return ebool.wrap(Impl.gt(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates min(euint64 a, uint64 b) and returns the result.
     */
    function min(euint64 a, uint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates min(uint64 a, euint64 b) and returns the result.
     */
    function min(uint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.min(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates max(euint64 a, uint64 b) and returns the result.
     */
    function max(euint64 a, uint64 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates max(uint64 a, euint64 b) and returns the result.
     */
    function max(uint64 a, euint64 b) internal returns (euint64) {
        if (!isInitialized(b)) {
            b = asEuint64(0);
        }
        return euint64.wrap(Impl.max(euint64.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates add(euint128 a, uint128 b) and returns the result.
     */
    function add(euint128 a, uint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.add(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates add(uint128 a, euint128 b) and returns the result.
     */
    function add(uint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.add(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates sub(euint128 a, uint128 b) and returns the result.
     */
    function sub(euint128 a, uint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.sub(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates sub(uint128 a, euint128 b) and returns the result.
     */
    function sub(uint128 a, euint128 b) internal returns (euint128) {
        euint128 aEnc = asEuint128(a);
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.sub(euint128.unwrap(aEnc), euint128.unwrap(b), false));
    }

    /**
     * @dev Evaluates mul(euint128 a, uint128 b) and returns the result.
     */
    function mul(euint128 a, uint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.mul(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates mul(uint128 a, euint128 b) and returns the result.
     */
    function mul(uint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.mul(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates div(euint128 a, uint128 b) and returns the result.
     */
    function div(euint128 a, uint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.div(euint128.unwrap(a), bytes32(uint256(b))));
    }

    /**
     * @dev Evaluates rem(euint128 a, uint128 b) and returns the result.
     */
    function rem(euint128 a, uint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.rem(euint128.unwrap(a), bytes32(uint256(b))));
    }

    /**
     * @dev Evaluates and(euint128 a, uint128 b) and returns the result.
     */
    function and(euint128 a, uint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.and(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates and(uint128 a, euint128 b) and returns the result.
     */
    function and(uint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.and(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates or(euint128 a, uint128 b) and returns the result.
     */
    function or(euint128 a, uint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.or(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates or(uint128 a, euint128 b) and returns the result.
     */
    function or(uint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.or(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates xor(euint128 a, uint128 b) and returns the result.
     */
    function xor(euint128 a, uint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.xor(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates xor(uint128 a, euint128 b) and returns the result.
     */
    function xor(uint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.xor(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates eq(euint128 a, uint128 b) and returns the result.
     */
    function eq(euint128 a, uint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return ebool.wrap(Impl.eq(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates eq(uint128 a, euint128 b) and returns the result.
     */
    function eq(uint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.eq(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates ne(euint128 a, uint128 b) and returns the result.
     */
    function ne(euint128 a, uint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return ebool.wrap(Impl.ne(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates ne(uint128 a, euint128 b) and returns the result.
     */
    function ne(uint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ne(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates ge(euint128 a, uint128 b) and returns the result.
     */
    function ge(euint128 a, uint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return ebool.wrap(Impl.ge(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates ge(uint128 a, euint128 b) and returns the result.
     */
    function ge(uint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.le(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates gt(euint128 a, uint128 b) and returns the result.
     */
    function gt(euint128 a, uint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return ebool.wrap(Impl.gt(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates gt(uint128 a, euint128 b) and returns the result.
     */
    function gt(uint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.lt(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates le(euint128 a, uint128 b) and returns the result.
     */
    function le(euint128 a, uint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return ebool.wrap(Impl.le(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates le(uint128 a, euint128 b) and returns the result.
     */
    function le(uint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.ge(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates lt(euint128 a, uint128 b) and returns the result.
     */
    function lt(euint128 a, uint128 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return ebool.wrap(Impl.lt(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates lt(uint128 a, euint128 b) and returns the result.
     */
    function lt(uint128 a, euint128 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return ebool.wrap(Impl.gt(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates min(euint128 a, uint128 b) and returns the result.
     */
    function min(euint128 a, uint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.min(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates min(uint128 a, euint128 b) and returns the result.
     */
    function min(uint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.min(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates max(euint128 a, uint128 b) and returns the result.
     */
    function max(euint128 a, uint128 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.max(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates max(uint128 a, euint128 b) and returns the result.
     */
    function max(uint128 a, euint128 b) internal returns (euint128) {
        if (!isInitialized(b)) {
            b = asEuint128(0);
        }
        return euint128.wrap(Impl.max(euint128.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates and(euint256 a, uint256 b) and returns the result.
     */
    function and(euint256 a, uint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates and(uint256 a, euint256 b) and returns the result.
     */
    function and(uint256 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.and(euint256.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates or(euint256 a, uint256 b) and returns the result.
     */
    function or(euint256 a, uint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates or(uint256 a, euint256 b) and returns the result.
     */
    function or(uint256 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.or(euint256.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates xor(euint256 a, uint256 b) and returns the result.
     */
    function xor(euint256 a, uint256 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates xor(uint256 a, euint256 b) and returns the result.
     */
    function xor(uint256 a, euint256 b) internal returns (euint256) {
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return euint256.wrap(Impl.xor(euint256.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates eq(euint256 a, uint256 b) and returns the result.
     */
    function eq(euint256 a, uint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates eq(uint256 a, euint256 b) and returns the result.
     */
    function eq(uint256 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.eq(euint256.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates ne(euint256 a, uint256 b) and returns the result.
     */
    function ne(euint256 a, uint256 b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates ne(uint256 a, euint256 b) and returns the result.
     */
    function ne(uint256 a, euint256 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEuint256(0);
        }
        return ebool.wrap(Impl.ne(euint256.unwrap(b), bytes32(uint256(a)), true));
    }

    /**
     * @dev Evaluates eq(ebytes64 a, bytes memory b) and returns the result.
     */
    function eq(ebytes64 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes64(padToBytes64(hex""));
        }
        return ebool.wrap(Impl.eq(ebytes64.unwrap(a), b, true));
    }

    /**
     * @dev Evaluates eq(bytes memory a, ebytes64 b) and returns the result.
     */
    function eq(bytes memory a, ebytes64 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes64(padToBytes64(hex""));
        }
        return ebool.wrap(Impl.eq(ebytes64.unwrap(b), a, true));
    }

    /**
     * @dev Evaluates ne(ebytes64 a, bytes memory b) and returns the result.
     */
    function ne(ebytes64 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes64(padToBytes64(hex""));
        }
        return ebool.wrap(Impl.ne(ebytes64.unwrap(a), b, true));
    }

    /**
     * @dev Evaluates ne(bytes memory a, ebytes64 b) and returns the result.
     */
    function ne(bytes memory a, ebytes64 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes64(padToBytes64(hex""));
        }
        return ebool.wrap(Impl.ne(ebytes64.unwrap(b), a, true));
    }

    /**
     * @dev Evaluates eq(ebytes128 a, bytes memory b) and returns the result.
     */
    function eq(ebytes128 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes128(padToBytes128(hex""));
        }
        return ebool.wrap(Impl.eq(ebytes128.unwrap(a), b, true));
    }

    /**
     * @dev Evaluates eq(bytes memory a, ebytes128 b) and returns the result.
     */
    function eq(bytes memory a, ebytes128 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes128(padToBytes128(hex""));
        }
        return ebool.wrap(Impl.eq(ebytes128.unwrap(b), a, true));
    }

    /**
     * @dev Evaluates ne(ebytes128 a, bytes memory b) and returns the result.
     */
    function ne(ebytes128 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes128(padToBytes128(hex""));
        }
        return ebool.wrap(Impl.ne(ebytes128.unwrap(a), b, true));
    }

    /**
     * @dev Evaluates ne(bytes memory a, ebytes128 b) and returns the result.
     */
    function ne(bytes memory a, ebytes128 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes128(padToBytes128(hex""));
        }
        return ebool.wrap(Impl.ne(ebytes128.unwrap(b), a, true));
    }

    /**
     * @dev Evaluates eq(ebytes256 a, bytes memory b) and returns the result.
     */
    function eq(ebytes256 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes256(padToBytes256(hex""));
        }
        return ebool.wrap(Impl.eq(ebytes256.unwrap(a), b, true));
    }

    /**
     * @dev Evaluates eq(bytes memory a, ebytes256 b) and returns the result.
     */
    function eq(bytes memory a, ebytes256 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes256(padToBytes256(hex""));
        }
        return ebool.wrap(Impl.eq(ebytes256.unwrap(b), a, true));
    }

    /**
     * @dev Evaluates ne(ebytes256 a, bytes memory b) and returns the result.
     */
    function ne(ebytes256 a, bytes memory b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEbytes256(padToBytes256(hex""));
        }
        return ebool.wrap(Impl.ne(ebytes256.unwrap(a), b, true));
    }

    /**
     * @dev Evaluates ne(bytes memory a, ebytes256 b) and returns the result.
     */
    function ne(bytes memory a, ebytes256 b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEbytes256(padToBytes256(hex""));
        }
        return ebool.wrap(Impl.ne(ebytes256.unwrap(b), a, true));
    }

    /**
     * @dev Evaluates eq(eaddress a, address b) and returns the result.
     */
    function eq(eaddress a, address b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        return ebool.wrap(Impl.eq(eaddress.unwrap(a), bytes32(uint256(uint160(b))), true));
    }

    /**
     * @dev Evaluates eq(address a, eaddress b) and returns the result.
     */
    function eq(address a, eaddress b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.eq(eaddress.unwrap(b), bytes32(uint256(uint160(a))), true));
    }

    /**
     * @dev Evaluates ne(eaddress a, address b) and returns the result.
     */
    function ne(eaddress a, address b) internal returns (ebool) {
        if (!isInitialized(a)) {
            a = asEaddress(address(0));
        }
        return ebool.wrap(Impl.ne(eaddress.unwrap(a), bytes32(uint256(uint160(b))), true));
    }

    /**
     * @dev Evaluates ne(address a, eaddress b) and returns the result.
     */
    function ne(address a, eaddress b) internal returns (ebool) {
        if (!isInitialized(b)) {
            b = asEaddress(address(0));
        }
        return ebool.wrap(Impl.ne(eaddress.unwrap(b), bytes32(uint256(uint160(a))), true));
    }

    /**
     * @dev Evaluates shl(euint8 a, euint8 b) and returns the result.
     */
    function shl(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.shl(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates shl(euint8 a, uint8) and returns the result.
     */
    function shl(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.shl(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates shr(euint8 a, euint8 b) and returns the result.
     */
    function shr(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.shr(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates shr(euint8 a, uint8) and returns the result.
     */
    function shr(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.shr(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotl(euint8 a, euint8 b) and returns the result.
     */
    function rotl(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.rotl(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates rotl(euint8 a, uint8) and returns the result.
     */
    function rotl(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.rotl(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotr(euint8 a, euint8 b) and returns the result.
     */
    function rotr(euint8 a, euint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint8.wrap(Impl.rotr(euint8.unwrap(a), euint8.unwrap(b), false));
    }

    /**
     * @dev Evaluates rotr(euint8 a, uint8) and returns the result.
     */
    function rotr(euint8 a, uint8 b) internal returns (euint8) {
        if (!isInitialized(a)) {
            a = asEuint8(0);
        }
        return euint8.wrap(Impl.rotr(euint8.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates shl(euint16 a, euint8 b) and returns the result.
     */
    function shl(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.shl(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates shl(euint16 a, uint8) and returns the result.
     */
    function shl(euint16 a, uint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.shl(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates shr(euint16 a, euint8 b) and returns the result.
     */
    function shr(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.shr(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates shr(euint16 a, uint8) and returns the result.
     */
    function shr(euint16 a, uint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.shr(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotl(euint16 a, euint8 b) and returns the result.
     */
    function rotl(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.rotl(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates rotl(euint16 a, uint8) and returns the result.
     */
    function rotl(euint16 a, uint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.rotl(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotr(euint16 a, euint8 b) and returns the result.
     */
    function rotr(euint16 a, euint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint16.wrap(Impl.rotr(euint16.unwrap(a), euint16.unwrap(asEuint16(b)), false));
    }

    /**
     * @dev Evaluates rotr(euint16 a, uint8) and returns the result.
     */
    function rotr(euint16 a, uint8 b) internal returns (euint16) {
        if (!isInitialized(a)) {
            a = asEuint16(0);
        }
        return euint16.wrap(Impl.rotr(euint16.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates shl(euint32 a, euint8 b) and returns the result.
     */
    function shl(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.shl(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates shl(euint32 a, uint8) and returns the result.
     */
    function shl(euint32 a, uint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.shl(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates shr(euint32 a, euint8 b) and returns the result.
     */
    function shr(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.shr(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates shr(euint32 a, uint8) and returns the result.
     */
    function shr(euint32 a, uint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.shr(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotl(euint32 a, euint8 b) and returns the result.
     */
    function rotl(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.rotl(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates rotl(euint32 a, uint8) and returns the result.
     */
    function rotl(euint32 a, uint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.rotl(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotr(euint32 a, euint8 b) and returns the result.
     */
    function rotr(euint32 a, euint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint32.wrap(Impl.rotr(euint32.unwrap(a), euint32.unwrap(asEuint32(b)), false));
    }

    /**
     * @dev Evaluates rotr(euint32 a, uint8) and returns the result.
     */
    function rotr(euint32 a, uint8 b) internal returns (euint32) {
        if (!isInitialized(a)) {
            a = asEuint32(0);
        }
        return euint32.wrap(Impl.rotr(euint32.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates shl(euint64 a, euint8 b) and returns the result.
     */
    function shl(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.shl(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates shl(euint64 a, uint8) and returns the result.
     */
    function shl(euint64 a, uint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.shl(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates shr(euint64 a, euint8 b) and returns the result.
     */
    function shr(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.shr(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates shr(euint64 a, uint8) and returns the result.
     */
    function shr(euint64 a, uint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.shr(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotl(euint64 a, euint8 b) and returns the result.
     */
    function rotl(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.rotl(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates rotl(euint64 a, uint8) and returns the result.
     */
    function rotl(euint64 a, uint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.rotl(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotr(euint64 a, euint8 b) and returns the result.
     */
    function rotr(euint64 a, euint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint64.wrap(Impl.rotr(euint64.unwrap(a), euint64.unwrap(asEuint64(b)), false));
    }

    /**
     * @dev Evaluates rotr(euint64 a, uint8) and returns the result.
     */
    function rotr(euint64 a, uint8 b) internal returns (euint64) {
        if (!isInitialized(a)) {
            a = asEuint64(0);
        }
        return euint64.wrap(Impl.rotr(euint64.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates shl(euint128 a, euint8 b) and returns the result.
     */
    function shl(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.shl(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates shl(euint128 a, uint8) and returns the result.
     */
    function shl(euint128 a, uint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.shl(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates shr(euint128 a, euint8 b) and returns the result.
     */
    function shr(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.shr(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates shr(euint128 a, uint8) and returns the result.
     */
    function shr(euint128 a, uint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.shr(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotl(euint128 a, euint8 b) and returns the result.
     */
    function rotl(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.rotl(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates rotl(euint128 a, uint8) and returns the result.
     */
    function rotl(euint128 a, uint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.rotl(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotr(euint128 a, euint8 b) and returns the result.
     */
    function rotr(euint128 a, euint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint128.wrap(Impl.rotr(euint128.unwrap(a), euint128.unwrap(asEuint128(b)), false));
    }

    /**
     * @dev Evaluates rotr(euint128 a, uint8) and returns the result.
     */
    function rotr(euint128 a, uint8 b) internal returns (euint128) {
        if (!isInitialized(a)) {
            a = asEuint128(0);
        }
        return euint128.wrap(Impl.rotr(euint128.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates shl(euint256 a, euint8 b) and returns the result.
     */
    function shl(euint256 a, euint8 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint256.wrap(Impl.shl(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates shl(euint256 a, uint8) and returns the result.
     */
    function shl(euint256 a, uint8 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        return euint256.wrap(Impl.shl(euint256.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates shr(euint256 a, euint8 b) and returns the result.
     */
    function shr(euint256 a, euint8 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint256.wrap(Impl.shr(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates shr(euint256 a, uint8) and returns the result.
     */
    function shr(euint256 a, uint8 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        return euint256.wrap(Impl.shr(euint256.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotl(euint256 a, euint8 b) and returns the result.
     */
    function rotl(euint256 a, euint8 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint256.wrap(Impl.rotl(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates rotl(euint256 a, uint8) and returns the result.
     */
    function rotl(euint256 a, uint8 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        return euint256.wrap(Impl.rotl(euint256.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev Evaluates rotr(euint256 a, euint8 b) and returns the result.
     */
    function rotr(euint256 a, euint8 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        if (!isInitialized(b)) {
            b = asEuint8(0);
        }
        return euint256.wrap(Impl.rotr(euint256.unwrap(a), euint256.unwrap(asEuint256(b)), false));
    }

    /**
     * @dev Evaluates rotr(euint256 a, uint8) and returns the result.
     */
    function rotr(euint256 a, uint8 b) internal returns (euint256) {
        if (!isInitialized(a)) {
            a = asEuint256(0);
        }
        return euint256.wrap(Impl.rotr(euint256.unwrap(a), bytes32(uint256(b)), true));
    }

    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(ebool control, ebool a, ebool b) internal returns (ebool) {
        return ebool.wrap(Impl.select(ebool.unwrap(control), ebool.unwrap(a), ebool.unwrap(b)));
    }
    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(ebool control, euint8 a, euint8 b) internal returns (euint8) {
        return euint8.wrap(Impl.select(ebool.unwrap(control), euint8.unwrap(a), euint8.unwrap(b)));
    }
    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(ebool control, euint16 a, euint16 b) internal returns (euint16) {
        return euint16.wrap(Impl.select(ebool.unwrap(control), euint16.unwrap(a), euint16.unwrap(b)));
    }
    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(ebool control, euint32 a, euint32 b) internal returns (euint32) {
        return euint32.wrap(Impl.select(ebool.unwrap(control), euint32.unwrap(a), euint32.unwrap(b)));
    }
    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(ebool control, euint64 a, euint64 b) internal returns (euint64) {
        return euint64.wrap(Impl.select(ebool.unwrap(control), euint64.unwrap(a), euint64.unwrap(b)));
    }
    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(ebool control, euint128 a, euint128 b) internal returns (euint128) {
        return euint128.wrap(Impl.select(ebool.unwrap(control), euint128.unwrap(a), euint128.unwrap(b)));
    }
    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(ebool control, euint256 a, euint256 b) internal returns (euint256) {
        return euint256.wrap(Impl.select(ebool.unwrap(control), euint256.unwrap(a), euint256.unwrap(b)));
    }
    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(ebool control, ebytes64 a, ebytes64 b) internal returns (ebytes64) {
        return ebytes64.wrap(Impl.select(ebool.unwrap(control), ebytes64.unwrap(a), ebytes64.unwrap(b)));
    }
    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(ebool control, ebytes128 a, ebytes128 b) internal returns (ebytes128) {
        return ebytes128.wrap(Impl.select(ebool.unwrap(control), ebytes128.unwrap(a), ebytes128.unwrap(b)));
    }
    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(ebool control, ebytes256 a, ebytes256 b) internal returns (ebytes256) {
        return ebytes256.wrap(Impl.select(ebool.unwrap(control), ebytes256.unwrap(a), ebytes256.unwrap(b)));
    }
    /**
     * @dev If 'control's value is 'true', the result has the same value as 'ifTrue'.
     *      If 'control's value is 'false', the result has the same value as 'ifFalse'.
     */
    function select(ebool control, eaddress a, eaddress b) internal returns (eaddress) {
        return eaddress.wrap(Impl.select(ebool.unwrap(control), eaddress.unwrap(a), eaddress.unwrap(b)));
    }
    /**
     * @dev Casts an encrypted integer from 'euint16' to 'euint8'.
     */
    function asEuint8(euint16 value) internal returns (euint8) {
        return euint8.wrap(Impl.cast(euint16.unwrap(value), FheType.Uint8));
    }

    /**
     * @dev Casts an encrypted integer from 'euint32' to 'euint8'.
     */
    function asEuint8(euint32 value) internal returns (euint8) {
        return euint8.wrap(Impl.cast(euint32.unwrap(value), FheType.Uint8));
    }

    /**
     * @dev Casts an encrypted integer from 'euint64' to 'euint8'.
     */
    function asEuint8(euint64 value) internal returns (euint8) {
        return euint8.wrap(Impl.cast(euint64.unwrap(value), FheType.Uint8));
    }

    /**
     * @dev Casts an encrypted integer from 'euint128' to 'euint8'.
     */
    function asEuint8(euint128 value) internal returns (euint8) {
        return euint8.wrap(Impl.cast(euint128.unwrap(value), FheType.Uint8));
    }

    /**
     * @dev Casts an encrypted integer from 'euint256' to 'euint8'.
     */
    function asEuint8(euint256 value) internal returns (euint8) {
        return euint8.wrap(Impl.cast(euint256.unwrap(value), FheType.Uint8));
    }

    /**
    /** 
     * @dev Converts an 'ebool' to an 'euint8'.
     */
    function asEuint8(ebool b) internal returns (euint8) {
        return euint8.wrap(Impl.cast(ebool.unwrap(b), FheType.Uint8));
    }

    /**
     * @dev Casts an encrypted integer from 'euint8' to 'ebool'.
     */
    function asEbool(euint8 value) internal returns (ebool) {
        return ne(value, 0);
    }

    /**
     * @dev Casts an encrypted integer from 'euint8' to 'euint16'.
     */
    function asEuint16(euint8 value) internal returns (euint16) {
        return euint16.wrap(Impl.cast(euint8.unwrap(value), FheType.Uint16));
    }

    /**
     * @dev Casts an encrypted integer from 'euint32' to 'euint16'.
     */
    function asEuint16(euint32 value) internal returns (euint16) {
        return euint16.wrap(Impl.cast(euint32.unwrap(value), FheType.Uint16));
    }

    /**
     * @dev Casts an encrypted integer from 'euint64' to 'euint16'.
     */
    function asEuint16(euint64 value) internal returns (euint16) {
        return euint16.wrap(Impl.cast(euint64.unwrap(value), FheType.Uint16));
    }

    /**
     * @dev Casts an encrypted integer from 'euint128' to 'euint16'.
     */
    function asEuint16(euint128 value) internal returns (euint16) {
        return euint16.wrap(Impl.cast(euint128.unwrap(value), FheType.Uint16));
    }

    /**
     * @dev Casts an encrypted integer from 'euint256' to 'euint16'.
     */
    function asEuint16(euint256 value) internal returns (euint16) {
        return euint16.wrap(Impl.cast(euint256.unwrap(value), FheType.Uint16));
    }

    /**
    /** 
     * @dev Converts an 'ebool' to an 'euint16'.
     */
    function asEuint16(ebool b) internal returns (euint16) {
        return euint16.wrap(Impl.cast(ebool.unwrap(b), FheType.Uint16));
    }

    /**
     * @dev Casts an encrypted integer from 'euint16' to 'ebool'.
     */
    function asEbool(euint16 value) internal returns (ebool) {
        return ne(value, 0);
    }

    /**
     * @dev Casts an encrypted integer from 'euint8' to 'euint32'.
     */
    function asEuint32(euint8 value) internal returns (euint32) {
        return euint32.wrap(Impl.cast(euint8.unwrap(value), FheType.Uint32));
    }

    /**
     * @dev Casts an encrypted integer from 'euint16' to 'euint32'.
     */
    function asEuint32(euint16 value) internal returns (euint32) {
        return euint32.wrap(Impl.cast(euint16.unwrap(value), FheType.Uint32));
    }

    /**
     * @dev Casts an encrypted integer from 'euint64' to 'euint32'.
     */
    function asEuint32(euint64 value) internal returns (euint32) {
        return euint32.wrap(Impl.cast(euint64.unwrap(value), FheType.Uint32));
    }

    /**
     * @dev Casts an encrypted integer from 'euint128' to 'euint32'.
     */
    function asEuint32(euint128 value) internal returns (euint32) {
        return euint32.wrap(Impl.cast(euint128.unwrap(value), FheType.Uint32));
    }

    /**
     * @dev Casts an encrypted integer from 'euint256' to 'euint32'.
     */
    function asEuint32(euint256 value) internal returns (euint32) {
        return euint32.wrap(Impl.cast(euint256.unwrap(value), FheType.Uint32));
    }

    /**
    /** 
     * @dev Converts an 'ebool' to an 'euint32'.
     */
    function asEuint32(ebool b) internal returns (euint32) {
        return euint32.wrap(Impl.cast(ebool.unwrap(b), FheType.Uint32));
    }

    /**
     * @dev Casts an encrypted integer from 'euint32' to 'ebool'.
     */
    function asEbool(euint32 value) internal returns (ebool) {
        return ne(value, 0);
    }

    /**
     * @dev Casts an encrypted integer from 'euint8' to 'euint64'.
     */
    function asEuint64(euint8 value) internal returns (euint64) {
        return euint64.wrap(Impl.cast(euint8.unwrap(value), FheType.Uint64));
    }

    /**
     * @dev Casts an encrypted integer from 'euint16' to 'euint64'.
     */
    function asEuint64(euint16 value) internal returns (euint64) {
        return euint64.wrap(Impl.cast(euint16.unwrap(value), FheType.Uint64));
    }

    /**
     * @dev Casts an encrypted integer from 'euint32' to 'euint64'.
     */
    function asEuint64(euint32 value) internal returns (euint64) {
        return euint64.wrap(Impl.cast(euint32.unwrap(value), FheType.Uint64));
    }

    /**
     * @dev Casts an encrypted integer from 'euint128' to 'euint64'.
     */
    function asEuint64(euint128 value) internal returns (euint64) {
        return euint64.wrap(Impl.cast(euint128.unwrap(value), FheType.Uint64));
    }

    /**
     * @dev Casts an encrypted integer from 'euint256' to 'euint64'.
     */
    function asEuint64(euint256 value) internal returns (euint64) {
        return euint64.wrap(Impl.cast(euint256.unwrap(value), FheType.Uint64));
    }

    /**
    /** 
     * @dev Converts an 'ebool' to an 'euint64'.
     */
    function asEuint64(ebool b) internal returns (euint64) {
        return euint64.wrap(Impl.cast(ebool.unwrap(b), FheType.Uint64));
    }

    /**
     * @dev Casts an encrypted integer from 'euint64' to 'ebool'.
     */
    function asEbool(euint64 value) internal returns (ebool) {
        return ne(value, 0);
    }

    /**
     * @dev Casts an encrypted integer from 'euint8' to 'euint128'.
     */
    function asEuint128(euint8 value) internal returns (euint128) {
        return euint128.wrap(Impl.cast(euint8.unwrap(value), FheType.Uint128));
    }

    /**
     * @dev Casts an encrypted integer from 'euint16' to 'euint128'.
     */
    function asEuint128(euint16 value) internal returns (euint128) {
        return euint128.wrap(Impl.cast(euint16.unwrap(value), FheType.Uint128));
    }

    /**
     * @dev Casts an encrypted integer from 'euint32' to 'euint128'.
     */
    function asEuint128(euint32 value) internal returns (euint128) {
        return euint128.wrap(Impl.cast(euint32.unwrap(value), FheType.Uint128));
    }

    /**
     * @dev Casts an encrypted integer from 'euint64' to 'euint128'.
     */
    function asEuint128(euint64 value) internal returns (euint128) {
        return euint128.wrap(Impl.cast(euint64.unwrap(value), FheType.Uint128));
    }

    /**
     * @dev Casts an encrypted integer from 'euint256' to 'euint128'.
     */
    function asEuint128(euint256 value) internal returns (euint128) {
        return euint128.wrap(Impl.cast(euint256.unwrap(value), FheType.Uint128));
    }

    /**
    /** 
     * @dev Converts an 'ebool' to an 'euint128'.
     */
    function asEuint128(ebool b) internal returns (euint128) {
        return euint128.wrap(Impl.cast(ebool.unwrap(b), FheType.Uint128));
    }

    /**
     * @dev Casts an encrypted integer from 'euint128' to 'ebool'.
     */
    function asEbool(euint128 value) internal returns (ebool) {
        return ne(value, 0);
    }

    /**
     * @dev Casts an encrypted integer from 'euint8' to 'euint256'.
     */
    function asEuint256(euint8 value) internal returns (euint256) {
        return euint256.wrap(Impl.cast(euint8.unwrap(value), FheType.Uint256));
    }

    /**
     * @dev Casts an encrypted integer from 'euint16' to 'euint256'.
     */
    function asEuint256(euint16 value) internal returns (euint256) {
        return euint256.wrap(Impl.cast(euint16.unwrap(value), FheType.Uint256));
    }

    /**
     * @dev Casts an encrypted integer from 'euint32' to 'euint256'.
     */
    function asEuint256(euint32 value) internal returns (euint256) {
        return euint256.wrap(Impl.cast(euint32.unwrap(value), FheType.Uint256));
    }

    /**
     * @dev Casts an encrypted integer from 'euint64' to 'euint256'.
     */
    function asEuint256(euint64 value) internal returns (euint256) {
        return euint256.wrap(Impl.cast(euint64.unwrap(value), FheType.Uint256));
    }

    /**
     * @dev Casts an encrypted integer from 'euint128' to 'euint256'.
     */
    function asEuint256(euint128 value) internal returns (euint256) {
        return euint256.wrap(Impl.cast(euint128.unwrap(value), FheType.Uint256));
    }

    /**
    /** 
     * @dev Converts an 'ebool' to an 'euint256'.
     */
    function asEuint256(ebool b) internal returns (euint256) {
        return euint256.wrap(Impl.cast(ebool.unwrap(b), FheType.Uint256));
    }

    /**
     * @dev Casts an encrypted integer from 'euint256' to 'ebool'.
     */
    function asEbool(euint256 value) internal returns (ebool) {
        return ne(value, 0);
    }

    /**
     * @dev Evaluates not(ebool value) and returns the result.
     */
    function not(ebool value) internal returns (ebool) {
        return ebool.wrap(Impl.not(ebool.unwrap(value)));
    }

    /**
     * @dev Evaluates neg(euint8 value) and returns the result.
     */
    function neg(euint8 value) internal returns (euint8) {
        return euint8.wrap(Impl.neg(euint8.unwrap(value)));
    }

    /**
     * @dev Evaluates not(euint8 value) and returns the result.
     */
    function not(euint8 value) internal returns (euint8) {
        return euint8.wrap(Impl.not(euint8.unwrap(value)));
    }

    /**
     * @dev Evaluates neg(euint16 value) and returns the result.
     */
    function neg(euint16 value) internal returns (euint16) {
        return euint16.wrap(Impl.neg(euint16.unwrap(value)));
    }

    /**
     * @dev Evaluates not(euint16 value) and returns the result.
     */
    function not(euint16 value) internal returns (euint16) {
        return euint16.wrap(Impl.not(euint16.unwrap(value)));
    }

    /**
     * @dev Evaluates neg(euint32 value) and returns the result.
     */
    function neg(euint32 value) internal returns (euint32) {
        return euint32.wrap(Impl.neg(euint32.unwrap(value)));
    }

    /**
     * @dev Evaluates not(euint32 value) and returns the result.
     */
    function not(euint32 value) internal returns (euint32) {
        return euint32.wrap(Impl.not(euint32.unwrap(value)));
    }

    /**
     * @dev Evaluates neg(euint64 value) and returns the result.
     */
    function neg(euint64 value) internal returns (euint64) {
        return euint64.wrap(Impl.neg(euint64.unwrap(value)));
    }

    /**
     * @dev Evaluates not(euint64 value) and returns the result.
     */
    function not(euint64 value) internal returns (euint64) {
        return euint64.wrap(Impl.not(euint64.unwrap(value)));
    }

    /**
     * @dev Evaluates neg(euint128 value) and returns the result.
     */
    function neg(euint128 value) internal returns (euint128) {
        return euint128.wrap(Impl.neg(euint128.unwrap(value)));
    }

    /**
     * @dev Evaluates not(euint128 value) and returns the result.
     */
    function not(euint128 value) internal returns (euint128) {
        return euint128.wrap(Impl.not(euint128.unwrap(value)));
    }

    /**
     * @dev Evaluates neg(euint256 value) and returns the result.
     */
    function neg(euint256 value) internal returns (euint256) {
        return euint256.wrap(Impl.neg(euint256.unwrap(value)));
    }

    /**
     * @dev Evaluates not(euint256 value) and returns the result.
     */
    function not(euint256 value) internal returns (euint256) {
        return euint256.wrap(Impl.not(euint256.unwrap(value)));
    }

    /**
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted ebool integer.
     */
    function asEbool(einput inputHandle, bytes memory inputProof) internal returns (ebool) {
        return ebool.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.Bool));
    }

    /**
     * @dev Converts a plaintext boolean to an encrypted boolean.
     */
    function asEbool(bool value) internal returns (ebool) {
        return ebool.wrap(Impl.trivialEncrypt(value ? 1 : 0, FheType.Bool));
    }

    /**
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted euint8 integer.
     */
    function asEuint8(einput inputHandle, bytes memory inputProof) internal returns (euint8) {
        return euint8.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.Uint8));
    }

    /**
     * @dev Convert a plaintext value to an encrypted euint8 integer.
     */
    function asEuint8(uint8 value) internal returns (euint8) {
        return euint8.wrap(Impl.trivialEncrypt(uint256(value), FheType.Uint8));
    }

    /**
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted euint16 integer.
     */
    function asEuint16(einput inputHandle, bytes memory inputProof) internal returns (euint16) {
        return euint16.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.Uint16));
    }

    /**
     * @dev Convert a plaintext value to an encrypted euint16 integer.
     */
    function asEuint16(uint16 value) internal returns (euint16) {
        return euint16.wrap(Impl.trivialEncrypt(uint256(value), FheType.Uint16));
    }

    /**
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted euint32 integer.
     */
    function asEuint32(einput inputHandle, bytes memory inputProof) internal returns (euint32) {
        return euint32.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.Uint32));
    }

    /**
     * @dev Convert a plaintext value to an encrypted euint32 integer.
     */
    function asEuint32(uint32 value) internal returns (euint32) {
        return euint32.wrap(Impl.trivialEncrypt(uint256(value), FheType.Uint32));
    }

    /**
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted euint64 integer.
     */
    function asEuint64(einput inputHandle, bytes memory inputProof) internal returns (euint64) {
        return euint64.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.Uint64));
    }

    /**
     * @dev Convert a plaintext value to an encrypted euint64 integer.
     */
    function asEuint64(uint64 value) internal returns (euint64) {
        return euint64.wrap(Impl.trivialEncrypt(uint256(value), FheType.Uint64));
    }

    /**
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted euint128 integer.
     */
    function asEuint128(einput inputHandle, bytes memory inputProof) internal returns (euint128) {
        return euint128.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.Uint128));
    }

    /**
     * @dev Convert a plaintext value to an encrypted euint128 integer.
     */
    function asEuint128(uint128 value) internal returns (euint128) {
        return euint128.wrap(Impl.trivialEncrypt(uint256(value), FheType.Uint128));
    }

    /**
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted euint256 integer.
     */
    function asEuint256(einput inputHandle, bytes memory inputProof) internal returns (euint256) {
        return euint256.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.Uint256));
    }

    /**
     * @dev Convert a plaintext value to an encrypted euint256 integer.
     */
    function asEuint256(uint256 value) internal returns (euint256) {
        return euint256.wrap(Impl.trivialEncrypt(uint256(value), FheType.Uint256));
    }

    /**
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted ebytes64 integer.
     */
    function asEbytes64(einput inputHandle, bytes memory inputProof) internal returns (ebytes64) {
        return ebytes64.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.Uint512));
    }

    /**
     * @dev Convert the plaintext bytes to a ebytes64 value.
     */
    function asEbytes64(bytes memory value) internal returns (ebytes64) {
        return ebytes64.wrap(Impl.trivialEncrypt(value, FheType.Uint512));
    }

    /**
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted ebytes128 integer.
     */
    function asEbytes128(einput inputHandle, bytes memory inputProof) internal returns (ebytes128) {
        return ebytes128.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.Uint1024));
    }

    /**
     * @dev Convert the plaintext bytes to a ebytes128 value.
     */
    function asEbytes128(bytes memory value) internal returns (ebytes128) {
        return ebytes128.wrap(Impl.trivialEncrypt(value, FheType.Uint1024));
    }

    /**
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted ebytes256 integer.
     */
    function asEbytes256(einput inputHandle, bytes memory inputProof) internal returns (ebytes256) {
        return ebytes256.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.Uint2048));
    }

    /**
     * @dev Convert the plaintext bytes to a ebytes256 value.
     */
    function asEbytes256(bytes memory value) internal returns (ebytes256) {
        return ebytes256.wrap(Impl.trivialEncrypt(value, FheType.Uint2048));
    }

    /**
     * @dev Convert an inputHandle with corresponding inputProof to an encrypted eaddress integer.
     */
    function asEaddress(einput inputHandle, bytes memory inputProof) internal returns (eaddress) {
        return eaddress.wrap(Impl.verify(einput.unwrap(inputHandle), inputProof, FheType.Uint160));
    }

    /**
     * @dev Convert a plaintext value to an encrypted eaddress integer.
     */
    function asEaddress(address value) internal returns (eaddress) {
        return eaddress.wrap(Impl.trivialEncrypt(uint256(uint160(value)), FheType.Uint160));
    }

    /**
     * @dev Generates a random encrypted value.
     */
    function randEbool() internal returns (ebool) {
        return ebool.wrap(Impl.rand(FheType.Bool));
    }

    /**
     * @dev Generates a random encrypted value.
     */
    function randEuint8() internal returns (euint8) {
        return euint8.wrap(Impl.rand(FheType.Uint8));
    }

    /**
     * @dev Generates a random encrypted 8-bit unsigned integer in the [0, upperBound) range.
     *      The upperBound must be a power of 2.
     */
    function randEuint8(uint8 upperBound) internal returns (euint8) {
        return euint8.wrap(Impl.randBounded(upperBound, FheType.Uint8));
    }

    /**
     * @dev Generates a random encrypted value.
     */
    function randEuint16() internal returns (euint16) {
        return euint16.wrap(Impl.rand(FheType.Uint16));
    }

    /**
     * @dev Generates a random encrypted 16-bit unsigned integer in the [0, upperBound) range.
     *      The upperBound must be a power of 2.
     */
    function randEuint16(uint16 upperBound) internal returns (euint16) {
        return euint16.wrap(Impl.randBounded(upperBound, FheType.Uint16));
    }

    /**
     * @dev Generates a random encrypted value.
     */
    function randEuint32() internal returns (euint32) {
        return euint32.wrap(Impl.rand(FheType.Uint32));
    }

    /**
     * @dev Generates a random encrypted 32-bit unsigned integer in the [0, upperBound) range.
     *      The upperBound must be a power of 2.
     */
    function randEuint32(uint32 upperBound) internal returns (euint32) {
        return euint32.wrap(Impl.randBounded(upperBound, FheType.Uint32));
    }

    /**
     * @dev Generates a random encrypted value.
     */
    function randEuint64() internal returns (euint64) {
        return euint64.wrap(Impl.rand(FheType.Uint64));
    }

    /**
     * @dev Generates a random encrypted 64-bit unsigned integer in the [0, upperBound) range.
     *      The upperBound must be a power of 2.
     */
    function randEuint64(uint64 upperBound) internal returns (euint64) {
        return euint64.wrap(Impl.randBounded(upperBound, FheType.Uint64));
    }

    /**
     * @dev Generates a random encrypted value.
     */
    function randEuint128() internal returns (euint128) {
        return euint128.wrap(Impl.rand(FheType.Uint128));
    }

    /**
     * @dev Generates a random encrypted 128-bit unsigned integer in the [0, upperBound) range.
     *      The upperBound must be a power of 2.
     */
    function randEuint128(uint128 upperBound) internal returns (euint128) {
        return euint128.wrap(Impl.randBounded(upperBound, FheType.Uint128));
    }

    /**
     * @dev Generates a random encrypted value.
     */
    function randEuint256() internal returns (euint256) {
        return euint256.wrap(Impl.rand(FheType.Uint256));
    }

    /**
     * @dev Generates a random encrypted 256-bit unsigned integer in the [0, upperBound) range.
     *      The upperBound must be a power of 2.
     */
    function randEuint256(uint256 upperBound) internal returns (euint256) {
        return euint256.wrap(Impl.randBounded(upperBound, FheType.Uint256));
    }

    /**
     * @dev Generates a random encrypted value.
     */
    function randEbytes64() internal returns (ebytes64) {
        return ebytes64.wrap(Impl.rand(FheType.Uint512));
    }

    /**
     * @dev Generates a random encrypted value.
     */
    function randEbytes128() internal returns (ebytes128) {
        return ebytes128.wrap(Impl.rand(FheType.Uint1024));
    }

    /**
     * @dev Generates a random encrypted value.
     */
    function randEbytes256() internal returns (ebytes256) {
        return ebytes256.wrap(Impl.rand(FheType.Uint2048));
    }

    /**
     * @dev Left-pad a bytes array with zeros such that it becomes of length 64.
     */
    function padToBytes64(bytes memory input) internal pure returns (bytes memory) {
        uint256 inputLength = input.length;

        if (inputLength > 64) {
            revert InputLengthAbove64Bytes(inputLength);
        }

        bytes memory result = new bytes(64);
        uint256 paddingLength = 64 - inputLength;

        for (uint256 i = 0; i < paddingLength; i++) {
            result[i] = 0;
        }

        for (uint256 i = 0; i < inputLength; i++) {
            result[paddingLength + i] = input[i];
        }
        return result;
    }

    /**
     * @dev Left-pad a bytes array with zeros such that it becomes of length 128.
     */
    function padToBytes128(bytes memory input) internal pure returns (bytes memory) {
        uint256 inputLength = input.length;

        if (inputLength > 128) {
            revert InputLengthAbove128Bytes(inputLength);
        }

        bytes memory result = new bytes(128);
        uint256 paddingLength = 128 - inputLength;

        for (uint256 i = 0; i < paddingLength; i++) {
            result[i] = 0;
        }

        for (uint256 i = 0; i < inputLength; i++) {
            result[paddingLength + i] = input[i];
        }
        return result;
    }

    /**
     * @dev Left-pad a bytes array with zeros such that it becomes of length 256.
     */
    function padToBytes256(bytes memory input) internal pure returns (bytes memory) {
        uint256 inputLength = input.length;

        if (inputLength > 256) {
            revert InputLengthAbove256Bytes(inputLength);
        }

        bytes memory result = new bytes(256);
        uint256 paddingLength = 256 - inputLength;

        for (uint256 i = 0; i < paddingLength; i++) {
            result[i] = 0;
        }

        for (uint256 i = 0; i < inputLength; i++) {
            result[paddingLength + i] = input[i];
        }
        return result;
    }

    /**
     * @dev This function cleans the transient storage for the ACL (accounts) and the InputVerifier
     *      (input proofs).
     *      This could be useful for integration with Account Abstraction when bundling several
     *      UserOps calling the TFHEExecutor.
     */
    function cleanTransientStorage() internal {
        Impl.cleanTransientStorageACL();
        Impl.cleanTransientStorageInputVerifier();
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(ebool value, address account) internal view returns (bool) {
        return Impl.isAllowed(ebool.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(ebool value) internal view returns (bool) {
        return Impl.isAllowed(ebool.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(ebool value, address account) internal returns (ebool) {
        Impl.allow(ebool.unwrap(value), account);
        return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(ebool value) internal returns (ebool) {
        Impl.allow(ebool.unwrap(value), address(this));
        return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(ebool value, address account) internal returns (ebool) {
        Impl.allowTransient(ebool.unwrap(value), account);
        return value;
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(euint8 value, address account) internal view returns (bool) {
        return Impl.isAllowed(euint8.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(euint8 value) internal view returns (bool) {
        return Impl.isAllowed(euint8.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(euint8 value, address account) internal returns (euint8) {
        Impl.allow(euint8.unwrap(value), account);
        return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(euint8 value) internal returns (euint8) {
        Impl.allow(euint8.unwrap(value), address(this));
        return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(euint8 value, address account) internal returns (euint8) {
        Impl.allowTransient(euint8.unwrap(value), account);
        return value;
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(euint16 value, address account) internal view returns (bool) {
        return Impl.isAllowed(euint16.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(euint16 value) internal view returns (bool) {
        return Impl.isAllowed(euint16.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(euint16 value, address account) internal returns (euint16) {
        Impl.allow(euint16.unwrap(value), account);
        return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(euint16 value) internal returns (euint16) {
        Impl.allow(euint16.unwrap(value), address(this));
        return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(euint16 value, address account) internal returns (euint16) {
        Impl.allowTransient(euint16.unwrap(value), account);
        return value;
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(euint32 value, address account) internal view returns (bool) {
        return Impl.isAllowed(euint32.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(euint32 value) internal view returns (bool) {
        return Impl.isAllowed(euint32.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(euint32 value, address account) internal returns (euint32) {
        Impl.allow(euint32.unwrap(value), account);
        return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(euint32 value) internal returns (euint32) {
        Impl.allow(euint32.unwrap(value), address(this));
        return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(euint32 value, address account) internal returns (euint32) {
        Impl.allowTransient(euint32.unwrap(value), account);
        return value;
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(euint64 value, address account) internal view returns (bool) {
        return Impl.isAllowed(euint64.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(euint64 value) internal view returns (bool) {
        return Impl.isAllowed(euint64.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(euint64 value, address account) internal returns (euint64) {
        Impl.allow(euint64.unwrap(value), account);
        return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(euint64 value) internal returns (euint64) {
        Impl.allow(euint64.unwrap(value), address(this));
        return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(euint64 value, address account) internal returns (euint64) {
        Impl.allowTransient(euint64.unwrap(value), account);
        return value;
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(euint128 value, address account) internal view returns (bool) {
        return Impl.isAllowed(euint128.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(euint128 value) internal view returns (bool) {
        return Impl.isAllowed(euint128.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(euint128 value, address account) internal returns (euint128) {
        Impl.allow(euint128.unwrap(value), account);
        return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(euint128 value) internal returns (euint128) {
        Impl.allow(euint128.unwrap(value), address(this));
        return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(euint128 value, address account) internal returns (euint128) {
        Impl.allowTransient(euint128.unwrap(value), account);
        return value;
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(euint256 value, address account) internal view returns (bool) {
        return Impl.isAllowed(euint256.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(euint256 value) internal view returns (bool) {
        return Impl.isAllowed(euint256.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(euint256 value, address account) internal returns (euint256) {
        Impl.allow(euint256.unwrap(value), account);
        return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(euint256 value) internal returns (euint256) {
        Impl.allow(euint256.unwrap(value), address(this));
        return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(euint256 value, address account) internal returns (euint256) {
        Impl.allowTransient(euint256.unwrap(value), account);
        return value;
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(ebytes64 value, address account) internal view returns (bool) {
        return Impl.isAllowed(ebytes64.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(ebytes64 value) internal view returns (bool) {
        return Impl.isAllowed(ebytes64.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(ebytes64 value, address account) internal returns (ebytes64) {
        Impl.allow(ebytes64.unwrap(value), account);
        return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(ebytes64 value) internal returns (ebytes64) {
        Impl.allow(ebytes64.unwrap(value), address(this));
        return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(ebytes64 value, address account) internal returns (ebytes64) {
        Impl.allowTransient(ebytes64.unwrap(value), account);
        return value;
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(ebytes128 value, address account) internal view returns (bool) {
        return Impl.isAllowed(ebytes128.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(ebytes128 value) internal view returns (bool) {
        return Impl.isAllowed(ebytes128.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(ebytes128 value, address account) internal returns (ebytes128) {
        Impl.allow(ebytes128.unwrap(value), account);
        return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(ebytes128 value) internal returns (ebytes128) {
        Impl.allow(ebytes128.unwrap(value), address(this));
        return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(ebytes128 value, address account) internal returns (ebytes128) {
        Impl.allowTransient(ebytes128.unwrap(value), account);
        return value;
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(ebytes256 value, address account) internal view returns (bool) {
        return Impl.isAllowed(ebytes256.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(ebytes256 value) internal view returns (bool) {
        return Impl.isAllowed(ebytes256.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(ebytes256 value, address account) internal returns (ebytes256) {
        Impl.allow(ebytes256.unwrap(value), account);
        return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(ebytes256 value) internal returns (ebytes256) {
        Impl.allow(ebytes256.unwrap(value), address(this));
        return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(ebytes256 value, address account) internal returns (ebytes256) {
        Impl.allowTransient(ebytes256.unwrap(value), account);
        return value;
    }

    /**
     * @dev Returns whether the account is allowed to use the value.
     */
    function isAllowed(eaddress value, address account) internal view returns (bool) {
        return Impl.isAllowed(eaddress.unwrap(value), account);
    }

    /**
     * @dev Returns whether the sender is allowed to use the value.
     */
    function isSenderAllowed(eaddress value) internal view returns (bool) {
        return Impl.isAllowed(eaddress.unwrap(value), msg.sender);
    }

    /**
     * @dev Allows the use of value for the address account.
     */
    function allow(eaddress value, address account) internal returns (eaddress) {
        Impl.allow(eaddress.unwrap(value), account);
        return value;
    }

    /**
     * @dev Allows the use of value for this address (address(this)).
     */
    function allowThis(eaddress value) internal returns (eaddress) {
        Impl.allow(eaddress.unwrap(value), address(this));
        return value;
    }

    /**
     * @dev Allows the use of value by address account for this transaction.
     */
    function allowTransient(eaddress value, address account) internal returns (eaddress) {
        Impl.allowTransient(eaddress.unwrap(value), account);
        return value;
    }
}
