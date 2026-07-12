// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// @notice An EOA identity: a private key and its derived address.
struct Signer {
    uint256 privateKey;
    address addr;
}
