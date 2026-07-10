import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { NativeClient } from '../../core/types/coreFhevmClient.js';
import { describe, expect, expectTypeOf, it } from 'vitest';
import { asBytes32Hex } from '../../core/base/bytes.js';
import { createCoreFhevm } from '../../core/runtime/CoreFhevm-p.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';

const chain = {
  id: 9223372036854788153n,
  fhevm: {
    relayerUrl: 'http://localhost:3000',
    acl: { domainKeys: [asBytes32Hex('0x1111111111111111111111111111111111111111111111111111111111111111')] },
  },
} as const satisfies FhevmSolanaChain;

describe('createFhevmBaseClient', () => {
  it('requires chain and client for explicit hosted core types', () => {
    if (false) {
      // @ts-expect-error Hosted core construction cannot omit its declared chain and client.
      createCoreFhevm<FhevmChain, FhevmRuntime, NativeClient>(Symbol(), { runtime: {} as FhevmRuntime });
    }
  });

  it('keeps the exact Solana chain while leaving EVM chain and native client absent', async () => {
    setFhevmRuntimeConfig({});

    const client = createFhevmBaseClient({ chain });

    expectTypeOf(client.chain).toEqualTypeOf<undefined>();
    expectTypeOf(client.client).toEqualTypeOf<undefined>();
    const inferredChain: typeof chain = client.solanaChain;
    expect(client.chain).toBeUndefined();
    expect(client.client).toBeUndefined();
    expect(client.solanaChain).toBe(chain);
    expect(inferredChain.id).toBe(9223372036854788153n);
    await expect(client.ready).resolves.toBeUndefined();
  });
});
