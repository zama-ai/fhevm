// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.8.20;

library Precompiles {
    uint256 public constant Add = 65;
    uint256 public constant Verify = 66;
    uint256 public constant Reencrypt = 67;
    uint256 public constant FhePubKey = 68;
    uint256 public constant LessThanOrEqual = 70;
    uint256 public constant Subtract = 71;
    uint256 public constant Multiply = 72;
    uint256 public constant LessThan = 73;
    uint256 public constant Rand = 74;
    uint256 public constant OptimisticRequire = 75;
    uint256 public constant Cast = 76;
    uint256 public constant TrivialEncrypt = 77;
    uint256 public constant BitwiseAnd = 78;
    uint256 public constant BitwiseOr = 79;
    uint256 public constant BitwiseXor = 80;
    uint256 public constant Equal = 81;
    uint256 public constant GreaterThanOrEqual = 82;
    uint256 public constant GreaterThan = 83;
    uint256 public constant ShiftLeft = 84;
    uint256 public constant ShiftRight = 85;
    uint256 public constant NotEqual = 86;
    uint256 public constant Min = 87;
    uint256 public constant Max = 88;
    uint256 public constant Negate = 89;
    uint256 public constant Not = 90;
    uint256 public constant Decrypt = 91;
    uint256 public constant Divide = 92;
}
