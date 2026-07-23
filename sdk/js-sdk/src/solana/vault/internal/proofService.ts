import type { Address } from '@solana/kit';

import { bytesToHex, hexToBytes } from '../../../core/base/bytes.js';
import { removeSuffix } from '../../../core/base/string.js';
import { MAX_MMR_SIBLINGS, MMR_MODE_PUBLIC, type MmrProof } from '../../proof.js';

/**
 * Leg 1 of a settle: fetch the burned lineage's public-decrypt inclusion proof from the standalone
 * solana-proof-service. The service resolves the leaf semantically from `(encrypted_value, handle)`
 * — the SDK asks the product question ("prove this handle is publicly decryptable") and never
 * computes, assumes, or supplies a leaf index (fhevm-internal#1721). The leaf index and leaf count
 * come back as OUTPUTS the SDK passes through; the proof is fetched from the service rather than
 * synthesized locally so the service's `verified: true` gate — a live on-chain peak comparison —
 * is what stands behind it (#1658 retired local proof synthesis).
 *
 * DTO ISOLATION: the wire response is parsed in ONE place, {@link parseMmrProofResponse}.
 * Everything else in the vault module consumes the normalized {@link SolanaMmrProofResult}.
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
  /** The decoded inclusion proof (leaf index + sibling path); `leafIndex` is a service output. */
  readonly proof: MmrProof;
  /** The lineage leaf count the service built the proof against (a service output, not an input). */
  readonly leafCount: bigint;
  /** Canonical `0x02 || Borsh(MmrProof)` transport blob, attached verbatim to the certificate request. */
  readonly mmrProofBytes: Uint8Array;
};

// ---------------------------------------------------------------------------
// The ONLY place that knows the proof-service wire shape.
// ---------------------------------------------------------------------------

/**
 * The proof-service response shape for the semantic endpoints:
 * `{mmr_proof, leaf_index?, leaf_count, rpc_context_slot?, verified, status, ...}`.
 * Chain-context fields (`rpc_context_slot`, `lineage_last_slot`, `commitment`, `proof_format_version`)
 * are carried on the wire but not consumed here.
 */
type MmrProofResponseWire = {
  readonly mmr_proof: { readonly leaf_index: number; readonly siblings: readonly string[] } | null;
  readonly leaf_count: number;
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
  // leafIndex is the service's resolved output — read from the proof, never supplied by the SDK.
  const proof: MmrProof = { leafIndex: BigInt(wire.mmr_proof.leaf_index), siblings };
  return {
    proof,
    leafCount: BigInt(wire.leaf_count),
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

/**
 * Fetches and normalizes the burned lineage's public-decrypt inclusion proof for `handle`, retrying
 * only on `lagging`. The service resolves the leaf from `(encryptedValue, handle)`; the SDK supplies
 * no leaf index.
 */
export async function fetchSolanaPublicDecryptProof(
  config: SolanaProofServiceConfig,
  encryptedValue: Address,
  handle: Uint8Array,
): Promise<SolanaMmrProofResult> {
  const maxRetries = config.maxRetries ?? 10;
  const retryDelayMs = config.retryDelayMs ?? 1000;
  const base = removeSuffix(config.proofServiceUrl, '/');
  const url = `${base}/internal/solana/public-proof?encrypted_value=${encryptedValue}&handle=${bytesToHex(handle)}`;

  for (let attempt = 0; ; attempt++) {
    const response = await fetch(url, { method: 'GET', headers: { accept: 'application/json' } });
    const body: unknown = await response.json().catch(() => null);

    if (response.ok) return parseMmrProofResponse(body);

    // 503 with a `lagging` body is the store catching up to the chain — bounded retry. Every other
    // status (leaf_not_found / lineage_not_found 404, corrupt cache / integrity 500, 4xx client
    // errors) is terminal.
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
