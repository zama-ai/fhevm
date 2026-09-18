import { expect, test } from "bun:test";
import { mkdtempSync, writeFileSync, readFileSync, rmSync } from "node:fs";
import path from "node:path";
import { tmpdir } from "node:os";
import YAML from "yaml";
import { loadInventory, selectCases } from "./inventory";
import { CI_STACK_LEGS, CI_SPECIAL_LEGS, assertCiLeg } from "./ci";
import { receiptArtifacts, verifyCheckoutArtifacts, validateBuildReceipt } from "./build-provenance";
const cli = path.resolve(import.meta.dir, "../..");
const workflow = YAML.parse(readFileSync(path.join(cli, "../../.github/workflows/test-suite-consensus.yml"), "utf8"));
const inventory = loadInventory();
test("inventory legs and workflow dispatch cannot drift silently", () => {
  for (const entry of inventory.cases.filter((c) => c.acceptance !== "deferred")) assertCiLeg(entry.ci.leg);
  const routed = new Set(workflow.jobs.consensus.steps.flatMap((step: any) => [...(step.if ?? "").matchAll(/matrix\.leg == '([^']+)'/g)].map((m: any) => m[1])));
  expect([...routed].sort()).toEqual([...CI_STACK_LEGS].sort());
  for (const leg of CI_SPECIAL_LEGS) expect(workflow.jobs[leg]).toBeDefined();
  expect(() => assertCiLeg("unwired-new-leg")).toThrow("no dispatch");
});
test("explicit leg selection excludes family delegates while family selection retains them", () => {
  const leg = selectCases(inventory, { terms: ["leg:failure-matrix"] }).cases;
  expect(leg.length).toBeGreaterThan(0); expect(leg.every((c) => c.ci.leg === "failure-matrix")).toBe(true);
  const family = selectCases(inventory, { terms: ["family:failure-matrix"] }).cases;
  expect(family.some((c) => c.id === "FM-TFHE-CRASH")).toBe(true);
  expect(family.some((c) => c.id === "FM-UPGRADE-CONTROLLER")).toBe(true);
  expect(workflow.on.workflow_dispatch.inputs.selection.options).toContain("leg:failure-matrix");
});
test("every crash failpoint consumer declares the capability and planner propagates it", () => {
  for (const id of ["CR-01-INTERRUPT-BEFORE-COMMIT", "CR-02-INTERRUPT-AFTER-COMMIT", "CR-03-EXPIRED-LEASE-RECLAIM", "FM-TFHE-CRASH"]) {
    const row = inventory.cases.find((c) => c.id === id)!;
    expect(row, id).toBeDefined(); expect(row.capabilities, id).toContain("failpoints");
  }
  const p = Bun.spawnSync([process.execPath, "scripts/consensus-inventory.ts", "plan", "full"], { cwd: cli });
  expect(p.exitCode, p.stderr.toString()).toBe(0);
  for (const job of JSON.parse(p.stdout.toString()).jobs) {
    const expected = [...new Set(job.cases.flatMap((id: string) => inventory.cases.find((c) => c.id === id)!.capabilities))].sort();
    expect(job.capabilities).toEqual(expected);
  }
});
test("rerun uploads explicitly replace fixed-name artifacts and keep independent legs", () => {
  let count = 0;
  for (const job of Object.values(workflow.jobs) as any[]) for (const step of job.steps ?? []) if (step.uses?.startsWith("actions/upload-artifact@")) {
    expect(step.with.overwrite).toBe(true); count++;
  }
  expect(count).toBe(5);
});
const id = (digit: string) => `sha256:${digit.repeat(64)}`;
const receipt = { revision: "a".repeat(40), mode: "checkout" as const, startedAt: "2026-09-13T00:00:00Z", completedAt: "2026-09-13T00:01:00Z",
  images: [{ ref: "suite:branch", id: id("1"), group: "test-suite" }, { ref: "worker:branch", id: id("2"), group: "coprocessor" }] };
const observed = { "image_fhevm-test-suite-e2e-debug": `${id("1")} (suite:branch)`, "image_coprocessor-tfhe-worker": `${id("2")} (worker:branch)` };
test("checkout labels cannot substitute for immutable image build provenance", () => {
  const good = { build_mode: "checkout", ...observed, ...receiptArtifacts(receipt, observed) };
  expect(() => verifyCheckoutArtifacts(good, receipt.revision)).not.toThrow();
  for (const changed of [{ ...good, "image_coprocessor-tfhe-worker": `${id("3")} (worker:branch)` },
    { ...good, "image_fhevm-test-suite-e2e-debug": `${id("4")} (suite:published)` },
    { build_mode: "checkout" }, { ...good, checkout_build_receipt_sha256: "forged" }]) {
    expect(() => verifyCheckoutArtifacts(changed, receipt.revision)).toThrow();
  }
  expect(() => verifyCheckoutArtifacts(good, "b".repeat(40))).toThrow("stale revision");
  expect(() => validateBuildReceipt({ ...receipt, images: receipt.images.slice(0, 1) })).toThrow("coprocessor");
  expect(() => validateBuildReceipt({ ...receipt, images: [{ ...receipt.images[0], id: "mutable:tag" }] })).toThrow();
});

test("published-runtime CI always builds its harness and failpoints come from planned capability", () => {
  const boot = workflow.jobs.consensus.steps.find((step: any) => step.name === "Boot the stack").run;
  expect(boot).toContain('args+=(--override test-suite)');
  expect(boot).toContain('${CAPABILITIES:-}');
  expect(workflow.jobs.consensus.env.CAPABILITIES).toContain("matrix.capabilities");
  const sql = workflow.jobs.harness.steps.find((step: any) => step.name.includes("fault audit SQL"));
  expect(sql.env.CONSENSUS_ORACLE_TEST_DATABASE_URL).toBe("postgres://postgres@127.0.0.1:5432/postgres");
  expect(sql.run).toContain("faultAuditDatabase.test.ts");
  expect(workflow.jobs.harness.services["oracle-postgres"].image).toBe("postgres:15.7");
});

test("receipt validation rejects coercible nested arrays and unknown image groups", () => {
  for (const value of [[], { ...receipt, revision: [receipt.revision] }, { ...receipt, mode: ["checkout"] },
    { ...receipt, startedAt: [receipt.startedAt] }, { ...receipt, completedAt: "yesterday" },
    { ...receipt, images: [{ ...receipt.images[0], id: [receipt.images[0].id] }] },
    { ...receipt, images: [{ ...receipt.images[0], group: "unowned" }] }, { ...receipt, images: [[]] }]) {
    expect(() => validateBuildReceipt(value)).toThrow();
  }
  const extra = { ...receipt, images: [...receipt.images, { ref: "unobserved-worker:branch", id: id("5"), group: "coprocessor" }] };
  expect(() => receiptArtifacts(extra, observed)).toThrow("no current container observation");
});

test("actual boot branch keeps default features and selectively builds a checkout harness", () => {
  const dir = mkdtempSync(path.join(tmpdir(), "ci-boot-features-"));
  try {
    const capture = path.join(dir, "args");
    writeFileSync(path.join(dir, "fhevm-cli"), '#!/bin/bash\nprintf "%s\\n" "$@" > "$CAPTURE"\nprintf "features=%s\\n" "${FHEVM_CONSENSUS_TEST_FEATURES:-}" >> "$CAPTURE"\n', { mode: 0o755 });
    const boot = workflow.jobs.consensus.steps.find((step: any) => step.name === "Boot the stack").run;
    for (const [build, capabilities, local, features] of [["false", "", false, ""], ["true", "", true, ""],
      ["false", "service-control failpoints", true, "tfhe-worker/test-failpoints"]] as const) {
      const output = Bun.spawnSync(["bash", "-e", "-c", `bun() { [[ "$1" == scripts/checkout-build-receipt.ts ]]; }; ${boot}`], {
        cwd: dir, env: { ...process.env, FHEVM_CONSENSUS_TEST_FEATURES: "", BUILD: build, CAPABILITIES: capabilities,
          SCENARIO: "three-of-three", CASES: "CR-01-INTERRUPT-BEFORE-COMMIT", LOCK_FILE: "/tmp/lock", CAPTURE: capture, GITHUB_ENV: path.join(dir, "env"), GITHUB_SHA: "branch" },
      });
      expect(output.exitCode, output.stderr.toString()).toBe(0);
      const args = readFileSync(capture, "utf8");
      expect(args.includes("--build\n")).toBe(local);
      expect(args.includes("--override\ntest-suite\n")).toBe(!local);
      expect(args).toContain(`features=${features}\n`);
    }
  } finally { rmSync(dir, { recursive: true, force: true }); }
});


test("CI explicitly executes the real private-journal ownership regression", () => {
  const step = workflow.jobs.harness.steps.find((item: any) => item.name === "Verify private journal ownership across harness recreation");
  expect(step.env.RUN_DOCKER_RECOVERY_TESTS).toBe("1");
  expect(step["timeout-minutes"]).toBe(2);
  expect(step.run).toContain("docker pull debian:12-slim");
  expect(step.run).toContain("bun test src/consensus/recreate-journal.test.ts");
});

test("crash recovery and replacement-block contracts require quorum evidence", () => {
  for (const id of ["CR-01-INTERRUPT-BEFORE-COMMIT", "CR-02-INTERRUPT-AFTER-COMMIT", "CR-03-EXPIRED-LEASE-RECLAIM", "REORG-01-REPLACEMENT-BLOCK"]) {
    const row = inventory.cases.find((entry) => entry.id === id)!;
    expect(row.participants.quorum, id).toBe("required");
    expect(row.assertions.some((item) => item.startsWith("quorum:")), id).toBe(true);
  }
});
