import { join } from 'node:path';
import { defineConfig } from 'vitest/config';

const chain = process.env.CHAIN ?? 'sepolia';

const PACKED_SDK = join(__dirname, '../manual-pack/node_modules/@fhevm/sdk');
const getfile = (path: string) => join(PACKED_SDK, '_esm', path, 'index.js');

export default defineConfig({
  resolve: {
    alias: {
      //
      // WARNING!!!! Order matters! '<xxx>/cleartext' MUST BE LISTED BEFORE '<xxx>' !!!
      //
      '@fhevm/sdk/ethers/cleartext': getfile('ethers/cleartext'),
      '@fhevm/sdk/ethers': getfile('ethers'),
      '@fhevm/sdk/viem/cleartext': getfile('viem/cleartext'),
      '@fhevm/sdk/viem': getfile('viem'),

      '@fhevm/sdk/base': getfile('core/base'),
      '@fhevm/sdk/chains': getfile('core/chains'),
      '@fhevm/sdk/types': getfile('core/types'),
      '@fhevm/sdk/actions/base': getfile('core/actions/base'),
      '@fhevm/sdk/actions/chain': getfile('core/actions/chain'),
      '@fhevm/sdk/actions/decrypt': getfile('core/actions/decrypt'),
      '@fhevm/sdk/actions/encrypt': getfile('core/actions/encrypt'),
      '@fhevm/sdk/actions/host': getfile('core/actions/host'),
    },
  },
  test: {
    include: ['test/fheTest/**/*.test.ts'],
    testTimeout: 120_000,
    hookTimeout: 120_000,
    retry: 0,
    fileParallelism: false,
    env: {
      CHAIN: chain,
    },
  },
});
