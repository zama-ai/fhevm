// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEVMConfig, TFHE} from "fhevm/lib/TFHE.sol";
import { Gateway } from "fhevm/gateway/GatewayCaller.sol";

address constant gatewayAddress = 0x7455c89669cdE1f7Cb6D026DFB87263422D821ca;

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
                ACLAddress: 0x9479B455904dCccCf8Bc4f7dF8e9A1105cBa2A8e,
                TFHEExecutorAddress: 0x199fB61DFdfE46f9F90C9773769c28D9623Bb90e,
                FHEPaymentAddress: 0x25FE5d92Ae6f89AF37D177cF818bF27EDFe37F7c,
                KMSVerifierAddress: 0x904Af2B61068f686838bD6257E385C2cE7a09195
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
