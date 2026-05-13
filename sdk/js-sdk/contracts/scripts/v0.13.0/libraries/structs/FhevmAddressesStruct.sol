// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {Signer} from "./SignerStruct.sol";

/// @notice Holds the six deterministic CREATE addresses produced by the FHEVM
///         host deployment nonce layout.
struct FhevmAddresses {
    address acl;
    address fhevmExecutor;
    address kmsVerifier;
    address inputVerifier;
    address hcuLimit;
    address pauserSet;
    address protocolConfig;
    address kmsGeneration;
    address verifyingContractAddressInputVerification;
    address verifyingContractAddressDecryption;
    Signer[] pausers;
}
