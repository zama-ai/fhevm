import { expect } from 'chai';

/**
 * Client-side corruption of KMS signcrypted shares for user decryption.
 *
 * Reconstruction happens in the SDK's wasm, not the relayer, so corruption has
 * to be injected between the relayer's response and the SDK. We patch
 * `globalThis.fetch` around one decrypt call; both `@fhevm/sdk` and
 * `@zama-fhe/relayer-sdk` resolve the bare global at request time, so a single
 * patch covers either. Shares live at `result.result[]` on the poll response;
 * anything without that array (202 accepts, in-flight polls) passes through.
 *
 * What each corruptor does, and why the outcomes differ (all three measured on a
 * 4-party `t=1` cluster returning every share; see `spareShareTolerance`):
 *
 * | corruptor               | bytes hit       | parses? | EIP-712     | outcome                    |
 * | ----------------------- | --------------- | ------- | ----------- | -------------------------- |
 * | `corruptSignature`      | signature       | yes     | fails       | that share dropped         |
 * | `corruptPayloadBody`    | payload content | yes     | fails       | that share dropped         |
 * | `corruptPayloadFraming` | payload length  | NO      | not reached | ENTIRE response discarded  |
 *
 * The first two are interchangeable: the EIP-712 signature covers the whole
 * serialized payload, so mutating content invalidates the signature exactly as
 * mutating the signature does. Both are survivable while `2t+1` valid shares
 * remain — the wasm charges every share missing from a full `3t+1` committee
 * against a fault budget of `t` (`num_bots = 3t+1 - accepted`), so tolerance is
 * `returned - (2t+1)`.
 *
 * The third is different in kind, and is a defect rather than a threshold
 * property: `js_to_resp` deserializes the whole response vector with `?` per
 * response, so one undecodable payload discards the good shares too, before any
 * signature is checked. Spares cannot help at any count.
 * See TODO(fhevm-internal#1738).
 *
 * Corollary: payload corruption is NOT inherently fatal. Only *framing*
 * corruption is. A flipped byte in the payload body behaves like a bad signature.
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

/**
 * Corrupt every share, whatever the topology returned. A fixed count is unsafe
 * for "must fail" assertions: reconstruction needs `2t+1` *valid* shares, so on a
 * 4-party cluster returning all `3t+1`, corrupting 2 of 4 still leaves 2 valid and
 * a fixed count of 2 would reconstruct where the test expected a failure.
 */
export const CORRUPT_ALL_SHARES = Number.MAX_SAFE_INTEGER;

/** Flip every bit of the byte at `byteOffset` of a hex-no-0x string. */
export function flipByteAt(hex: string, byteOffset: number): string {
  const start = byteOffset * 2;
  if (start + 2 > hex.length) {
    return hex;
  }
  const byte = parseInt(hex.slice(start, start + 2), 16);
  const flipped = ((byte ^ 0xff) & 0xff).toString(16).padStart(2, '0');
  return hex.slice(0, start) + flipped + hex.slice(start + 2);
}

/** Flip every bit of the first byte of a hex-no-0x string (guaranteed change). */
export function flipFirstByte(hex: string): string {
  return flipByteAt(hex, 0);
}

/**
 * Case 1: corrupt the payload's leading length prefix — a framing fault.
 *
 * Byte 0 of the bincode-encoded `UserDecryptionResponsePayload` belongs to the
 * length prefix of its first field (`verification_key`), so flipping it corrupts
 * no key material: it changes how many bytes the decoder is told to read. The
 * wasm then reads following bytes as a 64-bit length and aborts with
 * `OutsideUsizeRange` (`usize` is 32-bit on wasm32), which is why the reported
 * length differs on every run.
 *
 * This never reaches signature verification. `js_to_resp` deserializes the whole
 * response vector with `?` per response, so a single framing fault discards every
 * share — spares cannot help at any count. Contrast `corruptPayloadBody`.
 */
export const corruptPayloadFraming: ShareCorruptor = (share) => ({
  ...share,
  payload: flipFirstByte(share.payload),
});

/**
 * Case 3: corrupt the payload body, past every length prefix — a content fault.
 *
 * Three quarters in lands inside the signcrypted ciphertext, by far the largest
 * field, so the framing survives and the payload deserializes. The EIP-712
 * signature covers `bc2wrap::serialize(payload)` in full, so validation then
 * rejects this share and drops it — the same fate as a corrupted signature,
 * rather than the whole-response abort of `corruptPayloadFraming`.
 */
export const corruptPayloadBody: ShareCorruptor = (share) => {
  const byteLength = Math.floor(share.payload.length / 2);
  return { ...share, payload: flipByteAt(share.payload, Math.floor(byteLength * 0.75)) };
};

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

