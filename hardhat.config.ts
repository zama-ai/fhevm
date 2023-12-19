import '@nomicfoundation/hardhat-toolbox';
import { config as dotenvConfig } from 'dotenv';
import 'hardhat-deploy';
import type { HardhatUserConfig } from 'hardhat/config';
import type { NetworkUserConfig } from 'hardhat/types';
import { resolve } from 'path';

import './tasks/accounts';
import './tasks/getEthereumAddress';
import './tasks/mint';
import './tasks/taskDeploy';
import './tasks/taskIdentity';

const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || './.env';
dotenvConfig({ path: resolve(__dirname, dotenvConfigPath) });

// Ensure that we have all the environment variables we need.
const mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  throw new Error('Please set your MNEMONIC in a .env file');
}

const chainIds = {
  zama: 8009,
  local: 9000,
  localNetwork1: 9000,
};

function getChainConfig(chain: keyof typeof chainIds): NetworkUserConfig {
  let jsonRpcUrl: string;
  switch (chain) {
    case 'local':
      jsonRpcUrl = 'http://localhost:8545';
      break;
    case 'localNetwork1':
      jsonRpcUrl = 'http://127.0.0.1:9650/ext/bc/fhevm/rpc';
      break;
    case 'zama':
      jsonRpcUrl = 'https://devnet.zama.ai';
      break;
  }
  return {
    accounts: {
      count: 10,
      mnemonic,
      path: "m/44'/60'/0'/0",
    },
    chainId: chainIds[chain],
    url: jsonRpcUrl,
  };
}

const config: HardhatUserConfig = {
  defaultNetwork: 'local',
  namedAccounts: {
    deployer: 0,
  },
  mocha: {
    timeout: 500000,
  },
  gasReporter: {
    currency: 'USD',
    enabled: process.env.REPORT_GAS ? true : false,
    excludeContracts: [],
    src: './examples',
  },
  networks: {
    zama: getChainConfig('zama'),
    localDev: getChainConfig('local'),
    local: getChainConfig('local'),
    localNetwork1: getChainConfig('localNetwork1'),
  },
  paths: {
    artifacts: './artifacts',
    cache: './cache',
    sources: './examples',
    tests: './test',
  },
  solidity: {
    version: '0.8.19',
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
    },
  },
  typechain: {
    outDir: 'types',
    target: 'ethers-v6',
  },
};

export default config;
