/**
 * The per-run result contract, and the aggregate that compares results with the
 * plan.
 *
 * A consensus run's output used to be a shell log. Whether a cell had passed
 * was decided by grepping for `1 passing`, which an after-hook failure survives,
 * and whether a fault had landed was not recorded anywhere at all. Results are
 * therefore structured records now: one JSON object per case, appended to a
 * run's results file, carrying the evidence the inventory demands of that case.
 *
 * The aggregate is the gate. It refuses a run that is missing required cases,
 * that reports one case twice with conflicting states, that names a case the
 * inventory does not have, or that was produced against a different revision or
 * topology than it claims. A partial run may succeed for the cases it selected,
 * but is labeled partial and can never satisfy the full delivery gate.
 */
import { verifyCheckoutArtifacts } from "./build-provenance";
import { appendFileSync, existsSync, mkdirSync, readFileSync, readdirSync, statSync } from "node:fs";
import path from "node:path";

import { RUNTIME_DIR } from "../layout";
import { type CaseState, type Inventory, type InventoryCase, CASE_STATES } from "./inventory";

export const RESULT_SCHEMA_VERSION = 2;

export interface AssertionOutcome {
  name: string;
  outcome: "pass" | "fail" | "not_evaluated";
  detail?: string;
}

export interface ProcessIdentity {
  target: string;
  /** Container id, systemd invocation id, or pid - whatever identifies THIS run of the process. */
  identity: string;
  pid?: number;
}

export interface CaseResult {
  schemaVersion: number;
  runId: string;
  caseId: string;
  state: CaseState;
  /** Free text: why, in one line. Required for every non-PASS state. */
  detail?: string;
  revision: string;
  executionClass: { software: string; backend: string; hardware: string };
  schedulingClasses?: string;
  topology: { scenario: string; operators: number; threshold: number };
  /** Resolved image ids/digests and binary hashes, so a result names the code it ran. */
  artifactIdentities?: Record<string, string>;
  startedAt: string;
  endedAt: string;
  /** Transaction hashes, handles, request ids - the work whose fate is asserted. */
  workloadIds?: string[];
  processesBefore?: ProcessIdentity[];
  processesAfter?: ProcessIdentity[];
  faultObservedAt?: string;
  recoveryObservedAt?: string;
  assertions: AssertionOutcome[];
  cleanup: { state: "ok" | "failed" | "not_required"; detail?: string };
}

export class ResultError extends Error {
  constructor(message: string) {
    super(`consensus results: ${message}`);
    this.name = "ResultError";
  }
}

const SECRET_KEYS = /pass(word)?|secret|private[-_]?key|token|credential/i;

/**
 * Rejects a record that would put credentials or key material in an artifact.
 *
 * Results are uploaded from CI and read by people who are not the person who
 * ran them, so this is a hard check rather than a convention.
 */
export const assertNoSecrets = (record: unknown, where = "result"): void => {
  if (record === null || record === undefined) return;
  if (Array.isArray(record)) {
    record.forEach((entry, index) => assertNoSecrets(entry, `${where}[${index}]`));
    return;
  }
  if (typeof record === "object") {
    for (const [key, value] of Object.entries(record as Record<string, unknown>)) {
      if (SECRET_KEYS.test(key)) {
        throw new ResultError(`${where}.${key} looks like a credential; results are uploaded as artifacts`);
      }
      assertNoSecrets(value, `${where}.${key}`);
    }
    return;
  }
  if (typeof record === "string" && /postgres(ql)?:\/\/[^@\s]*:[^@\s]+@/.test(record)) {
    throw new ResultError(`${where} embeds a database password in a connection string`);
  }
};

const object = (value: unknown, where: string): Record<string, unknown> => {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new ResultError(`${where} must be an object`);
  return value as Record<string, unknown>;
};
const nonempty = (value: unknown, where: string): string => {
  if (typeof value !== "string" || !value.trim()) throw new ResultError(`${where} must be a nonempty string`);
  return value;
};
const optionalText = (value: unknown, where: string): string | undefined =>
  value === undefined ? undefined : nonempty(value, where);
