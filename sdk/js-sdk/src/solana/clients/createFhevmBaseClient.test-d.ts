import type { NativeClient } from '../../core/types/coreFhevmClient.js';
import type { FhevmChain } from '../../core/types/fhevmChain.js';
import type { FhevmRuntime } from '../../core/types/coreFhevmRuntime.js';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import { expectTypeOf } from 'vitest';
import { asBytes32Hex } from '../../core/base/bytes.js';
import { createCoreFhevm } from '../../core/runtime/CoreFhevm-p.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';

const chain = {
  id: 9223372036854788153n,
  fhevm: {
    relayerUrl: 'http://localhost:3000',
    acl: { domainKeys: [asBytes32Hex('0x1111111111111111111111111111111111111111111111111111111111111111')] },
  },
} as const satisfies FhevmSolanaChain;

const client = createFhevmBaseClient({ chain });

expectTypeOf(client.chain).toEqualTypeOf<undefined>();
expectTypeOf(client.client).toEqualTypeOf<undefined>();
expectTypeOf(client.solanaChain).toEqualTypeOf<typeof chain>();

// @ts-expect-error Hosted core construction cannot omit its declared chain and client.
createCoreFhevm<FhevmChain, FhevmRuntime, NativeClient>(Symbol(), { runtime: {} as FhevmRuntime });
