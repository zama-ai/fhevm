// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEVMConfig, TFHE} from "fhevm/lib/TFHE.sol";
import { Gateway } from "fhevm/gateway/GatewayCaller.sol";

address constant gatewayAddress = 0x096b4679d45fB675d4e2c1E4565009Cec99A12B1;

/**
 * @title   ZamaFHEVMConfig.
 * @notice  This library returns the TFHE config for different networks
 *          with the contract addresses for
 *          (1) ACL, (2) TFHEExecutor, (3) FHEPayment, (4) KMSVerifier,
 *          which are deployed & maintained by Zama.
 */
library DefaultFHEVMConfig {
    function getConfig() internal pure returns (FHEVMConfig.FHEVMConfigStruct memory) {
        return
            FHEVMConfig.FHEVMConfigStruct({
                ACLAddress: 0x339EcE85B9E11a3A3AA557582784a15d7F82AAf2,
                TFHEExecutorAddress: 0x596E6682c72946AF006B27C131793F2b62527A4b,
                FHEPaymentAddress: 0x6d5A11aC509C707c00bc3A0a113ACcC26c532547,
                KMSVerifierAddress: 0x208De73316E44722e16f6dDFF40881A3e4F86104 
            });
    }
}

/**
 * @title   MockZamaFHEVMConfig.
 * @dev     This contract can be inherited by a contract wishing to use these contracts on the mock
 *          environment provided by Zama.
 *          Other providers may offer similar contracts deployed at different addresses.
 *          If you wish to use them, you should rely on the instructions from these providers.
 */
contract E2EFHEVMConfig {
    constructor() {
        TFHE.setFHEVM(DefaultFHEVMConfig.getConfig());
        Gateway.setGateway(gatewayAddress);
    }
}
