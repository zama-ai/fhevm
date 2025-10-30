import 'dotenv/config'

import '@nomicfoundation/hardhat-toolbox'
import 'hardhat-deploy'
import '@nomiclabs/hardhat-ethers'
import '@nomicfoundation/hardhat-chai-matchers' // Version 1.0.6 is the latest using Ethers v5
import { HardhatUserConfig } from 'hardhat/types'

import 'hardhat/types/config'

import './tasks/blockExplorerVerify'

// Set your preferred authentication method
//
// If you prefer using a mnemonic, set a MNEMONIC environment variable
// to a valid mnemonic
const MNEMONIC = process.env.MNEMONIC

// If you prefer to be authenticated using a private key, set a PRIVATE_KEY environment variable
const PRIVATE_KEY = process.env.PRIVATE_KEY

const accounts = MNEMONIC ? { mnemonic: MNEMONIC } : PRIVATE_KEY ? [PRIVATE_KEY] : undefined

if (accounts == null) {
    console.warn(
        'Could not find MNEMONIC or PRIVATE_KEY environment variables. It will not be possible to execute transactions in your example.'
    )
}

const config: HardhatUserConfig = {
    solidity: {
        version: '0.8.24',
        settings: {
            metadata: {
                bytecodeHash: 'none',
            },
            optimizer: {
                enabled: true,
                runs: 800,
            },
            evmVersion: 'cancun',
        },
    },
    networks: {
        'ethereum-mainnet': {
            url: process.env.MAINNET_RPC_URL || '',
            accounts,
        },
        'ethereum-testnet': {
            url: process.env.SEPOLIA_RPC_URL || '',
            accounts,
        },
        hardhat: {
            chainId: 11155111,
        },
    },
    namedAccounts: {
        deployer: {
            default: 0,
        },
    },
    etherscan: {
        apiKey: process.env.ETHERSCAN_API_KEY || '',
    },
}

export default config
