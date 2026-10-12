/** Reused computations across host blocks, with handle-selected ct64 corruption on one node. */
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import { PreflightError } from "../errors";
import { coprocessorDatabaseName, envPath, TEST_SUITE_CONTAINER } from "../layout";
import { MANIFEST_INJECTION_PATH, manifestInjectionDir } from "../manifest-drift";
import { topologyForState } from "../stack-spec/stack-spec";
import type { State } from "../types";
import { readEnvFile } from "../utils/fs";
import { run } from "../utils/process";
import { captureDriftTables } from "./manifest-drift-report";
import { waitForManifestCondition } from "./manifest-lifecycle";
import { DISABLE_STRESS_INJECTION, INSTALL_STRESS_INJECTION, stressHandleIsCorrupted } from "./manifest-stress-injection";

export type StressPhase = "seed" | "advance" | "pin roots" | "decrypt";
type Fixture = { chainId: number; handles: string[]; heads: string[]; roots: string[]; rounds: number };
type Material = { handle: string; bytes: string; length: number };
type Inventory = { total: number; healed: number; unresolved: number; verified: number; inferred: number; lastHealed: string | null };
const NODES = [0, 1, 2];
const TARGET = 2;
const CHAIN_BLOCKS = 3;
const PATH = "/tmp/manifest-healing-stress-fixture.json";
const detector = (node: number) => `coprocessor${node || ""}-consensus-detector`;
const HEX32 = /^0x[0-9a-f]{64}$/;

export function validateStressFixture(f: Fixture, chainId: number) {
  assert.equal(f.chainId, chainId);
  assert(Number.isSafeInteger(f.rounds) && f.rounds >= 0);
  assert.equal(f.heads.length, 4);
  assert.equal(f.roots.length, 4);
  assert(f.handles.length >= 4 && new Set(f.handles).size === f.handles.length);
  for (const h of [...f.handles, ...f.heads, ...f.roots]) assert(HEX32.test(h), "invalid stress fixture handle");
  for (const h of [...f.heads, ...f.roots]) assert(f.handles.includes(h));
  return f;
}

export type StressStep = {
  inputs: string[];
  roots: string[];
  mixed: string[];
  heads: string[];
  independent: string;
};

/** Handles the trigger flipped, plus every later result that read one of them. */
export function classifyStressDrift(steps: StressStep[], injected: Iterable<string>): {
  verifiedRoots: string[];
  descendants: string[];
  clean: string[];
  byStep: string[][];
} {
  const handle = (value: string) => value.replace(/^0x/, "").toLowerCase();
  const flipped = new Set([...injected].map(handle));
  const tainted = new Set<string>();
  const verifiedRoots: string[] = [];
  const descendants: string[] = [];
  const produced: string[] = [];
  const byStep: string[][] = [];
  for (const step of steps) {
    const inputs = step.inputs.map(handle);
    const roots = step.roots.map(handle);
    const producedHere = [
      ...step.mixed.map((value, i) => ({ handle: handle(value), from: [inputs[i]!, inputs[(i + 1) % inputs.length]!] })),
      ...step.heads.map((value, i) => ({ handle: handle(value), from: [handle(step.mixed[i]!), roots[i]!] })),
      { handle: handle(step.independent), from: [] as string[] },
    ];
    const taintedHere: string[] = [];
    for (const item of producedHere) {
      produced.push(item.handle);
      const fromTainted = item.from.some(input => tainted.has(input));
      if (flipped.has(item.handle) && !fromTainted) verifiedRoots.push(item.handle);
      else if (fromTainted) descendants.push(item.handle);
      if (flipped.has(item.handle) || fromTainted) {
        tainted.add(item.handle);
        taintedHere.push(item.handle);
      }
    }
    byStep.push(taintedHere);
  }
  return { verifiedRoots, descendants, clean: produced.filter(value => !tainted.has(value)), byStep };
}