const integer = (value: unknown, where: string): number => {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new ResultError(`${where} must be a nonnegative integer`);
  return value;
};
const requireIsoTimestamp = (value: unknown, where: string): string => {
  if (typeof value !== "string" || !/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})$/.test(value) || Number.isNaN(Date.parse(value))) {
    throw new ResultError(`${where} must be an ISO-8601 timestamp`);
  }
  return value;
};
const list = (value: unknown, where: string): unknown[] => {
  if (!Array.isArray(value)) throw new ResultError(`${where} must be an array`);
  return value;
};
const processList = (value: unknown, where: string): ProcessIdentity[] | undefined =>
  value === undefined ? undefined : list(value, where).map((entry, index) => {
    const location = `${where}[${index}]`, row = object(entry, location);
    return { target: nonempty(row.target, `${location}.target`), identity: nonempty(row.identity, `${location}.identity`),
      ...(row.pid === undefined ? {} : { pid: integer(row.pid, `${location}.pid`) }) };
  });

/** Validate untrusted JSON before any aggregate rule reads its nested fields. */
export const parseCaseResult = (raw: unknown): CaseResult => {
  const row = object(raw, "a result record");
  const caseId = nonempty(row.caseId, "caseId");
  const state = row.state;
  if (typeof state !== "string" || !(CASE_STATES as readonly string[]).includes(state)) {
    throw new ResultError(`${caseId}: state must be one of ${CASE_STATES.join(", ")}`);
  }
  if (state !== "PASS" && (typeof row.detail !== "string" || !row.detail.trim())) {
    throw new ResultError(`${caseId}: a ${state} result must carry a detail explaining it`);
  }
  const runId = nonempty(row.runId, `${caseId}.runId`);
  if (!/^[a-zA-Z0-9][a-zA-Z0-9_.-]*$/.test(runId)) throw new ResultError(`${caseId}: runId must be a safe filename component`);
  if (typeof row.revision !== "string" || !row.revision.trim()) throw new ResultError(`${caseId}: missing revision, so the result cannot be tied to code`);
  const execution = object(row.executionClass, `${caseId}.executionClass`);
  const topology = object(row.topology, `${caseId}.topology`);
  const operators = integer(topology.operators, `${caseId}.topology.operators`);
  const threshold = integer(topology.threshold, `${caseId}.topology.threshold`);
  if (threshold > operators || (operators > 0 && threshold === 0)) throw new ResultError(`${caseId}: invalid topology threshold`);
  const assertions = list(row.assertions, `${caseId}.assertions`).map((entry, index): AssertionOutcome => {
    const location = `${caseId}.assertions[${index}]`, assertion = object(entry, location);
    if (!["pass", "fail", "not_evaluated"].includes(assertion.outcome as string)) throw new ResultError(`${location}.outcome is invalid`);
    return { name: nonempty(assertion.name, `${location}.name`), outcome: assertion.outcome as AssertionOutcome["outcome"],
      detail: optionalText(assertion.detail, `${location}.detail`) };
  });
  const cleanup = object(row.cleanup, `${caseId}.cleanup`);
  if (!["ok", "failed", "not_required"].includes(cleanup.state as string)) throw new ResultError(`${caseId}.cleanup.state is invalid`);
  const startedAt = requireIsoTimestamp(row.startedAt, `${caseId}.startedAt`);
  const endedAt = requireIsoTimestamp(row.endedAt, `${caseId}.endedAt`);
  if (Date.parse(endedAt) < Date.parse(startedAt)) throw new ResultError(`${caseId}: endedAt precedes startedAt`);
  const faultObservedAt = row.faultObservedAt === undefined ? undefined : requireIsoTimestamp(row.faultObservedAt, `${caseId}.faultObservedAt`);
  const recoveryObservedAt = row.recoveryObservedAt === undefined ? undefined : requireIsoTimestamp(row.recoveryObservedAt, `${caseId}.recoveryObservedAt`);
  if (faultObservedAt && recoveryObservedAt && Date.parse(recoveryObservedAt) < Date.parse(faultObservedAt)) throw new ResultError(`${caseId}: recovery precedes the observed fault`);
  const artifacts = row.artifactIdentities === undefined ? undefined : object(row.artifactIdentities, `${caseId}.artifactIdentities`);
  const artifactIdentities = artifacts && Object.fromEntries(Object.entries(artifacts).map(([key, value]) =>
    [nonempty(key, `${caseId}.artifactIdentities key`), nonempty(value, `${caseId}.artifactIdentities.${key}`)]));
  assertNoSecrets(row, `result ${caseId}`);
  return {
    schemaVersion: integer(row.schemaVersion, `${caseId}.schemaVersion`), runId, caseId, state: state as CaseState,
    detail: optionalText(row.detail, `${caseId}.detail`), revision: row.revision,
    executionClass: { software: nonempty(execution.software, `${caseId}.executionClass.software`),
      backend: nonempty(execution.backend, `${caseId}.executionClass.backend`), hardware: nonempty(execution.hardware, `${caseId}.executionClass.hardware`) },
    schedulingClasses: optionalText(row.schedulingClasses, `${caseId}.schedulingClasses`),
    topology: { scenario: nonempty(topology.scenario, `${caseId}.topology.scenario`), operators, threshold },
    artifactIdentities, startedAt, endedAt,
    workloadIds: row.workloadIds === undefined ? undefined : list(row.workloadIds, `${caseId}.workloadIds`).map((entry, index) => nonempty(entry, `${caseId}.workloadIds[${index}]`)),
    processesBefore: processList(row.processesBefore, `${caseId}.processesBefore`),
    processesAfter: processList(row.processesAfter, `${caseId}.processesAfter`),
    faultObservedAt, recoveryObservedAt, assertions,
    cleanup: { state: cleanup.state as CaseResult["cleanup"]["state"], detail: optionalText(cleanup.detail, `${caseId}.cleanup.detail`) },
  };
};

