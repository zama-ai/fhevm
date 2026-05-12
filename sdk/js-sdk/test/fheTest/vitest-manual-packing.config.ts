import { join } from 'node:path';
import { defineConfig } from 'vitest/config';

const chain = process.env.CHAIN ?? 'sepolia';

const PACKED_SDK = join(__dirname, '../standalone/packing/node_modules/@fhevm/sdk');

export default defineConfig({
  resolve: {
    alias: {
      //
      // WARNING!!!! Order matters! '<xxx>/cleartext' MUST BE LISTED BEFORE '<xxx>' !!!
      //
      '@fhevm/sdk/ethers/cleartext': join(PACKED_SDK, '_esm/ethers/cleartext/index.js'),
      '@fhevm/sdk/ethers': join(PACKED_SDK, '_esm/ethers/index.js'),
      '@fhevm/sdk/viem/cleartext': join(PACKED_SDK, '_esm/viem/cleartext/index.js'),
      '@fhevm/sdk/viem': join(PACKED_SDK, '_esm/viem/index.js'),

      '@fhevm/sdk/base': join(PACKED_SDK, '_esm/core/base/index.js'),
      '@fhevm/sdk/chains': join(PACKED_SDK, '_esm/core/chains/index.js'),
      '@fhevm/sdk/types': join(PACKED_SDK, '_esm/core/types/index.js'),
      '@fhevm/sdk/actions/base': join(PACKED_SDK, '_esm/core/actions/base/index.js'),
      '@fhevm/sdk/actions/chain': join(PACKED_SDK, '_esm/core/actions/chain/index.js'),
      '@fhevm/sdk/actions/decrypt': join(PACKED_SDK, '_esm/core/actions/decrypt/index.js'),
      '@fhevm/sdk/actions/encrypt': join(PACKED_SDK, '_esm/core/actions/encrypt/index.js'),
      '@fhevm/sdk/actions/host': join(PACKED_SDK, '_esm/core/actions/host/index.js'),
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
