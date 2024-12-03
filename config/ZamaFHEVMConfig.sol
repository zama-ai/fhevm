// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {TFHE} from "../lib/TFHE.sol";
import {FHEVMConfigStruct} from "../lib/Impl.sol";

/**
 * @title   ZamaFHEVMConfig.
 * @notice  This library returns the TFHE config for different networks
 *          with the contract addresses for
 *          (1) ACL, (2) TFHEExecutor, (3) FHEPayment, (4) KMSVerifier,
 *          which are deployed & maintained by Zama.
 */
library ZamaFHEVMConfig {
    function getSepoliaConfig() internal pure returns (FHEVMConfigStruct memory) {
        return
            FHEVMConfigStruct({
                ACLAddress: 0xFee8407e2f5e3Ee68ad77cAE98c434e637f516e5,
                TFHEExecutorAddress: 0x687408aB54661ba0b4aeF3a44156c616c6955E07,
                FHEPaymentAddress: 0xFb03BE574d14C256D56F09a198B586bdfc0A9de2,
                KMSVerifierAddress: 0x9D6891A6240D6130c54ae243d8005063D05fE14b
            });
    }

    function getEthereumConfig() internal pure returns (FHEVMConfigStruct memory) {
        /// TODO
    }
}

/**
 * @title   SepoliaZamaFHEVMConfig.
 * @dev     This contract can be inherited by a contract wishing to use the FHEVM contracts provided by Zama
 *          on the Sepolia network (chainId = 11155111).
 *          Other providers may offer similar contracts deployed at different addresses.
 *          If you wish to use them, you should rely on the instructions from these providers.
 */
contract SepoliaZamaFHEVMConfig {
    constructor() {
        TFHE.setFHEVM(ZamaFHEVMConfig.getSepoliaConfig());
    }
}

/**
 * @title   EthereumZamaFHEVMConfig.
 * @dev     This contract can be inherited by a contract wishing to use the FHEVM contracts provided by Zama
 *          on the Ethereum (mainnet) network (chainId = 1).
 *          Other providers may offer similar contracts deployed at different addresses.
 *          If you wish to use them, you should rely on the instructions from these providers.
 */
contract EthereumZamaFHEVMConfig {
    constructor() {
        TFHE.setFHEVM(ZamaFHEVMConfig.getEthereumConfig());
    }
}
