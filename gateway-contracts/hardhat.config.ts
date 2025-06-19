import "@nomicfoundation/hardhat-chai-matchers";
import "@nomicfoundation/hardhat-network-helpers";
import "@nomicfoundation/hardhat-verify";
import "@openzeppelin/hardhat-upgrades";
import "@typechain/hardhat";
import dotenv from "dotenv";
import "hardhat-ignore-warnings";
import { HardhatUserConfig, task, types } from "hardhat/config";
import { resolve } from "path";

import "./tasks/accounts";
import "./tasks/addHostChains";
import "./tasks/deployment/contracts";
import "./tasks/deployment/empty_proxies";
import "./tasks/deployment/mock_contracts";
import "./tasks/getters";
import "./tasks/upgradeContracts";

const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || "./.env";
dotenv.config({ path: resolve(__dirname, dotenvConfigPath) });

export const NUM_ACCOUNTS = 30;

const chainIds = {
  hardhat: 31337,
  localGateway: 123456,
  staging: 54321,
  zwsDev: 412346,
  testnet: 55815,
};

// If the mnemonic is not set, use a default one
let mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  mnemonic = "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer";
}

let rpcUrl: string | undefined = process.env.RPC_URL;
if (!rpcUrl) {
  rpcUrl = "http://127.0.0.1:8757";
}

task("compile:specific", "Compiles only the specified contract")
  .addParam("contract", "The contract's path")
  .setAction(async ({ contract }, hre) => {
    // Adjust the configuration to include only the specified contract
    hre.config.paths.sources = contract;

    await hre.run("compile");
  });

task("test", "Runs the test suite, optionally skipping setup tasks")
  .addOptionalParam("skipSetup", "Set to true to skip setup tasks", false, types.boolean)
  .setAction(async ({ skipSetup }, hre, runSuper) => {
    if (!skipSetup) {
      await hre.run("task:deployAllGatewayContracts");
      // Contrary to deployment, here we consider the GatewayConfig address from the `addresses/` directory
      // for local testing
      await hre.run("task:addHostChainsToGatewayConfig", { useInternalGatewayConfigAddress: true });
    } else {
      console.log("Skipping contracts setup.");
    }
    await runSuper();
  });

const config: HardhatUserConfig = {
  networks: {
    hardhat: {
      accounts: {
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: process.env.CHAIN_ID_GATEWAY ? Number(process.env.CHAIN_ID_GATEWAY) : chainIds.hardhat,
    },
    localGateway: {
      accounts: {
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: process.env.CHAIN_ID_GATEWAY ? Number(process.env.CHAIN_ID_GATEWAY) : chainIds.localGateway,
      url: rpcUrl,
    },
    staging: {
      accounts: {
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: process.env.CHAIN_ID_GATEWAY ? Number(process.env.CHAIN_ID_GATEWAY) : chainIds.staging,
      url: rpcUrl,
    },
    zwsDev: {
      accounts: {
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: process.env.CHAIN_ID_GATEWAY ? Number(process.env.CHAIN_ID_GATEWAY) : chainIds.zwsDev,
      url: rpcUrl,
    },
    testnet: {
      accounts: {
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: process.env.CHAIN_ID_GATEWAY ? Number(process.env.CHAIN_ID_GATEWAY) : chainIds.testnet,
      url: rpcUrl,
    },
  },
  sourcify: {
    enabled: false,
  },
  etherscan: {
    apiKey: {
      zwsDev: "empty",
      testnet: "empty",
    },
    customChains: [
      {
        network: "zwsDev",
        chainId: chainIds.zwsDev,
        urls: {
          apiURL: "http://l2-blockscout-zws-dev-blockscout-stack-blockscout-svc/api",
          browserURL: "https://l2-explorer-zws-dev.diplodocus-boa.ts.net",
        },
      },
      {
        network: "testnet",
        chainId: chainIds.testnet,
        urls: {
          apiURL: "https://explorer.testnet.zama.cloud/api",
          browserURL: "https://explorer.testnet.zama.cloud/",
        },
      },
    ],
  },
  // We use 0.8.24 to align with the solidity compiler version used in the host chain smart contracts
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
      viaIR: false,
    },
  },
  warnings: {
    // Turn off all warnings for mocked contracts
    "contracts/mocks/*": {
      default: "off",
    },
  },
  paths: {
    artifacts: "./artifacts",
    cache: "./cache",
    sources: "./contracts",
    tests: "./test",
  },
};

export default config;
