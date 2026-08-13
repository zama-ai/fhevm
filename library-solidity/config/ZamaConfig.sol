// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE} from "../lib/FHE.sol";
import {CoprocessorConfig} from "../lib/Impl.sol";

/**
 * @title   ZamaConfig.
 * @notice  This library returns the FHEVM config for different networks
 *          with the contract addresses for (1) ACL, (2) CoprocessorAddress, (3) KMSVerifier,
 *          which are deployed & maintained by Zama.
 */
library ZamaConfig {
    /// @notice Returned if the Zama protocol is not supported on the current chain
    error ZamaProtocolUnsupported();

    /**
     * @notice Returns the Zama coprocessor config for the current chain, routed by `block.chainid`.
     * @dev    Supports all networks where the Zama protocol is deployed: Ethereum mainnet (chainId = 1),
     *         Sepolia (chainId = 11155111), Polygon Amoy (chainId = 80002), and the local
     *         Hardhat/Anvil network (chainId = 31337).
     *         Reverts with {ZamaProtocolUnsupported} on any other chain.
     */
    function getCoprocessorConfig() internal view returns (CoprocessorConfig memory config) {
        if (block.chainid == 1) {
            config = _getEthereumConfig();
        } else if (block.chainid == 11155111) {
            config = _getSepoliaConfig();
        } else if (block.chainid == 80002) {
            config = _getPolygonAmoyConfig();
        } else if (block.chainid == 31337) {
            config = _getLocalConfig();
        } else {
            revert ZamaProtocolUnsupported();
        }
    }

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

    function getPolygonCoprocessorConfig() internal view returns (CoprocessorConfig memory config) {
        if (block.chainid == 80002) {
            config = _getPolygonAmoyConfig();
        } else if (block.chainid == 31337) {
            config = _getLocalConfig();
        } else {
            revert ZamaProtocolUnsupported();
        }
    }

    function getConfidentialProtocolId() internal view returns (uint256) {
        if (block.chainid == 1) {
            return _getZamaMainnetProtocolId();
        } else if (block.chainid == 11155111 || block.chainid == 80002) {
            return _getZamaTestnetProtocolId();
        } else if (block.chainid == 31337) {
            return _getLocalProtocolId();
        }
        return 0;
    }

    /// @dev chainid == 1
    function _getZamaMainnetProtocolId() private pure returns (uint256) {
        // Zama Mainnet protocol id is '1'
        return 1;
    }

    /// @dev chainid == 1
    function _getEthereumConfig() private pure returns (CoprocessorConfig memory) {
        // The addresses below are placeholders and should be replaced with actual addresses
        // once deployed on the Ethereum mainnet.
        return
            CoprocessorConfig({
                ACLAddress: 0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6,
                CoprocessorAddress: 0xD82385dADa1ae3E969447f20A3164F6213100e75,
                KMSVerifierAddress: 0x77627828a55156b04Ac0DC0eb30467f1a552BB03
            });
    }

    /// @dev chainid == 11155111 or chainid == 80002
    function _getZamaTestnetProtocolId() private pure returns (uint256) {
        // Zama Testnet protocol id is '10000 + Zama Mainnet protocol id'
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

    /// @dev chainid == 80002
    function _getPolygonAmoyConfig() private pure returns (CoprocessorConfig memory) {
        return
            CoprocessorConfig({
                ACLAddress: 0xD99Cb9Fc3c42c87f2A4A12e8Fd60318d6bDdf985,
                CoprocessorAddress: 0x89420269f61e4db00545cd99da0aEcA7fF0912f9,
                KMSVerifierAddress: 0xCD1D89E311bce4C8DEa9a0857a0c9A4E153D4041
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
                KMSVerifierAddress: 0x901F8942346f7AB3a01F6D7613119Bca447Bb030
            });
    }
}

/**
 * @title   ZamaEthereumConfig.
 * @dev     This contract can be inherited by a contract wishing to use the FHEVM contracts provided by Zama
 *          on the Ethereum (mainnet) network (chainId = 1), the Sepolia (testnet) network (chainId = 11155111).
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

/**
 * @title   ZamaPolygonConfig.
 * @dev     This contract can be inherited by a contract wishing to use the FHEVM contracts provided by Zama
 *          on the Polygon amoy (testnet) network (chainId = 80002) and later on Polygon mainnet.
 *          Other providers may offer similar contracts deployed at different addresses.
 *          If you wish to use them, you should rely on the instructions from these providers.
 */
abstract contract ZamaPolygonConfig {
    constructor() {
        FHE.setCoprocessor(ZamaConfig.getPolygonCoprocessorConfig());
    }

    function confidentialProtocolId() public view returns (uint256) {
        return ZamaConfig.getConfidentialProtocolId();
    }
}

/**
 * @title   ZamaMultiChainConfig.
 * @dev     This contract can be inherited by a contract wishing to use the FHEVM contracts provided by Zama
 *          on any supported network. The coprocessor configuration is selected automatically from
 *          `block.chainid` at construction time, so a single implementation can be deployed on the
 *          Ethereum (mainnet) network (chainId = 1), the Sepolia (testnet) network (chainId = 11155111),
 *          the Polygon Amoy (testnet) network (chainId = 80002), or a local network (chainId = 31337).
 *          Other providers may offer similar contracts deployed at different addresses.
 *          If you wish to use them, you should rely on the instructions from these providers.
 */
abstract contract ZamaMultiChainConfig {
    constructor() {
        FHE.setCoprocessor(ZamaConfig.getCoprocessorConfig());
    }

    function confidentialProtocolId() public view returns (uint256) {
        return ZamaConfig.getConfidentialProtocolId();
    }
}
