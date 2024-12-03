pragma solidity ^0.8.24;

import "fhevm-core-contracts/addresses/ACLAddress.sol";
import "fhevm-core-contracts/addresses/FHEPaymentAddress.sol";
import "fhevm-core-contracts/addresses/KMSVerifierAddress.sol";
import "fhevm-core-contracts/addresses/TFHEExecutorAddress.sol";
import "./Impl.sol";

/**
 * @title   FHEVMConfig
 * @notice  This library returns all addresses for the ACL, TFHEExecutor, FHEPayment,
 *          and KMSVerifier contracts.
 */
library FHEVMConfig {
    /**
     * @notice This function returns a struct containing all contract addresses.
     * @dev    It returns an immutable struct.
     */
    function defaultConfig() internal pure returns (FHEVMConfigStruct memory) {
        return
            FHEVMConfigStruct({
                ACLAddress: aclAdd,
                TFHEExecutorAddress: tfheExecutorAdd,
                FHEPaymentAddress: fhePaymentAdd,
                KMSVerifierAddress: kmsVerifierAdd
            });
    }
}
