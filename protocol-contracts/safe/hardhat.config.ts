import "@nomicfoundation/hardhat-toolbox";
import "hardhat-dependency-compiler";
import { HardhatUserConfig } from "hardhat/config";

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
    paths: ["@safe-global/safe-contracts/contracts/proxies/SafeProxyFactory.sol"],
  },
};

export default config;
