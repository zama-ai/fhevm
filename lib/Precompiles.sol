// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

library Precompiles {
    uint256 public constant Add = 65;
    uint256 public constant Verify = 66;
    uint256 public constant Reencrypt = 67;
    uint256 public constant FhePubKey = 68;
    uint256 public constant Require = 69;
    uint256 public constant LessThanOrEqual = 70;
    uint256 public constant Subtract = 71;
    uint256 public constant Multiply = 72;
    uint256 public constant LessThan = 73;
    uint256 public constant OptimisticRequire = 75;
    uint256 public constant Cast = 76;
    uint256 public constant TrivialEncrypt = 77;
    uint256 public constant BitwiseAnd = 78;
    uint256 public constant BitwiseOr = 79;
    uint256 public constant BitwiseXor = 80;
    uint256 public constant Equal = 81;
    uint256 public constant GreaterThanOrEqual = 82;
    uint256 public constant GreaterThan = 83;
}
