// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {
    ACL_ADDRESS,
    FHEVM_EXECUTOR_ADDRESS,
    KMS_VERIFIER_ADDRESS,
    INPUT_VERIFIER_ADDRESS,
    HCU_LIMIT_ADDRESS,
    PROTOCOL_CONFIG_ADDRESS,
    KMS_GENERATION_ADDRESS,
    PAUSER_SET_ADDRESS,
    CLEARTEXT_ARITHMETIC_ADDRESS,
    CLEARTEXT_DB_ADDRESS,
    PADDED_INPUTTER_ADDRESS,
    PROTOCOL_CONFIG_REPLICA_ADDRESS
} from "../../config/addresses.sol";

address constant aclAdd = ACL_ADDRESS;
address constant fhevmExecutorAdd = FHEVM_EXECUTOR_ADDRESS;
address constant kmsVerifierAdd = KMS_VERIFIER_ADDRESS;
address constant inputVerifierAdd = INPUT_VERIFIER_ADDRESS;
address constant hcuLimitAdd = HCU_LIMIT_ADDRESS;
address constant protocolConfigAdd = PROTOCOL_CONFIG_ADDRESS;
address constant kmsGenerationAdd = KMS_GENERATION_ADDRESS;
address constant pauserSetAdd = PAUSER_SET_ADDRESS;
address constant cleartextArithmeticAdd = CLEARTEXT_ARITHMETIC_ADDRESS;
address constant cleartextDbAdd = CLEARTEXT_DB_ADDRESS;
address constant paddedInputterAdd = PADDED_INPUTTER_ADDRESS;
/// @dev No contract bakes it: the deploy scripts read it to create the replica a non-canonical host runs.
address constant protocolConfigReplicaAdd = PROTOCOL_CONFIG_REPLICA_ADDRESS;

/// @dev A literal zero, not a patched placeholder: upstream's convention is "null iff there is no bridge on
///      this chain", and a cleartext stack never bridges. No deploy nonce, no patch site, nothing to keep in step.
address constant confidentialBridgeAdd = address(0);

