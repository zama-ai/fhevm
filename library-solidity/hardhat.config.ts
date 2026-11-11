import '@nomicfoundation/hardhat-toolbox';
import '@openzeppelin/hardhat-upgrades';
import dotenv from 'dotenv';
import * as fs from 'fs';
import * as path from 'path';
import 'hardhat-ignore-warnings';
import { task } from 'hardhat/config';
import { resolve } from 'path';
import { extendProvider } from 'hardhat/config';
import type { HardhatUserConfig } from 'hardhat/config';
import type { NetworkUserConfig } from 'hardhat/types';

import CustomProvider from './CustomProvider';
import './tasks/addPausers';
import './tasks/taskDeploy';

// Well-known test mnemonic, used when MNEMONIC is not set. Never use it to hold real funds.
const DEFAULT_MNEMONIC =
  'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';

const NUM_ACCOUNTS = 15;

// eslint-disable-next-line @typescript-eslint/require-await
extendProvider(async (provider, _config, _network): Promise<CustomProvider> => {
  const newProvider = new CustomProvider(provider);
  return newProvider;
});

task('compile:specific', 'Compiles only the specified contract')
  .addParam('contract', "The contract's path")
  .setAction(async ({ contract }: { contract: string }, hre) => {
    // Adjust the configuration to include only the specified contract
    hre.config.paths.sources = contract;
    await hre.run('compile');
  });

// DOTENV_CONFIG_PATH (relative to this file) overrides the default `.env` when set and non-empty.
const envDotenvConfigPath = process.env.DOTENV_CONFIG_PATH;
const dotenvConfigPath: string =
  envDotenvConfigPath !== undefined && envDotenvConfigPath !== '' ? envDotenvConfigPath : './.env';
dotenv.config({ path: resolve(__dirname, dotenvConfigPath) });

// Use DEFAULT_MNEMONIC when MNEMONIC is unset or empty (needed to avoid panicking when deploying on real network).
const envMnemonic = process.env.MNEMONIC;
const mnemonic: string = envMnemonic !== undefined && envMnemonic.trim() !== '' ? envMnemonic : DEFAULT_MNEMONIC;

task('coverage').setAction(async (taskArgs, hre, runSuper) => {
  hre.config.networks.hardhat.allowUnlimitedContractSize = true;
  hre.config.networks.hardhat.blockGasLimit = 1099511627775;

  await runSuper(taskArgs);
});

// Copies the host contracts from @fhevm/host-contracts into fhevmTemp/contracts.
function copyHostContracts(): void {
  const hostContractsSrcDir = path.resolve(__dirname, '../node_modules/@fhevm/host-contracts/contracts');
  const hostContractsDstDir = path.resolve(__dirname, 'fhevmTemp/contracts');
  // Ensure the tmp contracts directory exists.
  fs.mkdirSync(hostContractsDstDir, { recursive: true });
  fs.cpSync(hostContractsSrcDir, hostContractsDstDir, { recursive: true, dereference: true });
  // The host bridge contracts pull in LayerZero dependencies that library-solidity does not ship,
  // so they are excluded from the compile set. ACL.sol, however, imports the dependency-free
  // IConfidentialBridge interface (to resync the LZ delegate on ownership transfer), so preserve
  // that single file while dropping the rest of bridge/.
  const bridgeDstDir = path.join(hostContractsDstDir, 'bridge');
  const keptBridgeInterface = path.join(bridgeDstDir, 'interfaces/IConfidentialBridge.sol');
  const keptBridgeInterfaceContents = fs.readFileSync(keptBridgeInterface);
  fs.rmSync(bridgeDstDir, { recursive: true, force: true });
  fs.mkdirSync(path.dirname(keptBridgeInterface), { recursive: true });
  fs.writeFileSync(keptBridgeInterface, keptBridgeInterfaceContents);
}

// Prepares everything the tests need: copies the host contracts into fhevmTemp/, then, on the in-process hardhat
// network, deploys them (which generates fhevmTemp/addresses/ and compiles fhevmTemp/ and examples/) and adds
// the pausers.
task('task:prepareTestContracts', async (_taskArgs, hre) => {
  copyHostContracts();

  if (hre.network.name === 'hardhat') {
    await hre.run('task:deployAllHostContracts', { withKmsGeneration: true });
    await hre.run('task:addHostPausers', { useInternalProxyAddress: true });
  }
});

// Compiles every contract the tests use (lib/, fhevmTemp/ and examples/), doing the minimum. Each compile also
// generates the contracts' TypeChain types (hardhat-toolbox). Compiling fhevmTemp/ and examples/ requires fhevmTemp/addresses/FHEVMHostAddresses.sol, which only the empty
// proxies and the PauserSet deployments produce (like host-contracts' `make ensure-addresses`); they run on the
// throwaway in-process hardhat network. No host contract is deployed or initialized.
task('task:compileAll', async (_taskArgs, hre) => {
  if (hre.network.name !== 'hardhat') {
    throw new Error(`task:compileAll must run on the hardhat network, not ${hre.network.name}`);
  }
  copyHostContracts();

  // lib/ first: compile:specific below changes the compiled sources for the rest of the run.
  await hre.run('compile');
  await hre.run('task:deployEmptyUUPSProxies', { withKmsGeneration: true });
  await hre.run('compile:specific', { contract: 'fhevmTemp/contracts/immutable' });
  await hre.run('task:deployPauserSet');
  await hre.run('compile:specific', { contract: 'fhevmTemp/contracts' });
  await hre.run('compile:specific', { contract: 'examples' });
});

task('test', async (_taskArgs, hre, runSuper) => {
  await hre.run('task:prepareTestContracts');
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
  const jsonRpcUrl: string = process.env.RPC_URL ?? 'http://127.0.0.1:8756';

  // CHAIN_ID overrides the network's default chain id when set and non-empty.
  const envChainId = process.env.CHAIN_ID;
  const chainId = envChainId !== undefined && envChainId !== '' ? Number(envChainId) : chainIds[chain];
  if (!Number.isInteger(chainId) || chainId <= 0) {
    throw new Error(`Invalid CHAIN_ID "${envChainId ?? ''}": expected a positive integer`);
  }

  return {
    accounts: {
      count: NUM_ACCOUNTS,
      mnemonic,
      path: "m/44'/60'/0'/0",
    },
    chainId,
    url: jsonRpcUrl,
  };
}

const config: HardhatUserConfig = {
  mocha: {
    timeout: 500000,
  },
  gasReporter: {
    currency: 'USD',
    // Enabled when REPORT_GAS is set to any non-empty value.
    enabled: process.env.REPORT_GAS !== undefined && process.env.REPORT_GAS !== '',
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
    sources: './lib',
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
    // Only needed by `hardhat verify`; '' is the plugin's own default when no key is configured.
    apiKey: process.env.ETHERSCAN_API_KEY ?? '',
  },
  warnings: {
    '*': {
      'transient-storage': false,
    },
  },
  typechain: {
    target: 'ethers-v6',
  },
};

export default config;
