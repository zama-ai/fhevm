/**
 * QA case `extradata-echo` — the response half of the KMS-context extraData story:
 *
 *   Scenario: Every KMS share echoes the request's extraData, byte for byte
 *     Given the active pair is "C, E"
 *     When a user decryption is sent with extraData "0x00"
 *     And another with extraData "0x01 || C"
 *     And another with extraData "0x02 || C || E"
 *     Then all three must decrypt successfully
 *     And every share's response extraData must equal its request's, byte for byte
 *
 * ## The clause five scenarios asked for and none could assert
 *
 * *"The response extraData must be identical to the request extraData"* appears in the proposed
 * scenarios behind cases `epoch-rotation`, `context-switch`, `epoch-rotation-pending`,
 * `context-switch-pending`, and in the v0/v1/v2 scenario this case comes from. Every one of those
 * reports records it as deliberately uncovered, for a reason that is a property of the SDK rather
 * than a shortcut: the SDK receives the per-share response extraData and never compares or exposes
 * it — `equalsKmsExtraData` has zero production call sites, the response-signature check and the
 * cross-share consistency check are both commented out, and the value reaches no public return type.
 * The evidence is in `test-suite/fhevm/qa-extradata-check.md`.
 *
 * It is observable one layer down. The relayer stores both sides:
 * `user_decrypt_req.req->>'extra_data'` and `user_decrypt_share.extra_data`, joined on
 * `gw_reference_id` — one row per share. That is what this case reads.
 *
 * **Scope, stated plainly:** this proves the relayer received shares carrying the request's
 * extraData. It says nothing about the SDK, which still does not verify the echo. That half remains
 * a product decision and `qa-extradata-check.md` still governs it.
 *
 * ## Why three versions, and why v2 alone would prove nothing
 *
 * The failure this guards against is not a dropped field — it is the field being **regenerated**
 * downstream from the responder's own view of the active context, instead of passed through.
 *
 *   - `0x02 || C || E` is the **control**. Regenerated from the active pair it would be byte-identical
 *     to what was sent, so echo and regeneration are indistinguishable. It proves the path works.
 *   - `0x01 || C` **discriminates**: 33 bytes, no epoch. Any value rebuilt from the active pair comes
 *     back as 65.
 *   - `0x00` discriminates hardest: one byte. Any regeneration at all is visible.
 *
 * All three are known-good: each has a passing test in
 * `test-suite/e2e/test/unifiedUserDecryption/unifiedUserDecryption.ts` (v0 at :552, v1 at :583,
 * v2 at :618) proving it decrypts successfully.
 *
 * ## The one tolerated difference, measured rather than assumed
 *
 * The first live run found v2 and v1 echoed byte for byte, and v0's `0x00` coming back as `0x` on
 * every share. That is not regeneration — the connector normalizes the legacy marker to empty before
 * the KMS core sees it, and its own unit test says so by name
 * (`kms_decryption_extra_data_normalizes_legacy_zero_marker`). Both values mean "no context".
 *
 * The assertion tolerates exactly that one substitution and nothing else: a v0 request answered with
 * 33 or 65 bytes still fails. Widening it to "any length is fine for v0" would surrender the
 * discrimination v0 was chosen to provide. The reasoning is in `isAcceptableEcho`, with unit tests.
 *
 * The strongest positive result of that run is unrelated to v0: **v1 came back at 33 bytes, without
 * an epoch**. A field rebuilt from the active pair could not do that. There is no regeneration.
 *
 * ## What this case must NOT do
 *
 * If a share comes back with something other than what was sent, that is the finding — not a reason
 * to relax the assertion to whatever the implementation happens to produce. The comparison is
 * byte-exact on the stored strings, and the failure message prints both sides so the shape of the
 * divergence is visible without a database session.
 *
 * ## Changes nothing
 *
 * Three ordinary user decryptions and two reads. No lifecycle transaction, no container stopped, no
 * context or epoch advanced. Rerunnable against the same stack.
 */
import { PreflightError } from "../../errors";
import type { QaCase, QaCaseContext } from "../registry";
import { formatKmsId, readCurrentPair } from "../protocol-config";
import { isAcceptableEcho, readExtraDataEchoRows, readLatestUserDecryptRequestId } from "../relayer-db";

/** The extraData versions the probe drives, by their leading byte. */
const EXPECTED_VERSIONS = ["0x00", "0x01", "0x02"] as const;

/** Renders an extraData for a message: the leading bytes plus its length, which is what differs. */
const describeExtraData = (value: string) => `${value.slice(0, 14)}… (${(value.length - 2) / 2} bytes)`;

