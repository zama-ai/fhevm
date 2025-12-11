import './tasks/accounts';
import './tasks/addEligibleAccount';
import './tasks/deployment';
import './tasks/deposit';
import './tasks/ownership';
import './tasks/setFee';
import './tasks/setRewardRate';
import './tasks/verify';
import '@nomicfoundation/hardhat-chai-matchers';
import '@nomicfoundation/hardhat-ethers';
import '@nomicfoundation/hardhat-verify';
import '@openzeppelin/hardhat-upgrades';
import '@typechain/hardhat';
import dotenv from 'dotenv';
import { existsSync } from 'fs';
import 'hardhat-deploy';
import 'hardhat-exposed';
import 'hardhat-gas-reporter';
import 'hardhat-ignore-warnings';
import { task, types } from 'hardhat/config';
import { HardhatUserConfig, HttpNetworkAccountsUserConfig } from 'hardhat/types';
import { resolve } from 'path';
import 'solidity-coverage';

// Get the environment configuration from .env file
//
// To make use of automatic environment setup:
// - Duplicate .env.example file and name it .env
// - Fill in the environment variables
dotenv.config();

// Set your preferred authentication method
//
// If you prefer using a mnemonic, set a MNEMONIC environment variable
// to a valid mnemonic
const MNEMONIC = process.env.MNEMONIC;

// If you prefer to be authenticated using a private key, set a PRIVATE_KEY environment variable
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
task('test:tasks', 'Runs the test suite for tasks with environment variables from .env.example')
  .addOptionalParam('skipSetup', 'Set to true to skip setup tasks', false, types.boolean)
  .setAction(async (taskArgs, hre) => {
    // Load `.env.example`
    const envExamplePath = resolve(__dirname, '.env.example');
    if (existsSync(envExamplePath)) {
      dotenv.config({ path: envExamplePath, override: true });
    }

    if (!taskArgs.skipSetup) {
      // Compile the contracts
      await hre.run('compile');

      // Deploy mocked ERC20 Zama token
      await hre.run('task:deployERC20MockAndMintDeployer');

      // Deploy all protocol staking contracts
      await hre.run('task:deployAllProtocolStakingContracts');

      // Deploy all operator staking contracts
      await hre.run('task:deployAllOperatorStakingContracts');
    } else {
      console.log('Skipping contracts setup.');
    }
    await hre.run('test', ['test-tasks/**/*.test.ts']);
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
      // Need this to avoid deployment issues in test
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
  exposed: {
    imports: true,
    initializers: true,
  },
};

export default config;
