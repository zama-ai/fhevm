#!/usr/bin/env bun
/**
 * The inventory's command line: validate, list, select, record and aggregate.
 *
 * Runners and CI both go through this rather than carrying their own idea of
 * which cases exist. `select` is what a workflow uses to turn a leg name into a
 * concrete case list, and it fails on an unknown term instead of resolving to
 * nothing; `aggregate` is what decides whether a finished run stands.
 *
 *   consensus-inventory.ts validate
 *   consensus-inventory.ts list [--format table|ids|json]
 *   consensus-inventory.ts select <term>... [--backend cpu|gpu]
 *                                 [--capability <name>]... [--include-deferred]
 *                                 [--format ids|json|runners]
 *   consensus-inventory.ts plan <term>... [--format json|github]
 *   consensus-inventory.ts show <case-id>
 *   consensus-inventory.ts record --run <id> --case <id> --state <STATE> [...]
 *   consensus-inventory.ts aggregate --run <id> [--select <term>...]
 *                                    [--revision <sha>] [--partial]
 */
import { existsSync } from "node:fs";
import { assertCiLeg } from "../src/consensus/ci";
import { imageBindings, validateBuildReceipt, parseIdentityText, readIdentityFile } from "../src/consensus/build-provenance";
import path from "node:path";

import {
  type Backend,
  type InventoryCase,
  BACKENDS,
  InventoryError,
  loadInventory,
  selectCases,
} from "../src/consensus/inventory";
import {
  type AssertionOutcome,
  type CaseResult,
  RESULT_SCHEMA_VERSION,
  ResultError,
  aggregate,
  appendCaseResult,
  parseCaseResult,
  missingAssertionKinds,
  readCaseResults,
  resultsDirectory,
} from "../src/consensus/results";
import { CASE_STATES, type CaseState } from "../src/consensus/inventory";

const fail = (message: string): never => {
  console.error(`consensus-inventory: ${message}`);
  process.exit(2);
};

interface Flags {
  positional: string[];
  values: Map<string, string[]>;
  booleans: Set<string>;
}

const BOOLEAN_FLAGS = new Set(["include-deferred", "partial", "quiet", "ci"]);

const parseFlags = (argv: string[]): Flags => {
  const positional: string[] = [];
  const values = new Map<string, string[]>();
  const booleans = new Set<string>();
  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (!argument.startsWith("--")) {
      positional.push(argument);
      continue;
    }
    const name = argument.slice(2);
    if (BOOLEAN_FLAGS.has(name)) {
      booleans.add(name);
      continue;
    }
    const value = argv[index + 1];
    if (value === undefined || value.startsWith("--")) fail(`--${name} needs a value`);
    values.set(name, [...(values.get(name) ?? []), value]);
    index += 1;
  }
  return { positional, values, booleans };
};

const single = (flags: Flags, name: string): string | undefined => flags.values.get(name)?.at(-1);

const describe = (entry: InventoryCase): string =>
  [
    `id:          ${entry.id}`,
    `family:      ${entry.family}`,
    `acceptance:  ${entry.acceptance}`,
    `property:    ${entry.property}`,
    `topology:    ${entry.topology.scenario} ${entry.topology.operators}/${entry.topology.threshold} ` +
      `backends=${entry.topology.backends.join(",")}`,
    `services:    ${entry.services.join(", ") || "(none)"}`,
    `capabilities:${entry.capabilities.join(", ") || " (none)"}`,
    `workload:    ${entry.workload}`,
    `fault:       ${entry.fault.mechanism} at ${entry.fault.stage}`,
    `observation: ${entry.fault.observation}`,
    `quorum:      ${entry.participants.quorum} (compute ${entry.participants.compute}, submit ${entry.participants.submit})`,
    `timeout:     ${entry.timeoutSeconds}s`,
    `runner:      ${entry.runner}`,
    `ci leg:      ${entry.ci.leg} (backend ${entry.ci.backend}; other topology backends are local compatibility)`,
    `artifacts:   ${entry.artifacts.join(", ") || "(none)"}`,
    "assertions:",
    ...entry.assertions.map((line) => `  - ${line}`),
    "cleanup:",
    ...entry.cleanup.map((line) => `  - ${line}`),
    ...(entry.deferral
      ? ["deferral:", `  boundary: ${entry.deferral.boundary}`, `  reason:   ${entry.deferral.reason}`]
      : []),
  ].join("\n");