/**
 * Prefer `'tail'` when the decrypt must SUCCEED: if the wasm pins its pivot to
 * the first response, head corruption fails it for pivot reasons rather than for
 * lack of good shares.
 */
export type CorruptionEnd = 'head' | 'tail';

export interface CorruptionOptions {
  /** How to corrupt each targeted share. */
  corrupt: ShareCorruptor;
  /** How many shares to corrupt (default {@link DEFAULT_CORRUPT_COUNT}). */
  count?: number;
  /** Which end of the array to corrupt (default `'head'`). */
  from?: CorruptionEnd;
  /**
   * Drop the response to its first `keep` shares before corrupting, simulating a
   * relayer that returned fewer. Dumb on purpose: the count is the caller's to
   * derive, so a truncation cannot silently track whatever the relayer sent.
   */
  keep?: number;
  /** Called for every share-bearing response, with the count the SDK will see. */
  onShares?: (shareCount: number) => void;
}

/**
 * Run `fn` with `globalThis.fetch` patched so that user-decrypt share responses
 * are corrupted before the SDK reconstructs them. Restores `fetch` afterwards.
 */
export async function withCorruptedUserDecryptShares<T>(options: CorruptionOptions, fn: () => Promise<T>): Promise<T> {
  const { corrupt, onShares, keep } = options;
  const count = options.count ?? DEFAULT_CORRUPT_COUNT;
  const from = options.from ?? 'head';
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

    const received = shares.length;
    if (keep !== undefined && keep < shares.length) {
      shares.splice(keep);
    }
    onShares?.(shares.length);
    const corruptCount = Math.min(count, shares.length);
    const offset = from === 'tail' ? shares.length - corruptCount : 0;
    const dropped = received === shares.length ? '' : `, truncated to ${shares.length}`;
    // eslint-disable-next-line no-console
    console.log(
      `[corruption] user-decrypt response: ${received} shares received${dropped}, ` +
        `corrupting ${corruptCount} from the ${from}`,
    );
    for (let i = 0; i < corruptCount; i++) {
      shares[offset + i] = corrupt(shares[offset + i], offset + i);
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
 * Assert a decrypt fails with corrupted shares, printing the error. Corrupts ALL
 * shares unless `count` says otherwise — leaving good shares behind would make
 * the assertion depend on topology. Returns the observed share count.
 */
export async function expectCorruptedShareDecryptToFail(
  label: string,
  corrupt: ShareCorruptor,
  decrypt: () => Promise<unknown>,
  options: Pick<CorruptionOptions, 'count' | 'from' | 'keep'> = {},
): Promise<{ shareCount: number; message: string }> {
  let thrown: unknown;
  let observed = 0;
  try {
    await withCorruptedUserDecryptShares(
      { count: CORRUPT_ALL_SHARES, ...options, corrupt, onShares: (n) => (observed = n) },
      decrypt,
    );
  } catch (error) {
    thrown = error;
  }

  const message = thrown instanceof Error ? thrown.message : String(thrown);
  // The relayer SDK wraps wasm errors in a generic message and hides the real
  // one in `cause`, so log both or the diagnostic is empty for that SDK.
  const cause = thrown instanceof Error && thrown.cause !== undefined ? ` | cause = ${String(thrown.cause)}` : '';
  // eslint-disable-next-line no-console
  console.log(`[corruption] ${label}: error = ${message}${cause}`);

  expect(thrown, `Expected user decryption to fail with corrupted shares (${label})`).to.not.be.undefined;
  return { shareCount: observed, message: `${message}${cause}` };
}

/**
 * Assert a decrypt survives `count` corrupted shares. Returns the value and the
 * observed share count so the caller can check both. Not wrapped in try/catch:
 * a throw surfaces the real wasm error as the failure.
 */
export async function expectCorruptedShareDecryptToSucceed<T>(
  label: string,
  corrupt: ShareCorruptor,
  decrypt: () => Promise<T>,
  options: Pick<CorruptionOptions, 'count' | 'from' | 'keep'> = {},
): Promise<{ value: T; shareCount: number }> {
  let observed = 0;
  const value = await withCorruptedUserDecryptShares({ ...options, corrupt, onShares: (n) => (observed = n) }, decrypt);
  // eslint-disable-next-line no-console
  console.log(`[corruption] ${label}: reconstructed from ${observed} shares (${options.count ?? 0} corrupted)`);
  return { value, shareCount: observed };
}

/** Observe the share count of an uncorrupted decrypt (no mutation applied). */
export async function measureReturnedShares<T>(decrypt: () => Promise<T>): Promise<{ value: T; shareCount: number }> {
  let observed = 0;
  const value = await withCorruptedUserDecryptShares(
    { corrupt: (share) => share, count: 0, onShares: (n) => (observed = n) },
    decrypt,
  );
  return { value, shareCount: observed };
}
