import '@nomicfoundation/hardhat-toolbox';
import dotenv from 'dotenv';
import type { HardhatUserConfig, extendProvider } from 'hardhat/config';
import { task, vars } from 'hardhat/config';
import type { NetworkUserConfig } from 'hardhat/types';
import { resolve } from 'path';

const NUM_ACCOUNTS = 120;
const DEFAULT_NETWORK = 'staging';

task('compile:specific', 'Compiles only the specified contract')
  .addParam('contract', "The contract's path")
  .setAction(async ({ contract }, hre) => {
    // Adjust the configuration to include only the specified contract
    hre.config.paths.sources = contract;

    await hre.run('compile');
  });

const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || './.env';
dotenv.config({ path: resolve(__dirname, dotenvConfigPath) });

const defaultMnemonic =
  'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';
const mnemonic: string = process.env.MNEMONIC ?? vars.get('MNEMONIC', defaultMnemonic);

task('coverage').setAction(async (taskArgs, hre, runSuper) => {
  hre.config.networks.hardhat.allowUnlimitedContractSize = true;
  hre.config.networks.hardhat.blockGasLimit = 1099511627775;

  await runSuper(taskArgs);
});

task('test', async (taskArgs, hre, runSuper) => {
  // Run modified test task
  if (network.name === 'hardhat') {
    const privKeyFhevmDeployer = process.env.PRIVATE_KEY_FHEVM_DEPLOYER;
    // await hre.run('task:faucetToPrivate', { privateKey: privKeyFhevmDeployer });
    // await hre.run('task:faucetToPrivate', { privateKey: privKeyFhevmRelayer });

    await hre.run('compile:specific', { contract: 'contracts/emptyProxy' });
    await hre.run('task:deployEmptyUUPSProxies', {
      privateKey: privKeyFhevmDeployer,
      useCoprocessorAddress: false,
    });

    await hre.run('compile:specific', { contract: 'contracts' });
    await hre.run('compile:specific', { contract: 'lib' });

    await hre.run('task:deployACL', { privateKey: privKeyFhevmDeployer });
    await hre.run('task:deployTFHEExecutor', {
      privateKey: privKeyFhevmDeployer,
    });
    await hre.run('task:deployKMSVerifier', {
      privateKey: privKeyFhevmDeployer,
    });
    await hre.run('task:deployInputVerifier', {
      privateKey: privKeyFhevmDeployer,
    });
    await hre.run('task:deployHCULimit', {
      privateKey: privKeyFhevmDeployer,
    });

    await hre.run('task:addSigners', {
      numSigners: process.env.NUM_KMS_SIGNERS!,
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
  const defaultRpcUrl = 'http://localhost:8545';
  const requestedNetwork = (() => {
    const idx = process.argv.indexOf('--network');
    if (idx !== -1 && process.argv[idx + 1] && !process.argv[idx + 1].startsWith('-')) {
      return process.argv[idx + 1];
    }
    return process.env.HARDHAT_NETWORK;
  })();
  const shouldWarn = requestedNetwork ? requestedNetwork === chain : chain === DEFAULT_NETWORK;

  switch (chain) {
    case 'staging':
      jsonRpcUrl = process.env.RPC_URL ?? (vars.has('RPC_URL') ? vars.get('RPC_URL') : undefined) ?? defaultRpcUrl;
      if (shouldWarn && jsonRpcUrl === defaultRpcUrl) {
        console.warn(`WARN: RPC_URL not set for network '${chain}'. Using default: ${defaultRpcUrl}`);
      }
      break;
    case 'zwsDev':
      jsonRpcUrl = process.env.RPC_URL ?? (vars.has('RPC_URL') ? vars.get('RPC_URL') : undefined) ?? defaultRpcUrl;
      if (shouldWarn && jsonRpcUrl === defaultRpcUrl) {
        console.warn(`WARN: RPC_URL not set for network '${chain}'. Using default: ${defaultRpcUrl}`);
      }
      break;
    case 'sepolia':
      jsonRpcUrl =
        process.env.SEPOLIA_ETH_RPC_URL ??
        (vars.has('SEPOLIA_ETH_RPC_URL') ? vars.get('SEPOLIA_ETH_RPC_URL') : undefined) ??
        process.env.RPC_URL ??
        (vars.has('RPC_URL') ? vars.get('RPC_URL') : undefined) ??
        defaultRpcUrl;
      if (shouldWarn && jsonRpcUrl === defaultRpcUrl) {
        console.warn(
          `WARN: SEPOLIA_ETH_RPC_URL or RPC_URL not set for network '${chain}'. Using default: ${defaultRpcUrl}`,
        );
      }
      break;
    case 'mainnet':
      jsonRpcUrl =
        process.env.MAINNET_ETH_RPC_URL ??
        (vars.has('MAINNET_ETH_RPC_URL') ? vars.get('MAINNET_ETH_RPC_URL') : undefined) ??
        process.env.RPC_URL ??
        (vars.has('RPC_URL') ? vars.get('RPC_URL') : undefined) ??
        defaultRpcUrl;
      if (shouldWarn && jsonRpcUrl === defaultRpcUrl) {
        console.warn(
          `WARN: MAINNET_ETH_RPC_URL or RPC_URL not set for network '${chain}'. Using default: ${defaultRpcUrl}`,
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
  defaultNetwork: DEFAULT_NETWORK,
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
    mainnet: getChainConfig('mainnet'),
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