/**
 * Where a run's structured results live.
 *
 * Resolved through the CLI's own state layout rather than a second copy of the
 * `FHEVM_STATE_DIR` rule: a custom state directory that only some components
 * honor is how a run ends up reading one stack's results while driving
 * another's.
 */
export const resultsDirectory = (env: NodeJS.ProcessEnv = process.env): string =>
  env.CONSENSUS_RESULTS_DIR ?? path.join(RUNTIME_DIR, "consensus-results");

/** Appends one record to a run's results file. */
export const appendCaseResult = (result: CaseResult, directory = resultsDirectory()): string => {
  parseCaseResult(result);
  mkdirSync(directory, { recursive: true });
  const file = path.join(directory, `${result.runId}.jsonl`);
  appendFileSync(file, `${JSON.stringify(result)}\n`, "utf8");
  return file;
};

/** Reads every result record under a directory, or from one file. */
export const readCaseResults = (target: string): CaseResult[] => {
  if (!existsSync(target)) throw new ResultError(`no results at ${target}`);
  const files = statSync(target).isDirectory()
    ? readdirSync(target)
        .filter((name) => name.endsWith(".jsonl"))
        .map((name) => path.join(target, name))
    : [target];
  const results: CaseResult[] = [];
  for (const file of files) {
    const text = readFileSync(file, "utf8");
    text
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0)
      .forEach((line, index) => {
        let parsed: unknown;
        try {
          parsed = JSON.parse(line);
        } catch {
          throw new ResultError(`${file}:${index + 1} is not valid JSON`);
        }
        results.push(parseCaseResult(parsed));
      });
  }
  return results;
};

export interface AggregateOptions {
  inventory: Inventory;
  results: CaseResult[];
  /** Cases the run claims to have selected. A full gate passes every required case. */
  selected: InventoryCase[];
  /** Revision the run must have been produced against. */
  revision?: string;
  /** True when the selection is deliberately a subset of the required set. */
  partial?: boolean;
  /** Enforce the backend selected by CI, rather than all locally compatible backends. */
  ci?: boolean;
  /** CI may require locally built branch code; a checkout SHA alone is insufficient. */
  requireBuildMode?: "checkout";
}

export const missingAssertionKinds = (entry: InventoryCase, result: CaseResult): string[] => {
  if (result.state !== "PASS") return [];
  const kinds = new Set(entry.assertions.map((assertion) => assertion.split(":", 1)[0].trim()));
  return [...kinds].filter((kind) => !result.assertions.some((assertion) =>
    assertion.name === kind && assertion.outcome === "pass",
  ));
};

export interface AggregateReport {
  ok: boolean;
  partial: boolean;
  lines: string[];
  problems: string[];
  states: Map<string, CaseState>;
}

/**
 * Compares results against the plan and decides whether the run stands.
 *
 * Everything here is a rejection rule rather than a summary: the value of the
 * aggregate is that a run cannot look complete when it is not.
 */
