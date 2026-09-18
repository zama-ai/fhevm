/**
 * The coverage inventory: loader, validator and selector.
 *
 * `consensus/inventory.yaml` is the single description of what the consensus
 * suite covers. Everything that selects cases, runs them, or reports on them
 * reads it from here, so a case cannot exist in a runner without existing in
 * the plan, and a CI selection cannot quietly resolve to zero work.
 *
 * Validation is deliberately strict rather than tolerant. A malformed row is a
 * defect in the plan, and the failure mode of accepting it -- a case that looks
 * covered and is not -- is exactly what this file exists to prevent.
 */
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { parse as parseYaml } from "yaml";

/** Result states, in the order the plan defines them. */
export const CASE_STATES = ["PASS", "FAIL", "INVALID", "NOT_APPLICABLE", "NOT_RUN"] as const;
export type CaseState = (typeof CASE_STATES)[number];

/** States that count towards a case's acceptance. */
export const ACCEPTING_STATES: readonly CaseState[] = ["PASS"];

export const QUORUM_MODES = ["required", "forbidden", "not_checked"] as const;
export type QuorumMode = (typeof QUORUM_MODES)[number];

export const ACCEPTANCE = ["required", "smoke", "deferred"] as const;
export type Acceptance = (typeof ACCEPTANCE)[number];

export const BACKENDS = ["cpu", "gpu", "none"] as const;
export type Backend = (typeof BACKENDS)[number];

export interface CaseTopology {
  scenario: string;
  operators: number;
  threshold: number;
  backends: Backend[];
}

export interface CaseFault {
  mechanism: string;
  stage: string;
  observation: string;
}

export interface CaseParticipants {
  compute: string;
  submit: string;
  quorum: QuorumMode;
}

export interface InventoryCase {
  id: string;
  family: string;
  property: string;
  topology: CaseTopology;
  services: string[];
  capabilities: string[];
  workload: string;
  fault: CaseFault;
  participants: CaseParticipants;
  assertions: string[];
  timeoutSeconds: number;
  cleanup: string[];
  runner: string;
  ci: { leg: string; backend: Backend };
  artifacts: string[];
  acceptance: Acceptance;
  deferral?: { boundary: string; reason: string };
}

export interface Inventory {
  schemaVersion: number;
  families: Map<string, string>;
  /** Named selections, expanded to inventory terms. */
  selections: Map<string, string[]>;
  cases: InventoryCase[];
}

export class InventoryError extends Error {
  constructor(message: string) {
    super(`consensus inventory: ${message}`);
    this.name = "InventoryError";
  }
}

const CASE_ID = /^[A-Z][A-Z0-9]*(-[A-Z0-9]+)+$/;

const requireString = (value: unknown, where: string): string => {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new InventoryError(`${where} must be a non-empty string`);
  }
  return value.trim();
};

const requireStringList = (value: unknown, where: string): string[] => {
  if (!Array.isArray(value)) throw new InventoryError(`${where} must be a list`);
  return value.map((entry, index) => {
    if (typeof entry === "string") return entry.trim();
    // An assertion row may be written as `kind: text` for readability; both
    // shapes carry the same information and both are kept verbatim.
    if (entry && typeof entry === "object") {
      const pairs = Object.entries(entry as Record<string, unknown>);
      if (pairs.length === 1 && typeof pairs[0][1] === "string") {
        return `${pairs[0][0]}: ${pairs[0][1].trim()}`;
      }
    }
    throw new InventoryError(`${where}[${index}] must be a string or a single-key mapping`);
  });
};

const requireInteger = (value: unknown, where: string, min = 0): number => {
  if (typeof value !== "number" || !Number.isInteger(value) || value < min) {
    throw new InventoryError(`${where} must be an integer >= ${min}`);
  }
  return value;
};

const requireEnum = <T extends string>(value: unknown, allowed: readonly T[], where: string): T => {
  const text = requireString(value, where);
  if (!(allowed as readonly string[]).includes(text)) {
    throw new InventoryError(`${where} must be one of ${allowed.join(", ")}; got ${text}`);
  }
  return text as T;
};