const resolveBackend = (value: string | undefined): Backend | undefined => {
  if (!value) return undefined;
  if (!(BACKENDS as readonly string[]).includes(value)) fail(`--backend must be one of ${BACKENDS.join(", ")}`);
  return value as Backend;
};

const runValidate = () => {
  const inventory = loadInventory();
  const required = inventory.cases.filter((entry) => entry.acceptance === "required").length;
  const smoke = inventory.cases.filter((entry) => entry.acceptance === "smoke").length;
  const deferred = inventory.cases.filter((entry) => entry.acceptance === "deferred").length;
  console.log(
    `inventory OK: schema ${inventory.schemaVersion}, ${inventory.cases.length} case(s) in ` +
      `${inventory.families.size} famil(ies) — ${required} required, ${smoke} smoke, ${deferred} deferred`,
  );
};

const runList = (flags: Flags) => {
  const inventory = loadInventory();
  const format = single(flags, "format") ?? "table";
  if (format === "json") {
    console.log(JSON.stringify(inventory.cases, null, 2));
    return;
  }
  if (format === "ids") {
    for (const entry of inventory.cases) console.log(entry.id);
    return;
  }
  console.log(`${"CASE".padEnd(32)} ${"FAMILY".padEnd(19)} ${"LEG".padEnd(18)} ${"ACCEPT".padEnd(9)} PROPERTY`);
  for (const entry of inventory.cases) {
    console.log(
      `${entry.id.padEnd(32)} ${entry.family.padEnd(19)} ${entry.ci.leg.padEnd(18)} ` +
        `${entry.acceptance.padEnd(9)} ${entry.property.replace(/\s+/g, " ").slice(0, 96)}`,
    );
  }
};

const runSelect = (flags: Flags) => {
  const inventory = loadInventory();
  const selection = selectCases(inventory, {
    terms: flags.positional,
    backend: resolveBackend(single(flags, "backend")),
    capabilities: flags.values.has("capability") ? flags.values.get("capability") : undefined,
    includeDeferred: flags.booleans.has("include-deferred"),
  });
  const format = single(flags, "format") ?? "ids";
  if (format === "json") {
    console.log(JSON.stringify({ cases: selection.cases, unavailable: selection.unavailable }, null, 2));
    return;
  }
  if (format === "runners") {
    for (const entry of [...new Set(selection.cases.map((row) => row.runner))]) console.log(entry);
  } else {
    for (const entry of selection.cases) console.log(entry.id);
  }
  for (const entry of selection.unavailable) {
    console.error(
      `NOT_APPLICABLE ${entry.case.id}: missing capabilit(ies) ${entry.missing.join(", ")} on this topology`,
    );
  }
};

/**
 * Resolves a selection into a CI job plan.
 *
 * This is what makes a workflow's selection honest. The legs used to be a
 * hardcoded `strategy.matrix` with `if:` conditions, so a selection could name
 * a leg the inventory has no cases for, or provision a GPU instance for a
 * CPU-only selection, and neither showed up as anything other than a green job
 * that ran nothing. Here the plan comes from the inventory: an unknown term is
 * an error, and the emitted plan says exactly which legs, scenarios, cases and
 * runners a run will execute.
 */
