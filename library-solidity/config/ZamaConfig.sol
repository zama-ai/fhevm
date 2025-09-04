// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {SepoliaZamaOracleAddress} from "@zama-fhe/oracle-solidity/address/ZamaOracleAddress.sol";
import {FHE} from "../lib/FHE.sol";
import {CoprocessorConfig} from "../lib/Impl.sol";

/**
 * @title   ZamaConfig.
 * @notice  This library returns the FHEVM config for different networks
 *          with the contract addresses for (1) ACL, (2) CoprocessorAddress, (3) DecryptionOracleAddress, (4) KMSVerifier,
 *          which are deployed & maintained by Zama. It also returns the address of the decryption oracle.
 */
library ZamaConfig {
    function getSepoliaProtocolId() internal pure returns (uint256) {
        /// @note Zama Ethereum Sepolia protocol id is '10000 + Zama Ethereum protocol id'
        return 10001;
    }

    function getSepoliaConfig() internal pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0x687820221192C5B662b25367F70076A37bc79b6c,
                CoprocessorAddress: 0x848B0066793BcC60346Da1F49049357399B8D595,
                DecryptionOracleAddress: SepoliaZamaOracleAddress,
                KMSVerifierAddress: 0x1364cBBf2cDF5032C47d8226a6f6FBD2AFCDacAC
            });
    }

    function getEthereumProtocolId() internal pure returns (uint256) {
        /// @note Zama Ethereum protocol id is '1'
        return 1;
    }

    function getEthereumConfig() internal pure returns (CoprocessorConfig memory) {
        /// @note The addresses below are placeholders and should be replaced with actual addresses
        /// once deployed on the Ethereum mainnet.
        return
            CoprocessorConfig({
                ACLAddress: address(0),
                CoprocessorAddress: address(0),
                DecryptionOracleAddress: address(0),
                KMSVerifierAddress: address(0)
            });
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
    }

    function protocolId() public pure returns (uint256) {
        return ZamaConfig.getSepoliaProtocolId();
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
    }

    function protocolId() public pure returns (uint256) {
        return ZamaConfig.getEthereumProtocolId();
    }
}
