import "@nomicfoundation/hardhat-toolbox";
import "dotenv/config";
import "hardhat-deploy";
import type { HardhatUserConfig } from "hardhat/config";

import "./tasks/blockExplorerVerify";
import "./tasks/deployFeesBurner";

// Set your preferred authentication method
//
// If you prefer using a mnemonic, set a MNEMONIC environment variable
// to a valid mnemonic
const MNEMONIC = process.env.MNEMONIC;

// If you prefer to be authenticated using a private key, set a PRIVATE_KEY environment variable
const PRIVATE_KEY = process.env.PRIVATE_KEY;

const accounts = MNEMONIC ? { mnemonic: MNEMONIC } : PRIVATE_KEY ? [PRIVATE_KEY] : undefined;

if (accounts == null) {
  console.warn(
    "Could not find MNEMONIC or PRIVATE_KEY environment variables. It will not be possible to execute transactions in your example.",
  );
}

const config: HardhatUserConfig = {
  solidity: "0.8.28",
  networks: {
    "ethereum-mainnet": {
      url: process.env.MAINNET_RPC_URL || "",
      accounts,
    },
    "ethereum-testnet": {
      url: process.env.SEPOLIA_RPC_URL || "",
      accounts,
    },
    "gateway-mainnet": {
      url: process.env.RPC_URL_ZAMA_GATEWAY_MAINNET || "",
      accounts,
    },
    "gateway-testnet": {
      url: process.env.RPC_URL_ZAMA_GATEWAY_TESTNET || "",
      accounts,
    },
    hardhat: {
      chainId: 11155111, // to make hardhat tests pass because of security check in FeesSenderToBurner constructor
    },
  },
  namedAccounts: {
    deployer: {
      default: 0,
    },
  },
  etherscan: {
    apiKey: process.env.ETHERSCAN_API || "",
    customChains: [
      {
        network: "gateway-mainnet",
        chainId: 261131,
        urls: {
          apiURL: "https://explorer-zama-gateway-mainnet.t.conduit.xyz/api",
          browserURL: "https://explorer-zama-gateway-mainnet.t.conduit.xyz",
        },
      },
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
};

export default config;
