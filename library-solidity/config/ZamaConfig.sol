// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE} from "../lib/FHE.sol";
import {FHEVMConfigStruct} from "../lib/Impl.sol";

/**
 * @title   ZamaConfig.
 * @notice  This library returns the FHEVM config for different networks
 *          with the contract addresses for (1) ACL, (2) FHEVMExecutor, (3) KMSVerifier, (4) InputVerifier
 *          which are deployed & maintained by Zama.
 */
library ZamaConfig {
    function getSepoliaConfig() internal pure returns (FHEVMConfigStruct memory) {
        return
            FHEVMConfigStruct({
                ACLAddress: 0x687820221192C5B662b25367F70076A37bc79b6c,
                FHEVMExecutorAddress: 0x848B0066793BcC60346Da1F49049357399B8D595,
                KMSVerifierAddress: 0x1364cBBf2cDF5032C47d8226a6f6FBD2AFCDacAC,
                InputVerifierAddress: 0xbc91f3daD1A5F19F8390c400196e58073B6a0BC4
            });
    }

    function getSepoliaDecryptionOracleAddress() internal pure returns (address) {
        return 0xa02Cda4Ca3a71D7C46997716F4283aa851C28812;
    }

    function getEthereumConfig() internal pure returns (FHEVMConfigStruct memory) {
        /// @note The addresses below are placeholders and should be replaced with actual addresses
        /// once deployed on the Ethereum mainnet.
        return
            FHEVMConfigStruct({
                ACLAddress: address(0),
                FHEVMExecutorAddress: address(0),
                KMSVerifierAddress: address(0),
                InputVerifierAddress: address(0)
            });
    }

    function getEthereumDecryptionOracleAddress() internal pure returns (address) {
        /// @note Placeholder, should be replaced with actual address once deployed.
        return address(0);
    }
}

/**
 * @title   SepoliaConfig.
 * @dev     This contract can be inherited by a contract wishing to use the FHEVM contracts provided by Zama
 *          on the Sepolia network (chainId = 11155111).
 *          Other providers may offer similar contracts deployed at different addresses.
 *          If you wish to use them, you should rely on the instructions from these providers.
 */
contract SepoliaConfig {
    constructor() {
        FHE.setCoprocessor(ZamaConfig.getSepoliaConfig());
        FHE.setDecryptionOracle(ZamaConfig.getSepoliaDecryptionOracleAddress());
    }
}

/**
 * @title   EthereumConfig.
 * @dev     This contract can be inherited by a contract wishing to use the FHEVM contracts provided by Zama
 *          on the Ethereum (mainnet) network (chainId = 1).
 *          Other providers may offer similar contracts deployed at different addresses.
 *          If you wish to use them, you should rely on the instructions from these providers.
 */
contract EthereumConfig {
    constructor() {
        FHE.setCoprocessor(ZamaConfig.getEthereumConfig());
        FHE.setDecryptionOracle(ZamaConfig.getEthereumDecryptionOracleAddress());
    }
}
