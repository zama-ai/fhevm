import type { Address } from '@solana/kit';

import { hexToBytes } from '../../../core/base/bytes.js';
import { removeSuffix } from '../../../core/base/string.js';
import { MAX_MMR_SIBLINGS, MMR_MODE_PUBLIC, type MmrProof } from '../../proof.js';

/**
 * Leg 1 of a settle: fetch the burned lineage's MMR inclusion proof from the standalone
 * solana-proof-service. The batcher's burned lineage always holds exactly one leaf (depth-0 proof,
 * empty siblings), but the proof is fetched from the service rather than synthesized locally so the
 * service's `verified: true` gate — a live on-chain peak comparison — is what stands behind it
 * (#1658 retired local proof synthesis).
 *
 * DTO ISOLATION (fhevm-internal#1828): the wire response is parsed in ONE place, {@link parseMmrProofResponse}.
 * #1828 renames the misnamed `proof_slot` (it is really the leaf count the proof was built against)
 * and adds chain-context fields; when it lands, this parser is the single edit. Everything else in
 * the vault module consumes the normalized {@link SolanaMmrProofResult}.
 */

/** Vault-module config for the proof-service leg. Isolated here, never threaded into the shared SDK config. */
export type SolanaProofServiceConfig = {
  /** Base URL of the standalone solana-proof-service (e.g. `http://localhost:8080`). */
  readonly proofServiceUrl: string;
  /** Bounded retries on a `lagging` (503) response while the store catches up to the chain. Default 10. */
  readonly maxRetries?: number | undefined;
  /** Delay between `lagging` retries, in milliseconds. Default 1000. */
  readonly retryDelayMs?: number | undefined;
};

/** The normalized proof leg, ready to feed the public-decrypt certificate request (leg 2). */
export type SolanaMmrProofResult = {
  /** The decoded inclusion proof (leaf index + sibling path). */
  readonly proof: MmrProof;
  /** The live leaf count of the lineage. */
  readonly leafCount: bigint;
  /** The lineage leaf count the proof was built against (the wire's `proof_slot`; see #1828). */
  readonly proofSlot: bigint;
  /** Canonical `0x02 || Borsh(MmrProof)` transport blob, attached verbatim to the certificate request. */
  readonly mmrProofBytes: Uint8Array;
};

// ---------------------------------------------------------------------------
// #1828 seam: the ONLY place that knows the current proof-service wire shape.
// ---------------------------------------------------------------------------

/** The current proof-service response shape: `{mmr_proof, leaf_count, proof_slot, verified, status}`. */
type MmrProofResponseWire = {
  readonly mmr_proof: { readonly leaf_index: number; readonly siblings: readonly string[] } | null;
  readonly leaf_count: number;
  readonly proof_slot: number;
  readonly verified: boolean;
  readonly status: string;
};

function parseMmrProofResponse(body: unknown): SolanaMmrProofResult {
  if (typeof body !== 'object' || body === null || !('mmr_proof' in body)) {
    throw new Error('proof-service response is not an MMR-proof envelope');
  }
  const wire = body as MmrProofResponseWire;
  if (!wire.verified || wire.mmr_proof === null) {
    throw new Error(`proof-service returned an unverified proof (status "${wire.status}")`);
  }
  const siblings = wire.mmr_proof.siblings.map((s) => {
    const bytes = hexToBytes(s);
    if (bytes.length !== 32) throw new Error(`proof-service sibling must be 32 bytes, got ${bytes.length}`);
    return bytes;
  });
  if (siblings.length > MAX_MMR_SIBLINGS) {
    throw new Error(
      `proof-service proof carries ${siblings.length} siblings, exceeding the cap of ${MAX_MMR_SIBLINGS}`,
    );
  }
  const proof: MmrProof = { leafIndex: BigInt(wire.mmr_proof.leaf_index), siblings };
  return {
    proof,
    leafCount: BigInt(wire.leaf_count),
    proofSlot: BigInt(wire.proof_slot),
    mmrProofBytes: encodeMmrProofTransportBlob(proof),
  };
}

/** `status` values that mean "retry later" versus "give up". Only `lagging` is retryable. */
function isLaggingStatus(body: unknown): boolean {
  return typeof body === 'object' && body !== null && (body as { status?: string }).status === 'lagging';
}

// ---------------------------------------------------------------------------

/** Encodes `0x02 || Borsh(MmrProof)`, the transport blob the certificate request and SDK decoder share. */
function encodeMmrProofTransportBlob(proof: MmrProof): Uint8Array {
  const out = new Uint8Array(1 + 8 + 4 + proof.siblings.length * 32);
  const view = new DataView(out.buffer);
  out[0] = MMR_MODE_PUBLIC;
  view.setBigUint64(1, proof.leafIndex, true);
  view.setUint32(9, proof.siblings.length, true);
  proof.siblings.forEach((sibling, i) => {
    out.set(sibling, 13 + i * 32);
  });
  return out;
}

const sleep = (ms: number): Promise<void> => new Promise((resolve) => setTimeout(resolve, ms));

/** Fetches and normalizes the burned lineage's public-decrypt MMR proof, retrying only on `lagging`. */
export async function fetchSolanaMmrProof(
  config: SolanaProofServiceConfig,
  encryptedValue: Address,
  leafIndex: bigint,
): Promise<SolanaMmrProofResult> {
  const maxRetries = config.maxRetries ?? 10;
  const retryDelayMs = config.retryDelayMs ?? 1000;
  const base = removeSuffix(config.proofServiceUrl, '/');
  const url = `${base}/internal/solana/mmr-proof?encrypted_value=${encryptedValue}&leaf_index=${leafIndex}`;

  for (let attempt = 0; ; attempt++) {
    const response = await fetch(url, { method: 'GET', headers: { accept: 'application/json' } });
    const body: unknown = await response.json().catch(() => null);

    if (response.ok) return parseMmrProofResponse(body);

    // 503 with a `lagging` body is the store catching up to the chain — bounded retry. Every other
    // status (corrupt cache / integrity 500, 4xx client errors) is terminal.
    if (response.status === 503 && isLaggingStatus(body) && attempt < maxRetries) {
      await sleep(retryDelayMs);
      continue;
    }
    const detail = (body as { status?: string; code?: string; error?: string } | null) ?? {};
    const statusNote = detail.status !== undefined ? `, status "${detail.status}"` : '';
    const codeNote = detail.code !== undefined ? `, code "${detail.code}"` : '';
    throw new Error(`proof-service request failed (HTTP ${response.status}${statusNote}${codeNote})`);
  }
}
