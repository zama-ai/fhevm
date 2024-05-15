import '@nomicfoundation/hardhat-toolbox';
import dotenv from 'dotenv';
import * as fs from 'fs';
import 'hardhat-deploy';
import 'hardhat-preprocessor';
import { TASK_PREPROCESS } from 'hardhat-preprocessor';
import type { HardhatUserConfig } from 'hardhat/config';
import { task } from 'hardhat/config';
import type { NetworkUserConfig } from 'hardhat/types';
import { resolve } from 'path';
import * as path from 'path';

import './tasks/accounts';
import './tasks/getEthereumAddress';
import './tasks/mint';
import './tasks/taskDeploy';
import './tasks/taskIdentity';
import './tasks/taskOracleRelayer';
import './tasks/taskTFHE';

// Function to recursively get all .sol files in a folder
function getAllSolidityFiles(dir: string, fileList: string[] = []): string[] {
  fs.readdirSync(dir).forEach((file) => {
    const filePath = path.join(dir, file);
    if (fs.statSync(filePath).isDirectory()) {
      getAllSolidityFiles(filePath, fileList);
    } else if (filePath.endsWith('.sol')) {
      fileList.push(filePath);
    }
  });
  return fileList;
}

task('compile:specific', 'Compiles only the specified contract')
  .addParam('contract', "The contract's path")
  .setAction(async ({ contract }, hre) => {
    // Adjust the configuration to include only the specified contract
    hre.config.paths.sources = contract;

    await hre.run('compile');
  });

task('coverage-mock', 'Run coverage after running pre-process task').setAction(async function (args, env) {
  // Get all .sol files in the examples/ folder
  const examplesPath = path.join(env.config.paths.root, 'examples/');
  const solidityFiles = getAllSolidityFiles(examplesPath);

  // Backup original files
  const originalContents: Record<string, string> = {};
  solidityFiles.forEach((filePath) => {
    originalContents[filePath] = fs.readFileSync(filePath, { encoding: 'utf8' });
  });

  try {
    // Run pre-process task
    await env.run(TASK_PREPROCESS);

    // Run coverage task
    await env.run('coverage');
  } finally {
    // Restore original files
    for (const filePath in originalContents) {
      fs.writeFileSync(filePath, originalContents[filePath], { encoding: 'utf8' });
    }
  }
});

const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || './.env';
dotenv.config({ path: resolve(__dirname, dotenvConfigPath) });

// Ensure that we have all the environment variables we need.
const mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  throw new Error('Please set your MNEMONIC in a .env file');
}

const network = process.env.HARDHAT_NETWORK;

function getRemappings() {
  return fs
    .readFileSync('remappings.txt', 'utf8')
    .split('\n')
    .filter(Boolean) // remove empty lines
    .map((line: string) => line.trim().split('='));
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

task('test', async (taskArgs, hre, runSuper) => {
  // Run modified test task

  if (network === 'hardhat') {
    // in fhevm mode all this block is done when launching the node via `pnmp fhevm:start`
    const privKeyDeployer = process.env.PRIVATE_KEY_ORACLE_DEPLOYER;
    const privKeyOwner = process.env.PRIVATE_KEY_ORACLE_OWNER;
    const privKeyRelayer = process.env.PRIVATE_KEY_ORACLE_RELAYER;
    const deployerAddress = new hre.ethers.Wallet(privKeyDeployer!).address;
    const ownerAddress = new hre.ethers.Wallet(privKeyOwner!).address;
    const relayerAddress = new hre.ethers.Wallet(privKeyRelayer!).address;

    await hre.run('task:computePredeployAddress', { privateKey: privKeyDeployer });

    const bal = '0x1000000000000000000000000000000000000000';
    const p1 = hre.network.provider.send('hardhat_setBalance', [deployerAddress, bal]);
    const p2 = hre.network.provider.send('hardhat_setBalance', [ownerAddress, bal]);
    const p3 = hre.network.provider.send('hardhat_setBalance', [relayerAddress, bal]);
    await Promise.all([p1, p2, p3]);
    await hre.run('compile');
    await hre.run('task:deployOracle', { privateKey: privKeyDeployer, ownerAddress: ownerAddress });

    const parsedEnv = dotenv.parse(fs.readFileSync('oracle/.env.oracle'));
    const oraclePredeployAddress = parsedEnv.ORACLE_CONTRACT_PREDEPLOY_ADDRESS;

    await hre.run('task:addRelayer', {
      privateKey: privKeyOwner,
      oracleAddress: oraclePredeployAddress,
      relayerAddress: relayerAddress,
    });
  }

  await runSuper();
});

const config: HardhatUserConfig = {
  preprocess: {
    eachLine: (hre) => ({
      transform: (line: string) => {
        if (network === 'hardhat') {
          // checks if HARDHAT_NETWORK env variable is set to "hardhat" to use the remapping for the mocked version of TFHE.sol
          if (line.match(/".*.sol";$/)) {
            // match all lines with `"<any-import-path>.sol";`
            for (const [from, to] of getRemappings()) {
              if (line.includes(from)) {
                line = line.replace(from, to);
                break;
              }
            }
          }
        }
        return line;
      },
    }),
  },
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
    version: '0.8.25',
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
  typechain: {
    outDir: 'types',
    target: 'ethers-v6',
  },
};

export default config;
