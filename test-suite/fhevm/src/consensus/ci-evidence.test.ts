import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync, mkdirSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import YAML from "yaml";
import { loadInventory, selectCases, type InventoryCase } from "./inventory";
import { receiptArtifacts } from "./build-provenance";
import { aggregate, type CaseResult, RESULT_SCHEMA_VERSION } from "./results";
const cli = path.resolve(import.meta.dir, "../..");
const workflow = YAML.parse(readFileSync(path.join(cli, "../../.github/workflows/test-suite-consensus.yml"), "utf8"));
const inventory = loadInventory();
const resultFor = (entry: InventoryCase): CaseResult => ({
  schemaVersion: RESULT_SCHEMA_VERSION, runId: "test", caseId: entry.id, state: "PASS", revision: "branch-sha",
  executionClass: { software: "checkout:branch-sha", backend: entry.ci.backend, hardware: "fixture" },
  topology: entry.topology, startedAt: "2026-09-13T10:00:00Z", endedAt: "2026-09-13T10:01:00Z",
  assertions: [...new Set(entry.assertions.map((item) => item.split(":", 1)[0]))].map((name) => ({ name, outcome: "pass" })),
  artifactIdentities: { build_mode: "checkout" }, cleanup: { state: "ok" },
});
const entry = inventory.cases.find((item) => item.id === "MAT-01-BOUNDARY-FANOUT")!;
test("unrelated or incomplete assertion outcomes cannot satisfy inventory contracts", () => {
  const good = resultFor(entry);
  expect(aggregate({ inventory, results: [good], selected: [entry], partial: true }).ok).toBe(true);
  for (const assertions of [[{ name: "anything", outcome: "pass" as const }], good.assertions.slice(1),
    good.assertions.map((item, index) => index ? item : { ...item, outcome: "not_evaluated" as const })]) {
    const report = aggregate({ inventory, results: [{ ...good, assertions }], selected: [entry], partial: true });
    expect(report.ok).toBe(false);
    expect(report.problems.join("\n")).toContain("missing passing inventory assertion kinds");
  }
});
test("a checkout SHA with published binaries cannot satisfy branch validation", () => {
  const good = resultFor(entry);
  for (const build_mode of [undefined, "published"]) {
    const report = aggregate({ inventory, results: [{ ...good, artifactIdentities: build_mode ? { build_mode } : {} }],
      selected: [entry], partial: true, revision: "branch-sha", requireBuildMode: "checkout" });
    expect(report.ok).toBe(false);
    expect(report.problems.join("\n")).toContain("branch validation requires build_mode=checkout");
  }
  expect(aggregate({ inventory, results: [good], selected: [entry], partial: true, requireBuildMode: "checkout" }).ok).toBe(false);
  good.revision = "a".repeat(40);
  const images = [{ ref: "suite:local", id: `sha256:${"1".repeat(64)}`, group: "test-suite" }, { ref: "worker:local", id: `sha256:${"2".repeat(64)}`, group: "coprocessor" }];
  const observed = { "image_fhevm-test-suite-e2e-debug": `${images[0].id} (suite:local)`, "image_coprocessor-tfhe-worker": `${images[1].id} (worker:local)` };
  good.artifactIdentities = { build_mode: "checkout", ...observed, ...receiptArtifacts({ revision: good.revision, mode: "checkout", features: "none", startedAt: good.startedAt, completedAt: good.endedAt, images }, observed) };
  expect(aggregate({ inventory, results: [good], selected: [entry], partial: true, requireBuildMode: "checkout" }).ok).toBe(true);
});
test("CI requires its planned backend; compatible alternatives are not additional coverage", () => {
  const result = resultFor(entry); result.executionClass.backend = "gpu-cuda";
  expect(aggregate({ inventory, results: [result], selected: [entry], partial: true }).ok).toBe(true);
  const report = aggregate({ inventory, results: [result], selected: [entry], partial: true, ci: true });
  expect(report.ok).toBe(false); expect(report.problems.join("\n")).toContain("CI requires backend cpu");
});
test("core degraded CI outcomes require the observed default auto-revert setting", () => {
  const core = inventory.cases.find((item) => item.id === "DEG-01-AGREEMENT-QUORUM")!;
  const result = resultFor(core);
  for (const drift_auto_revert of [undefined, "false"]) {
    result.artifactIdentities = drift_auto_revert ? { drift_auto_revert } : {};
    const report = aggregate({ inventory, results: [result], selected: [core], partial: true, ci: true });
    expect(report.ok).toBe(false); expect(report.problems.join("\n")).toContain("observed drift_auto_revert=true");
  }
  result.artifactIdentities = { drift_auto_revert: "true" };
  expect(aggregate({ inventory, results: [result], selected: [core], partial: true, ci: true }).ok).toBe(true);
});
test("standard covers every CPU and stackless case; DEG06 has an isolated job", () => {
  const selected = new Set(selectCases(inventory, { terms: ["standard"] }).cases.map((item) => item.id));
  for (const item of inventory.cases.filter((item) => item.acceptance !== "deferred" && item.ci.backend !== "gpu")) expect(selected.has(item.id), item.id).toBe(true);
  const output = Bun.spawnSync([process.execPath, "scripts/consensus-inventory.ts", "plan", "standard"], { cwd: cli });
  expect(output.exitCode, output.stderr.toString()).toBe(0);
  const plan = JSON.parse(output.stdout.toString()); expect(plan.needsGpu).toBe(false);
  for (const leg of ["fork", "failure-matrix-db", "rust-regression"]) expect(plan.legs).toContain(leg);
  const jobs = plan.jobs.filter((job: any) => job.cases.includes("DEG-06-GW-LISTENER-INFLIGHT"));
  expect(jobs).toHaveLength(1); expect(jobs[0].cases).toEqual(["DEG-06-GW-LISTENER-INFLIGHT"]);
  expect(jobs[0].shard).toBe("gateway");
  expect(plan.jobs.every((job: any) => job.backend === "cpu" || job.backend === "none")).toBe(true);
});
test("workflow verdict and identity pipelines preserve an upstream failure", () => {
  const fixture = mkdtempSync(path.join(os.tmpdir(), "consensus-pipeline-"));
  try {
    mkdirSync(path.join(fixture, "scripts"));
    mkdirSync(path.join(fixture, "consensus-inputs"));
    writeFileSync(path.join(fixture, "scripts/record-run-identity.sh"), "#!/bin/bash\nexit 37\n", { mode: 0o755 });
    let checked = 0;
    for (const job of Object.values(workflow.jobs) as any[]) for (const step of job.steps ?? []) {
      if (!step.run || !/record-run-identity\.sh \| tee|consensus-inventory\.ts aggregate[\s\S]*\| tee/.test(step.run)) continue;
      const shell = step.shell ?? job.defaults?.run?.shell ?? workflow.defaults?.run?.shell;
      const flags = shell === "bash" ? ["--noprofile", "--norc", "-eo", "pipefail"] : ["-e"];
      const command = step.run.replaceAll("/tmp/consensus-", `${fixture}/consensus-`);
      const output = Bun.spawnSync(["bash", ...flags, "-c", `bun() { return 37; }; ${command}`], {
        cwd: fixture, env: { ...process.env, GITHUB_ENV: path.join(fixture, "env"), LEG: "degraded", SCENARIO: "three-of-three",
          CASES: "DEG-01-AGREEMENT-QUORUM", CONSENSUS_RUN_ID: "test", SELECTION: "harness", REVISION: "branch", PARTIAL: "true", REQUIRE_BRANCH_BUILD: "true" },
      });
      expect(output.exitCode, `${step.name}: ${output.stderr.toString()}`).toBe(37); checked++;
    }
    expect(checked).toBe(7);
  } finally { rmSync(fixture, { recursive: true, force: true }); }
});
test("a harness dispatch cannot cancel full branch validation", () => {
  const group = workflow.concurrency.group;
  const resolve = (selection: string, build: boolean) => group.replace("${{ inputs.selection }}", selection).replace("${{ inputs.build }}", String(build));
  expect(resolve("harness", true)).not.toBe(resolve("full", true));
  expect(resolve("full", false)).not.toBe(resolve("full", true));
});

test("runner summaries cannot invent assertion outcomes from a case ID", () => {
  const output = Bun.spawnSync(["bash", "-c", 'source scripts/lib/runner-assertions.sh; cr_record() { printf "%s\\n" "$@"; }; cr_record_checked_pass MAT-01-BOUNDARY-FANOUT'], { cwd: cli });
  expect(output.exitCode).toBe(0);
  expect(output.stdout.toString()).not.toContain("assert=");
});
