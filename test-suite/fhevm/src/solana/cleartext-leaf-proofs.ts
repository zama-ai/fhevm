// cleartext-leaf-proofs — the cleartext stack's stand-in for the host listener's leaf-proof endpoint:
// `/v1/solana/leaf-proofs` in the host-listener's wire format
// (`coprocessor/fhevm-engine/host-listener/openapi/solana_leaf_proofs.json`), so dapps fetch and
// verify proofs exactly as they do against the real stack. Each request rebuilds the store's
// history from the validator's transactions, so a proof always matches the chain as it is.
import { mmrBuildProof, reconstructSolanaStoreHistory } from '@fhevm/sdk/solana';
import { fetchSolanaStoreHistory } from '@fhevm/sdk/solana/cleartext';
import { createSolanaRpc, fetchEncodedAccount, getAddressDecoder } from '@solana/kit';

import { ZAMA_HOST_PROGRAM_ADDRESS } from '../../../../solana/deploy/src/generated/zamaHost/programAddress.js';
import { SOLANA_LEAF_PROOF_API_KEY } from '../generate/solana';

const LEAF_PROOFS_PATH = '/v1/solana/leaf-proofs';

type LeafQuery = { readonly encryptedStore?: unknown; readonly handle?: unknown; readonly kind?: unknown; readonly key?: unknown };
type LeafProof =
  | { status: 'found'; leafIndex: number; leafCount: number; siblings: string[] }
  | { status: 'notFound'; leafCount: number }
  | { status: 'unknownAccount' };

class MalformedQuery extends Error {}

const hex = (bytes: Uint8Array): string => Buffer.from(bytes).toString('hex');

const hex32 = (field: string, value: unknown): Uint8Array => {
  const digits = typeof value === 'string' ? value.replace(/^0x/, '') : '';
  if (!/^[0-9a-fA-F]{64}$/.test(digits)) throw new MalformedQuery(`${field}: expected 32 bytes as hex`);
  return Uint8Array.from(Buffer.from(digits, 'hex'));
};

const errorResponse = (status: number, code: string, message: string) =>
  Response.json({ code, message, retryable: false }, { status });

/** Serves leaf proofs for the host on the validator at `rpcUrl` until the server is stopped. */
export const serveCleartextLeafProofs = (rpcUrl: string, port: number) => {
  const rpc = createSolanaRpc(rpcUrl);

  const prove = async (query: LeafQuery): Promise<LeafProof> => {
    const storeBytes = hex32('encryptedStore', query.encryptedStore);
    const handle = hex(hex32('handle', query.handle));
    if (query.kind !== 'public' && query.kind !== 'allowed') throw new MalformedQuery('kind: expected public or allowed');
    if ((query.kind === 'allowed') !== (query.key !== undefined && query.key !== null)) {
      throw new MalformedQuery('an allowed leaf needs a key and a public leaf takes none');
    }
    const key = query.kind === 'allowed' ? hex(hex32('key', query.key)) : undefined;

    const store = getAddressDecoder().decode(storeBytes);
    const account = await fetchEncodedAccount(rpc, store, { commitment: 'confirmed' });
    if (!account.exists || account.programAddress !== ZAMA_HOST_PROGRAM_ADDRESS) return { status: 'unknownAccount' };
    const history = await fetchSolanaStoreHistory(rpc, store, ZAMA_HOST_PROGRAM_ADDRESS);
    // The first matching leaf, as the listener answers.
    const leafIndex = history.findIndex((event) =>
      event.kind === 'markedPublic'
        ? query.kind === 'public' && hex(event.handle) === handle
        : query.kind === 'allowed' && hex(event.handle) === handle && hex(event.key) === key,
    );
    if (leafIndex === -1) return { status: 'notFound', leafCount: history.length };
    const proof = mmrBuildProof(reconstructSolanaStoreHistory(storeBytes, history).leaves, BigInt(leafIndex));
    if (proof === undefined) throw new Error(`leaf ${leafIndex} is outside the history it was found in`);
    return { status: 'found', leafIndex, leafCount: history.length, siblings: proof.siblings.map(hex) };
  };

  return Bun.serve({
    hostname: '127.0.0.1',
    port,
    async fetch(request) {
      if (new URL(request.url).pathname !== LEAF_PROOFS_PATH || request.method !== 'POST') {
        return new Response('not found', { status: 404 });
      }
      if (request.headers.get('authorization') !== `Bearer ${SOLANA_LEAF_PROOF_API_KEY}`) {
        return errorResponse(401, 'sender_authentication_failed', 'missing or wrong bearer API key');
      }
      try {
        const body = (await request.json()) as { leaves?: readonly LeafQuery[] };
        if (!Array.isArray(body.leaves)) throw new MalformedQuery('leaves: expected an array');
        const proofs: LeafProof[] = [];
        for (const query of body.leaves) proofs.push(await prove(query));
        return Response.json({ proofs });
      } catch (error) {
        if (error instanceof MalformedQuery || error instanceof SyntaxError) {
          return errorResponse(400, 'malformed', error.message);
        }
        // A history that cannot be rebuilt is a bug here or in the host, not a retryable state.
        return errorResponse(500, 'upstream_transient', String(error));
      }
    },
  });
};
