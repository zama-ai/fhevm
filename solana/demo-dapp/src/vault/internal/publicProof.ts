import { bytesToHex, hexToBytes } from '@fhevm/sdk/base';
import { type Address } from '@solana/kit';
import { base58 } from '@scure/base';
import type { FhevmSolanaBaseClient } from '@fhevm/sdk/solana';
import { verifyPublicDecryptProof, type MmrProof } from '@fhevm/sdk/solana';

export type ProofService = { readonly url: string; readonly apiKey: string };

/** Fetches an untrusted proof and verifies it against a fresh on-chain state snapshot. */
export async function publicProof(
  client: Pick<FhevmSolanaBaseClient, 'fetchEncryptedStore'>, service: ProofService, state: Address, handle: Uint8Array,
): Promise<MmrProof> {
  const deadline = Date.now() + 15_000;
  do {
    const response = await fetch(`${service.url}/v1/solana/leaf-proofs`, {
      method: 'POST', headers: { 'content-type': 'application/json', authorization: `Bearer ${service.apiKey}` },
      body: JSON.stringify({ leaves: [{ encryptedStore: bytesToHex(base58.decode(state)), handle: bytesToHex(handle), kind: 'public' }] }),
      signal: AbortSignal.timeout(5_000),
    });
    if (!response.ok) throw new Error(`public listener proof endpoint returned HTTP ${response.status}`);
    const body = await response.json() as { proofs?: { status: string; leafIndex?: number; siblings?: string[] }[] };
    const answer = body.proofs?.[0];
    if (answer?.status === 'found') {
      if (!Number.isSafeInteger(answer.leafIndex) || answer.leafIndex! < 0 || !Array.isArray(answer.siblings)) {
        throw new Error('public listener proof endpoint returned a malformed proof');
      }
      const proof = { leafIndex: BigInt(answer.leafIndex!), siblings: answer.siblings.map(hexToBytes) };
      const live = await client.fetchEncryptedStore(state, { commitment: 'confirmed' });
      if (verifyPublicDecryptProof(base58.decode(state), live.peaks, live.leafCount, handle, proof)) return proof;
    } else if (answer?.status !== 'notFound' && answer?.status !== 'unknownAccount') {
      throw new Error(`public proof unavailable: ${answer?.status ?? 'missing response'}`);
    }
    await new Promise(resolve => setTimeout(resolve, 250));
  } while (Date.now() < deadline);
  throw new Error('public proof did not catch up to the on-chain state within 15 seconds');
}
