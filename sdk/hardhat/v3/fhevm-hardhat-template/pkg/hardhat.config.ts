import fhevmPlugin from '@fhevm/hardhat-plugin-v3';
import hardhatToolboxViemPlugin from '@nomicfoundation/hardhat-toolbox-viem';
import { configVariable, defineConfig } from 'hardhat/config';

import templateTasks from './tasks/index.js';

export default defineConfig({
  plugins: [hardhatToolboxViemPlugin, fhevmPlugin, templateTasks],
  solidity: {
    profiles: {
      default: {
        version: '0.8.27',
      },
      production: {
        version: '0.8.27',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      },
    },
  },
  networks: {
    hardhatMainnet: {
      type: 'edr-simulated',
      chainType: 'l1',
    },
    localhost: {
      type: 'http',
      chainType: 'l1',
      chainId: 31337,
      url: 'http://127.0.0.1:8545',
    },
    sepolia: {
      type: 'http',
      chainType: 'l1',
      chainId: 11155111,
      url: configVariable('SEPOLIA_RPC_URL'),
      accounts: [configVariable('SEPOLIA_PRIVATE_KEY')],
    },
  },
});
