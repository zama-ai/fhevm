import '@nomicfoundation/hardhat-toolbox';
import dotenv from 'dotenv';
import type { HardhatUserConfig, extendProvider } from 'hardhat/config';
import { task } from 'hardhat/config';
import type { NetworkUserConfig } from 'hardhat/types';
import { resolve } from 'path';
import './tasks/erc20';
import './tasks/userDecryptBenchmark';
import './tasks/publicDecryptBenchmark';

// This value needs to be above the number of accounts used in the tasks
const NUM_ACCOUNTS = 1001;

task('compile:specific', 'Compiles only the specified contract')
  .addParam('contract', "The contract's path")
  .setAction(async ({ contract }, hre) => {
    // Adjust the configuration to include only the specified contract
    hre.config.paths.sources = contract;

    await hre.run('compile');
  });

const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || './.env';
dotenv.config({ path: resolve(__dirname, dotenvConfigPath) });

// Ensure that we have all the environment variables we need.
let mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  mnemonic = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer'; // default mnemonic in case it is undefined (needed to avoid panicking when deploying on real network)
}


const chainIds = {
  localNative: 8009,
  devnetNative: 9000,
  localCoprocessor: 12345,
  staging: 12345,
  zwsDev: 1337,
  sepolia: 11155111,
  mainnet: 1,
  localCoprocessorL1: 123456,
  localCoprocessorL2: 654321,
  composeCoprocessorL1: 123456,
  composeCoprocessorL2: 654321,
  localCoprocessorL1Input: 123456,
};

function getChainConfig(chain: keyof typeof chainIds): NetworkUserConfig {
  let jsonRpcUrl: string;
  let defaultRpcUrl = 'http://localhost:8545';

  switch (chain) {
    case 'staging':
      jsonRpcUrl = process.env.RPC_URL || defaultRpcUrl;
      if (jsonRpcUrl === defaultRpcUrl && !process.env.RPC_URL) {
        console.warn(
          `WARN: RPC_URL environment variable not set for network '${chain}'. Using default: ${defaultRpcUrl}`,
        );
      }
      break;
    case 'zwsDev':
      jsonRpcUrl = process.env.RPC_URL || defaultRpcUrl;
      if (jsonRpcUrl === defaultRpcUrl && !process.env.RPC_URL) {
        console.warn(
          `WARN: RPC_URL environment variable not set for network '${chain}'. Using default: ${defaultRpcUrl}`,
        );
      }
      break;
    case 'sepolia':
      defaultRpcUrl = "https://eth-sepolia.public.blastapi.io"
      jsonRpcUrl = process.env.RPC_URL || defaultRpcUrl;
      if (jsonRpcUrl === defaultRpcUrl && !process.env.RPC_URL) {
        console.warn(
          `WARN: RPC_URL environment variable not set for network '${chain}'. Using default: ${defaultRpcUrl}`,
        );
      }
      break;
    case 'localCoprocessor':
      jsonRpcUrl = 'http://localhost:8746';
      break;
    case 'localCoprocessorL1Input':
      jsonRpcUrl = 'http://localhost:8756';
      break;
    case 'localNative':
      jsonRpcUrl = 'http://localhost:8545';
      break;
    case 'localCoprocessorL1':
      jsonRpcUrl = 'http://localhost:8756';
      break;
    case 'localCoprocessorL2':
      jsonRpcUrl = 'http://localhost:8757';
      break;
    case 'composeCoprocessorL1':
      jsonRpcUrl = 'http://mock-httpz-1:8756';
      break;
    case 'composeCoprocessorL2':
      jsonRpcUrl = 'http://mock-gateway-1:8757';
      break;
    default:
      throw new Error(`unsupported chain: ${chain}`);
  }
  return {
    accounts: {
      count: NUM_ACCOUNTS,
      mnemonic,
      path: "m/44'/60'/0'/0",
    },
    chainId: chainIds[chain],
    url: jsonRpcUrl,
  };
}

const config: HardhatUserConfig = {
  // workaround a hardhat bug with --parallel --network
  // https://github.com/NomicFoundation/hardhat/issues/2756
  defaultNetwork: 'staging',
  mocha: {
    timeout: 300000,
  },
  gasReporter: {
    currency: 'USD',
    enabled: process.env.REPORT_GAS ? true : false,
    excludeContracts: [],
    src: './contracts',
  },
  networks: {
    hardhat: {
      accounts: {
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: process.env.CUSTOM_CHAIN_ID ? Number(process.env.CUSTOM_CHAIN_ID) : 31337,
      mining: {
        auto: true,
        interval: 1000,
      },
    },
    staging: getChainConfig('staging'),
    zwsDev: getChainConfig('zwsDev'),
    sepolia: getChainConfig('sepolia'),
    localNative: getChainConfig('localNative'),
    localCoprocessor: getChainConfig('localCoprocessor'),
    localCoprocessorL1: getChainConfig('localCoprocessorL1'),
    localCoprocessorL1Input: getChainConfig('localCoprocessorL1Input'),
    localCoprocessorL2: getChainConfig('localCoprocessorL2'),
    composeCoprocessorL1: getChainConfig('composeCoprocessorL1'),
    composeCoprocessorL2: getChainConfig('composeCoprocessorL2'),
  },
  paths: {
    artifacts: './artifacts',
    cache: './cache',
    sources: './contracts',
    tests: './test',
  },
  solidity: {
    version: '0.8.24',
    settings: {
      metadata: {
        // Not including the metadata hash
        // https://github.com/paulrberg/hardhat-template/issues/31
        bytecodeHash: 'none',
      },
      // Disable the optimizer when debugging
      // https://hardhat.org/hardhat-network/#solidity-optimizer-support
      optimizer: {
        enabled: true,
        runs: 800,
      },
      evmVersion: 'cancun',
      viaIR: false,
    },
  },
  sourcify: {
    enabled: false,
  },
  etherscan: {
    apiKey: {
      mainnet: process.env.ETHERSCAN_API_KEY!,
      sepolia: process.env.ETHERSCAN_API_KEY!,
      zwsDev: 'empty',
    },
    customChains: [
      {
        network: 'zwsDev',
        chainId: 1337,
        urls: {
          apiURL: 'http://l1-blockscout-zws-dev-blockscout-stack-blockscout-svc.ethereum-blockchain/api',
          browserURL: 'https://l1-explorer-zws-dev.diplodocus-boa.ts.net',
        },
      },
    ],
  },
  warnings: {
    '*': {
      'transient-storage': false,
    },
  },
  typechain: {
    outDir: 'types',
    target: 'ethers-v6',
  },
};

export default config;
