// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

enum ContextStatus {
    NotInitialized,
    Generating,
    PreActivation,
    Active,
    Suspended,
    Deactivated,
    Compromised,
    Destroyed
}
