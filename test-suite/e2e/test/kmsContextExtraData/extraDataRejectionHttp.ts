// Harness for the KMS-context QA case `extradata-rejection`: capturing what the SDK puts on the
// wire, replaying it corrupted, and asserting WHICH field the relayer rejected.
//
// Two things live here that the existing unified helper
// (`test/sdk/unified/unifiedUserDecrypt.ts`) deliberately does not do:
//
//   1. **Capturing what the public SDK actually sends.** That helper builds and signs the unified
//      envelope itself, which is right for every scenario that needs to control fields the SDK does
//      not expose. This scenario needs the opposite: the bytes the real `@fhevm/sdk` put on the
//      wire, corrupted afterwards. Only an interception gives that.
//
//   2. **Asserting WHICH field was rejected.** A malformed `extraData` and a bad signature are both
//      `400` with `label: "validation_failed"` (relayer `src/http/endpoints/v2/types/error.rs:239`
//      and `responses.rs:182`). They differ only in `error.details[].field`. Asserting the status
//      alone cannot tell them apart — which matters here more than anywhere else, because a request
//      whose extraData was tampered after signing carries BOTH defects.
import { expect } from 'chai';

/** The route the unified (RFC-016) user-decryption envelope is posted to. */
export const V3_USER_DECRYPT_ROUTE = 'v3/user-decrypt';

/** The `error.label` the relayer returns for every request-validation rejection. */
export const VALIDATION_FAILED_LABEL = 'validation_failed';

/**
 * The unified envelope, as the SDK serializes it.
 *
 * Mirrors `FetchUserDecryptPayloadV2` in `sdk/js-sdk/src/core/types/relayer-p.ts` and
 * `AttestedUserDecryptRequestJson` in the relayer. Only the fields this scenario touches are typed;
 * the rest is carried through verbatim, because the point is to replay the SDK's own bytes.
 */
export interface UnifiedEnvelope {
  attestationType: string;
  attestedPayload: { extraData: string } & Record<string, unknown>;
  signature: string;
}

/** A raw POST outcome: the status and the parsed body, with no interpretation applied. */
export interface RawPostResult {
  readonly httpStatus: number;
  readonly raw: Record<string, unknown>;
}

interface ErrorDetail {
  field?: string;
  issue?: string;
}

/** Strips a trailing `/vN` from the configured relayer URL, matching the unified helper. */
const relayerBaseUrl = (url: string): string => url.replace(/\/(v[0-9]+)\/?$/, '').replace(/\/$/, '');

const jsonHeaders = (apiKey?: string): Record<string, string> => ({
  'Content-Type': 'application/json',
  ...(apiKey ? { 'x-api-key': apiKey } : {}),
});

const readJson = async (response: Response): Promise<Record<string, unknown>> => {
  const text = await response.text();
  try {
    return text ? (JSON.parse(text) as Record<string, unknown>) : {};
  } catch {
    return { nonJsonBody: text };
  }
};

/**
 * Runs `action` with `globalThis.fetch` instrumented, and returns the unified envelope the SDK
 * tried to POST — without letting it reach the relayer.
 *
 * Why nothing is sent: the captured request is replayed later, tampered. Letting the original
 * through first would run a real decryption and, worse, make the tampered replay a *duplicate* of an
 * already-accepted request, which the relayer could legitimately treat differently.
 *
 * **The probe must be let through.** `fetchFeatures`
 * (`sdk/js-sdk/src/core/modules/relayer/module/fetchFeatures.ts:41`) POSTs `{}` to this same route
 * to detect whether the relayer supports it, and expects a real `400`/`404` back. An interceptor
 * keyed on the route alone would capture that empty body and break feature detection, so the filter
 * is on the payload actually carrying an `extraData` — the real request, never the probe.
 *
 * `action` is expected to reject (its POST is answered with a synthetic rejection); the rejection is
 * swallowed and the captured body is what matters.
 */
export const captureUnifiedRequest = async (action: () => Promise<unknown>): Promise<UnifiedEnvelope> => {
  const originalFetch = globalThis.fetch;
  let captured: UnifiedEnvelope | undefined;
  let blocked = 0;

  const isUnifiedRequestBody = (body: unknown): body is UnifiedEnvelope => {
    const payload = (body as UnifiedEnvelope | undefined)?.attestedPayload;
    return typeof payload?.extraData === 'string';
  };

  globalThis.fetch = (async (input: RequestInfo | URL, init?: RequestInit): Promise<Response> => {
    const url =
      typeof input === 'string' ? input : input instanceof URL ? input.toString() : (input as Request).url;
    const method = (init?.method ?? 'GET').toUpperCase();

    if (method === 'POST' && url.includes(V3_USER_DECRYPT_ROUTE) && typeof init?.body === 'string') {
      let parsed: unknown;
      try {
        parsed = JSON.parse(init.body);
      } catch {
        parsed = undefined;
      }
      if (isUnifiedRequestBody(parsed)) {
        captured ??= parsed;
        blocked += 1;
        // Shaped like a real relayer rejection so the SDK treats it as terminal rather than
        // retrying: RelayerAsyncRequest retries transport failures, not 400s.
        return new Response(
          JSON.stringify({
            status: 'failed',
            requestId: null,
            error: { label: VALIDATION_FAILED_LABEL, message: 'intercepted by the QA harness', details: [] },
          }),
          { status: 400, headers: { 'content-type': 'application/json' } },
        );
      }
    }
    return originalFetch(input, init);
  }) as typeof fetch;

  try {
    await action().catch(() => undefined);
  } finally {
    globalThis.fetch = originalFetch;
  }

  if (!captured) {
    throw new Error(
      `kms-context-extradata-rejection: the SDK never POSTed a unified envelope to ${V3_USER_DECRYPT_ROUTE} ` +
        `(${blocked} intercepted call(s)). Either the client fell back to the legacy /v2 route — which carries no ` +
        `versioned extraData — or the decryption failed before reaching the relayer.`,
    );
  }
  return captured;
};

