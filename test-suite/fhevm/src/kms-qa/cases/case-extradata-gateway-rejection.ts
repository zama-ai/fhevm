/**
 * QA case `extradata-gateway-rejection` — the contract-level half of the extraData rejection story:
 *
 *   Feature: Gateway handling of directly submitted corrupted extraData
 *
 *     Scenario: The Gateway reverts calldata with corrupted extraData
 *       Given ProtocolConfig returns a valid active pair "C1, E1"
 *       And an instrumented SDK has built a valid decryption request
 *       And the original Gateway calldata contains extraData "0x02 || C1 || E1"
 *       And the test harness changes only the extraData version byte to an unsupported version
 *       When the altered ABI calldata is submitted directly to the Gateway Decryption contract
 *       Then the transaction must revert
 *       And no decryption request event must be emitted
 *
 * ## Why this exists on top of `extradata-rejection`
 *
 * That case proves the Relayer refuses a corrupted extraData over HTTP. This one bypasses the
 * Relayer entirely: the Relayer is a service that can be replaced, misconfigured or simply skipped
 * by anyone willing to send their own transaction, so a check that lives only there is a
 * convenience, not a guarantee. The contract is the boundary that has to hold, and this case is what
 * says so.
 *
 * ## Which overload, and why it is the one that can be tested
 *
 * `Decryption.sol` has two `userDecryptionRequest` functions whose check order is opposite:
 *
 *   - legacy `(CtHandleContractPair[], …)` verifies the EIP-712 signature on chain (line 506) and
 *     extracts the context only afterwards (line 542). A request tampered after signing reverts for
 *     the signature, and the version check is never reached — the scenario would be untestable.
 *   - unified `(HandleEntry[], …)` extracts the context as its fourth statement (line 690), before
 *     the fee and before the handles are read, and never verifies the signature on chain at all
 *     (authorization moved to the KMS Connector).
 *
 * The case targets the unified overload — what the SDK and the Relayer use, and the one on which
 * the scenario's claim is unambiguous. The legacy path is `@custom:deprecated` and is left alone.
 *
 * This is the mirror of the Relayer case: there the format check wins over the signature check by
 * ORDER, here it wins by the signature check not existing on chain at all.
 *
 * ## Changes nothing
 *
 * Like `extradata-rejection`, this case sends no lifecycle transaction and advances no context or
 * epoch. It does send one transaction to the Gateway, which reverts — so it costs gas on the gateway
 * chain and lands a status-0 receipt, but writes no state. Rerunnable against the same stack.
 */
import type { QaCase, QaCaseContext } from "../registry";
import { formatKmsId, readCurrentPair } from "../protocol-config";

/**
 * Runs the gateway-rejection scenario.
 *
 * Sequence: read the active pair -> hand it to the container spec, which captures a real SDK request,
 * re-addresses it from the Relayer to the Gateway's Decryption contract, corrupts one byte, and
 * asserts the transaction reverts with `UnsupportedExtraDataVersion` and emits nothing.
 */
const run = async (ctx: QaCaseContext): Promise<void> => {
  const { target, owner, evidence, runExtraDataGatewayRejection } = ctx;

  evidence.note("note", "target", {
    protocolConfig: target.address,
    rpcUrl: target.rpcUrl,
    where: target.where,
    owner: owner.address,
  });

  const active = await readCurrentPair(target, evidence, "active pair the calldata must carry");

  evidence.note("note", "raw ids handed to the container-side gateway check", {
    context: active.contextId.toString(),
    epoch: active.epochId.toString(),
    expectedExtraDataVersion: "0x02",
  });

  await evidence.step(
    "probe",
    "the Gateway reverts calldata carrying a corrupted extraData version, and emits nothing",
    { contextId: formatKmsId(active.contextId), epochId: formatKmsId(active.epochId) },
    () =>
      runExtraDataGatewayRejection(
        `kms-context-qa/extradata-gateway-rejection: corrupted calldata reverts on the Gateway ` +
          `(contextId=${active.contextId})`,
        { contextId: active.contextId, epochId: active.epochId },
      ),
  );

  console.log(
    `[kms-context-qa] extradata-gateway-rejection complete: calldata built from an SDK-signed request for ` +
      `(${active.contextId}, ${active.epochId}), corrupted in its extraData version byte and submitted straight to ` +
      `the Gateway's Decryption contract, reverts with UnsupportedExtraDataVersion and emits no decryption ` +
      `request. The check does not depend on the Relayer being in the path.`,
  );
};

/** Registry entry for the `extradata-gateway-rejection` case. */
export const extraDataGatewayRejectionCase: QaCase = {
  id: "extradata-gateway-rejection",
  title: "The Gateway reverts directly submitted calldata with a corrupted extraData (both halves)",
  proves:
    "calldata built from a request the SDK signed for the active (context, epoch) pair, corrupted afterwards in " +
    "its extraData version byte and submitted directly to the Gateway's Decryption contract — bypassing the " +
    "Relayer — reverts with UnsupportedExtraDataVersion and emits no decryption request event, so the format " +
    "check is enforced by the contract and not merely by the service in front of it",
  requirements: {
    // None. The case needs a valid active pair and a reachable Gateway; the container half skips
    // itself when the Gateway endpoint or key is not configured.
  },
  mutatesLifecycle: false,
  run,
};
