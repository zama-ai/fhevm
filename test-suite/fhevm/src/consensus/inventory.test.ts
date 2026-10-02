/**
 * Contract tests for the inventory and the aggregate (inventory case
 * HAR-02-INVENTORY-AGGREGATE).
 *
 * These exercise the rejection rules rather than the happy path, because the
 * whole value of the aggregate is that it refuses a run which looks complete
 * and is not. Each test corrupts exactly one thing and requires the specific
 * refusal.
 */
import { describe, expect, it } from "bun:test";

import {
  type Inventory,
  InventoryError,
  loadInventory,
  parseInventory,
  selectCases,
} from "./inventory";
import {
  type CaseResult,
  RESULT_SCHEMA_VERSION,
  ResultError,
  aggregate,
  assertNoSecrets,
  parseCaseResult,
} from "./results";

const MINIMAL = `
schema_version: 2
families:
  demo:
    description: A family for tests.
cases:
  - id: DEMO-01-AGREE
    family: demo
    property: Operators agree.
    topology: { scenario: three-of-three, operators: 3, threshold: 3, backends: [cpu] }
    services: [tfhe-worker]
    capabilities: []
    workload: One add.
    fault: { mechanism: none, stage: none, observation: none }
    participants: { compute: all, submit: all, quorum: required }
    assertions:
      - bytes: identical
    timeout_s: 60
    cleanup: [none]
    runner: scripts/demo.sh
    ci: { leg: demo-leg }
    artifacts: [case-results]
    acceptance: required
`;

const withCase = (patch: string) => `${MINIMAL}${patch}`;

const demoResult = (overrides: Partial<CaseResult> = {}): CaseResult => ({
  schemaVersion: RESULT_SCHEMA_VERSION,
  runId: "run-1",
  caseId: "DEMO-01-AGREE",
  state: "PASS",
  revision: "abc123",
  executionClass: { software: "abc123", backend: "cpu", hardware: "cpu-x86_64" },
  topology: { scenario: "three-of-three", operators: 3, threshold: 3 },
  startedAt: "2026-09-09T10:00:00.000Z",
  endedAt: "2026-09-09T10:01:00.000Z",
  assertions: [{ name: "bytes", outcome: "pass" }],
  cleanup: { state: "not_required" },
  ...overrides,
});

