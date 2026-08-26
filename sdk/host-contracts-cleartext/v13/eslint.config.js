// DEFAULT config — the one `eslint` discovers with no --config, so it is also what editors use.
// It must therefore work on a bare checkout, which means ignoring the `test/ts` files that import the
// package by its PUBLISHED name: those resolve only after `npm run prepare:tarball-consumer` has
// installed the tarball fixture. They are linted by `lint:tarball-consumer` instead.
//
// Three configs, one job each:
//   eslint.config.base.js                  the shared rule set (never run directly)
//   eslint.config.js                       base + ignores — the DEFAULT, safe on a bare checkout
//   eslint.config.with-tarball-consumer.js base as-is — the stricter gate, needs the fixture
import baseConfig from './eslint.config.base.js';

export default [
  ...baseConfig,
  {
    ignores: [
      'test/ts/adapter-nonce-diagnostics.test.ts',
      'test/ts/precompute-addresses.test.ts',
      'test/ts/ethers-adapter.test.ts',
      'test/ts/utils/ethersEthereumLib.ts',
      'test/ts/tarball-consumer.test.ts',
      'test/ts/acl-owner-upgrade.test.ts',
      'test/ts/deploy-v13.test.ts',
      'test/ts/create2-precompute.test.ts',
      'test/ts/fhe-rand.test.ts',
      'test/ts/define-kms-context.test.ts',
      'test/ts/destroy-kms-context.test.ts',
      'test/ts/utils/viemEthereumLib.ts',
      'test/ts/utils/ethUtils.ts',
      'test/ts/utils/deployStack.ts',
    ],
  },
];
