// The access-proof client: one question to the proof service, in the service's own terms.
//
// The client asks by `(encrypted value account, handle, subject)` and never by leaf index — the
// index is the service's answer, and a client that supplied one would be claiming the very thing
// the proof exists to establish. Only a `verified: true` answer is accepted: that flag is the
// service's own comparison against live peaks, and a proof without it proves nothing worth handing
// to the request builder (which verifies again, against the peaks of the caller's own account
// read — the service's check does not replace that one, it just fails cheaper).
//
// One kind of answer is waited out here: `503` with a `lagging` body, the proof store catching up
// to the chain. The wait is bounded and belongs to this client rather than the session's retry
// loop — the session cannot re-run a failed evidence resolution, only a failed submission. Every
// other answer is terminal on the spot: a missing leaf or a corrupt cache retried into a timeout
// would be reported as slowness instead of what it is.
//
// The service speaks snake_case, unlike the relayer; this module is the one place that knows it.

import { getAddressDecoder } from '@solana/kit';
import { MAX_MMR_SIBLINGS, encodeMmrProof, hexToBytes, type MmrProof } from '../proof.js';
import { bytesToHexNo0x } from '../../core/base/bytes.js';
import { removeSuffix } from '../../core/base/string.js';

/** Where the standalone proof service lives, and how long a lagging store is waited out. */
export interface SolanaAccessProofServiceConfig {
  /** Base URL of the standalone solana-proof-service. */
  readonly proofServiceUrl: string;
  /** Bounded retries while the store reports `lagging`; default {@link SOLANA_ACCESS_PROOF_LAGGING_RETRIES}. */
  readonly laggingRetries?: number | undefined;
  /** Delay between `lagging` retries, in milliseconds; default {@link SOLANA_ACCESS_PROOF_LAGGING_DELAY_MS}. */
  readonly laggingDelayMs?: number | undefined;
}

/**
 * The lagging budget: about half a minute of catch-up, the tolerance the retired Rust live-client
 * had, at a poll twice as fast so a recovered store is picked up sooner.
 */
export const SOLANA_ACCESS_PROOF_LAGGING_RETRIES = 28;

/** The pause between lagging retries. */
export const SOLANA_ACCESS_PROOF_LAGGING_DELAY_MS = 1_000;

/** A verified historical-access proof, in both forms its consumers need. */
export interface SolanaHistoricalAccessProof {
  /** The decoded proof; `leafIndex` is the service's answer, not an input. */
  readonly proof: MmrProof;
  /** The bare borsh encoding — exactly what a request entry's `accessProof` carries. */
  readonly accessProof: Uint8Array;
  /** The leaf count the service built the proof against. */
  readonly leafCount: bigint;
}

/**
 * Fetches the historical-access proof for one handle an update has replaced.
 *
 * @param config - The service location and lagging budget.
 * @param query.encryptedValueAccount - The 32-byte pubkey of the `EncryptedValue` account itself —
 * the PDA, which is what the service indexes by, not the wire identity.
 * @param query.handle - The 32-byte handle whose past access is being proven.
 * @param query.subject - The 32-byte pubkey whose access it was.
 * @throws If the service answers with anything but a verified proof within the lagging budget.
 */
export async function fetchSolanaHistoricalAccessProof(
  config: SolanaAccessProofServiceConfig,
  query: { readonly encryptedValueAccount: Uint8Array; readonly handle: Uint8Array; readonly subject: Uint8Array },
): Promise<SolanaHistoricalAccessProof> {
  const address = getAddressDecoder();
  const url =
    `${removeSuffix(config.proofServiceUrl, '/')}/internal/solana/access-proof` +
    `?encrypted_value=${address.decode(query.encryptedValueAccount)}` +
    `&handle=${bytesToHexNo0x(query.handle)}` +
    `&subject=${address.decode(query.subject)}`;

  const laggingRetries = config.laggingRetries ?? SOLANA_ACCESS_PROOF_LAGGING_RETRIES;
  const laggingDelayMs = config.laggingDelayMs ?? SOLANA_ACCESS_PROOF_LAGGING_DELAY_MS;

  for (let attempt = 0; ; attempt += 1) {
    const response = await fetch(url, { method: 'GET', headers: { accept: 'application/json' } });
    const body: unknown = await response.json().catch(() => null);

    if (response.ok) {
      return parseAccessProof(body);
    }
    if (response.status === 503 && isLagging(body) && attempt < laggingRetries) {
      await delay(laggingDelayMs);
      continue;
    }
    const status = typeof body === 'object' && body !== null ? (body as { status?: string }).status : undefined;
    throw new Error(
      `the access-proof request failed (HTTP ${response.status}${status === undefined ? '' : `, status "${status}"`})${
        isLagging(body) ? ': the proof store is still lagging behind the chain' : ''
      }`,
    );
  }
}

/**
 * The one place that knows the service's wire shape.
 *
 * @param body - The parsed response body.
 */
function parseAccessProof(body: unknown): SolanaHistoricalAccessProof {
  if (typeof body !== 'object' || body === null || !('mmr_proof' in body)) {
    throw new Error('the proof-service response is not an MMR-proof envelope');
  }
  const wire = body as {
    readonly mmr_proof: { readonly leaf_index: number; readonly siblings: readonly string[] } | null;
    readonly leaf_count: number;
    readonly verified: boolean;
    readonly status?: string;
  };
  if (!wire.verified || wire.mmr_proof === null) {
    throw new Error(`the proof service returned an unverified access proof (status "${wire.status ?? '?'}")`);
  }
  const siblings = wire.mmr_proof.siblings.map((sibling) => {
    const bytes = hexToBytes(sibling);
    if (bytes.length !== 32) {
      throw new Error(`an access-proof sibling must be 32 bytes, got ${bytes.length}`);
    }
    return bytes;
  });
  if (siblings.length > MAX_MMR_SIBLINGS) {
    throw new Error(`the access proof carries ${siblings.length} siblings, above the cap of ${MAX_MMR_SIBLINGS}`);
  }
  const proof: MmrProof = { leafIndex: BigInt(wire.mmr_proof.leaf_index), siblings };
  return { proof, accessProof: encodeMmrProof(proof), leafCount: BigInt(wire.leaf_count) };
}

/**
 * True when the body is the store's "still catching up" answer — the one retryable case.
 *
 * @param body - The parsed response body.
 */
function isLagging(body: unknown): boolean {
  return typeof body === 'object' && body !== null && (body as { status?: string }).status === 'lagging';
}

/**
 * A plain wait. The session's clock port does not reach this layer — the lagging wait is the
 * client's own, not a retry the session scheduled.
 *
 * @param ms - Milliseconds to wait.
 */
function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