const parseCase = (raw: unknown, position: number, families: Set<string>): InventoryCase => {
  if (!raw || typeof raw !== "object") throw new InventoryError(`cases[${position}] must be a mapping`);
  const row = raw as Record<string, unknown>;
  const id = requireString(row.id, `cases[${position}].id`);
  if (!CASE_ID.test(id)) {
    throw new InventoryError(`case id ${id} must be upper-kebab (e.g. MAT-01-BOUNDARY), so runners can select it`);
  }
  const where = `case ${id}`;
  const family = requireString(row.family, `${where}.family`);
  if (!families.has(family)) {
    throw new InventoryError(`${where} names family ${family}, which is not declared under families:`);
  }

  const topologyRaw = row.topology;
  if (!topologyRaw || typeof topologyRaw !== "object") throw new InventoryError(`${where}.topology must be a mapping`);
  const topologyRow = topologyRaw as Record<string, unknown>;
  const backends = requireStringList(topologyRow.backends, `${where}.topology.backends`).map((value) =>
    requireEnum(value, BACKENDS, `${where}.topology.backends`),
  );
  if (backends.length === 0) throw new InventoryError(`${where}.topology.backends must name at least one backend`);
  const topology: CaseTopology = {
    scenario: requireString(topologyRow.scenario, `${where}.topology.scenario`),
    operators: requireInteger(topologyRow.operators, `${where}.topology.operators`),
    threshold: requireInteger(topologyRow.threshold, `${where}.topology.threshold`),
    backends,
  };
  if (topology.threshold > topology.operators) {
    throw new InventoryError(`${where} declares threshold ${topology.threshold} above operators ${topology.operators}`);
  }

  const faultRaw = row.fault;
  if (!faultRaw || typeof faultRaw !== "object") throw new InventoryError(`${where}.fault must be a mapping`);
  const faultRow = faultRaw as Record<string, unknown>;
  const fault: CaseFault = {
    mechanism: requireString(faultRow.mechanism, `${where}.fault.mechanism`),
    stage: requireString(faultRow.stage, `${where}.fault.stage`),
    observation: requireString(faultRow.observation, `${where}.fault.observation`),
  };
  // A case that injects a fault must say how the fault is independently
  // observed. "none" is only coherent when no fault is injected.
  if (fault.mechanism !== "none" && fault.observation === "none") {
    throw new InventoryError(
      `${where} injects ${fault.mechanism} but declares no observation; a fault with no independent ` +
        "postcondition cannot be distinguished from a no-op injection",
    );
  }

  const participantsRaw = row.participants;
  if (!participantsRaw || typeof participantsRaw !== "object") {
    throw new InventoryError(`${where}.participants must be a mapping`);
  }
  const participantsRow = participantsRaw as Record<string, unknown>;
  const participants: CaseParticipants = {
    compute: requireString(participantsRow.compute, `${where}.participants.compute`),
    submit: requireString(participantsRow.submit, `${where}.participants.submit`),
    quorum: requireEnum(participantsRow.quorum, QUORUM_MODES, `${where}.participants.quorum`),
  };

  const assertions = requireStringList(row.assertions, `${where}.assertions`);
  if (assertions.length === 0) throw new InventoryError(`${where}.assertions must state at least one contract`);

  const ciRaw = row.ci;
  if (!ciRaw || typeof ciRaw !== "object") throw new InventoryError(`${where}.ci must be a mapping`);
  const leg = requireString((ciRaw as Record<string, unknown>).leg, `${where}.ci.leg`);
  const ciBackend = requireEnum(
    (ciRaw as Record<string, unknown>).backend ?? (leg === "gpu" ? "gpu" : topology.backends[0]),
    BACKENDS, `${where}.ci.backend`,
  );
  if (!topology.backends.includes(ciBackend)) {
    throw new InventoryError(`${where}.ci.backend ${ciBackend} is not supported by its topology`);
  }

  const acceptance = requireEnum(row.acceptance, ACCEPTANCE, `${where}.acceptance`);
  let deferral: InventoryCase["deferral"];
  if (acceptance === "deferred") {
    const deferralRaw = row.deferral;
    if (!deferralRaw || typeof deferralRaw !== "object") {
      throw new InventoryError(
        `${where} is deferred, so it must carry a deferral: with an exact boundary and a reason; ` +
          "unexplained deferral is how coverage silently disappears",
      );
    }
    const deferralRow = deferralRaw as Record<string, unknown>;
    deferral = {
      boundary: requireString(deferralRow.boundary, `${where}.deferral.boundary`),
      reason: requireString(deferralRow.reason, `${where}.deferral.reason`),
    };
  } else if (row.deferral !== undefined) {
    throw new InventoryError(`${where} is ${acceptance} but carries a deferral block`);
  }

  return {
    id,
    family,
    property: requireString(row.property, `${where}.property`),
    topology,
    services: requireStringList(row.services ?? [], `${where}.services`),
    capabilities: requireStringList(row.capabilities ?? [], `${where}.capabilities`),
    workload: requireString(row.workload, `${where}.workload`),
    fault,
    participants,
    assertions,
    timeoutSeconds: requireInteger(row.timeout_s, `${where}.timeout_s`, 1),
    cleanup: requireStringList(row.cleanup ?? [], `${where}.cleanup`),
    runner: requireString(row.runner, `${where}.runner`),
    ci: { leg, backend: ciBackend },
    artifacts: requireStringList(row.artifacts ?? [], `${where}.artifacts`),
    acceptance,
    deferral,
  };
};

