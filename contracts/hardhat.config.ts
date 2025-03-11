import '@nomicfoundation/hardhat-toolbox';
import '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import fs from 'fs';
import 'hardhat-deploy';
import 'hardhat-ignore-warnings';
import type { HardhatUserConfig, extendProvider } from 'hardhat/config';
import { task } from 'hardhat/config';
import type { NetworkUserConfig } from 'hardhat/types';
import { resolve } from 'path';

import CustomProvider from './CustomProvider';
// Adjust the import path as needed
import './tasks/etherscanVerify';
import './tasks/taskDeploy';
import './tasks/taskUtils';
import './tasks/upgradeProxy';

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
    const privKeyFhevmDeployer = process.env.PRIVATE_KEY_FHEVM_DEPLOYER;
    const privKeyFhevmRelayer = process.env.PRIVATE_KEY_DECRYPTION_ORACLE_RELAYER;
    const parsedEnv2 = dotenv.parse(fs.readFileSync('addressesL2/.env.decryptionmanager'));
    const decryptionManagerAddress = parsedEnv2.DECRYPTION_MANAGER_ADDRESS;
    const parsedEnv3 = dotenv.parse(fs.readFileSync('addressesL2/.env.zkpokmanager'));
    const zkpokManagerAddress = parsedEnv3.ZKPOK_MANAGER_ADDRESS;
    await hre.run('task:faucetToPrivate', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:faucetToPrivate', { privateKey: privKeyFhevmRelayer });

    await hre.run('compile:specific', { contract: 'contracts/emptyProxy' });
    await hre.run('task:deployEmptyUUPSProxies', { privateKey: privKeyFhevmDeployer, useCoprocessorAddress: false });

    await hre.run('compile:specific', { contract: 'contracts' });
    await hre.run('compile:specific', { contract: 'lib' });
    await hre.run('compile:specific', { contract: 'decryptionOracle' });

    await hre.run('task:deployACL', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:deployTFHEExecutor', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:deployKMSVerifier', {
      privateKey: privKeyFhevmDeployer,
      decryptionManagerAddress: decryptionManagerAddress,
    });
    await hre.run('task:deployInputVerifier', {
      privateKey: privKeyFhevmDeployer,
      zkpokManagerAddress: zkpokManagerAddress,
    });
    await hre.run('task:deployFHEGasLimit', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:deployDecryptionOracle', { privateKey: privKeyFhevmDeployer });

    await hre.run('task:addSigners', {
      numSigners: process.env.NUM_KMS_SIGNERS!,
      privateKey: privKeyFhevmDeployer,
      useAddress: false,
    });
    await hre.run('task:addInputSigners', {
      numSigners: process.env.NUM_COPROCESSOR_SIGNERS!,
      privateKey: privKeyFhevmDeployer,
      useAddress: false,
    });
  }
  await hre.run('compile:specific', { contract: 'examples' });
  await runSuper();
});

const chainIds = {
  localNative: 8009,
  devnetNative: 9000,
  localCoprocessor: 12345,
  sepolia: 11155111,
  staging: 12345,
  mainnet: 1,
};

function getChainConfig(chain: keyof typeof chainIds): NetworkUserConfig {
  let jsonRpcUrl: string;
  switch (chain) {
    case 'sepolia':
      jsonRpcUrl = process.env.SEPOLIA_RPC_URL!;
      break;
    case 'staging':
      jsonRpcUrl = process.env.STAGING_RPC_URL!;
      break;
    case 'localCoprocessor':
      jsonRpcUrl = 'http://localhost:8745';
      break;
    case 'localNative':
      jsonRpcUrl = 'http://localhost:8545';
      break;
    default:
      throw new Error(`unsupported chain: ${chain}`);
  }
  return {
    accounts: {
      count: 15,
      mnemonic,
      path: "m/44'/60'/0'/0",
    },
    chainId: chainIds[chain],
    url: jsonRpcUrl,
  };
}

const config: HardhatUserConfig = {
  defaultNetwork: 'sepolia',
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
        count: 10,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
    },
    staging: getChainConfig('staging'),
    sepolia: getChainConfig('sepolia'),
    localNative: getChainConfig('localNative'),
    localCoprocessor: getChainConfig('localCoprocessor'),
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
  },
  typechain: {
    outDir: 'types',
    target: 'ethers-v6',
  },
};

export default config;
