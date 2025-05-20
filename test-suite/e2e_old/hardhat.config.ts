import "@nomicfoundation/hardhat-toolbox";
import dotenv from "dotenv";
import "hardhat-deploy";
import "hardhat-ignore-warnings";
import type { HardhatUserConfig } from "hardhat/config";
import { task } from "hardhat/config";
import type { NetworkUserConfig } from "hardhat/types";
import { resolve } from "path";

// Adjust the import path as needed
import "./tasks/accounts";
import "./tasks/deployERC20";
import "./tasks/getEthereumAddress";

const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || "./.env";
dotenv.config({ path: resolve(__dirname, dotenvConfigPath) });

// Ensure that we have all the environment variables we need.
const mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  throw new Error("Please set your MNEMONIC in a .env file");
}

const chainIds = {
  zama: 9000,
  localCoprocessor: 12345,
  local: 9000,
  ethereum: 1,
  sepolia: 11155111,
};

function getChainConfig(chain: keyof typeof chainIds): NetworkUserConfig {
  let jsonRpcUrl: string;
  switch (chain) {
    case "local":
      jsonRpcUrl = "http://localhost:8545";
      break;
    case 'localCoprocessor':
      jsonRpcUrl = 'http://localhost:8745';
      break;
    case "zama":
      jsonRpcUrl = "https://devnet.zama.ai";
      break;
    case "sepolia":
      jsonRpcUrl = "https://eth-sepolia.public.blastapi.io";
      break;
    case "ethereum":
      jsonRpcUrl = "https://eth-mainnet.public.blastapi.io";
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

// NOTE: we should probably add the coverage as a metric pushed to open-telemetry
task("coverage").setAction(async (taskArgs, hre, runSuper) => {
  hre.config.networks.hardhat.allowUnlimitedContractSize = true;
  hre.config.networks.hardhat.blockGasLimit = 1099511627775;
  await runSuper(taskArgs);
});

const config: HardhatUserConfig = {
  defaultNetwork: "sepolia",
  namedAccounts: {
    deployer: 0,
  },
  mocha: {
    timeout: 500000,
    // NOTE: `spec` is the default mocha reporter
    // List of other available reporters can be found in https://github.com/mochajs/mocha/tree/main/lib/reporters
    reporter: ((process.env["MOCHA_REPORTER"] || "BASE") === "OTEL") ? './instrument-tests/json-file-to-prom-reporter.js' : 'spec',
    // NOTE: We can't use the `--require` flags because of a discrepancy in Mocha as used in Hardhat
    // https://github.com/mochajs/mocha/issues/5006
    // require: ['./instrument-tests/otel-setup'],
    parallel: false,
  },
  gasReporter: {
    currency: "USD",
    enabled: process.env.REPORT_GAS ? true : false,
    excludeContracts: [],
    src: "./contracts",
  },
  networks: {
    hardhat: {
      accounts: {
        count: 10,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
    },
    zama: getChainConfig("zama"),
    local: getChainConfig("local"),
    localCoprocessor: getChainConfig('localCoprocessor'),
    sepolia: getChainConfig("sepolia"),
    ethereum: getChainConfig("ethereum"),
  },
  paths: {
    artifacts: "./artifacts",
    cache: "./cache",
    sources: "./contracts",
    tests: "./test",
  },
  solidity: {
    version: "0.8.24",
    settings: {
      metadata: {
        // Not including the metadata hash
        // https://github.com/paulrberg/hardhat-template/issues/31
        bytecodeHash: "none",
      },
      // Disable the optimizer when debugging
      // https://hardhat.org/hardhat-network/#solidity-optimizer-support
      optimizer: {
        enabled: true,
        runs: 800,
      },
      evmVersion: "cancun",
    },
  },
  warnings: {
    "*": {
      "transient-storage": false,
    },
  },
  typechain: {
    outDir: "types",
    target: "ethers-v6",
  },
};

export default config;
