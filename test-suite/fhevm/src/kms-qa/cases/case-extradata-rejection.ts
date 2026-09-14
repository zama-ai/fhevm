/**
 * QA case `extradata-rejection` — the negative half of the KMS-context extraData story:
 *
 *   Feature: Rejection of corrupted extraData
 *
 *     Scenario: A request carrying a corrupted extraData version is refused
 *       Given ProtocolConfig returns a valid active pair "C1, E1"
 *       And an instrumented SDK has built a valid decryption request
 *       And the original request extraData is "0x02 || C1 || E1"
 *       And the test harness changes only the version byte to an unsupported version
 *       When the altered HTTP body is sent directly to the Relayer API
 *       Then the request must be refused as invalid extraData
 *
 * ## Why this belongs to the KMS-context profile
 *
 * `extraData` is how a decryption request names the KMS context and epoch it is for. The other four
 * cases prove the SDK puts the *right* pair in there across every lifecycle transition; this one
 * proves a *wrong* one does not get through. The check happens to be enforced by the Relayer, but
 * the subject is the context envelope, not the Relayer — so it sits here, next to its siblings, and
 * its container spec lives beside theirs in `test/kmsContextExtraData/`.
 *
 * ## The only case here that changes nothing
 *
 * It sends no lifecycle transaction, stops no container, and advances no on-chain state. Its single
 * side effect is one accepted decryption job — the control, which proves the captured request was
 * genuinely valid before it was corrupted. `mutatesLifecycle` is therefore false, and unlike every
 * other case this one can be rerun against the same stack as often as you like.
 *
 * ## What the container half does, and why the host half is thin
 *
 * Everything substantive is client-side: instrument `globalThis.fetch`, let the real SDK build and
 * sign a unified user-decryption request, capture the bytes without sending them, change one byte,
 * and POST it. The host's only jobs are to read the pair the request should carry and hand it over,
 * so the spec can cross-check that nothing moved between the two reads.
 *
 * ## What it does NOT cover
 *
 * The semantic checks on the ids — an unknown contextId, an inactive epochId. Those are enforced by
 * the KMS Connector and the Gateway, asynchronously, and already have coverage in
 * `test-suite/e2e/test/unifiedUserDecryption/unifiedUserDecryption.ts`. The line between the two is
 * the layer, not the value: format at the Relayer, meaning at the KMS.
 */
import type { QaCase, QaCaseContext } from "../registry";
import { formatKmsId, readCurrentPair } from "../protocol-config";

/**
 * Runs the extraData-rejection scenario.
 *
 * Sequence: read the active pair -> hand it to the container spec, which captures a real SDK request,
 * corrupts it, and asserts the refusal names the extraData field rather than the signature.
 */
const run = async (ctx: QaCaseContext): Promise<void> => {
  const { target, owner, evidence, runExtraDataRejection } = ctx;

  evidence.note("note", "target", {
    protocolConfig: target.address,
    rpcUrl: target.rpcUrl,
    where: target.where,
    owner: owner.address,
  });

  // The pair a compliant SDK must embed. The spec reads it itself too; handing it over lets the spec
  // also assert the chain did not move between the two reads.
  const active = await readCurrentPair(target, evidence, "active pair the request must carry");

  evidence.note("note", "raw ids handed to the container-side rejection check", {
    context: active.contextId.toString(),
    epoch: active.epochId.toString(),
    expectedExtraDataVersion: "0x02",
  });

  await evidence.step(
    "probe",
    "a corrupted extraData is refused, and refused for the extraData rather than the signature",
    { contextId: formatKmsId(active.contextId), epochId: formatKmsId(active.epochId) },
    () =>
      runExtraDataRejection(
        `kms-context-qa/extradata-rejection: corrupted extraData is refused (contextId=${active.contextId})`,
        { contextId: active.contextId, epochId: active.epochId },
      ),
  );

  console.log(
    `[kms-context-qa] extradata-rejection complete: a request built and signed by the SDK for ` +
      `(${active.contextId}, ${active.epochId}), then corrupted in its version byte, is refused as invalid ` +
      `extraData — not as a bad signature. On-chain state is untouched.`,
  );
};

/** Registry entry for the `extradata-rejection` case. */
export const extraDataRejectionCase: QaCase = {
  id: "extradata-rejection",
  title: "A corrupted extraData is refused, and refused as extraData (both halves)",
  proves:
    "a decryption request the SDK built and signed for the active (context, epoch) pair, corrupted afterwards in " +
    "its extraData version byte, is refused with a field-level extraData error rather than a signature error — " +
    "pinning that the envelope's format is validated before its signature — and that every other malformed " +
    "extraData shape is refused the same way",
  requirements: {
    // None. The case needs a valid active pair and nothing else: no committee, no spare, no
    // particular committee size. The profile's own preflight still pins the scenario and the
    // threshold mode.
  },
  mutatesLifecycle: false,
  run,
};
