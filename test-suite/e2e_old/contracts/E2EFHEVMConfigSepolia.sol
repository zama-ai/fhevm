// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEVMConfig, TFHE} from "fhevm/lib/TFHE.sol";
import { Gateway } from "fhevm/gateway/GatewayCaller.sol";

address constant gatewayAddress = 0x33347831500F1e73f0ccCBb95c9f86B94d7b1123;

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
                ACLAddress: 0xFee8407e2f5e3Ee68ad77cAE98c434e637f516e5,
                TFHEExecutorAddress: 0x687408aB54661ba0b4aeF3a44156c616c6955E07,
                FHEPaymentAddress: 0xFb03BE574d14C256D56F09a198B586bdfc0A9de2,
                KMSVerifierAddress: 0x9D6891A6240D6130c54ae243d8005063D05fE14b
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
