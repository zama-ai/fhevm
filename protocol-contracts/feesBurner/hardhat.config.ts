import type { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const PRIVATE_KEY = process.env.PRIVATE_KEY

const config: HardhatUserConfig = {
  solidity: "0.8.28",
  'arbitrum-testnet': {
    url: process.env.RPC_URL_ARBITRUM_SEPOLIA || 'https://sepolia-rollup.arbitrum.io/rpc',
    accounts,
  },
  'ethereum-testnet': {
      url: process.env.SEPOLIA_RPC_URL,
      accounts,
  },
};

export default config;
