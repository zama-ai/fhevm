import { createSolanaRpc } from '@solana/kit';
import type { NativeClient } from '../../core/types/coreFhevmClient.js';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import { expectTypeOf } from 'vitest';
import { createCoreFhevm } from '../../core/runtime/CoreFhevm-p.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { asBytes32Hex } from '../../core/base/bytes.js';

const rpc = createSolanaRpc('http://localhost:8899');

const chain = {
  id: 9223372036854788153n,
  fhevm: {
    relayerUrl: 'http://localhost:3000',
    programs: { host: { address: asBytes32Hex(`0x${'22'.repeat(32)}`) } },
  },
} as const satisfies FhevmSolanaChain;

const client = createFhevmBaseClient({ rpc, chain });

expectTypeOf(client.chain).toEqualTypeOf<typeof chain>();
expectTypeOf(client.rpc).toEqualTypeOf<typeof rpc>();

// @ts-expect-error Hosted core construction cannot omit its declared chain and client.
createCoreFhevm<FhevmChain, FhevmRuntime, NativeClient>(Symbol(), { runtime: {} as FhevmRuntime });
