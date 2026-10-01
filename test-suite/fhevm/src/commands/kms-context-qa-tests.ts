/**
 * The `kms-context-qa-tests` acceptance profile for `fhevm-cli test`.
 *
 * A QA-driven companion to `kms-context-switch`, built one scenario at a time from the KMS
 * context/epoch QA catalogue. Where `kms-context-switch` runs the full lifecycle as a single
 * scripted sequence, this profile is a registry of independent cases, each mapping to one QA
 * scenario and each producing a structured evidence record of what the protocol actually did.
 *
 * `kms-context-switch` is deliberately left untouched: it gates CI, and the two profiles are
 * allowed to overlap.
 *
 * ## Layout
 *
 * This module owns only preflight, wiring, sequencing and reporting. All protocol work lives in
 * `src/kms-qa/`:
 *   - `protocol-config.ts` — ProtocolConfig views, lifecycle transactions, event decoding, waits;
 *   - `nodes.ts`           — scoped container control with a tracked, guaranteed restore;
 *   - `evidence.ts`        — the timed, structured audit trail;
 *   - `registry.ts`        — the case catalogue and selection;
 *   - `cases/`             — one file per QA scenario.
 *
 * Adding a scenario is a new file under `cases/` plus one entry in `QA_CASES`. This file does not
 * change.
 *
 * ## Running
 *
 *   fhevm-cli up --target latest-main --scenario five-party-swap-threshold-kms
 *   fhevm-cli test kms-context-qa-tests
 *   KMS_QA_CASES=epoch-rotation fhevm-cli test kms-context-qa-tests   # one case while iterating
 *
 * ## Disruptive and single-run
 *
 * Cases advance the on-chain context/epoch and may take KMS nodes offline. Container state is
 * always restored; chain state is not. Re-up between runs.
 */
import { PreflightError } from "../errors";
import { resolveKmsGenerationTarget } from "../flow/readiness";
import { loadHostOwner } from "../kms-qa/onchain";
import { EvidenceRecorder } from "../kms-qa/evidence";
import { NodeSupervisor } from "../kms-qa/nodes";
import {
  CASE_SELECTOR_ENV,
  QA_CASES,
  checkCaseRequirements,
  selectCases,
  type ExtraDataCheckRunner,
  type ExtraDataEchoRunner,
  type ExtraDataGatewayRejectionRunner,
  type ExtraDataRejectionRunner,
  type QaCase,
} from "../kms-qa/registry";
import type { ProtocolConfigTarget } from "../kms-qa/protocol-config";
import { readCurrentPair } from "../kms-qa/protocol-config";
import type { DecryptionRunner } from "./kms-generation";
import type { SmokeRunner } from "./kms-context-switch";
import type { State } from "../types";

/** Scenario this profile is designed against; see {@link ALLOW_ANY_SCENARIO_ENV} to override. */
const REQUIRED_SCENARIO = "five-party-swap-threshold-kms";

/**
 * Escape hatch for running the profile on a different topology.
 *
 * Not every case needs the 5-core swap scenario — a same-context epoch rotation, for instance, only
 * needs a committee — and a lighter stack iterates far faster under emulation. Per-case
 * `requirements` are still enforced, so this relaxes the scenario pin, not the safety checks.
 */
const ALLOW_ANY_SCENARIO_ENV = "KMS_QA_ALLOW_ANY_SCENARIO";

/** The remediation line every preflight failure ends with. */
const REUP_HINT = `rerun \`fhevm-cli down && fhevm-cli up --scenario ${REQUIRED_SCENARIO}\``;

/**
 * Rejects a stack this profile cannot run against, before any transaction is sent.
 *
 * Checks the KMS mode and the scenario identity. Per-case topology requirements are checked
 * separately, for the whole selected set, so a long run never dies halfway through on something
 * knowable up front.
 */
const assertProfilePreconditions = (state: State): void => {
  const kms = state.scenario.kms;
  if (kms.mode !== "threshold") {
    throw new PreflightError(
      `kms-context-qa-tests requires a threshold-mode KMS cluster (the active scenario is ${kms.mode}); ${REUP_HINT}`,
    );
  }
  if (process.env[ALLOW_ANY_SCENARIO_ENV] === "1") {
    console.log(
      `[kms-context-qa] ${ALLOW_ANY_SCENARIO_ENV}=1: skipping the ${REQUIRED_SCENARIO} scenario check; ` +
        "per-case topology requirements are still enforced",
    );
    return;
  }
  const sourcePath = state.scenarioSourcePath ?? state.scenario.sourcePath ?? "";
  const scenarioName = state.scenario.name ?? "(unnamed)";
  const matchesScenario =
    sourcePath.includes(REQUIRED_SCENARIO) || scenarioName.toLowerCase().includes("node-swap");
  if (!matchesScenario) {
    throw new PreflightError(
      `kms-context-qa-tests expects the ${REQUIRED_SCENARIO} scenario, but the active stack is ` +
        `"${scenarioName}"${sourcePath ? ` (${sourcePath})` : ""}. Set ${ALLOW_ANY_SCENARIO_ENV}=1 to run ` +
        `anyway on a topology that satisfies the selected cases' requirements, or ${REUP_HINT}`,
    );
  }
};

/**
 * Checks every selected case's topology requirements up front.
 *
 * @throws PreflightError naming each case that cannot run and why.
 */
