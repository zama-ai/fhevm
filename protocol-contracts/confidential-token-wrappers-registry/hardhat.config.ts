import '@nomicfoundation/hardhat-chai-matchers';
import '@nomicfoundation/hardhat-ethers';
import '@nomicfoundation/hardhat-verify';
import '@openzeppelin/hardhat-upgrades';
import '@typechain/hardhat';
import dotenv from 'dotenv';
import { existsSync } from 'fs';
import { task, types } from 'hardhat/config';
import { resolve } from 'path';
import 'hardhat-deploy';
import 'hardhat-gas-reporter';
import { HardhatUserConfig, HttpNetworkAccountsUserConfig } from 'hardhat/types';
import 'solidity-coverage';

import './tasks/accounts';
import './tasks/deploy';
import './tasks/verify';
import './tasks/mocks/deployMocks';
import './tasks/mocks/verify';

// Get the environment configuration from .env file
//
// To make use of automatic environment setup:
// - Duplicate .env.example file and name it .env
// - Fill in the environment variables
dotenv.config();

const MNEMONIC = process.env.MNEMONIC;
const PRIVATE_KEY = process.env.PRIVATE_KEY;

const accounts: HttpNetworkAccountsUserConfig | undefined = MNEMONIC
  ? { mnemonic: MNEMONIC }
  : PRIVATE_KEY
    ? [PRIVATE_KEY]
    : undefined;

if (accounts == null) {
  console.warn(
    'Could not find MNEMONIC or PRIVATE_KEY environment variables. It will not be possible to execute transactions in your example.',
  );
}

// Run the test suite for tasks with environment variables from `.env.example`
task('test', 'Runs the test suite for tasks with environment variables from .env.example')
  .addOptionalParam('skipSetup', 'Set to true to skip setup tasks', false, types.boolean)
  .setAction(async (taskArgs, hre, runSuper) => {
    // Load `.env.example`
    const envExamplePath = resolve(__dirname, '.env.example');
    if (existsSync(envExamplePath)) {
      dotenv.config({ path: envExamplePath, override: true });
    }

    if (!taskArgs.skipSetup) {
      // Compile the contracts
      await hre.run('compile');

      // Deploy ConfidentialTokenWrappersRegistry contract
      await hre.run('task:deployConfidentialTokenWrappersRegistry');
    } else {
      console.log('Skipping contracts setup.');
    }

    // Call the original test task
    await runSuper(taskArgs);
  });

const config: HardhatUserConfig = {
  solidity: {
    version: '0.8.27',
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
      evmVersion: 'cancun',
    },
  },
  networks: {
    mainnet: {
      url: process.env.MAINNET_RPC_URL || '',
      accounts,
    },
    testnet: {
      url: process.env.SEPOLIA_RPC_URL || '',
      accounts,
    },
    hardhat: {
      saveDeployments: false,
    },
  },
  namedAccounts: {
    deployer: {
      default: 0,
    },
  },
  gasReporter: {
    currency: 'USD',
    enabled: !!process.env.REPORT_GAS,
    showMethodSig: true,
    includeBytecodeInJSON: true,
  },
  typechain: {
    outDir: 'types',
    target: 'ethers-v6',
  },
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY!,
  },
};

export default config;
