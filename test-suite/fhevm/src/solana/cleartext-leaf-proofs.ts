// cleartext-leaf-proofs — the cleartext stack's stand-in for the host listener's leaf-proof endpoint:
// `/v1/solana/leaf-proofs` in the host-listener's wire format
// (`coprocessor/fhevm-engine/host-listener/openapi/solana_leaf_proofs.json`), answered from the
// SDK's leaf record of the validator (`createSolanaLeafRecord`), which lives as long as the server.
import {
  createSolanaLeafRecord,
  decodeSolanaLeafQuery,
  encodeSolanaLeafProofOutcome,
  SOLANA_LEAF_PROOFS_PATH,
  SOLANA_MAX_LEAVES_PER_READ,
} from '@fhevm/sdk/solana/cleartext';
import { createSolanaRpc } from '@solana/kit';

import { ZAMA_HOST_PROGRAM_ADDRESS } from '../../../../solana/deploy/src/generated/zamaHost/programAddress.js';
import { SOLANA_LEAF_PROOF_API_KEY } from '../generate/solana';

const errorResponse = (status: number, code: string, message: string, retryable = false) =>
  Response.json({ code, message, retryable }, { status });

/** Serves leaf proofs for the host on the validator at `rpcUrl` until the server is stopped. */
export const serveCleartextLeafProofs = (rpcUrl: string, port: number) => {
  const readLeafProofs = createSolanaLeafRecord(createSolanaRpc(rpcUrl), ZAMA_HOST_PROGRAM_ADDRESS);

  return Bun.serve({
    hostname: '127.0.0.1',
    port,
    async fetch(request) {
      if (new URL(request.url).pathname !== SOLANA_LEAF_PROOFS_PATH || request.method !== 'POST') {
        return new Response('not found', { status: 404 });
      }
      if (request.headers.get('authorization') !== `Bearer ${SOLANA_LEAF_PROOF_API_KEY}`) {
        return errorResponse(401, 'sender_authentication_failed', 'missing or wrong bearer API key');
      }
      let queries;
      try {
        const body = (await request.json()) as { leaves?: unknown };
        if (!Array.isArray(body.leaves)) throw new Error('leaves: expected an array');
        if (body.leaves.length > SOLANA_MAX_LEAVES_PER_READ) {
          throw new Error(`leaves: at most ${SOLANA_MAX_LEAVES_PER_READ} per request`);
        }
        queries = body.leaves.map(decodeSolanaLeafQuery);
      } catch (error) {
        return errorResponse(400, 'malformed', error instanceof Error ? error.message : String(error));
      }
      try {
        const outcomes = await readLeafProofs(queries);
        return Response.json({ proofs: outcomes.map(encodeSolanaLeafProofOutcome) });
      } catch (error) {
        // The record drops a store whose read failed, so the next request rebuilds it.
        return errorResponse(502, 'upstream_transient', String(error), true);
      }
    },
  });
};
