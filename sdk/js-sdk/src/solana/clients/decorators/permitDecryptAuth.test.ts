// Runtime relayer auth on the permit path.
//
// This lives in its own file because the runtime config is set-once per module instance, and these
// tests need it set WITH auth — unlike everything else on the decorator, which runs authless.
//
// The one observable seam without a network: auth over plain http to a non-localhost relayer is
// refused at transport construction. Runtime-configured auth reaching that refusal proves it is
// merged into every user-decrypt submission — the same merge the public-decrypt action runs — and a
// per-call option overriding it proves the merge's precedence.

import type { FhevmSolanaChain } from '../../../core/types/fhevmSolanaChain.js';
import type { SolanaPermitSession } from '../../userDecrypt/index.js';
import { beforeAll, describe, expect, it } from 'vitest';
import { asBytes32Hex } from '../../../core/base/bytes.js';
import { createFhevmDecryptClient } from '../createFhevmDecryptClient.js';
import { setFhevmRuntimeConfig } from '../../internal/config.js';

const chain = {
  id: 9223372036854788153n,
  fhevm: {
    relayerUrl: 'http://relayer.local',
    acl: { domainKeys: [asBytes32Hex(`0x${'11'.repeat(32)}`)] },
    rpcUrl: 'http://rpc.local',
    proofServiceUrl: 'http://proofs.local',
    verifyingProgramId: asBytes32Hex(`0x${'22'.repeat(32)}`),
  },
} as const satisfies FhevmSolanaChain;

const trust = {
  kmsSigners: [{ partyId: 1, address: '0x0000000000000000000000000000000000000001' }],
  kmsContextId: asBytes32Hex(`0x${'33'.repeat(32)}`),
  kmsEpochId: asBytes32Hex(`0x${'44'.repeat(32)}`),
  fheParameter: 'test',
};

/** Just enough session for `userDecrypt` to reach the transport; nothing here is ever signed. */
const session = {
  signedPermit: { fields: { userPubkey: new Uint8Array(32).fill(0x07) } },
} as unknown as SolanaPermitSession;

beforeAll(() => {
  setFhevmRuntimeConfig({ auth: { type: 'ApiKeyHeader', value: 'runtime-key' } });
});

describe('relayer authentication on the permit path', () => {
  it('carries the runtime-configured auth into the user-decrypt transport', async () => {
    const client = createFhevmDecryptClient({ chain, trust });

    await expect(client.userDecrypt({ session, entries: [] })).rejects.toThrow(
      'HTTPS is required when auth credentials are provided',
    );
  });

  it('lets a per-call option override the runtime auth', async () => {
    const client = createFhevmDecryptClient({ chain, trust });

    // With auth overridden away, the http URL is admissible again and the run proceeds past the
    // transport to the next refusal — the empty handle list.
    await expect(client.userDecrypt({ session, entries: [], options: { auth: undefined } })).rejects.toThrow(
      'at least one handle',
    );
  });
});
