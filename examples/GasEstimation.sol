// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "../lib/TFHE.sol";

contract GasEstimation {
    function empty() public {}

    function randEuint8() public {
        euint8 random = TFHE.randEuint8();
    }

    function randEuint16() public {
        euint16 random = TFHE.randEuint16();
    }

    function randEuint32() public {
        euint32 random = TFHE.randEuint32();
    }

    function add8() public {
        TFHE.asEuint8(1) + TFHE.asEuint8(2);
    }

    function sub8() public {
        TFHE.asEuint8(2) - TFHE.asEuint8(1);
    }

    function mul8() public {
        TFHE.asEuint8(2) * TFHE.asEuint8(10);
    }

    function div8() public {
        TFHE.div(TFHE.asEuint8(2), 2);
    }

    function rem8() public {
        TFHE.rem(TFHE.asEuint8(2), 2);
    }

    function shr8() public {
        TFHE.shr(TFHE.asEuint8(2), 1);
    }

    function shl8() public {
        TFHE.shr(TFHE.asEuint8(2), 1);
    }

    function add16() public {
        TFHE.asEuint16(1) + TFHE.asEuint16(2);
    }

    function sub16() public {
        TFHE.asEuint16(2) - TFHE.asEuint16(1);
    }

    function mul16() public {
        TFHE.asEuint16(2) * TFHE.asEuint16(10);
    }

    function div16() public {
        TFHE.div(TFHE.asEuint16(2), 2);
    }

    function rem16() public {
        TFHE.rem(TFHE.asEuint16(2), 2);
    }

    function shr16() public {
        TFHE.shr(TFHE.asEuint16(2), 1);
    }

    function shl16() public {
        TFHE.shr(TFHE.asEuint16(2), 1);
    }

    function add32() public {
        TFHE.asEuint32(1) + TFHE.asEuint32(2);
    }

    function sub32() public {
        TFHE.asEuint32(2) - TFHE.asEuint32(1);
    }

    function mul32() public {
        TFHE.asEuint32(2) * TFHE.asEuint32(10);
    }

    function div32() public {
        TFHE.div(TFHE.asEuint32(2), 2);
    }

    function rem32() public {
        TFHE.rem(TFHE.asEuint32(2), 2);
    }

    function shr32() public {
        TFHE.shr(TFHE.asEuint32(2), 1);
    }

    function shl32() public {
        TFHE.shr(TFHE.asEuint32(2), 1);
    }

    function or8() public {
        TFHE.or(TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function or16() public {
        TFHE.or(TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function or32() public {
        TFHE.or(TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function and8() public {
        TFHE.and(TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function and16() public {
        TFHE.and(TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function and32() public {
        TFHE.and(TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function xor8() public {
        TFHE.xor(TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function xor16() public {
        TFHE.xor(TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function xor32() public {
        TFHE.xor(TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function eq8() public {
        TFHE.eq(TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function eq16() public {
        TFHE.eq(TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function eq32() public {
        TFHE.eq(TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function ne8() public {
        TFHE.ne(TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function ne16() public {
        TFHE.ne(TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function ne32() public {
        TFHE.ne(TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function le8() public {
        TFHE.le(TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function le16() public {
        TFHE.le(TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function le32() public {
        TFHE.le(TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function lt8() public {
        TFHE.lt(TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function lt16() public {
        TFHE.lt(TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function lt32() public {
        TFHE.lt(TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function gt8() public {
        TFHE.gt(TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function gt16() public {
        TFHE.gt(TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function gt32() public {
        TFHE.gt(TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function ge8() public {
        TFHE.ge(TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function ge16() public {
        TFHE.ge(TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function ge32() public {
        TFHE.ge(TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function min8() public {
        TFHE.min(TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function min16() public {
        TFHE.min(TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function min32() public {
        TFHE.min(TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function max8() public {
        TFHE.max(TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function max16() public {
        TFHE.max(TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function max32() public {
        TFHE.max(TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function not8() public {
        TFHE.not(TFHE.asEuint8(2));
    }

    function not16() public {
        TFHE.not(TFHE.asEuint16(2));
    }

    function not32() public {
        TFHE.not(TFHE.asEuint32(2));
    }

    function neg8() public {
        TFHE.neg(TFHE.asEuint8(2));
    }

    function neg16() public {
        TFHE.neg(TFHE.asEuint16(2));
    }

    function neg32() public {
        TFHE.neg(TFHE.asEuint32(2));
    }

    function cmux8() public {
        TFHE.cmux(TFHE.asEbool(true), TFHE.asEuint8(2), TFHE.asEuint8(1));
    }

    function cmux16() public {
        TFHE.cmux(TFHE.asEbool(true), TFHE.asEuint16(2), TFHE.asEuint16(1));
    }

    function cmux32() public {
        TFHE.cmux(TFHE.asEbool(true), TFHE.asEuint32(2), TFHE.asEuint32(1));
    }

    function decrypt8() public {
        TFHE.decrypt(TFHE.asEuint8(2));
    }

    function decrypt16() public {
        TFHE.decrypt(TFHE.asEuint16(2));
    }

    function decrypt32() public {
        TFHE.decrypt(TFHE.asEuint32(2));
    }
}
