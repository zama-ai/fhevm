import { expect } from 'chai';

/**
 * Client-side corruption of KMS signcrypted shares for user decryption.
 *
 * WHY THIS EXISTS
 * ---------------
 * We want to observe how the SDK behaves when it receives corrupted shares
 * (see the RFC-016 discussion: "what concrete errors are reported by the wasm
 * today"). For user decryption the relayer only *collects and forwards* the
 * KMS parties' signcrypted shares — reconstruction happens client-side inside
 * the SDK's wasm (tkms) module. So the corruption has to be injected between
 * the relayer HTTP response and the SDK's reconstruction step.
 *
 * HOW
 * ---
 * We intercept `globalThis.fetch` for the duration of a single decrypt call.
 * Both `@fhevm/sdk` and `@zama-fhe/relayer-sdk` call the bare global `fetch`
 * dynamically at request time, so one interception in the test process catches
 * either SDK. The poll (GET) response for every user-decrypt endpoint
 * (`/user-decrypt`, `/v2/user-decrypt`, `/v3/user-decrypt`,
 * `/delegated-user-decrypt`, and their `/{jobId}` poll URLs) carries the same
 * wire shape:
 *
 *   { status: "succeeded", requestId, result: { result: [ { payload, signature, extraData } ] } }
 *
 * We mutate a fixed number of those shares and hand the SDK a corrupted-but-
 * well-formed response.
 *
 * SCOPE / SAFETY
 * --------------
 * The patch is installed and restored around a single callback (`try/finally`),
 * so only the wrapped decrypt call is affected and the original `fetch` is
 * always put back — even when the decrypt throws (the expected outcome here).
 * Only responses whose URL contains `user-decrypt` and that actually carry a
 * `result.result[]` share array are touched; everything else passes through.
 *
 * BUILDING ON THIS LATER
 * ----------------------
 * The corruption itself is a pluggable `ShareCorruptor`. Today we ship the two
 * naive strategies from the RFC-016 thread (bit-flip the payload; corrupt the
 * signature). The systematic cases discussed there — wrong signature, correct
 * sig but wrong key, correct key but wrong share — slot in as additional
 * `ShareCorruptor` implementations without touching the interception plumbing.
 */

/** One signcrypted share as it appears on the wire (hex, no `0x` prefix). */
export interface WireShare {
  payload: string;
  signature: string;
  extraData?: string;
  [key: string]: unknown;
}

/** Transforms a single share into a corrupted one. Pure; returns a new object. */
export type ShareCorruptor = (share: WireShare, index: number) => WireShare;

/** Number of shares corrupted by default (see the "2 out of 9" RFC-016 case). */
export const DEFAULT_CORRUPT_COUNT = 2;

/** Flip every bit of the first byte of a hex-no-0x string (guaranteed change). */
export function flipFirstByte(hex: string): string {
  if (hex.length < 2) {
    return hex;
  }
  const firstByte = parseInt(hex.slice(0, 2), 16);
  const flipped = ((firstByte ^ 0xff) & 0xff).toString(16).padStart(2, '0');
  return flipped + hex.slice(2);
}

/** Case 1: bit-flip the signcrypted payload. */
export const bitFlipPayload: ShareCorruptor = (share) => ({
  ...share,
  payload: flipFirstByte(share.payload),
});

/** Case 2: corrupt the KMS party's signature (length preserved, so it clears
 * the SDK's 65-byte length guard and reaches wasm signature verification). */
export const corruptSignature: ShareCorruptor = (share) => ({
  ...share,
  signature: flipFirstByte(share.signature),
});

/** Locate the shares array in a parsed relayer response, if present. */
function findShares(body: unknown): WireShare[] | undefined {
  const result = (body as { result?: { result?: unknown } } | null)?.result?.result;
  if (Array.isArray(result) && result.length > 0 && typeof (result[0] as WireShare)?.payload === 'string') {
    return result as WireShare[];
  }
  return undefined;
}

/** Rebuild a Response from consumed text, dropping length/encoding headers. */
function rebuildResponse(original: Response, text: string): Response {
  const headers = new Headers();
  original.headers.forEach((value, key) => {
    const lower = key.toLowerCase();
    if (lower === 'content-length' || lower === 'content-encoding') {
      return;
    }
    headers.set(key, value);
  });
  return new Response(text, {
    status: original.status,
    statusText: original.statusText,
    headers,
  });
}

export interface CorruptionOptions {
  /** How to corrupt each targeted share. */
  corrupt: ShareCorruptor;
  /** How many shares to corrupt (default {@link DEFAULT_CORRUPT_COUNT}). */
  count?: number;
}

/**
 * Run `fn` with `globalThis.fetch` patched so that user-decrypt share responses
 * are corrupted before the SDK reconstructs them. Restores `fetch` afterwards.
 */
export async function withCorruptedUserDecryptShares<T>(options: CorruptionOptions, fn: () => Promise<T>): Promise<T> {
  const { corrupt } = options;
  const count = options.count ?? DEFAULT_CORRUPT_COUNT;
  const realFetch = globalThis.fetch;

  const patched: typeof fetch = async (input, init) => {
    const response = await realFetch(input, init);
    const url = input instanceof Request ? input.url : String(input);
    if (!url.includes('user-decrypt')) {
      return response;
    }

    const text = await response.text();
    let body: unknown;
    try {
      body = JSON.parse(text);
    } catch {
      // Not JSON (e.g. an error page) — hand back the original body untouched.
      return rebuildResponse(response, text);
    }

    const shares = findShares(body);
    if (shares === undefined) {
      // Submit (202) / still-processing polls carry no shares — pass through.
      return rebuildResponse(response, text);
    }

    const corruptCount = Math.min(count, shares.length);
    // eslint-disable-next-line no-console
    console.log(`[corruption] user-decrypt response: ${shares.length} shares received, corrupting ${corruptCount}`);
    for (let i = 0; i < corruptCount; i++) {
      shares[i] = corrupt(shares[i], i);
    }

    return rebuildResponse(response, JSON.stringify(body));
  };

  globalThis.fetch = patched;
  try {
    return await fn();
  } finally {
    globalThis.fetch = realFetch;
  }
}

/**
 * Run a decrypt with corrupted shares, print the resulting error, and assert
 * that the decrypt failed. Shared by the direct / delegated / unified suites.
 */
export async function expectCorruptedShareDecryptToFail(
  label: string,
  corrupt: ShareCorruptor,
  decrypt: () => Promise<unknown>,
): Promise<void> {
  let thrown: unknown;
  try {
    await withCorruptedUserDecryptShares({ corrupt }, decrypt);
  } catch (error) {
    thrown = error;
  }

  const message = thrown instanceof Error ? thrown.message : String(thrown);
  // eslint-disable-next-line no-console
  console.log(`[corruption] ${label}: error = ${message}`);

  expect(thrown, `Expected user decryption to fail with corrupted shares (${label})`).to.not.be.undefined;
}
