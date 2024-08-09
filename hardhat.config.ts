import '@nomicfoundation/hardhat-toolbox';
import dotenv from 'dotenv';
import 'hardhat-deploy';
import 'hardhat-ignore-warnings';
import type { HardhatUserConfig, extendProvider } from 'hardhat/config';
import { task } from 'hardhat/config';
import type { NetworkUserConfig } from 'hardhat/types';
import { resolve } from 'path';

import CustomProvider from './CustomProvider';
// Adjust the import path as needed
import './tasks/accounts';
import './tasks/getEthereumAddress';
import './tasks/mint';
import './tasks/taskDeploy';
import './tasks/taskGatewayRelayer';
import './tasks/taskIdentity';
import './tasks/taskTFHE';

extendProvider(async (provider, config, network) => {
  const newProvider = new CustomProvider(provider);
  return newProvider;
});

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
const mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  throw new Error('Please set your MNEMONIC in a .env file');
}

const chainIds = {
  zama: 8009,
  local: 9000,
  localNetwork1: 9000,
  multipleValidatorTestnet: 8009,
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
    case 'multipleValidatorTestnet':
      jsonRpcUrl = 'https://rpc.fhe-ethermint.zama.ai';
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

task('coverage').setAction(async (taskArgs, hre, runSuper) => {
  hre.config.networks.hardhat.allowUnlimitedContractSize = true;
  hre.config.networks.hardhat.blockGasLimit = 1099511627775;

  await runSuper(taskArgs);
});

task('test', async (taskArgs, hre, runSuper) => {
  // Run modified test task
  if (hre.network.name === 'hardhat') {
    // in fhevm mode all this block is done when launching the node via `pnmp fhevm:start`
    await hre.run('compile:specific', { contract: 'lib' });

    const targetAddress = '0x000000000000000000000000000000000000005d';
    const NeverRevert = await hre.artifacts.readArtifact('MockedPrecompile');
    const bytecode = NeverRevert.deployedBytecode;
    await hre.network.provider.send('hardhat_setCode', [targetAddress, bytecode]);
    console.log(`Code of Mocked Pre-compile set at address: ${targetAddress}`);

    await hre.run('compile:specific', { contract: 'gateway' });

    const privKeyDeployer = process.env.PRIVATE_KEY_GATEWAY_DEPLOYER;
    await hre.run('task:computePredeployAddress', { privateKey: privKeyDeployer });
    await hre.run('task:computeACLAddress');
    await hre.run('task:computeTFHEExecutorAddress');
    await hre.run('task:computeKMSVerifierAddress');
    await hre.run('task:deployACL');
    await hre.run('task:deployTFHEExecutor');
    await hre.run('task:deployKMSVerifier');
    await hre.run('task:launchFhevm', { skipGetCoin: false });
  }
  await hre.run('compile:specific', { contract: 'examples' });
  await runSuper();
});

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
    hardhat: {
      accounts: {
        count: 10,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
    },
    zama: getChainConfig('zama'),
    localDev: getChainConfig('local'),
    local: getChainConfig('local'),
    localNetwork1: getChainConfig('localNetwork1'),
    multipleValidatorTestnet: getChainConfig('multipleValidatorTestnet'),
  },
  paths: {
    artifacts: './artifacts',
    cache: './cache',
    sources: './examples',
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
    },
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
