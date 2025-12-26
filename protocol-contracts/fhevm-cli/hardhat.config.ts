import "@fhevm/hardhat-plugin";
import "@nomicfoundation/hardhat-chai-matchers";
import "@nomicfoundation/hardhat-ethers";
import "dotenv/config";
import { HardhatUserConfig } from "hardhat/config";

import "./tasks/encrypt";
import "./tasks/publicDecrypt";
import "./tasks/userDecrypt";

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.28",
    settings: {
      evmVersion: "cancun",
    },
  },
  networks: {
    // ChainID must be specified in order to be able to verify contracts using the fhevm hardhat plugin
    mainnet: {
      url: process.env.MAINNET_RPC_URL || "",
      chainId: 1,
    },
    // ChainID must be specified in order to be able to verify contracts using the fhevm hardhat plugin
    testnet: {
      url: process.env.TESTNET_RPC_URL || "",
      chainId: 11155111,
    },
  },
};

export default config;