describe("inventory validation", () => {
  it("accepts a minimal well-formed document", () => {
    const inventory = parseInventory(MINIMAL);
    expect(inventory.cases).toHaveLength(1);
    expect(inventory.cases[0].participants.quorum).toBe("required");
  });

  it("rejects a case id that a runner could not select", () => {
    expect(() => parseInventory(MINIMAL.replace("DEMO-01-AGREE", "demo one"))).toThrow(InventoryError);
  });

  it("rejects a fault with no independent observation", () => {
    const document = MINIMAL.replace(
      "fault: { mechanism: none, stage: none, observation: none }",
      "fault: { mechanism: kill, stage: work acquired, observation: none }",
    );
    expect(() => parseInventory(document)).toThrow(/cannot be distinguished from a no-op injection/);
  });

  it("rejects a deferred case with no stated boundary", () => {
    const document = MINIMAL.replace("acceptance: required", "acceptance: deferred");
    expect(() => parseInventory(document)).toThrow(/exact boundary and a reason/);
  });

  it("rejects a quorum mode outside the declared set", () => {
    const document = MINIMAL.replace("quorum: required", "quorum: probably");
    expect(() => parseInventory(document)).toThrow(InventoryError);
  });

  it("rejects a threshold above the operator count", () => {
    const document = MINIMAL.replace("operators: 3, threshold: 3", "operators: 2, threshold: 3");
    expect(() => parseInventory(document)).toThrow(/threshold 3 above operators 2/);
  });

  it("rejects a case with no assertions, which asserts nothing by construction", () => {
    const document = MINIMAL.replace("    assertions:\n      - bytes: identical\n", "    assertions: []\n");
    expect(() => parseInventory(document)).toThrow(/must state at least one contract/);
  });

  it("rejects a duplicate case id", () => {
    const document = withCase(`
  - id: DEMO-01-AGREE
    family: demo
    property: A second row claiming the same id.
    topology: { scenario: three-of-three, operators: 3, threshold: 3, backends: [cpu] }
    services: []
    capabilities: []
    workload: One add.
    fault: { mechanism: none, stage: none, observation: none }
    participants: { compute: all, submit: all, quorum: required }
    assertions: [bytes identical]
    timeout_s: 60
    cleanup: [none]
    runner: scripts/demo.sh
    ci: { leg: demo-leg }
    artifacts: []
    acceptance: required
`);
    expect(() => parseInventory(document)).toThrow(/appears more than once/);
  });

  it("rejects a family declared with no cases, so a leg cannot select an empty set", () => {
    const document = MINIMAL.replace(
      "families:\n  demo:",
      "families:\n  orphan:\n    description: Nothing uses this.\n  demo:",
    );
    expect(() => parseInventory(document)).toThrow(/declared but has no cases/);
  });

  it("the `full` selection covers every required case", () => {
    // The selection named full must mean full. `--column all` was documented as
    // omitting the database cells, so the only selection a reader would reach
    // for could not satisfy the delivery gate -- and nothing said so.
    const inventory = loadInventory();
    const full = inventory.selections.get("full");
    expect(full, "the inventory must define a `full` selection").toBeDefined();
    const selected = new Set(selectCases(inventory, { terms: ["full"] }).cases.map((entry) => entry.id));
    const missing = inventory.cases
      .filter((entry) => entry.acceptance === "required")
      .filter((entry) => !selected.has(entry.id))
      .map((entry) => entry.id);
    expect(missing, `the full selection omits required case(s): ${missing.join(", ")}`).toHaveLength(0);
  });

  it("every named selection resolves", () => {
    const inventory = loadInventory();
    for (const name of inventory.selections.keys()) {
      expect(() => selectCases(inventory, { terms: [name] }), `selection ${name}`).not.toThrow();
    }
  });

  it("loads the checked-in inventory", () => {
    const inventory = loadInventory();
    expect(inventory.cases.length).toBeGreaterThan(20);
    // Every required case must name a runner that could execute it.
    for (const entry of inventory.cases) {
      expect(entry.runner.length, `${entry.id} has no runner`).toBeGreaterThan(0);
    }
    // Every deferred case must state its boundary; the parser enforces it, and
    // this keeps the checked-in file honest as it grows.
    for (const entry of inventory.cases.filter((row) => row.acceptance === "deferred")) {
      expect(entry.deferral?.boundary.length, `${entry.id} deferral has no boundary`).toBeGreaterThan(0);
    }
  });
});

describe("case selection", () => {
  const inventory = parseInventory(MINIMAL);

  it("selects by id, family and leg", () => {
    expect(selectCases(inventory, { terms: ["DEMO-01-AGREE"] }).cases).toHaveLength(1);
    expect(selectCases(inventory, { terms: ["demo"] }).cases).toHaveLength(1);
    expect(selectCases(inventory, { terms: ["demo-leg"] }).cases).toHaveLength(1);
  });

  it("unions a term that names both a family and a leg", () => {
    // `degraded` and `crash-retry` are both a family and a CI leg in the real
    // inventory, and a first-match rule dropped the cases that belong to the
    // leg without belonging to the family -- including the canary the whole
    // suite's credibility rests on: MAT-04 is filed under `materialization`
    // and runs in the `degraded` leg.
    const real = loadInventory();
    const degraded = selectCases(real, { terms: ["degraded"] }).cases.map((entry) => entry.id);
    expect(degraded).toContain("MAT-04-CANARY-COMPUTE-DIGEST");
    expect(degraded).toContain("DEG-01-AGREEMENT-QUORUM");
    const crashRetry = selectCases(real, { terms: ["crash-retry"] }).cases.map((entry) => entry.id);
    expect(crashRetry).toContain("REG-03-SUPERVISED-DAEMON-RECOVERY");
    expect(crashRetry).toContain("CR-01-INTERRUPT-BEFORE-COMMIT");
  });

  it("rejects an unknown term rather than selecting nothing", () => {
    expect(() => selectCases(inventory, { terms: ["stall"] })).toThrow(/matches no case id, family or CI leg/);
  });

  it("reports a case as unavailable rather than dropping it when a capability is missing", () => {
    const document = MINIMAL.replace("capabilities: []", "capabilities: [gpu]");
    const withGpu = parseInventory(document);
    const selection = selectCases(withGpu, { terms: ["all"], capabilities: [] });
    expect(selection.cases).toHaveLength(0);
    expect(selection.unavailable[0].missing).toEqual(["gpu"]);
  });

  it("filters by backend, and a backend with no cases is an error rather than an empty green", () => {
    expect(selectCases(inventory, { terms: ["all"], backend: "cpu" }).cases).toHaveLength(1);
    expect(() => selectCases(inventory, { terms: ["all"], backend: "gpu" })).toThrow(/resolved to zero cases/);
  });

  it("excludes deferred cases unless asked for them", () => {
    const document = MINIMAL.replace(
      "acceptance: required",
      "acceptance: deferred\n    deferral:\n      boundary: Not delivered here.\n      reason: Needs hardware.",
    );
    const deferred = parseInventory(document);
    expect(() => selectCases(deferred, { terms: ["all"] })).toThrow(/resolved to zero cases/);
    expect(selectCases(deferred, { terms: ["all"], includeDeferred: true }).cases).toHaveLength(1);
  });
});

