import "@nomicfoundation/hardhat-toolbox";
import dotenv from "dotenv";
import type { HardhatUserConfig, extendProvider } from "hardhat/config";
import { task } from "hardhat/config";
import type { NetworkUserConfig } from "hardhat/types";
import { resolve } from "path";

const NUM_ACCOUNTS = 15;

task("compile:specific", "Compiles only the specified contract")
  .addParam("contract", "The contract's path")
  .setAction(async ({ contract }, hre) => {
    // Adjust the configuration to include only the specified contract
    hre.config.paths.sources = contract;

    await hre.run("compile");
  });

const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || "./.env";
dotenv.config({ path: resolve(__dirname, dotenvConfigPath) });

// Ensure that we have all the environment variables we need.
let mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  mnemonic =
    "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer"; // default mnemonic in case it is undefined (needed to avoid panicking when deploying on real network)
}

task("coverage").setAction(async (taskArgs, hre, runSuper) => {
  hre.config.networks.hardhat.allowUnlimitedContractSize = true;
  hre.config.networks.hardhat.blockGasLimit = 1099511627775;

  await runSuper(taskArgs);
});

task("test", async (taskArgs, hre, runSuper) => {
  // Run modified test task
  if (network.name === "hardhat") {
    const privKeyFhevmDeployer = process.env.PRIVATE_KEY_FHEVM_DEPLOYER;
    const privKeyFhevmRelayer =
      process.env.PRIVATE_KEY_DECRYPTION_ORACLE_RELAYER;
    await hre.run("task:faucetToPrivate", { privateKey: privKeyFhevmDeployer });
    await hre.run("task:faucetToPrivate", { privateKey: privKeyFhevmRelayer });

    await hre.run("compile:specific", { contract: "contracts/emptyProxy" });
    await hre.run("task:deployEmptyUUPSProxies", {
      privateKey: privKeyFhevmDeployer,
      useCoprocessorAddress: false,
    });

    await hre.run("compile:specific", { contract: "contracts" });
    await hre.run("compile:specific", { contract: "lib" });
    await hre.run("compile:specific", { contract: "decryptionOracle" });

    await hre.run("task:deployACL", { privateKey: privKeyFhevmDeployer });
    await hre.run("task:deployTFHEExecutor", {
      privateKey: privKeyFhevmDeployer,
    });
    await hre.run("task:deployKMSVerifier", {
      privateKey: privKeyFhevmDeployer,
    });
    await hre.run("task:deployInputVerifier", {
      privateKey: privKeyFhevmDeployer,
    });
    await hre.run("task:deployFHEGasLimit", {
      privateKey: privKeyFhevmDeployer,
    });
    await hre.run("task:deployDecryptionOracle", {
      privateKey: privKeyFhevmDeployer,
    });

    await hre.run("task:addSigners", {
      numSigners: process.env.NUM_KMS_SIGNERS!,
      privateKey: privKeyFhevmDeployer,
      useAddress: false,
    });
  }
  await hre.run("compile:specific", { contract: "examples" });
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
  const defaultRpcUrl = "http://localhost:8545";

  switch (chain) {
    case "staging":
      jsonRpcUrl = process.env.RPC_URL || defaultRpcUrl;
      if (jsonRpcUrl === defaultRpcUrl && !process.env.RPC_URL) {
        console.warn(
          `WARN: RPC_URL environment variable not set for network '${chain}'. Using default: ${defaultRpcUrl}`
        );
      }
      break;
    case "zwsDev":
      jsonRpcUrl = process.env.RPC_URL || defaultRpcUrl;
      if (jsonRpcUrl === defaultRpcUrl && !process.env.RPC_URL) {
        console.warn(
          `WARN: RPC_URL environment variable not set for network '${chain}'. Using default: ${defaultRpcUrl}`
        );
      }
      break;
    case "sepolia":
      jsonRpcUrl = process.env.RPC_URL || defaultRpcUrl;
      if (jsonRpcUrl === defaultRpcUrl && !process.env.RPC_URL) {
        console.warn(
          `WARN: RPC_URL environment variable not set for network '${chain}'. Using default: ${defaultRpcUrl}`
        );
      }
      break;
    case "localCoprocessor":
      jsonRpcUrl = "http://localhost:8746";
      break;
    case "localCoprocessorL1Input":
      jsonRpcUrl = "http://localhost:8756";
      break;
    case "localNative":
      jsonRpcUrl = "http://localhost:8545";
      break;
    case "localCoprocessorL1":
      jsonRpcUrl = "http://localhost:8756";
      break;
    case "localCoprocessorL2":
      jsonRpcUrl = "http://localhost:8757";
      break;
    case "composeCoprocessorL1":
      jsonRpcUrl = "http://mock-httpz-1:8756";
      break;
    case "composeCoprocessorL2":
      jsonRpcUrl = "http://mock-gateway-1:8757";
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
  mocha: {
    timeout: 300000,
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
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: process.env.CUSTOM_CHAIN_ID
        ? Number(process.env.CUSTOM_CHAIN_ID)
        : 31337,
      mining: {
        auto: true,
        interval: 1000,
      },
    },
    staging: getChainConfig("staging"),
    zwsDev: getChainConfig("zwsDev"),
    sepolia: getChainConfig("sepolia"),
    localNative: getChainConfig("localNative"),
    localCoprocessor: getChainConfig("localCoprocessor"),
    localCoprocessorL1: getChainConfig("localCoprocessorL1"),
    localCoprocessorL1Input: getChainConfig("localCoprocessorL1Input"),
    localCoprocessorL2: getChainConfig("localCoprocessorL2"),
    composeCoprocessorL1: getChainConfig("composeCoprocessorL1"),
    composeCoprocessorL2: getChainConfig("composeCoprocessorL2"),
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
      viaIR: false,
    },
  },
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY!,
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
