import "@nomicfoundation/hardhat-toolbox";
import "@openzeppelin/hardhat-upgrades";
import dotenv from "dotenv";
import { HardhatUserConfig, task } from "hardhat/config";
import { resolve } from "path";

import "./tasks/accounts";
import "./tasks/addNetworks";
import "./tasks/deploy";
import "./tasks/faucet";
import "./tasks/upgradeProxy";

const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || "./.env";
dotenv.config({ path: resolve(__dirname, dotenvConfigPath) });

const NUM_ACCOUNTS = 20;

const chainIds = {
  hardhat: 31337,
  localHTTPZGateway: 123456,
  staging: 54321,
  zwsDev: 412346,
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

task("test", async (_taskArgs, hre, runSuper) => {
  await hre.run("compile:specific", { contract: "contracts/emptyProxy" });
  await hre.run("task:faucetToPrivate", { privateKey: process.env.DEPLOYER_PRIVATE_KEY });
  await hre.run("task:deployEmptyUUPSProxies");

  // The deployEmptyUUPSProxies task may have updated the contracts' addresses in `addresses/*.sol`.
  // Thus, we must re-compile the contracts with these new addresses, otherwise the old ones will be
  // used during the tests which will make them fail.
  await hre.run("compile:specific", { contract: "contracts" });

  await hre.run("task:deployHttpz");
  await hre.run("task:deployZkpokManager");
  await hre.run("task:deployKeyManager");
  await hre.run("task:deployCiphertextManager");
  await hre.run("task:deployAclManager");
  await hre.run("task:deployDecryptionManager");

  // Contrary to deployment, here we consider the HTTPZ address from the `addresses/` directory
  // for local testing
  await hre.run("task:addNetworksToHttpz", { useInternalHttpzAddress: true });

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
      chainId: process.env.CHAIN_ID ? Number(process.env.CHAIN_ID) : chainIds.hardhat,
    },
    localHTTPZGateway: {
      accounts: {
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: process.env.CHAIN_ID ? Number(process.env.CHAIN_ID) : chainIds.localHTTPZGateway,
      url: `http://127.0.0.1:8546`,
    },
    staging: {
      accounts: {
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: process.env.CHAIN_ID ? Number(process.env.CHAIN_ID) : chainIds.staging,
      url: rpcUrl,
    },
    zwsDev: {
      accounts: {
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: process.env.CHAIN_ID ? Number(process.env.CHAIN_ID) : chainIds.zwsDev,
      url: rpcUrl,
    },
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
  paths: {
    artifacts: "./artifacts",
    cache: "./cache",
    sources: "./contracts",
    tests: "./test",
  },
};

export default config;
