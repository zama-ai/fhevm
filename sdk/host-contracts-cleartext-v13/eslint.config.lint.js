import baseConfig from './eslint.config.js';

// Config for the standalone `lint` script, which runs on a bare checkout (no tarball consumer fixture
// installed). It is the base config plus extra ignores for the `test/ts` files that import the
// not-yet-built `@fhevm/host-contracts-cleartext` consumer package: the type-aware project service
// can't resolve those imports without the fixture, so linting them here would error. They ARE linted
// by `lint:tarball-consumer` (which uses the base config) once the consumer has been prepped.
export default [
  ...baseConfig,
  {
    ignores: [
      'test/ts/tarball-consumer.test.ts',
      'test/ts/acl-owner-upgrade.test.ts',
      'test/ts/deploy-v13.test.ts',
      'test/ts/define-kms-context.test.ts',
      'test/ts/destroy-kms-context.test.ts',
      'test/ts/viemEthereumLib.ts',
      'test/ts/ethUtils.ts',
    ],
  },
];