/** Parses and validates an inventory document. */
export const parseInventory = (text: string): Inventory => {
  const document = parseYaml(text) as unknown;
  if (!document || typeof document !== "object") throw new InventoryError("the document must be a mapping");
  const root = document as Record<string, unknown>;
  const schemaVersion = requireInteger(root.schema_version, "schema_version", 1);

  const familiesRaw = root.families;
  if (!familiesRaw || typeof familiesRaw !== "object") throw new InventoryError("families must be a mapping");
  const families = new Map<string, string>();
  for (const [name, value] of Object.entries(familiesRaw as Record<string, unknown>)) {
    if (!value || typeof value !== "object") throw new InventoryError(`families.${name} must be a mapping`);
    families.set(name, requireString((value as Record<string, unknown>).description, `families.${name}.description`));
  }

  const selections = new Map<string, string[]>();
  if (root.selections !== undefined) {
    if (!root.selections || typeof root.selections !== "object") {
      throw new InventoryError("selections must be a mapping of name to term list");
    }
    for (const [name, value] of Object.entries(root.selections as Record<string, unknown>)) {
      selections.set(name, requireStringList(value, `selections.${name}`));
    }
  }

  if (!Array.isArray(root.cases)) throw new InventoryError("cases must be a list");
  const familyNames = new Set(families.keys());
  const cases = root.cases.map((raw, index) => parseCase(raw, index, familyNames));

  const seen = new Set<string>();
  for (const entry of cases) {
    if (seen.has(entry.id)) throw new InventoryError(`case id ${entry.id} appears more than once`);
    seen.add(entry.id);
  }
  const usedFamilies = new Set(cases.map((entry) => entry.family));
  for (const name of familyNames) {
    if (!usedFamilies.has(name)) throw new InventoryError(`family ${name} is declared but has no cases`);
  }
  for (const [name, terms] of selections) {
    if (terms.length === 0) throw new InventoryError(`selection ${name} names no terms`);
  }
  return { schemaVersion, families, selections, cases };
};

/** Default on-disk location of the inventory, honoring an explicit override. */
export const inventoryPath = (env: NodeJS.ProcessEnv = process.env): string =>
  env.CONSENSUS_INVENTORY ?? path.join(import.meta.dir, "..", "..", "consensus", "inventory.yaml");

export const loadInventory = (file: string = inventoryPath()): Inventory => {
  if (!existsSync(file)) throw new InventoryError(`no inventory at ${file}`);
  return parseInventory(readFileSync(file, "utf8"));
};