const runPlan = (flags: Flags) => {
  const inventory = loadInventory();
  const selection = selectCases(inventory, {
    terms: flags.positional,
    capabilities: flags.values.has("capability") ? flags.values.get("capability") : undefined,
    includeDeferred: flags.booleans.has("include-deferred"),
  });

  // One job per (leg, scenario): a leg whose cases need two topologies cannot
  // run as one job, and pretending otherwise is how a case ends up executed on
  // the wrong stack.
  const jobs = new Map<
    string,
    { leg: string; scenario: string; operators: number; threshold: number; cases: string[]; runners: string[]; capabilities: string[]; backend: Backend; shard: string; timeoutMinutes: number }
  >();
  for (const entry of selection.cases) {
    assertCiLeg(entry.ci.leg);
    const shard = entry.id === "DEG-06-GW-LISTENER-INFLIGHT" ? "gateway" : entry.ci.leg !== "failure-matrix" ? "all" :
      entry.acceptance === "smoke" ? "smoke" :
      /FM-(RELAYER|KMS|OBJECT|BROKER)/.test(entry.id) ? "dependencies" : "workers";
    const key = `${entry.ci.leg}|${entry.topology.scenario}|${entry.ci.backend}|${shard}`;
    const job = jobs.get(key) ?? {
      leg: entry.ci.leg,
      backend: entry.ci.backend,
      scenario: entry.topology.scenario,
      operators: entry.topology.operators,
      threshold: entry.topology.threshold,
      cases: [],
      runners: [],
      capabilities: [],
      shard,
      timeoutMinutes: 60, // Build and bootstrap, in addition to every case budget.
    };
    job.cases.push(entry.id);
    job.capabilities = [...new Set([...job.capabilities, ...entry.capabilities])].sort();
    job.timeoutMinutes += Math.ceil(entry.timeoutSeconds / 60);
    if (!job.runners.includes(entry.runner)) job.runners.push(entry.runner);
    jobs.set(key, job);
  }

  const legs = [...new Set(selection.cases.map((entry) => entry.ci.leg))].sort();
  const plan = {
    legs,
    // Whether this selection needs hardware, so provisioning can be gated on
    // the plan rather than on a job condition that runs after the instance is
    // already paid for.
    needsGpu: selection.cases.some((entry) => entry.capabilities.includes("gpu")),
    needsTwoGpus: selection.cases.some((entry) => entry.capabilities.includes("two-gpus")),
    needsDualAnvil: selection.cases.some((entry) => entry.capabilities.includes("dual-anvil")),
    needsIsolatedStack: selection.cases.some((entry) => entry.capabilities.includes("isolated-stack")),
    // A selection that omits required cases is partial, and the aggregate must
    // be told so rather than inferring it.
    partial: inventory.cases
      .filter((entry) => entry.acceptance === "required")
      .some((entry) => !selection.cases.some((chosen) => chosen.id === entry.id)),
    jobs: [...jobs.values()].sort((a, b) => a.leg.localeCompare(b.leg)),
    unavailable: selection.unavailable.map((entry) => ({ id: entry.case.id, missing: entry.missing })),
  };

  if ((single(flags, "format") ?? "json") === "github") {
    // `key=value` lines for $GITHUB_OUTPUT, with the matrix as compact JSON.
    const cpuJobs = plan.jobs.filter((job) => job.scenario !== "none" && job.cases.every((id) =>
      !inventory.cases.find((entry) => entry.id === id)!.capabilities.includes("gpu"),
    ));
    console.log(`cpu-legs=${[...new Set(cpuJobs.map((job) => job.leg))].join(",")}`);
    console.log(`cpu-matrix=${JSON.stringify({ include: cpuJobs })}`);
    console.log(`legs=${plan.legs.join(",")}`);
    console.log(`needs-gpu=${plan.needsGpu}`);
    console.log(`needs-two-gpus=${plan.needsTwoGpus}`);
    console.log(`partial=${plan.partial}`);
    console.log(`matrix=${JSON.stringify({ include: plan.jobs })}`);
    return;
  }
  console.log(JSON.stringify(plan, null, 2));
};

const runShow = (flags: Flags) => {
  const inventory = loadInventory();
  const id = flags.positional[0] ?? fail("show needs a case id");
  const entry = inventory.cases.find((row) => row.id === id);
  if (!entry) fail(`no case ${id}`);
  console.log(describe(entry!));
};

/**
 * Records one case result.
 *
 * Shell runners call this rather than writing JSON by hand, so a record cannot
 * omit a field the aggregate requires and quietly become unverifiable.
 */
