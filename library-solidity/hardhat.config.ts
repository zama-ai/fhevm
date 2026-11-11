import hardhatEthers from '@nomicfoundation/hardhat-ethers';
import hardhatTypechain from '@nomicfoundation/hardhat-typechain';
import hardhatUpgrades from '@openzeppelin/hardhat-upgrades';
import { defineConfig, overrideTask, task } from 'hardhat/config';

import { isFhevmTempPrepared } from './tasks/hostContracts.js';

// Well-known test mnemonic (HH2's DEFAULT_MNEMONIC, .env.example's MNEMONIC). Never use it to hold real funds.
// The host contract deployer (DEPLOYER_PRIVATE_KEY in .env.example) is its account 5: it must be funded on the
// in-process network, and the host contract addresses (and config/ZamaConfig.sol) depend on it.
const TEST_MNEMONIC =
  'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';

export default defineConfig({
  plugins: [hardhatEthers, hardhatTypechain, hardhatUpgrades],
  networks: {
    // The in-process network, with HH2's accounts.
    default: {
      type: 'edr-simulated',
      accounts: { mnemonic: TEST_MNEMONIC, path: "m/44'/60'/0'/0", count: 20 },
    },
  },
  typechain: {
    // HH3's default is `types/`. Keep HH2's folder: the tests, the codegen-generated tests
    // (codegen.config.json), `clean`, .gitignore and ESLint all use `typechain-types/`.
    outDir: 'typechain-types',
  },
  tasks: [
    task('task:networkInfo', 'Prints the chain id, the signers and the first signer balance of a network')
      .setAction(() => import('./tasks/networkInfo.js'))
      .build(),
    task('task:compileAll', 'Prepares fhevmTemp/ (host contracts and addresses), then compiles every source folder')
      .setAction(() => import('./tasks/compileAll.js'))
      .build(),
    task(
      'task:prepareTestContracts',
      'Compiles everything, deploys the host contracts and adds the pausers, on the in-process chain of the tests',
    )
      .setAction(() => import('./tasks/prepareTestContracts.js'))
      .build(),
    // A full HH3 build deletes the artifacts of every file outside `paths.sources`, so every folder the tests use is
    // a source folder. fhevmTemp/ must be prepared first (task:compileAll): fail loudly instead of with an unclear
    // "file doesn't exist" error. `run` and `test` also go through `build`.
    overrideTask('build')
      .setInlineAction(async (taskArguments, _hre, runSuper) => {
        if (!isFhevmTempPrepared()) {
          throw new Error(
            'fhevmTemp/ is not prepared: `hardhat build` cannot compile fhevmTemp/contracts and examples/.\n' +
              'Run `npm run build` (or `npx hardhat task:compileAll`) instead.',
          );
        }
        // Pass the build result through unchanged: callers such as `test` may use it.
        const buildResult: unknown = await runSuper(taskArguments);
        return buildResult;
      })
      .build(),
  ],
  paths: {
    sources: [
      // Everything published in @fhevm/solidity (package.json.publish.template: lib/**/*.sol, config/**/*.sol).
      './lib',
      './config',
      // Host contracts copied from @fhevm/host-contracts by task:compileAll (git-ignored).
      './fhevmTemp/contracts',
      // Example contracts, including the codegen-generated test suites (examples/tests).
      './examples',
    ],
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
