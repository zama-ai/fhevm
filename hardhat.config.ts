import '@nomicfoundation/hardhat-toolbox';
import { config as dotenvConfig } from 'dotenv';
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
dotenvConfig({ path: resolve(__dirname, dotenvConfigPath) });

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
    version: '0.8.22',
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
      evmVersion: 'paris',
    },
  },
  typechain: {
    outDir: 'types',
    target: 'ethers-v6',
  },
};

export default config;