const run = async (ctx: QaCaseContext): Promise<void> => {
  const { target, owner, evidence, runExtraDataEcho } = ctx;

  evidence.note("note", "target", {
    protocolConfig: target.address,
    rpcUrl: target.rpcUrl,
    where: target.where,
    owner: owner.address,
  });

  const active = await readCurrentPair(target, evidence, "active pair the probes will reference");

  // The baseline bounds the read to this case's own requests. Taken before the probe so the rows
  // inspected afterwards are exactly the three it drives — no clock skew, no overlap with whatever
  // else has touched the relayer.
  const baselineRequestId = await evidence.step(
    "call",
    "read the relayer's latest user_decrypt_req id as a baseline",
    {},
    () => readLatestUserDecryptRequestId(),
  );
  evidence.note("note", "relayer request-id baseline", { baselineRequestId: String(baselineRequestId) });

  await evidence.step(
    "probe",
    "drive one successful decryption per extraData version (v0, v1, v2)",
    { contextId: formatKmsId(active.contextId), epochId: formatKmsId(active.epochId) },
    () =>
      runExtraDataEcho(
        `kms-context-qa/extradata-echo: three decryptions under (${active.contextId}, ${active.epochId})`,
        { contextId: active.contextId, epochId: active.epochId },
      ),
  );

  const rows = await evidence.step(
    "call",
    "read the request/share extraData pairs the relayer recorded",
    { afterRequestId: String(baselineRequestId) },
    () => readExtraDataEchoRows(baselineRequestId),
  );

  const byRequest = new Map<number, typeof rows>();
  for (const row of rows) {
    byRequest.set(row.requestId, [...(byRequest.get(row.requestId) ?? []), row]);
  }
  evidence.note("note", "relayer rows recorded by the probe", {
    requests: String(byRequest.size),
    shares: String(rows.length),
    versions: [...new Set(rows.map((row) => row.requestExtraData.slice(0, 4)))].sort().join(","),
  });

  // Without this, an echo assertion over zero rows would pass silently and report success for a
  // probe that never reached the KMS.
  await evidence.step(
    "assert",
    "the probe produced one request per version, each with shares",
    { requests: String(byRequest.size), shares: String(rows.length) },
    async () => {
      if (!rows.length) {
        throw new PreflightError(
          "kms-context-qa/extradata-echo: the relayer recorded no shares for the probe's requests. Either the " +
            "decryptions never reached the KMS, or the probe ran against a different relayer than the one read here.",
        );
      }
      const versions = new Set(rows.map((row) => row.requestExtraData.slice(0, 4)));
      const missing = EXPECTED_VERSIONS.filter((version) => !versions.has(version));
      if (missing.length) {
        throw new PreflightError(
          `kms-context-qa/extradata-echo: no shares recorded for extraData version(s) ${missing.join(", ")}. ` +
            `Observed: ${[...versions].sort().join(", ")}. The discriminating versions are v0 and v1 — without them ` +
            `the case cannot tell an echo from a value regenerated out of the active context.`,
        );
      }
    },
  );

  // The claim itself. Byte-exact on the stored strings, with one measured exception: the legacy
  // `0x00` marker comes back empty, because the connector normalizes it before the KMS core sees it
  // (`kms_decryption_extra_data_normalizes_legacy_zero_marker`). Both values mean "no context".
  //
  // The exception admits ONLY the empty payload — see `isAcceptableEcho`. A v0 request answered with
  // 33 or 65 bytes still fails, which is the discrimination v0 was chosen for. A decoded comparison
  // would instead accept a v1 request answered with a v2 payload naming the same context, which is
  // precisely the regeneration this case exists to detect.
  await evidence.step(
    "assert",
    "every share echoed its request's extraData byte for byte",
    { shares: String(rows.length) },
    async () => {
      const mismatches = rows.filter((row) => !isAcceptableEcho(row.requestExtraData, row.shareExtraData));
      if (mismatches.length) {
        const detail = mismatches
          .slice(0, 6)
          .map(
            (row) =>
              `  request #${row.requestId} share ${row.shareIndex}: sent ${describeExtraData(row.requestExtraData)}, ` +
              `got ${describeExtraData(row.shareExtraData)}`,
          )
          .join("\n");
        throw new PreflightError(
          `kms-context-qa/extradata-echo: ${mismatches.length} of ${rows.length} share(s) did not echo the ` +
            `extraData that was sent:\n${detail}\n` +
            `A response whose length differs from the request's means the field was rebuilt from the responder's own ` +
            `view of the active context instead of being passed through. The ONE tolerated difference is the legacy ` +
            `0x00 marker coming back empty; anything else — including a v0 request answered with 33 or 65 bytes — ` +
            `is a real divergence. Inspect with:\n` +
            `  docker exec fhevm-relayer-db psql -U postgres -d relayer_db -c "select r.id, r.req->>'extra_data', ` +
            `s.share_index, s.extra_data from user_decrypt_req r join user_decrypt_share s on ` +
            `r.gw_reference_id = s.gw_reference_id where r.id > ${baselineRequestId} order by r.id, s.share_index;"`,
        );
      }
    },
  );

  for (const [requestId, shares] of [...byRequest.entries()].sort((a, b) => a[0] - b[0])) {
    evidence.note("note", "echo confirmed for one request", {
      requestId: String(requestId),
      extraData: describeExtraData(shares[0]!.requestExtraData),
      shares: String(shares.length),
    });
  }

  console.log(
    `[kms-context-qa] extradata-echo complete: ${rows.length} share(s) across ${byRequest.size} request(s) echoed ` +
      `their request's extraData byte for byte, including the 1-byte v0 and 33-byte v1 payloads that a regenerated ` +
      `field could not reproduce. This is the relayer/KMS contract only — the SDK still does not verify the echo ` +
      `(see test-suite/fhevm/qa-extradata-check.md).`,
  );
};

/** Registry entry for the `extradata-echo` case. */
export const extraDataEchoCase: QaCase = {
  id: "extradata-echo",
  title: "Every KMS share echoes the request's extraData, byte for byte (v0, v1 and v2)",
  proves:
    "a user decryption sent with extraData v0, v1 or v2 comes back with every share carrying byte-for-byte the " +
    "value that was sent — including the 1-byte v0 and the epoch-less 33-byte v1, which a field regenerated from " +
    "the active context could not reproduce — closing, at the relayer/KMS layer, the response-echo clause that five " +
    "QA scenarios asked for and the SDK cannot express",
  requirements: {
    // None: three ordinary decryptions against whatever pair is active.
  },
  mutatesLifecycle: false,
  run,
};
