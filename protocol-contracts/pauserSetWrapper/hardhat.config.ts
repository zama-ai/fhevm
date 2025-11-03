import 'dotenv/config'

import '@nomicfoundation/hardhat-toolbox'
import 'hardhat-deploy'
import '@nomicfoundation/hardhat-ethers'
import '@nomicfoundation/hardhat-chai-matchers' // Version 1.0.6 is the latest using Ethers v5
import { HardhatUserConfig } from 'hardhat/types'
import { task, types } from 'hardhat/config'

import 'hardhat/types/config'

import './tasks/blockExplorerVerify'
import './tasks/deployMocks'

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

// Deploy contracts by default before running tests
task('test', 'Runs the test suite, optionally skipping setup tasks')
    .addOptionalParam('skipSetup', 'Set to true to skip setup tasks', false, types.boolean)
    .setAction(async ({ skipSetup }, hre, runSuper) => {
        if (!skipSetup) {
            // Compile the contracts
            await hre.run('compile')

            // Deploy the mock contracts
            await hre.run('task:deployMocks')

            // Deploy the PauserSetWrapper contract
            await hre.run('deploy')
        } else {
            console.log('Skipping contracts setup.')
        }
        await runSuper()
    })

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
            saveDeployments: false,
        },
    },
    namedAccounts: {
        deployer: {
            default: 0, // wallet address of index[0], of the mnemonic in .env
        },
        alice: {
            default: 1, // wallet address of index[1], of the mnemonic in .env
        },
        bob: {
            default: 2, // wallet address of index[2], of the mnemonic in .env
        },
    },
    etherscan: {
        apiKey: process.env.ETHERSCAN_API_KEY || '',
    },
    // Add this section to disable gas reporter
    gasReporter: {
        enabled: false,
    },
}

export default config
