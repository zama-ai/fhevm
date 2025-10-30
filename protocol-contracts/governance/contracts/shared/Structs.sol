// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @notice A Safe transaction operation.
/// @custom:variant Call The Safe transaction is executed with the `CALL` opcode.
/// @custom:variant Delegatecall The Safe transaction is executed with the `DELEGATECALL` opcode.
enum Operation {
    Call,
    DelegateCall
}
