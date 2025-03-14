import "@nomicfoundation/hardhat-toolbox";
import dotenv from "dotenv";
import { HardhatUserConfig } from "hardhat/config";
import { resolve } from "path";

import "./tasks/accounts";
import "./tasks/deploy";

const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || "./.env";
dotenv.config({ path: resolve(__dirname, dotenvConfigPath) });

const NUM_ACCOUNTS = 15;

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
  solidity: "0.8.24",
  paths: {
    artifacts: "./artifacts",
    cache: "./cache",
    sources: "./contracts",
    tests: "./test",
  },
};

export default config;
