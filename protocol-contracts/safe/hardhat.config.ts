import "@nomicfoundation/hardhat-toolbox";
import "dotenv/config";
import "hardhat-dependency-compiler";
import "hardhat-deploy";
import { task, types } from "hardhat/config";
import {
  HardhatUserConfig,
  HttpNetworkAccountsUserConfig,
} from "hardhat/types";

import "./tasks/accounts";
import "./tasks/deploy";
import "./tasks/safeOwnershipTransfer";
import "./tasks/verify";

// Get the environment configuration from .env file
//
// To make use of automatic environment setup:
// - Duplicate .env.example file and name it .env
// - Fill in the environment variables

// Set your preferred authentication method
//
// If you prefer using a mnemonic, set a MNEMONIC environment variable
// to a valid mnemonic
const MNEMONIC = process.env.MNEMONIC;

// If you prefer to be authenticated using a private key, set a PRIVATE_KEY environment variable
const PRIVATE_KEY = process.env.PRIVATE_KEY;

const accounts: HttpNetworkAccountsUserConfig | undefined = MNEMONIC
  ? { mnemonic: MNEMONIC }
  : PRIVATE_KEY
    ? [PRIVATE_KEY]
    : undefined;

if (accounts == null) {
  console.warn(
    "Could not find MNEMONIC or PRIVATE_KEY environment variables. It will not be possible to execute transactions in your example.",
  );
}

task("test", "Runs the test suite, optionally skipping setup tasks")
  .addOptionalParam(
    "skipSetup",
    "Set to true to skip setup tasks",
    false,
    types.boolean,
  )
  .setAction(async ({ skipSetup }, hre, runSuper) => {
    if (!skipSetup) {
      // Compile the contracts
      await hre.run("compile");

      // Deploy the SafeL2 contract
      await hre.run("task:deploySafeL2");

      // Deploy the AdminModule contract
      // Safe address is fixed in the .env file but should match the one deployed above
      await hre.run("task:deployAdminModule");

      // Deploy the MultiSend contract
      await hre.run("task:deployMultiSend");
    } else {
      console.log("Skipping contracts setup.");
    }
    await runSuper();
  });

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.24",
    settings: {
      metadata: {
        // Not including the metadata hash
        // https://github.com/paulrberg/hardhat-template/issues/31
        bytecodeHash: "none",
      },
      optimizer: {
        enabled: true,
        runs: 800,
      },
      evmVersion: "cancun",
      viaIR: false,
    },
  },
  dependencyCompiler: {
    paths: [
      "@safe-global/safe-contracts/contracts/proxies/SafeProxyFactory.sol",
    ],
  },
  networks: {
    "gateway-testnet": {
      url: process.env.RPC_URL_ZAMA_GATEWAY_TESTNET,
      accounts,
    },
    hardhat: {
      // Need this to avoid deployment issues in test
      saveDeployments: false,
    },
  },
  namedAccounts: {
    deployer: {
      default: 0, // wallet address of index[0], of the mnemonic in .env
    },
    alice: {
      default: 1, // wallet address of index[1], of the mnemonic in .env
    },
    bob: {
      default: 2, // wallet address of index[2], of the mnemonic in .env
    },
    charlie: {
      default: 3, // wallet address of index[3], of the mnemonic in .env
    },
    dave: {
      default: 4, // wallet address of index[4], of the mnemonic in .env
    },
    eve: {
      default: 5, // wallet address of index[5], of the mnemonic in .env
    },
    frank: {
      default: 6, // wallet address of index[6], of the mnemonic in .env
    },
    grace: {
      default: 7, // wallet address of index[7], of the mnemonic in .env
    },
    heidi: {
      default: 8, // wallet address of index[8], of the mnemonic in .env
    },
    ivan: {
      default: 9, // wallet address of index[9], of the mnemonic in .env
    },
    judy: {
      default: 10, // wallet address of index[10], of the mnemonic in .env
    },
  },
  etherscan: {
    apiKey: {
      "gateway-testnet": "empty",
    },
    customChains: [
      {
        network: "gateway-testnet",
        chainId: 10901,
        urls: {
          apiURL: "https://explorer-zama-testnet-0.t.conduit.xyz/api",
          browserURL: "https://explorer-zama-testnet-0.t.conduit.xyz/",
        },
      },
    ],
  },
  // Add this section to disable gas reporter
  gasReporter: {
    enabled: false,
  },
};

export default config;