const runRecord = (flags: Flags) => {
  const inventory = loadInventory();
  const caseId = single(flags, "case") ?? fail("--case is required");
  const entry = inventory.cases.find((row) => row.id === caseId);
  if (!entry) fail(`--case ${caseId} is not in the inventory`);
  const state = (single(flags, "state") ?? fail("--state is required")) as CaseState;
  if (!(CASE_STATES as readonly string[]).includes(state)) fail(`--state must be one of ${CASE_STATES.join(", ")}`);

  const assertions: AssertionOutcome[] = (flags.values.get("assert") ?? []).map((raw) => {
    // `name=outcome[:detail]`
    const separator = raw.indexOf("=");
    const namePart = raw.slice(0, separator);
    const rest = separator < 0 ? undefined : raw.slice(separator + 1);
    if (!rest) fail(`--assert must be name=outcome[:detail]; got ${raw}`);
    const [outcome, ...detail] = rest!.split(":");
    if (!["pass", "fail", "not_evaluated"].includes(outcome)) {
      fail(`--assert outcome must be pass, fail or not_evaluated; got ${outcome}`);
    }
    return {
      name: namePart,
      outcome: outcome as AssertionOutcome["outcome"],
      detail: detail.length > 0 ? detail.join(":") : undefined,
    };
  });

  let sharedIdentities: Record<string, string> = {};
  if (process.env.CONSENSUS_ARTIFACT_IDENTITIES_FILE) {
    const boot = readIdentityFile(process.env.CONSENSUS_ARTIFACT_IDENTITIES_FILE);
    // Observe current immutable container IDs for each verdict, including after recreation.
    const current = Bun.spawnSync(["bash", path.join(import.meta.dir, "record-run-identity.sh")], { timeout: 60_000 });
    if (current.exitCode !== 0 && state === "PASS") fail("could not obtain current run image identity");
    const currentText = current.exitCode === 0 ? current.stdout.toString() : "identity_observation_error=current Docker identity unavailable";
    sharedIdentities = { ...boot, ...parseIdentityText(currentText) };
    // A removed container must not inherit its earlier observation.
    const observed = parseIdentityText(currentText);
    for (const key of Object.keys(boot)) if (key.startsWith("image_") && !(key in observed)) delete sharedIdentities[key];
  }
  const now = new Date().toISOString();
  const record: CaseResult = {
    schemaVersion: RESULT_SCHEMA_VERSION,
    runId: single(flags, "run") ?? fail("--run is required"),
    caseId,
    state,
    detail: single(flags, "detail"),
    revision: single(flags, "revision") ?? fail("--revision is required"),
    executionClass: {
      software: single(flags, "software-class") ?? process.env.CONSENSUS_SOFTWARE_CLASS ?? single(flags, "revision")!,
      backend: single(flags, "backend-class") ?? "unknown",
      hardware: single(flags, "hardware-class") ?? "unknown",
    },
    schedulingClasses: single(flags, "scheduling-classes"),
    topology: {
      scenario: single(flags, "scenario") ?? entry!.topology.scenario,
      operators: Number(single(flags, "operators") ?? String(entry!.topology.operators)),
      threshold: Number(single(flags, "threshold") ?? String(entry!.topology.threshold)),
    },
    artifactIdentities: { ...sharedIdentities, ...Object.fromEntries(
      [
        ...(process.env.CONSENSUS_BUILD_MODE ? [`build_mode=${process.env.CONSENSUS_BUILD_MODE}`] : []),
        ...(process.env.CONSENSUS_DRIFT_AUTO_REVERT_ENABLED ? [`drift_auto_revert=${process.env.CONSENSUS_DRIFT_AUTO_REVERT_ENABLED}`] : []),
        ...(flags.values.get("artifact-identity") ?? []),
      ].map((raw) => {
        const index = raw.indexOf("=");
        if (index < 0) fail(`--artifact-identity must be name=value; got ${raw}`);
        return [raw.slice(0, index), raw.slice(index + 1)];
      }),
    ) },
    startedAt: single(flags, "started-at") ?? now,
    endedAt: single(flags, "ended-at") ?? now,
    workloadIds: flags.values.get("workload"),
    processesBefore: (flags.values.get("process-before") ?? []).map((raw) => {
      const separator = raw.indexOf("=");
      if (separator < 1 || separator === raw.length - 1) fail(`process identity must be target=identity; got ${raw}`);
      return { target: raw.slice(0, separator), identity: raw.slice(separator + 1) };
    }),
    processesAfter: (flags.values.get("process-after") ?? []).map((raw) => {
      const separator = raw.indexOf("=");
      if (separator < 1 || separator === raw.length - 1) fail(`process identity must be target=identity; got ${raw}`);
      return { target: raw.slice(0, separator), identity: raw.slice(separator + 1) };
    }),
    faultObservedAt: single(flags, "fault-observed-at"),
    recoveryObservedAt: single(flags, "recovery-observed-at"),
    assertions,
    cleanup: {
      state: (single(flags, "cleanup") ?? "not_required") as CaseResult["cleanup"]["state"],
      detail: single(flags, "cleanup-detail"),
    },
  };

  for (const [key, value] of Object.entries(sharedIdentities)) {
    if (record.artifactIdentities?.[key] !== value) fail(`record cannot override observed run identity ${key}`);
  }

  if (state === "PASS" && sharedIdentities.checkout_build_receipt) {
    imageBindings(validateBuildReceipt(JSON.parse(sharedIdentities.checkout_build_receipt)), sharedIdentities);
  }

  // Round-trip through the parser so a record that the aggregate would reject
  // is refused at write time, where the runner can still say something useful.
  parseCaseResult(JSON.parse(JSON.stringify(record)));
  const missing = missingAssertionKinds(entry!, record);
  if (missing.length) fail(`${caseId}: missing passing inventory assertion kinds: ${missing.join(", ")}`);
  const file = appendCaseResult(record, single(flags, "results-dir") ?? resultsDirectory());
  if (!flags.booleans.has("quiet")) console.log(`recorded ${caseId} ${state} -> ${file}`);
};

