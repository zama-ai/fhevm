import 'hardhat/types/config';
import 'hardhat/types/runtime';

// Merge types for extension config

declare module 'hardhat/types/config' {
  export interface HttpNetworkUserConfig {
    gatewayUrl?: string;
  }
  export interface HttpNetworkConfig {
    gatewayUrl?: string;
  }
}

declare module 'hardhat/types/runtime' {
  // This is an example of an extension to the Hardhat Runtime Environment.
  // This new field will be available in tasks' actions, scripts, and tests.
  export interface HardhatRuntimeEnvironment {
    __SOLIDITY_COVERAGE_RUNNING: boolean;
  }
}