export function stressConverged(handles: string[], materials: Material[][], inventories: Inventory[]) {
  const expected = handles.map(h => h.slice(2)).sort();
  return handles.length > 0 && materials.length === 3 && inventories.length === 3
    && inventories.every(row => row.unresolved === 0)
    && materials.every(rows => rows.length === handles.length
      && rows.every((row, i) => row.handle === expected[i])
      && JSON.stringify(rows) === JSON.stringify(materials[0]));
}

export async function runManifestHealingStressProfile(
  state: State, query: (database: string, sql: string) => Promise<string>,
  runFixture: (phase: StressPhase) => Promise<void>,
) {
  const topology = topologyForState(state);
  if (state.scenario.kind !== "coprocessor-consensus" || topology.count !== 3 || topology.threshold !== 2) {
    throw new PreflightError("manifest-healing-stress requires a fresh --scenario manifest-lifecycle stack");
  }
  for (const node of NODES) {
    const [container] = JSON.parse((await run(["docker", "inspect", detector(node)])).stdout);
    assert(container.State.Running && container.Config.Cmd.includes(`--dangerous-drift-injection=${MANIFEST_INJECTION_PATH}`)
      && container.Mounts.some((m: { Source: string; Destination: string }) => m.Source === manifestInjectionDir(node) && m.Destination === "/manifest-drift"),
    `node ${node} requires a running local manifest-lifecycle scenario`);
    await fs.access(path.join(manifestInjectionDir(node), "injection.json")).then(
      () => { throw new PreflightError(`node ${node} has a manifest injection file; use a fresh stack`); },
      (error: NodeJS.ErrnoException) => { if (error.code !== "ENOENT") throw error; });
  }
  const chainId = Number((await readEnvFile(envPath("coprocessor"))).CHAIN_ID);
  assert(Number.isSafeInteger(chainId) && chainId >= 0);
  const q = (node: number, sql: string) => query(coprocessorDatabaseName(node), sql);
  const json = async <T>(node: number, sql: string): Promise<T> => JSON.parse(await q(node, sql));
  const fixture = async () => validateStressFixture(JSON.parse((await run(["docker", "exec", TEST_SUITE_CONTAINER, "cat", PATH])).stdout), chainId);
  const report: Record<string, unknown> = { chainId, target: TARGET, chainBlocks: CHAIN_BLOCKS, checkpoints: [] };
  const bytes = (handles: string[]) => handles.map(h => `decode('${h.slice(2)}','hex')`).join(",");
  const reportPath = path.join(manifestInjectionDir(TARGET), "stress-report.json");
  const checkpoint = async (phase: string, value: unknown) => {
    (report.checkpoints as unknown[]).push({ phase, at: new Date().toISOString(), value });
    await fs.writeFile(reportPath, JSON.stringify(report, null, 2));
  };
  const inventory = (node: number) => json<Inventory>(node, `SELECT json_build_object(
    'total',count(*),'healed',count(*) FILTER(WHERE healed_at IS NOT NULL),
    'unresolved',count(*) FILTER(WHERE healed_at IS NULL AND
      (can_be_healed OR is_contained OR reason IN ('ct64_mismatch','missing_here','error_here','uncomputed_here'))),
    'verified',count(*) FILTER(WHERE detection_kind='verified'),
    'inferred',count(*) FILTER(WHERE detection_kind='inferred'), 'lastHealed',max(healed_at))::text FROM drifted_handle`);
  const materials = (node: number, f: Fixture) => json<Material[]>(node, `SELECT COALESCE(json_agg(r ORDER BY handle),'[]'::json)::text FROM (
    SELECT encode(c.handle,'hex') handle,encode(sha256(c.ciphertext),'hex') bytes,octet_length(c.ciphertext) length
    FROM ciphertexts c
    WHERE c.ciphertext_version=0 AND c.handle IN (${f.handles.map(h => `decode('${h.slice(2)}','hex')`).join(',')})
      AND EXISTS(SELECT 1 FROM computations w WHERE w.host_chain_id=${chainId} AND w.output_handle=c.handle AND w.is_completed AND NOT w.is_error)) r`);
  const observe = async (f: Fixture) => {
    const inventories = await Promise.all(NODES.map(inventory));
    const copies = await Promise.all(NODES.map(node => materials(node, f)));
    return { inventories, copies };
  };
  let detectorStopped = false;
  let injectionOwned = false;
  try {
    for (const node of NODES) assert.equal((await inventory(node)).total, 0, `node ${node}: fresh drift inventory required`);
    await run(["docker", "exec", TEST_SUITE_CONTAINER, "rm", "-f", PATH]);
    await runFixture("seed");
    let f = await fixture();
    await waitForManifestCondition("healthy stress baseline", () => observe(f), v => stressConverged(f.handles, v.copies, v.inventories));
    // Publication stays off until the reused chain exists, so containment cannot
    // freeze the later blocks before they are computed.
    await run(["docker", "stop", detector(TARGET)]);
    detectorStopped = true;
    await q(TARGET, INSTALL_STRESS_INJECTION);
    injectionOwned = true;
    for (let block = 0; block < CHAIN_BLOCKS; block++) await runFixture("advance");
    f = await fixture();
    const ready = async () => Number(await q(TARGET, `SELECT count(*) FROM ciphertext_digest
      WHERE host_chain_id=${chainId} AND ciphertext IS NOT NULL AND handle IN (${bytes(f.handles)})`));
    await waitForManifestCondition("serial chain stored on node 2", ready, n => n === f.handles.length);
    const noise = await json<{ handle: string; injected: boolean }[]>(TARGET, `SELECT COALESCE(json_agg(r),'[]'::json)::text FROM
      (SELECT encode(handle,'hex') handle, injected FROM e2e_manifest_noise) r`);
    for (const row of noise) assert.equal(row.injected, stressHandleIsCorrupted(row.handle), row.handle);
    const corrupted = noise.filter(row => row.injected).map(row => row.handle);
    const chainSteps = [...((f as Fixture & { steps?: StressStep[] }).steps ?? [])];
    assert(corrupted.length > 0, "handle selector corrupted no eligible ciphertext");
    await checkpoint("serial chain", { rounds: f.rounds, corrupted, sampled: noise.length });
    await q(TARGET, DISABLE_STRESS_INJECTION);
    await run(["docker", "start", detector(TARGET)]);
    detectorStopped = false;
    const deadline = Date.now() + 15 * 60_000;
    let stableRounds = 0;
    let stableSince = 0;
    let previousInventory = "";
    while (Date.now() < deadline) {
      await runFixture("advance");
      f = await fixture();
      // Healthy peers must keep completing all submitted work despite node 2's faults.
      for (const node of [0, 1]) await waitForManifestCondition(`healthy node ${node} progress`,
        () => materials(node, f), rows => rows.length === f.handles.length, Math.max(1, deadline - Date.now()));
      // Give the newly submitted round time to finish before assessing stability.
      // Keep generating rounds when the damaged peer still has a recovery backlog.
      let observation = await observe(f);
      for (let poll = 0; poll < 10 && Date.now() < deadline
        && !stressConverged(f.handles, observation.copies, observation.inventories); poll++) {
        await Bun.sleep(2_000);
        observation = await observe(f);
      }
      const signature = JSON.stringify(observation.inventories);
      if (stressConverged(f.handles, observation.copies, observation.inventories)) {
        if (signature !== previousInventory) { stableRounds = 0; stableSince = Date.now(); }
        stableRounds++;
      } else { stableRounds = 0; stableSince = Date.now(); }
      previousInventory = signature;
      await checkpoint("recovery traffic", { round: f.rounds, stableRounds, inventories: observation.inventories,
        completed: observation.copies.map(rows => rows.length), expected: f.handles.length });
      console.log(`[manifest-healing-stress] round=${f.rounds} stable=${stableRounds} unresolved=${observation.inventories.map(v => v.unresolved).join(',')}`);
      if (stableRounds >= 3 && Date.now() - stableSince >= 30_000) break;
      await Bun.sleep(10_000);
    }
    assert(stableRounds >= 3 && Date.now() - stableSince >= 30_000, "healing did not converge under continued mixing traffic within 15 minutes");
    const rows = await json<{ handle: string; kind: string; reason: string; healed: boolean; block: number }[]>(TARGET,
      `SELECT COALESCE(json_agg(r),'[]'::json)::text FROM (
        SELECT encode(handle,'hex') handle, detection_kind kind, reason, healed_at IS NOT NULL healed, block_number block
        FROM drifted_handle) r`);
    const expected = classifyStressDrift(chainSteps, corrupted);
    const byHandle = new Map(rows.map(row => [row.handle, row]));
    for (const handle of expected.verifiedRoots) {
      const row = byHandle.get(handle);
      assert(row, `corrupted handle ${handle} has no drift row`);
      assert.equal(row.kind, "verified", handle);
      assert.equal(row.reason, "ct64_mismatch", handle);
      assert.equal(row.healed, true, handle);
    }
    for (const handle of expected.descendants) {
      const row = byHandle.get(handle);
      assert(row, `reused handle ${handle} has no drift row`);
      assert.ok(row.kind === "verified" || row.kind === "inferred", handle);
      assert.equal(row.reason, "ct64_mismatch", handle);
      assert.equal(row.healed, true, handle);
    }
    for (const handle of expected.clean) assert.equal(byHandle.has(handle), false, `clean handle ${handle} was stored`);
    const named = new Set([...expected.verifiedRoots, ...expected.descendants, ...expected.clean]);
    for (const row of rows) {
      assert.equal(row.healed, true, row.handle);
      if (named.has(row.handle) && row.kind === "inferred") {
        assert.ok(expected.descendants.includes(row.handle), `inferred handle ${row.handle} did not reuse a corrupted ciphertext`);
      }
    }
    const generations = expected.byStep.filter(group => group.length > 0);
    if (generations.length >= 2) {
      const generationBlocks = generations.map(group => byHandle.get(group[0]!)!.block);
      assert.equal(new Set(generationBlocks).size, generationBlocks.length, `reused drifts share a host block: ${generationBlocks.join(",")}`);
    }
    for (const node of [0, 1]) assert.equal((await inventory(node)).total, 0, `node ${node} stored a drift`);
    const final = await observe(f);
    assert(stressConverged(f.handles, final.copies, final.inventories));
    assert(final.inventories[TARGET]!.healed > 0, "target never healed a finding");
    await runFixture("decrypt");
    const afterDecryption = await observe(f);
    assert(stressConverged(f.handles, afterDecryption.copies, afterDecryption.inventories));
    assert.deepEqual(afterDecryption.inventories, final.inventories, "drift/healing restarted during decryption");
    report.fixture = f;
    report.result = "passed";
  } catch (error) {
    report.result = "failed";
    report.error = String(error);
    throw error;
  } finally {
    const cleanupErrors: string[] = [];
    if (detectorStopped) {
      try { await run(["docker", "start", detector(TARGET)]); }
      catch (error) { cleanupErrors.push(String(error)); }
    }
    if (injectionOwned) {
      try {
        await q(TARGET, DISABLE_STRESS_INJECTION);
        report.injections = await json(TARGET, `SELECT COALESCE(json_agg(r ORDER BY recorded_at,handle),'[]'::json)::text FROM
          (SELECT encode(handle,'hex') handle,injected,byte_offset,original_byte,recorded_at FROM e2e_manifest_noise) r`);
        await fs.writeFile(reportPath, JSON.stringify(report, null, 2));
        await q(TARGET, "DROP TABLE e2e_manifest_noise");
      } catch (error) { cleanupErrors.push(String(error)); }
    }
    report.finalDriftTables = await captureDriftTables(q);
    if (cleanupErrors.length) { report.cleanupErrors = cleanupErrors; report.result = "failed"; }
    await fs.writeFile(reportPath, JSON.stringify(report, null, 2));
    console.log(`[manifest-healing-stress] report: ${reportPath}`);
    if (cleanupErrors.length) throw new Error(`stress cleanup failed: ${cleanupErrors.join('; ')}`);
  }
}
