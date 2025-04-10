// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {HTTPZ} from "../lib/HTTPZ.sol";
import {HTTPZConfigStruct} from "../lib/Impl.sol";

/**
 * @title   HTTPZConfig.
 * @notice  This library returns the HTTPZ config for different networks
 *          with the contract addresses for
 *          (1) ACL, (2) TFHEExecutor, (3) KMSVerifier, (4) InputVerifier
 *          which are deployed & maintained by Zama.
 */
library HTTPZConfig {
    function getSepoliaConfig() internal pure returns (HTTPZConfigStruct memory) {
        return
            HTTPZConfigStruct({
                ACLAddress: 0xFee8407e2f5e3Ee68ad77cAE98c434e637f516e5,
                TFHEExecutorAddress: 0x687408aB54661ba0b4aeF3a44156c616c6955E07,
                KMSVerifierAddress: 0x9D6891A6240D6130c54ae243d8005063D05fE14b,
                InputVerifierAddress: 0x3a2DA6f1daE9eF988B48d9CF27523FA31a8eBE50
            });
    }

    function getEthereumConfig() internal pure returns (HTTPZConfigStruct memory) {
        /// TODO
    }
}

/**
 * @title   SepoliaHTTPZConfig.
 * @dev     This contract can be inherited by a contract wishing to use the HTTPZ contracts provided by Zama
 *          on the Sepolia network (chainId = 11155111).
 *          Other providers may offer similar contracts deployed at different addresses.
 *          If you wish to use them, you should rely on the instructions from these providers.
 */
contract SepoliaHTTPZConfig {
    constructor() {
        HTTPZ.setCoprocessor(HTTPZConfig.getSepoliaConfig());
    }
}

/**
 * @title   EthereumHTTPZConfig.
 * @dev     This contract can be inherited by a contract wishing to use the HTTPZ contracts provided by Zama
 *          on the Ethereum (mainnet) network (chainId = 1).
 *          Other providers may offer similar contracts deployed at different addresses.
 *          If you wish to use them, you should rely on the instructions from these providers.
 */
contract EthereumHTTPZConfig {
    constructor() {
        HTTPZ.setCoprocessor(HTTPZConfig.getEthereumConfig());
    }
}
