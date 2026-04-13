import '@nomicfoundation/hardhat-toolbox';
import '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import fs from 'fs';
import 'hardhat-deploy';
import 'hardhat-ignore-warnings';
import { TASK_COMPILE_SOLIDITY_GET_SOURCE_PATHS } from 'hardhat/builtin-tasks/task-names';
import { subtask, task } from 'hardhat/config';
import { type HardhatUserConfig, extendProvider } from 'hardhat/config';
import type { NetworkUserConfig } from 'hardhat/types';
import { resolve } from 'path';

import CustomProvider from './CustomProvider';
import './tasks/accounts';
import './tasks/addPausers';
import './tasks/blockExplorerVerify';
import './tasks/generateKmsMaterials';
import './tasks/ownership';
import './tasks/pauseContracts';
import './tasks/taskDeploy';
import './tasks/taskUtils';
import './tasks/upgradeContracts';

export const ADDRESSES_DIR = resolve(__dirname, 'addresses');
export const HOST_ADDRESSES_ENV_FILE_NAME = '.env.host';

const NUM_ACCOUNTS = 15;

extendProvider(async (provider, config, network) => {
  const newProvider = new CustomProvider(provider);
  return newProvider;
});

// Let compile:specific target either a directory or a single .sol file.
subtask(TASK_COMPILE_SOLIDITY_GET_SOURCE_PATHS).setAction(async ({ sourcePath }, hre, runSuper) => {
  const resolvedSourcePath = resolve(hre.config.paths.root, sourcePath);

  let sourcePathStats: fs.Stats;
  try {
    sourcePathStats = fs.statSync(resolvedSourcePath);
  } catch {
    throw new Error(`Invalid source path ${sourcePath}`);
  }

  if (sourcePathStats.isFile()) {
    return [resolvedSourcePath];
  }

  return runSuper({ sourcePath: resolvedSourcePath });
});

task('compile:specific', 'Compiles only the specified contract')
  .addParam('contract', "The contract's path")
  .setAction(async ({ contract }, hre) => {
    const previousSourcesPath = hre.config.paths.sources;
    const resolvedSourcePath = resolve(hre.config.paths.root, contract);

    hre.config.paths.sources = resolvedSourcePath;
    try {
      await hre.run('compile');
    } finally {
      hre.config.paths.sources = previousSourcesPath;
    }
  });

const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || './.env';
dotenv.config({ path: resolve(__dirname, dotenvConfigPath) });

// Ensure that we have all the environment variables we need.
let mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  mnemonic = 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer'; // default mnemonic in case it is undefined (needed to avoid panicking when deploying on real network)
}

task('coverage').setAction(async (taskArgs, hre, runSuper) => {
  hre.config.networks.hardhat.allowUnlimitedContractSize = true;
  hre.config.networks.hardhat.blockGasLimit = 1099511627775;

  await runSuper(taskArgs);
});

task('test', async (taskArgs, hre, runSuper) => {
  // Run modified test task
  if (hre.network.name === 'hardhat') {
    await hre.run('task:deployAllHostContracts');
    // Contrary to deployment, here we consider the PauserSet address from the `addresses/` directory
    // for local testing
    await hre.run('task:addHostPausers', { useInternalProxyAddress: true });
  }
  await hre.run('compile:specific', { contract: 'examples' });
  await runSuper();
});

const chainIds = {
  localHostChain: 123456,
  sepolia: 11155111,
  staging: 12345,
  zwsDev: 1337,
  mainnet: 1,
  custom: 9999,
};

function getChainConfig(chain: keyof typeof chainIds): NetworkUserConfig {
  let jsonRpcUrl: string | undefined = process.env.RPC_URL;
  if (!jsonRpcUrl) {
    jsonRpcUrl = 'http://127.0.0.1:8756';
  }
  return {
    accounts: {
      count: NUM_ACCOUNTS,
      mnemonic,
      path: "m/44'/60'/0'/0",
    },
    chainId: process.env.CHAIN_ID ? Number(process.env.CHAIN_ID) : chainIds[chain],
    url: jsonRpcUrl,
  };
}

const config: HardhatUserConfig = {
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
    src: './contracts',
  },
  networks: {
    hardhat: {
      accounts: {
        count: 20,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
    },
    staging: getChainConfig('staging'),
    zwsDev: getChainConfig('zwsDev'),
    sepolia: getChainConfig('sepolia'),
    localHostChain: getChainConfig('localHostChain'),
    mainnet: getChainConfig('mainnet'),
    custom: getChainConfig('custom'),
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
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY!,
  },
  warnings: {
    '*': {
      'transient-storage': false,
    },
    'examples/TracingSubCalls.sol': { default: 'off' },
  },
  typechain: {
    outDir: 'types',
    target: 'ethers-v6',
  },
};

export default config;
