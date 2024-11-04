import '@nomicfoundation/hardhat-toolbox';
import '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
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
import './tasks/taskGatewayRelayer';
import './tasks/taskTFHE';
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

const chainIds = {
  zama: 8009,
  local: 9000,
  localCoprocessor: 12345,
  localNetwork1: 9000,
  multipleValidatorTestnet: 8009,
  sepolia: 11155111,
  mainnet: 1,
};

function getChainConfig(chain: keyof typeof chainIds): NetworkUserConfig {
  let jsonRpcUrl: string;
  switch (chain) {
    case 'local':
      jsonRpcUrl = 'http://localhost:8545';
      break;
    case 'localCoprocessor':
      jsonRpcUrl = 'http://localhost:8745';
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
    case 'sepolia':
      jsonRpcUrl = process.env.SEPOLIA_RPC_URL!;
    case 'mainnet':
      jsonRpcUrl = process.env.MAINNET_RPC_URL!;
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
    const privKeyGatewayDeployer = process.env.PRIVATE_KEY_GATEWAY_DEPLOYER;
    const privKeyFhevmDeployer = process.env.PRIVATE_KEY_FHEVM_DEPLOYER;
    await hre.run('task:computeGatewayAddress', { privateKey: privKeyGatewayDeployer });
    await hre.run('task:computeACLAddress', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:computeTFHEExecutorAddress', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:computeKMSVerifierAddress', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:computeInputVerifierAddress', { privateKey: privKeyFhevmDeployer, useAddress: false });
    await hre.run('task:computeFHEPaymentAddress', { privateKey: privKeyFhevmDeployer });
    await hre.run('compile:specific', { contract: 'contracts' });
    await hre.run('compile:specific', { contract: 'lib' });
    await hre.run('compile:specific', { contract: 'gateway' });
    await hre.run('compile:specific', { contract: 'payment' });
    await hre.run('task:faucetToPrivate', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:deployACL', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:deployTFHEExecutor', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:deployKMSVerifier', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:deployInputVerifier', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:deployFHEPayment', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:addSigners', {
      numSigners: process.env.NUM_KMS_SIGNERS!,
      privateKey: privKeyFhevmDeployer,
      useAddress: false,
    });
    await hre.run('task:launchFhevm', { skipGetCoin: false, useAddress: false });
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
    sepolia: getChainConfig('sepolia'),
    mainnet: getChainConfig('mainnet'),
    zama: getChainConfig('zama'),
    localDev: getChainConfig('local'),
    local: getChainConfig('local'),
    localCoprocessor: getChainConfig('localCoprocessor'),
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