const runAggregate = (flags: Flags) => {
  const inventory = loadInventory();
  const runId = single(flags, "run");
  const directory = single(flags, "results-dir") ?? resultsDirectory();
  const target = runId ? path.join(directory, `${runId}.jsonl`) : directory;
  if (!existsSync(target)) fail(`no results at ${target}; a run that wrote nothing has not passed`);
  const results = readCaseResults(target);

  const terms = flags.values.get("select") ?? [];
  const selection = selectCases(inventory, {
    terms,
    backend: resolveBackend(single(flags, "backend")),
    capabilities: flags.values.has("capability") ? flags.values.get("capability") : undefined,
    includeDeferred: flags.booleans.has("include-deferred"),
  });

  const buildMode = single(flags, "require-build-mode");
  if (buildMode && buildMode !== "checkout") fail("--require-build-mode must be checkout");
  const report = aggregate({
    inventory,
    results,
    selected: selection.cases,
    revision: single(flags, "revision"),
    partial: flags.booleans.has("partial"),
    ci: flags.booleans.has("ci"),
    requireBuildMode: single(flags, "require-build-mode") === "checkout" ? "checkout" : undefined,
  });

  console.log(`aggregate over ${results.length} result record(s) for ${selection.cases.length} selected case(s)`);
  for (const line of report.lines) console.log(line);
  for (const entry of selection.unavailable) {
    console.log(`  ${entry.case.id.padEnd(32)} NOT_APPLICABLE   (missing ${entry.missing.join(", ")})`);
  }
  if (report.partial) {
    console.log("PARTIAL: this selection is a subset of the required inventory and cannot satisfy the full gate");
  }
  if (report.problems.length > 0) {
    console.error("\nthe run does not stand:");
    for (const problem of report.problems) console.error(`  - ${problem}`);
    process.exit(1);
  }
  console.log(report.partial ? "\nselected cases all pass (PARTIAL run)" : "\nfull required inventory passes");
};

const main = () => {
  const [command, ...rest] = process.argv.slice(2);
  const flags = parseFlags(rest);
  try {
    switch (command) {
      case "validate":
        return runValidate();
      case "list":
        return runList(flags);
      case "select":
        return runSelect(flags);
      case "plan":
        return runPlan(flags);
      case "show":
        return runShow(flags);
      case "record":
        return runRecord(flags);
      case "aggregate":
        return runAggregate(flags);
      default:
        return void fail(
          "usage: consensus-inventory.ts <validate|list|select|plan|show|record|aggregate> [options]",
        );
    }
  } catch (error) {
    if (error instanceof InventoryError || error instanceof ResultError) fail(error.message);
    throw error;
  }
};

main();