const assertCaseRequirements = (cases: readonly QaCase[], state: State): void => {
  const failures = cases
    .map((item) => {
      const reason = checkCaseRequirements(item.requirements, state.scenario.kms);
      return reason ? `  - ${item.id}: ${reason}` : undefined;
    })
    .filter(Boolean);
  if (failures.length) {
    throw new PreflightError(
      `kms-context-qa-tests: the active scenario does not satisfy every selected case:\n${failures.join("\n")}\n` +
        REUP_HINT,
    );
  }
};

/** Prints what is about to run, so a long run is legible from its first screen. */
const printRunBanner = (cases: readonly QaCase[], state: State): void => {
  const kms = state.scenario.kms;
  console.log(
    `[kms-context-qa] scenario="${state.scenario.name ?? "(unnamed)"}" kms=${kms.mode} parties=${kms.parties} ` +
      `threshold=${kms.threshold} committeeSize=${kms.committeeSize}`,
  );
  console.log(`[kms-context-qa] running ${cases.length} case(s) of ${QA_CASES.length} registered:`);
  for (const item of cases) {
    console.log(`[kms-context-qa]   - ${item.id}: ${item.title}`);
  }
  if (cases.some((item) => item.mutatesLifecycle)) {
    console.log(
      "[kms-context-qa] note: selected cases advance on-chain context/epoch state. This run is single-use — " +
        `${REUP_HINT} before running again.`,
    );
  }
};

/**
 * Runs the selected QA cases against a live stack.
 *
 * The container-facing runners are injected by `test()` rather than imported, matching how
 * `kms-context-switch` receives them: this module decides *when* to probe, `test.ts` owns *how* the
 * probe reaches the test-suite container.
 */
export const runKmsContextQaTestsProfile = async (
  state: State,
  runDecryption: DecryptionRunner,
  runSmoke: SmokeRunner,
  runExtraDataCheck: ExtraDataCheckRunner,
  runExtraDataRejection: ExtraDataRejectionRunner,
  runExtraDataGatewayRejection: ExtraDataGatewayRejectionRunner,
  runExtraDataEcho: ExtraDataEchoRunner,
): Promise<void> => {
  assertProfilePreconditions(state);

  const cases = selectCases(QA_CASES, process.env[CASE_SELECTOR_ENV]);
  if (!cases.length) {
    throw new PreflightError(
      `kms-context-qa-tests: no cases selected. Unset ${CASE_SELECTOR_ENV} to run all of them.`,
    );
  }
  assertCaseRequirements(cases, state);
  printRunBanner(cases, state);

  const { rpcUrl, configAddress, where } = resolveKmsGenerationTarget(state);
  if (!configAddress) {
    throw new PreflightError(
      `kms-context-qa-tests: no ProtocolConfig address on ${where} — cannot read or drive the KMS context. ` +
        `The stack must have completed the discover step; ${REUP_HINT}`,
    );
  }
  const target: ProtocolConfigTarget = { rpcUrl, address: configAddress, where };
  const owner = await loadHostOwner();

  const recorder = new EvidenceRecorder();
  const nodes = new NodeSupervisor();
  const completed: QaCase[] = [];
  const startedAt = Date.now();

  try {
    for (const item of cases) {
      const evidence = recorder.forCase(item.id);
      console.log(`[kms-context-qa] ---- ${item.id}: ${item.title} ----`);
      await item.run({
        state,
        target,
        owner,
        nodes,
        evidence,
        runDecryption,
        runSmoke,
        runExtraDataCheck,
        runExtraDataRejection,
        runExtraDataGatewayRejection,
        runExtraDataEcho,
      });
      completed.push(item);
      console.log(
        `[kms-context-qa] ---- ${item.id} PASSED (${Math.round(recorder.durationForCase(item.id) / 1000)}s) ----`,
      );
    }
  } finally {
    // Restore anything a failing case left stopped, then always print the evidence: a failed run
    // needs its audit trail more than a passing one.
    await nodes.restoreAll();
    console.log(recorder.renderSummary());
    if (completed.length !== cases.length) {
      await reportStackState(target, recorder, completed, cases);
    }
  }

  console.log(
    `[kms-context-qa] PASS (${Math.round((Date.now() - startedAt) / 1000)}s) — ${completed.length} case(s):\n` +
      completed.map((item) => `  - ${item.id}: ${item.proves}`).join("\n"),
  );
};

/**
 * Prints the post-failure forensic state and the remediation command.
 *
 * On-chain state is never rolled back: a half-finished lifecycle operation is evidence, not litter,
 * and destroying it automatically would erase the most useful artifact of a failed run.
 */
const reportStackState = async (
  target: ProtocolConfigTarget,
  recorder: EvidenceRecorder,
  completed: readonly QaCase[],
  selected: readonly QaCase[],
): Promise<void> => {
  const failed = selected.find((item) => !completed.includes(item));
  console.log(
    `[kms-context-qa] run incomplete: ${completed.length}/${selected.length} case(s) passed` +
      (failed ? `, failed at "${failed.id}"` : ""),
  );
  try {
    const evidence = recorder.forCase("stack-state");
    const pair = await readCurrentPair(target, evidence, "post-failure active pair");
    console.log(
      `[kms-context-qa] on-chain state left behind: contextId=${pair.contextId} epochId=${pair.epochId} ` +
        `(ProtocolConfig ${target.address} on ${target.where})`,
    );
  } catch (error) {
    console.log(`[kms-context-qa][warn] could not read the post-failure state: ${String(error)}`);
  }
  console.log(`[kms-context-qa] chain state is not rolled back — ${REUP_HINT} before the next run.`);
};