describe("result records", () => {
  it("requires a detail for every non-PASS state", () => {
    expect(() => parseCaseResult({ ...demoResult(), state: "FAIL" })).toThrow(/must carry a detail/);
    expect(() => parseCaseResult({ ...demoResult(), state: "FAIL", detail: "bytes differed" })).not.toThrow();
  });

  it("rejects a record with no revision, which cannot be tied to code", () => {
    const record = { ...demoResult() } as Record<string, unknown>;
    delete record.revision;
    expect(() => parseCaseResult(record)).toThrow(/missing revision/);
  });

  it("rejects a record carrying credentials", () => {
    expect(() => assertNoSecrets({ databasePassword: "hunter2" })).toThrow(ResultError);
    expect(() => assertNoSecrets({ url: "postgresql://postgres:postgres@db:5432/coprocessor" })).toThrow(
      /embeds a database password/,
    );
    expect(() => assertNoSecrets({ url: "postgresql://db:5432/coprocessor" })).not.toThrow();
  });
});

describe("aggregate", () => {
  const inventory: Inventory = parseInventory(MINIMAL);
  const selected = inventory.cases;

  it("passes a complete run", () => {
    const report = aggregate({ inventory, results: [demoResult()], selected, revision: "abc123" });
    expect(report.ok).toBe(true);
    expect(report.partial).toBe(false);
  });

  it("rejects a selected case that produced no result", () => {
    const report = aggregate({ inventory, results: [], selected });
    expect(report.ok).toBe(false);
    expect(report.problems.join(" ")).toMatch(/NOT_RUN, not passed/);
    expect(report.states.get("DEMO-01-AGREE")).toBe("NOT_RUN");
  });

  it("rejects an empty selection", () => {
    const report = aggregate({ inventory, results: [], selected: [] });
    expect(report.ok).toBe(false);
    expect(report.problems.join(" ")).toMatch(/empty selection is not a passing aggregate/);
  });

  it("rejects a result for a case the inventory does not have", () => {
    const report = aggregate({
      inventory,
      results: [demoResult(), demoResult({ caseId: "GHOST-01" })],
      selected,
    });
    expect(report.problems.join(" ")).toMatch(/unknown case GHOST-01/);
  });

  it("rejects conflicting duplicate results and keeps the worse state", () => {
    const report = aggregate({
      inventory,
      results: [demoResult(), demoResult({ state: "FAIL", detail: "bytes differed" })],
      selected,
    });
    expect(report.ok).toBe(false);
    expect(report.problems.join(" ")).toMatch(/conflicting states/);
    expect(report.states.get("DEMO-01-AGREE")).toBe("FAIL");
  });

  it("rejects a bare PASS even when the case declares no fault mechanism", () => {
    for (const assertions of [[], [{name: "bytes", outcome: "not_evaluated" as const}]]) {
      const report = aggregate({inventory, results: [demoResult({assertions})], selected});
      expect(report.ok).toBe(false);
      expect(report.problems.join(" ")).toMatch(/without a passing assertion/);
    }
  });

  it("rejects a result produced against a different revision", () => {
    const report = aggregate({ inventory, results: [demoResult()], selected, revision: "def456" });
    expect(report.problems.join(" ")).toMatch(/produced against revision abc123/);
  });

  it("rejects a result produced on a different topology", () => {
    const report = aggregate({
      inventory,
      results: [demoResult({ topology: { scenario: "two-of-three", operators: 3, threshold: 2 } })],
      selected,
    });
    expect(report.problems.join(" ")).toMatch(/ran on scenario two-of-three/);
  });

  it("accepts a NOT_APPLICABLE recorded on a topology the case is not declared under", () => {
    // The whole point of NOT_APPLICABLE is a stack saying "I cannot establish
    // this". The topology it carries is the stack that declined, so treating
    // that as a topology violation would make it impossible to record the
    // decline at all -- and silence is what the inventory exists to prevent.
    const report = aggregate({
      inventory,
      results: [
        demoResult({
          state: "NOT_APPLICABLE",
          detail: "this stack is two-of-three, which cannot establish it",
          topology: { scenario: "two-of-three", operators: 3, threshold: 2 },
        }),
      ],
      selected,
    });
    expect(report.problems.join(" ")).not.toMatch(/scenario|ran at/);
  });

  it("requires a scenario-none PASS to report how much of it ran", () => {
    // A contract suite's evidence is not a fault timestamp -- its fault is
    // injected in-process and the assertion that catches it is the observation
    // -- it is that the tests ran at all. `cargo test` with a filter matching
    // nothing exits 0 having established nothing.
    const stackless = parseInventory(MINIMAL.replace("scenario: three-of-three, operators: 3, threshold: 3", "scenario: none, operators: 0, threshold: 0"));
    const bare = aggregate({
      inventory: stackless,
      results: [demoResult({ topology: { scenario: "none", operators: 0, threshold: 0 } })],
      selected,
    });
    expect(bare.problems.join(" ")).toMatch(/without reporting how much of it ran/);

    const counted = aggregate({
      inventory: stackless,
      results: [
        demoResult({
          topology: { scenario: "none", operators: 0, threshold: 0 },
          artifactIdentities: { tests_run: "34" },
        }),
      ],
      selected,
    });
    expect(counted.problems).toHaveLength(0);
  });

  it("rejects results earned with an undeclared execution backend", () => {
    const report = aggregate({ inventory, selected, results: [demoResult({
      executionClass: { software: "abc123", backend: "gpu", hardware: "H100" },
    })] });
    expect(report.problems.join(" ")).toMatch(/backend gpu.*requires cpu/);
  });

  it("accepts the CUDA execution class for a GPU case", () => {
    const gpuInventory = parseInventory(MINIMAL.replace("backends: [cpu]", "backends: [gpu]"));
    const report = aggregate({ inventory: gpuInventory, selected: gpuInventory.cases, results: [demoResult({
      executionClass: { software: "abc123", backend: "gpu-cuda", hardware: "H100" },
    })] });
    expect(report.problems).toHaveLength(0);
  });

  it("lets a real outcome stand over a stack that declined the case", () => {
    // The full run is a union of sessions: the three-of-three session declines
    // DEG-03 because its threshold cannot exercise it, and the two-of-three
    // session establishes it. That pair is the expected shape of a complete
    // inventory, not a disagreement.
    const report = aggregate({
      inventory,
      results: [
        demoResult({
          state: "NOT_APPLICABLE",
          detail: "this stack is two-of-three, which cannot establish it",
          topology: { scenario: "two-of-three", operators: 3, threshold: 2 },
        }),
        demoResult(),
      ],
      selected,
    });
    expect(report.problems).toHaveLength(0);
    expect(report.lines.join(" ")).toMatch(/DEMO-01-AGREE\s+PASS/);
  });

  it("still reports two real outcomes that disagree", () => {
    const report = aggregate({
      inventory,
      results: [demoResult(), demoResult({ state: "FAIL", detail: "it failed" })],
      selected,
    });
    expect(report.problems.join(" ")).toMatch(/conflicting states/);
    expect(report.lines.join(" ")).toMatch(/DEMO-01-AGREE\s+FAIL/);
  });

  it("rejects a PASS with a failed cleanup", () => {
    const report = aggregate({
      inventory,
      results: [demoResult({ cleanup: { state: "failed", detail: "container left paused" } })],
      selected,
    });
    expect(report.problems.join(" ")).toMatch(/failed cleanup/);
  });

  it("rejects a PASS whose assertion list contains a failure", () => {
    const report = aggregate({
      inventory,
      results: [demoResult({ assertions: [{ name: "bytes", outcome: "fail", detail: "differed" }] })],
      selected,
    });
    expect(report.problems.join(" ")).toMatch(/failed assertion/);
  });

  it("rejects a fault case that passed without a fault observation or a named workload", () => {
    const faulty = parseInventory(
      MINIMAL.replace(
        "fault: { mechanism: none, stage: none, observation: none }",
        "fault: { mechanism: kill, stage: work acquired, observation: process death and restart }",
      ),
    );
    const report = aggregate({ inventory: faulty, results: [demoResult()], selected: faulty.cases });
    expect(report.problems.join(" ")).toMatch(/without a fault observation timestamp/);
    expect(report.problems.join(" ")).toMatch(/without naming the workload/);

    const complete = aggregate({
      inventory: faulty,
      results: [
        demoResult({ faultObservedAt: "2026-09-09T10:00:30.000Z", workloadIds: ["0xdeadbeef"] }),
      ],
      selected: faulty.cases,
    });
    expect(complete.ok).toBe(true);
  });

  it("labels a subset run partial and refuses to call it the full gate", () => {
    const two = parseInventory(
      withCase(`
  - id: DEMO-02-QUORUM
    family: demo
    property: Quorum forms.
    topology: { scenario: three-of-three, operators: 3, threshold: 3, backends: [cpu] }
    services: []
    capabilities: []
    workload: One add.
    fault: { mechanism: none, stage: none, observation: none }
    participants: { compute: all, submit: all, quorum: required }
    assertions: [quorum forms]
    timeout_s: 60
    cleanup: [none]
    runner: scripts/demo.sh
    ci: { leg: demo-leg }
    artifacts: []
    acceptance: required
`),
    );
    const subset = two.cases.filter((entry) => entry.id === "DEMO-01-AGREE");
    const undeclared = aggregate({ inventory: two, results: [demoResult()], selected: subset });
    expect(undeclared.ok).toBe(false);
    expect(undeclared.problems.join(" ")).toMatch(/omits 1 required case/);

    const declared = aggregate({
      inventory: two,
      results: [demoResult()],
      selected: subset,
      partial: true,
    });
    expect(declared.ok).toBe(true);
    expect(declared.partial).toBe(true);
  });

  it("requires PASS for required cases and permits declared absence only for optional cases", () => {
    const capable = parseInventory(MINIMAL.replace("capabilities: []", "capabilities: [two-gpus]"));
    const allowed = aggregate({
      inventory: capable,
      results: [demoResult({ state: "NOT_APPLICABLE", detail: "no second GPU on this runner" })],
      selected: capable.cases,
    });
    expect(allowed.ok).toBe(false);
    expect(allowed.problems.join(" ")).toMatch(/NOT_APPLICABLE/);
    const optional = parseInventory(MINIMAL.replace("acceptance: required", "acceptance: smoke"));
    const optionalReport = aggregate({
      inventory: optional, selected: optional.cases,
      results: [demoResult({ state: "NOT_APPLICABLE", detail: "optional service absent" })],
    });
    expect(optionalReport.ok).toBe(true);

    const bare = parseInventory(MINIMAL.replace("services: [tfhe-worker]", "services: []"));
    const refused = aggregate({
      inventory: bare,
      results: [demoResult({ state: "NOT_APPLICABLE", detail: "claimed inapplicable" })],
      selected: bare.cases,
    });
    expect(refused.ok).toBe(false);
    expect(refused.problems.join(" ")).toMatch(/declares no capability or service that could be absent/);
  });
});
