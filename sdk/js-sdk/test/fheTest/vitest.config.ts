import { join } from 'node:path';
import { defineConfig } from 'vitest/config';

const chain = process.env.CHAIN ?? 'sepolia';

export default defineConfig({
  resolve: {
    alias: {
      '@fhevm/sdk/ethers': join(__dirname, '../../src/ethers/index.ts'),
      '@fhevm/sdk/viem': join(__dirname, '../../src/viem/index.ts'),
      '@fhevm/sdk/chains': join(__dirname, '../../src/core/chains/index.ts'),
      '@fhevm/sdk/actions/base': join(__dirname, '../../src/core/actions/base/index.ts'),
      '@fhevm/sdk/actions/chain': join(__dirname, '../../src/core/actions/chain/index.ts'),
      '@fhevm/sdk/actions/decrypt': join(__dirname, '../../src/core/actions/decrypt/index.ts'),
      '@fhevm/sdk/actions/encrypt': join(__dirname, '../../src/core/actions/encrypt/index.ts'),
      '@fhevm/sdk/actions/host': join(__dirname, '../../src/core/actions/host/index.ts'),
    },
  },
  test: {
    include: ['test/fheTest/**/*.test.ts'],
    testTimeout: 120_000,
    hookTimeout: 120_000,
    retry: 0,
    env: {
      CHAIN: chain,
    },
  },
});
