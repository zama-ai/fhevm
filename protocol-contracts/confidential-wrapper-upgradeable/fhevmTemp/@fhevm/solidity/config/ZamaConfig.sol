// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE} from "@fhevm/solidity/lib/FHE.sol";
import {CoprocessorConfig} from "@fhevm/solidity/lib/Impl.sol";

/**
 * @title   ZamaConfig.
 * @notice  This library returns the FHEVM config for different networks
 *          with the contract addresses for (1) ACL, (2) CoprocessorAddress, (3) KMSVerifier,
 *          which are deployed & maintained by Zama.
 */
library ZamaConfig {
    /// @notice Returned if the Zama protocol is not supported on the current chain
    error ZamaProtocolUnsupported();

    function getEthereumCoprocessorConfig() internal view returns (CoprocessorConfig memory config) {
        if (block.chainid == 1) {
            config = _getEthereumConfig();
        } else if (block.chainid == 11155111) {
            config = _getSepoliaConfig();
        } else if (block.chainid == 31337) {
            config = _getLocalConfig();
        } else {
            revert ZamaProtocolUnsupported();
        }
    }

    function getConfidentialProtocolId() internal view returns (uint256) {
        if (block.chainid == 1) {
            return _getEthereumProtocolId();
        } else if (block.chainid == 11155111) {
            return _getSepoliaProtocolId();
        } else if (block.chainid == 31337) {
            return _getLocalProtocolId();
        }
        return 0;
    }

    /// @dev chainid == 1
    function _getEthereumProtocolId() private pure returns (uint256) {
        // Zama Ethereum protocol id is '1'
        return 1;
    }

    /// @dev chainid == 1
    function _getEthereumConfig() private pure returns (CoprocessorConfig memory) {
        // The addresses below are placeholders and should be replaced with actual addresses
        // once deployed on the Ethereum mainnet.
        return
            CoprocessorConfig({ACLAddress: address(0), CoprocessorAddress: address(0), KMSVerifierAddress: address(0)});
    }

    /// @dev chainid == 11155111
    function _getSepoliaProtocolId() private pure returns (uint256) {
        // Zama Ethereum Sepolia protocol id is '10000 + Zama Ethereum protocol id'
        return 10001;
    }

    /// @dev chainid == 11155111
    function _getSepoliaConfig() private pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D,
                CoprocessorAddress: 0x92C920834Ec8941d2C77D188936E1f7A6f49c127,
                KMSVerifierAddress: 0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A
            });
    }

    /// @dev chainid == 31337
    function _getLocalProtocolId() private pure returns (uint256) {
        return type(uint256).max;
    }

    function _getLocalConfig() private pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D,
                CoprocessorAddress: 0xe3a9105a3a932253A70F126eb1E3b589C643dD24,
                KMSVerifierAddress: 0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A
            });
    }
}

/**
 * @title   ZamaEthereumConfig.
 * @dev     This contract can be inherited by a contract wishing to use the FHEVM contracts provided by Zama
 *          on the Ethereum (mainnet) network (chainId = 1) or Sepolia (testnet) network (chainId = 11155111).
 *          Other providers may offer similar contracts deployed at different addresses.
 *          If you wish to use them, you should rely on the instructions from these providers.
 */
abstract contract ZamaEthereumConfig {
    constructor() {
        FHE.setCoprocessor(ZamaConfig.getEthereumCoprocessorConfig());
    }

    function confidentialProtocolId() public view returns (uint256) {
        return ZamaConfig.getConfidentialProtocolId();
    }
}
