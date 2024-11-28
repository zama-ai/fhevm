// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHEVMConfig, TFHE} from "../lib/TFHE.sol";

/**
 * @title   ZamaFHEVMConfig.
 * @notice  This library returns the TFHE config for different networks
 *          with the contract addresses for
 *          (1) ACL, (2) TFHEExecutor, (3) FHEPayment, (4) KMSVerifier,
 *          which are deployed & maintained by Zama.
 */
library ZamaFHEVMConfig {
    function getMockConfig() internal pure returns (FHEVMConfig.FHEVMConfigStruct memory) {
        return
            FHEVMConfig.FHEVMConfigStruct({
                ACLAddress: 0xB4d8d77f7F9B465B60c190480c6160b69d695c9D,
                TFHEExecutorAddress: 0xFdee168C46e1dFD082E78192b3C622cA78B58669,
                FHEPaymentAddress: 0x2527DD76195fD3BFdd2c76D821e1f5d433d82C25,
                KMSVerifierAddress: 0x89842EA0b44EF85391Bd1A9f3AC8B382CCF0d3F1
            });
    }

    function getSepoliaConfig() internal pure returns (FHEVMConfig.FHEVMConfigStruct memory) {
        return
            FHEVMConfig.FHEVMConfigStruct({
                ACLAddress: 0x9479B455904dCccCf8Bc4f7dF8e9A1105cBa2A8e,
                TFHEExecutorAddress: 0x199fB61DFdfE46f9F90C9773769c28D9623Bb90e,
                FHEPaymentAddress: 0x25FE5d92Ae6f89AF37D177cF818bF27EDFe37F7c,
                KMSVerifierAddress: 0x904Af2B61068f686838bD6257E385C2cE7a09195
            });
    }

    function getEthereumConfig() internal pure returns (FHEVMConfig.FHEVMConfigStruct memory) {
        /// TODO
    }
}

/**
 * @title   MockZamaFHEVMConfig.
 * @dev     This contract can be inherited by a contract wishing to use these contracts on the mock
 *          environment provided by Zama.
 *          Other providers may offer similar contracts deployed at different addresses.
 *          If you wish to use them, you should rely on the instructions from these providers.
 */
contract MockZamaFHEVMConfig {
    constructor() {
        TFHE.setFHEVM(ZamaFHEVMConfig.getMockConfig());
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
