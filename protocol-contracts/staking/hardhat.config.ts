import './tasks/accounts';
import './tasks/addEligibleAccount';
import './tasks/deployment';
import './tasks/ownership';
import './tasks/setRewardRate';
import './tasks/verify';
import '@nomicfoundation/hardhat-chai-matchers';
import '@nomicfoundation/hardhat-ethers';
import '@nomicfoundation/hardhat-verify';
import '@openzeppelin/hardhat-upgrades';
import '@typechain/hardhat';
import 'dotenv/config';
import 'hardhat-deploy';
import 'hardhat-exposed';
import 'hardhat-gas-reporter';
import 'hardhat-ignore-warnings';
import { HardhatUserConfig, HttpNetworkAccountsUserConfig } from 'hardhat/types';
import 'solidity-coverage';

// Get the environment configuration from .env file
//
// To make use of automatic environment setup:
// - Duplicate .env.example file and name it .env
// - Fill in the environment variables

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

const config: HardhatUserConfig = {
  solidity: {
    version: '0.8.29',
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
      evmVersion: 'cancun',
    },
  },
  networks: {
    'mainnet': {
      url: process.env.MAINNET_RPC_URL || '',
      accounts,
    },
    'testnet': {
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
    bob: {
      default: 2, // wallet address of index[2], of the mnemonic in .env
    },
    charlie: {
      default: 3, // wallet address of index[3], of the mnemonic in .env
    },
    dave: {
      default: 4, // wallet address of index[4], of the mnemonic in .env
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