/** POSTs an envelope verbatim, with no signing or reshaping. */
export const postUnifiedEnvelope = async (
  relayerUrl: string,
  apiKey: string | undefined,
  envelope: UnifiedEnvelope,
): Promise<RawPostResult> => {
  const url = `${relayerBaseUrl(relayerUrl)}/${V3_USER_DECRYPT_ROUTE}`;
  console.log(`[kms-context-extradata-rejection] POST ${url} extraData=${envelope.attestedPayload.extraData.slice(0, 10)}…`);
  const response = await fetch(url, {
    method: 'POST',
    headers: jsonHeaders(apiKey),
    body: JSON.stringify(envelope),
  });
  return { httpStatus: response.status, raw: await readJson(response) };
};

/** The per-field issues the relayer reported, or an empty list. */
const errorDetails = (raw: Record<string, unknown>): ErrorDetail[] => {
  const error = raw.error as { details?: ErrorDetail[] } | undefined;
  return error?.details ?? [];
};

/**
 * True when a reported field path refers to `name`.
 *
 * The relayer reports NESTED paths: `extra_data` lives on the inner `attested_payload`, which the v3
 * envelope declares `#[validate(nested)]`, so a rejection names `attestedPayload.extraData` rather
 * than `extraData`. Matching on the leaf keeps the assertion readable and survives the envelope being
 * re-nested — which is exactly the kind of change that should not silently turn this suite green.
 */
const flags = (details: readonly ErrorDetail[], name: string): boolean =>
  details.some((detail) => detail.field === name || (detail.field ?? '').endsWith(`.${name}`));

/**
 * Asserts the relayer rejected the request **because of the extraData**, and not for anything else.
 *
 * The third assertion is the one that earns this helper's existence. A request whose extraData was
 * tampered after signing is also badly signed, so "rejected with 400" proves nothing on its own:
 * the relayer would return the same status either way. Requiring the `extraData` field and
 * forbidding the `signature` field pins the order the v3 handler actually implements — request
 * validation at `user_decrypt.rs:173`, signature pre-check at `:202` — and turns a silent reordering
 * of those two steps into a test failure.
 */
export const expectExtraDataRejection = (post: RawPostResult, what: string): void => {
  const context = `${what}: ${JSON.stringify(post.raw)}`;
  expect(post.httpStatus, context).to.equal(400);

  const label = (post.raw.error as { label?: string } | undefined)?.label;
  expect(label, context).to.equal(VALIDATION_FAILED_LABEL);

  const details = errorDetails(post.raw);
  expect(flags(details, 'extraData'), `${what}: the relayer did not flag the extraData field. ${context}`).to.equal(
    true,
  );
  expect(
    flags(details, 'signature'),
    `${what}: the relayer rejected the SIGNATURE, not the extraData — the request was validated in the wrong ` +
      `order, so a corrupted extraData is no longer reported as such. ${context}`,
  ).to.equal(false);

  const issue =
    details.find((detail) => detail.field === 'extraData' || (detail.field ?? '').endsWith('.extraData'))?.issue ?? '';
  expect(issue, `${what}: unexpected extraData issue text. ${context}`).to.match(/versioned format/i);
};

/** Asserts the relayer accepted the envelope — used by the control, to prove the harness is sound. */
export const expectAccepted = (post: RawPostResult, what: string): void => {
  expect(post.httpStatus, `${what}: ${JSON.stringify(post.raw)}`).to.equal(202);
};

////////////////////////////////////////////////////////////////////////////////
// extraData corruption
//
// All of these operate on the hex string, never on the semantic values: the scenario is about the
// wire format the relayer validates, and the contextId/epochId stay whatever the SDK put there.
////////////////////////////////////////////////////////////////////////////////

/** Replaces the version byte, leaving every other byte untouched. `version` is two hex chars. */
export const withVersionByte = (extraData: string, version: string): string =>
  `0x${version}${extraData.slice(4)}`;

/** Keeps the first `bytes` bytes, producing a payload too short for its own version. */
export const truncateToBytes = (extraData: string, bytes: number): string =>
  extraData.slice(0, 2 + bytes * 2);

/** Appends bytes past the version's fixed length. `suffix` is hex without `0x`. */
export const appendBytes = (extraData: string, suffix: string): string => `${extraData}${suffix}`;

/** Overwrites the domain tag (the first byte) of the id starting at `byteOffset`. */
export const untagIdAt = (extraData: string, byteOffset: number): string => {
  const start = 2 + byteOffset * 2;
  return `${extraData.slice(0, start)}00${extraData.slice(start + 2)}`;
};
