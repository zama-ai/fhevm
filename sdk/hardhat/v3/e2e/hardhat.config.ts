import fhevmPlugin from '@fhevm/hardhat-plugin-v3';
import hardhatEthers from '@nomicfoundation/hardhat-ethers';
import hardhatEthersChaiMatchers from '@nomicfoundation/hardhat-ethers-chai-matchers';
import hardhatMocha from '@nomicfoundation/hardhat-mocha';
import hardhatTypechain from '@nomicfoundation/hardhat-typechain';
import { configVariable, defineConfig } from 'hardhat/config';

// The v2 suite's accounts: hardhat's own default mnemonic, spelled out so anvil and the in-process
// networks derive the same ten signers.
export const MNEMONIC = 'test test test test test test test test test test test junk';
export const HD_PATH = "m/44'/60'/0'/0/";

export default defineConfig({
  plugins: [fhevmPlugin, hardhatMocha, hardhatEthers, hardhatEthersChaiMatchers, hardhatTypechain],
  networks: {
    default: {
      type: 'edr-simulated',
      chainId: 31337,
      accounts: { mnemonic: MNEMONIC },
    },
    // `hardhat node` serves this network; same accounts as `default` so a suite runs unchanged on both.
    node: {
      type: 'edr-simulated',
      chainId: 31337,
      accounts: { mnemonic: MNEMONIC },
    },
    anvil: {
      type: 'http',
      chainId: 31337,
      url: 'http://localhost:8545',
      accounts: { mnemonic: MNEMONIC, path: HD_PATH, count: 10 },
    },
    sepolia: {
      type: 'http',
      chainId: 11155111,
      url: configVariable('SEPOLIA_RPC_URL'),
      accounts: { mnemonic: configVariable('MNEMONIC', { default: MNEMONIC }), path: HD_PATH, count: 10 },
    },
  },
  paths: {
    sources: './contracts',
    tests: { mocha: './test' },
  },
  solidity: {
    version: '0.8.27',
    settings: {
      metadata: {
        // Not including the metadata hash
        // https://github.com/paulrberg/hardhat-template/issues/31
        bytecodeHash: 'none',
      },
      optimizer: {
        enabled: true,
        runs: 800,
      },
      evmVersion: 'cancun',
    },
  },
});