export interface Selection {
  /** Case ids, families or CI legs. `all` selects every non-deferred case. */
  terms: string[];
  /** Restrict to cases runnable on this backend. */
  backend?: Backend;
  /** Restrict to cases whose declared capabilities are all available. */
  capabilities?: string[];
  /** Include cases marked deferred (normally excluded from a selection). */
  includeDeferred?: boolean;
}

export interface SelectionResult {
  cases: InventoryCase[];
  /** Cases excluded because a declared capability is missing. */
  unavailable: { case: InventoryCase; missing: string[] }[];
}

/**
 * Resolves a selection to a concrete case list, rejecting unknown terms.
 *
 * An unknown term is an error rather than an empty selection: a mistyped
 * column or leg previously produced a job that ran nothing and reported
 * success, which is the most expensive kind of green there is.
 */
export const selectCases = (inventory: Inventory, selection: Selection): SelectionResult => {
  const byId = new Map(inventory.cases.map((entry) => [entry.id, entry]));
  const families = new Set(inventory.cases.map((entry) => entry.family));
  const legs = new Set(inventory.cases.map((entry) => entry.ci.leg));

  // A named selection expands to its terms first, so a workflow can pass
  // `standard` and the expansion lives with the inventory rather than in a
  // shell `case` that can drift from it.
  const requested = selection.terms.length > 0 ? selection.terms : ["all"];
  const terms = requested.flatMap((term) => inventory.selections.get(term) ?? [term]);
  const chosen = new Map<string, InventoryCase>();
  for (const term of terms) {
    if (term === "all") {
      for (const entry of inventory.cases) chosen.set(entry.id, entry);
      continue;
    }
    if (term.startsWith("leg:") || term.startsWith("family:")) {
      const [kind, name] = term.split(":");
      const matches = inventory.cases.filter((entry) => kind === "leg" ? entry.ci.leg === name : entry.family === name);
      if (!matches.length) throw new InventoryError(`unknown selection term: ${term}`);
      for (const entry of matches) chosen.set(entry.id, entry);
      continue;
    }
    // A term is matched against ids, families AND legs, and the results are
    // UNIONED rather than resolved by precedence. Several names are both a
    // family and a leg -- `degraded` and `crash-retry` are -- and a
    // first-match rule silently dropped the cases that belong to the leg
    // without belonging to the family: MAT-04's canary is filed under
    // `materialization` and runs in the `degraded` leg, so selecting
    // `degraded` left the load-bearing falsification out of the run.
    let matched = false;
    if (byId.has(term)) {
      chosen.set(term, byId.get(term)!);
      matched = true;
    }
    if (families.has(term)) {
      for (const entry of inventory.cases.filter((row) => row.family === term)) chosen.set(entry.id, entry);
      matched = true;
    }
    if (legs.has(term)) {
      for (const entry of inventory.cases.filter((row) => row.ci.leg === term)) chosen.set(entry.id, entry);
      matched = true;
    }
    if (!matched) {
      throw new InventoryError(
        `selection term "${term}" matches no case id, family or CI leg. Known families: ` +
          `${[...families].sort().join(", ")}; known legs: ${[...legs].sort().join(", ")}`,
      );
    }
  }

  let cases = [...chosen.values()];
  if (!selection.includeDeferred) cases = cases.filter((entry) => entry.acceptance !== "deferred");
  if (selection.backend) {
    const backend = selection.backend;
    cases = cases.filter((entry) => entry.topology.backends.includes(backend));
  }

  const unavailable: SelectionResult["unavailable"] = [];
  if (selection.capabilities) {
    const available = new Set(selection.capabilities);
    const runnable: InventoryCase[] = [];
    for (const entry of cases) {
      const missing = entry.capabilities.filter((capability) => !available.has(capability));
      if (missing.length > 0) unavailable.push({ case: entry, missing });
      else runnable.push(entry);
    }
    cases = runnable;
  }

  if (cases.length === 0 && unavailable.length === 0) {
    throw new InventoryError(
      `selection ${terms.join(",")} resolved to zero cases; an empty selection is not a passing run`,
    );
  }
  return { cases: cases.sort((a, b) => a.id.localeCompare(b.id)), unavailable };
};
