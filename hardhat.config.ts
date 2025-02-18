import "@nomicfoundation/hardhat-toolbox";
import dotenv from "dotenv";
import { HardhatUserConfig } from "hardhat/config";
import { resolve } from "path";

import "./tasks/accounts";
import "./tasks/deploy";

const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || "./.env";
dotenv.config({ path: resolve(__dirname, dotenvConfigPath) });

const NUM_ACCOUNTS = 15;

const localGatewayL2ChainId = 123456;

// If the mnemonic is not set, use a default one
let mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  mnemonic = "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer";
}

const config: HardhatUserConfig = {
  networks: {
    hardhat: {
      accounts: {
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: process.env.CUSTOM_CHAIN_ID ? Number(process.env.CUSTOM_CHAIN_ID) : 31337,
    },
    localGatewayL2: {
      accounts: {
        count: NUM_ACCOUNTS,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
      chainId: localGatewayL2ChainId,
      url: `http://127.0.0.1:8546`,
    },
  },
  solidity: "0.8.28",
  paths: {
    artifacts: "./artifacts",
    cache: "./cache",
    sources: "./contracts",
    tests: "./test",
  },
};

export default config;
