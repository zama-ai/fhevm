import hardhatEthers from '@nomicfoundation/hardhat-ethers';
import hardhatEthersChaiMatchers from '@nomicfoundation/hardhat-ethers-chai-matchers';
import hardhatMocha from '@nomicfoundation/hardhat-mocha';
import hardhatTypechain from '@nomicfoundation/hardhat-typechain';
import hardhatUpgrades from '@openzeppelin/hardhat-upgrades';
import { defineConfig, overrideTask, task } from 'hardhat/config';
import type { HardhatPlugin } from 'hardhat/types/plugins';

import { isFhevmTempPrepared } from './tasks/hostContracts.js';

// Project-local plugin: the network hook the tests need (HH2: extendProvider(CustomProvider)).
const fhevmTestNetworkPlugin: HardhatPlugin = {
  id: 'fhevm-library-solidity-test-network',
  npmPackage: null,
  hookHandlers: {
    network: () => import('./test/networkHooks.js'),
  },
};

// Well-known test mnemonic (HH2's DEFAULT_MNEMONIC). Never use it to hold real funds.
// The host contract deployer (DEPLOYER_PRIVATE_KEY in .env.example) is its account 5: it must be funded on the
// in-process network, and the host contract addresses (and config/ZamaConfig.sol) depend on it.
const TEST_MNEMONIC =
  'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer';

export default defineConfig({
  plugins: [
    hardhatEthers,
    hardhatEthersChaiMatchers,
    hardhatMocha,
    hardhatTypechain,
    hardhatUpgrades,
    fhevmTestNetworkPlugin,
  ],
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
    // `hardhat test` builds before running its subtasks: prepare fhevmTemp/ first, as `npm run build` does (on a
    // clean checkout). The host contracts are then deployed by the `test mocha` override below.
    //
    // Gas stats (`--gas-stats`): run `hardhat task:compileAll` in a separate process first (`npm run test:gas-stats`).
    // HH3 builds the contract decoder that maps gas measurements to contracts once per process, from the artifacts on
    // disk when the first network connection opens. Here that connection necessarily opens before the full build (the
    // empty host proxies are deployed to get the addresses the host contracts and examples are compiled with), so on a
    // clean or stale tree the decoder misses or misreads those contracts and the gas tables are empty or wrong.
    overrideTask('test')
      .setInlineAction(async (taskArguments, hre, runSuper) => {
        if (!isFhevmTempPrepared()) {
          await hre.tasks.getTask('task:compileAll').run({});
        }
        const testResult: unknown = await runSuper(taskArguments);
        return testResult;
      })
      .build(),
    // The mocha tests run in this process (not in parallel mode): deploy the host contracts first, on the
    // `network.getOrCreate()` connection the tests share (HH2: the `test` task ran task:prepareTestContracts first).
    overrideTask(['test', 'mocha'])
      .setInlineAction(async (taskArguments, hre, runSuper) => {
        await hre.tasks.getTask('task:prepareTestContracts').run({});
        const testResult: unknown = await runSuper(taskArguments);
        return testResult;
      })
      .build(),
  ],
  test: {
    mocha: {
      // As in HH2: the mocked coprocessor and some tests are slow.
      timeout: 500000,
    },
  },
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
      // The Foundry tests in test_forge/ run with `forge test`, not Hardhat. This folder does not exist, so
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