export const aggregate = (options: AggregateOptions): AggregateReport => {
  const { inventory, results, selected } = options;
  const known = new Map(inventory.cases.map((entry) => [entry.id, entry]));
  const problems: string[] = [];
  const lines: string[] = [];
  const states = new Map<string, CaseState>();

  for (const raw of results) {
    let result: CaseResult;
    try { result = parseCaseResult(raw); }
    catch (error) { problems.push(error instanceof ResultError ? error.message : "unreadable result record"); continue; }
    if (!known.has(result.caseId)) {
      problems.push(`result names unknown case ${result.caseId}; results and inventory disagree`);
      continue;
    }
    if (result.schemaVersion !== RESULT_SCHEMA_VERSION) {
      problems.push(
        `${result.caseId}: result schema version ${result.schemaVersion} != ${RESULT_SCHEMA_VERSION}; ` +
          "a result written by a different harness version is not comparable",
      );
    }
    if (options.revision && result.revision !== options.revision) {
      problems.push(
        `${result.caseId}: produced against revision ${result.revision}, but the run claims ${options.revision}`,
      );
    }
    const expected = known.get(result.caseId)!;
    const missingKinds = missingAssertionKinds(expected, result);
    if (missingKinds.length) problems.push(`${result.caseId}: missing passing inventory assertion kinds: ${missingKinds.join(", ")}`);
    if (result.state === "PASS" && options.ci && expected.ci.backend !== "none" &&
        result.executionClass.backend.split("-")[0] !== expected.ci.backend) {
      problems.push(`${result.caseId}: CI requires backend ${expected.ci.backend}; compatible backends are not additional CI coverage`);
    }
    if (result.state === "PASS" && options.ci && expected.ci.leg === "degraded") {
      const setting = expected.id === "DEG-06-GW-LISTENER-INFLIGHT" ? "false" : "true";
      if (result.artifactIdentities?.drift_auto_revert !== setting) {
        problems.push(`${result.caseId}: CI requires observed drift_auto_revert=${setting}`);
      }
    }
    if (result.state === "PASS" && expected.topology.scenario !== "none" && options.requireBuildMode &&
        result.artifactIdentities?.build_mode !== options.requireBuildMode) {
      problems.push(`${result.caseId}: branch validation requires build_mode=${options.requireBuildMode}, got ${result.artifactIdentities?.build_mode ?? "unspecified"}`);
    }
    if (result.state === "PASS" && expected.topology.scenario !== "none" && options.requireBuildMode) {
      try { verifyCheckoutArtifacts(result.artifactIdentities, result.revision); }
      catch (error) { problems.push(`${result.caseId}: ${error instanceof Error ? error.message : "invalid build provenance"}`); }
    }
    // The topology check exists to stop an outcome earned on the wrong stack
    // from counting. A NOT_APPLICABLE explicitly declines to establish the
    // case, and the topology it carries is the stack that declined -- which is
    // the mismatched one by definition, and is information rather than a false
    // claim. So the check applies to results that assert an outcome.
    const claimsOutcome = result.state === "PASS" || result.state === "FAIL" || result.state === "INVALID";
    if (
      claimsOutcome &&
      result.topology.scenario !== expected.topology.scenario &&
      expected.topology.scenario !== "none"
    ) {
      problems.push(
        `${result.caseId}: ran on scenario ${result.topology.scenario}, but the case requires ` +
          `${expected.topology.scenario}`,
      );
    }
    if (
      claimsOutcome &&
      expected.topology.operators > 0 &&
      (result.topology.operators !== expected.topology.operators ||
        result.topology.threshold !== expected.topology.threshold)
    ) {
      problems.push(
        `${result.caseId}: ran at ${result.topology.operators}/${result.topology.threshold}, but the case ` +
          `requires ${expected.topology.operators}/${expected.topology.threshold}`,
      );
    }
    if (
      claimsOutcome &&
      !expected.topology.backends.includes("none") &&
      !expected.topology.backends.some((backend) => backend === result.executionClass.backend.split("-")[0])
    ) {
      problems.push(`${result.caseId}: ran with backend ${result.executionClass.backend}, but the case requires ${expected.topology.backends.join(" or ")}`);
    }
    if (result.state === "PASS" && result.cleanup.state === "failed") {
      problems.push(
        `${result.caseId}: reported PASS with a failed cleanup (${result.cleanup.detail ?? "no detail"}); ` +
          "a contaminated stack cannot carry a passing result",
      );
    }
    if (result.state === "PASS" && !result.assertions.some((entry) => entry.outcome === "pass")) {
      problems.push(`${result.caseId}: reported PASS without a passing assertion outcome`);
    }
    if (result.state === "PASS" && result.assertions.some((entry) => entry.outcome === "fail")) {
      problems.push(`${result.caseId}: reported PASS with a failed assertion`);
    }
    // A stack case that declares a fault must show it was observed, and must
    // name the work whose recovery it asserts. Both requirements exist to rule
    // out a green that came from a fault which never landed or from fresh work
    // substituted for the interrupted work.
    //
    // Neither is meaningful for a case with no topology: a contract test
    // injects its fault inside its own process and the assertion that catches
    // it IS the observation, and its "workload" is the test suite. Demanding a
    // timestamp and a handle there would be satisfied by writing something
    // down, which is the box-ticking these fields exist to prevent. What such
    // a case must carry instead is evidence that its tests actually ran --
    // exit status 0 with zero tests is the failure mode -- so a scenario-none
    // PASS is required to report a count.
    const stackCase = expected.topology.scenario !== "none";
    if (stackCase && result.state === "PASS" && expected.fault.mechanism !== "none" && !result.faultObservedAt) {
      problems.push(
        `${result.caseId}: reported PASS without a fault observation timestamp, so the ` +
          `${expected.fault.mechanism} injection cannot be distinguished from a no-op`,
      );
    }
    if (
      stackCase &&
      result.state === "PASS" &&
      expected.acceptance !== "smoke" &&
      expected.fault.mechanism !== "none"
    ) {
      if (!result.workloadIds || result.workloadIds.length === 0) {
        problems.push(
          `${result.caseId}: reported PASS without naming the workload whose recovery it asserts; ` +
            "fresh-work substitution is exactly what this field exists to rule out",
        );
      }
    }
    if (!stackCase && result.state === "PASS") {
      const identities = Object.keys(result.artifactIdentities ?? {});
      if (identities.length === 0) {
        problems.push(
          `${result.caseId}: reported PASS without reporting how much of it ran; a contract suite ` +
            "that exits 0 having run no tests is indistinguishable from a passing one",
        );
      }
    }

    const previous = states.get(result.caseId);
    if (previous !== undefined && previous !== result.state) {
      // NOT_APPLICABLE is not a verdict about the case; it is a verdict about
      // the stack that declined it. A full run is a union of sessions -- the
      // three-of-three session declines DEG-03 and the two-of-three session
      // establishes it -- so a decline meeting a real outcome is the expected
      // shape, not a contradiction. The outcome stands and nothing is
      // reported.
      //
      // Everything else still conflicts, and still resolves to the worse of
      // the two, so a genuine disagreement cannot settle green.
      const declined = (state: CaseState) => state === "NOT_APPLICABLE";
      if (declined(previous) !== declined(result.state)) {
        states.set(result.caseId, declined(previous) ? result.state : previous);
      } else {
        problems.push(`${result.caseId}: reported twice with conflicting states (${previous} and ${result.state})`);
        const rank = (state: CaseState) => CASE_STATES.indexOf(state);
        states.set(result.caseId, rank(result.state) > rank(previous) ? result.state : previous);
      }
    } else {
      states.set(result.caseId, result.state);
    }
  }

  for (const entry of selected) {
    const state = states.get(entry.id);
    if (state === undefined) {
      problems.push(`${entry.id} was selected but produced no result; it is NOT_RUN, not passed`);
      states.set(entry.id, "NOT_RUN");
      continue;
    }
    if (state === "NOT_APPLICABLE" && entry.capabilities.length === 0 && entry.services.length === 0) {
      problems.push(`${entry.id} reported NOT_APPLICABLE but declares no capability or service that could be absent`);
    }
  }

  const requiredEverywhere = inventory.cases.filter((entry) => entry.acceptance === "required");
  const selectedIds = new Set(selected.map((entry) => entry.id));
  const missingRequired = requiredEverywhere.filter((entry) => !selectedIds.has(entry.id));
  const partial = options.partial === true || missingRequired.length > 0;
  if (missingRequired.length > 0 && options.partial !== true) {
    problems.push(
      `this run omits ${missingRequired.length} required case(s) but was not declared partial: ` +
        `${missingRequired.map((entry) => entry.id).join(", ")}`,
    );
  }

  for (const entry of selected) {
    const state = states.get(entry.id) ?? "NOT_RUN";
    const acceptable =
      state === "PASS" ||
      (entry.acceptance !== "required" && state === "NOT_APPLICABLE" && (entry.capabilities.length > 0 || entry.services.length > 0));
    lines.push(`${acceptable ? " " : "!"} ${entry.id.padEnd(32)} ${state.padEnd(15)} ${entry.acceptance}`);
    if (!acceptable && entry.acceptance !== "deferred") {
      problems.push(`${entry.id} is ${state}`);
    }
  }

  if (selected.length === 0) {
    problems.push("the run selected no cases; an empty selection is not a passing aggregate");
  }

  return { ok: problems.length === 0, partial, lines, problems, states };
};
