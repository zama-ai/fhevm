import hardhatEthers from '@nomicfoundation/hardhat-ethers';
import { defineConfig, task } from 'hardhat/config';

export default defineConfig({
  plugins: [hardhatEthers],
  tasks: [
    task('task:networkInfo', 'Prints the chain id, the signers and the first signer balance of a network')
      .setAction(() => import('./tasks/networkInfo.js'))
      .build(),
  ],
  paths: {
    // Everything published in @fhevm/solidity (package.json.publish.template: lib/**/*.sol, config/**/*.sol).
    sources: ['./lib', './config'],
    tests: {
      // The Foundry tests in test/*.t.sol run with `forge test`, not Hardhat. This folder does not exist, so
      // Hardhat finds no Solidity tests to build.
      solidity: './hardhat-solidity-tests',
    },
  },
  solidity: {
    version: '0.8.24',
    settings: {
      evmVersion: 'cancun',
      metadata: {
        // Not including the metadata hash
        // https://github.com/paulrberg/hardhat-template/issues/31
        bytecodeHash: 'none',
      },
      optimizer: {
        enabled: true,
        runs: 800,
      },
    },
  },
});
