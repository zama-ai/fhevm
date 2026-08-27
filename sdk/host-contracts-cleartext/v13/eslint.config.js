import eslintCleartext from '../eslint.cleartext.mjs';

export default [
  ...eslintCleartext(import.meta.dirname),
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
