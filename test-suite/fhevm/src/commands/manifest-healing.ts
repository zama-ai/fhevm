/** Multi-root manifest fault matrix, real inferred containment, and recovery. */
import assert from "node:assert/strict";
import { captureDriftTables } from "./manifest-drift-report";
import fs from "node:fs/promises";
import path from "node:path";
import { PreflightError } from "../errors";
import { MANIFEST_INJECTION_PATH, manifestInjectionDir } from "../manifest-drift";
import { coprocessorDatabaseName, envPath, TEST_SUITE_CONTAINER } from "../layout";
import { topologyForState } from "../stack-spec/stack-spec";
import { readEnvFile } from "../utils/fs";
import { run } from "../utils/process";
import type { State } from "../types";
import { assertRestoredMaterial, flippedDigest, publicationReady, waitForManifestCondition, type PublicationReadiness } from "./manifest-lifecycle";

export const DRIFT_CASES = [
  { reason: "ct64_mismatch", fault: "ct64_digest_bit_flip", peer: false, healable: true },
  { reason: "ct64_mismatch", fault: "ct64_digest_bit_flip", peer: false, healable: true },
  { reason: "missing_here", fault: "missing_here", peer: false, healable: true },
  { reason: "error_here", fault: "error_here", peer: false, healable: true },
  { reason: "uncomputed_here", fault: "uncomputed_here", peer: false, healable: true },
  { reason: "ct128_mismatch", fault: "ct128_digest_bit_flip", peer: false, healable: false },
  { reason: "metadata_mismatch", fault: "keyset_id_bit_flip", peer: false, healable: false },
  { reason: "unknown_on_peer", fault: "missing_here", peer: true, healable: false },
  { reason: "error_on_peer", fault: "error_here", peer: true, healable: false },
  { reason: "uncomputed_on_peer", fault: "uncomputed_here", peer: true, healable: false },
] as const;
export type HealingPhase = "seed matrix" | "submit consumers" | "decrypt recovered" | "reuse healed chain" | "decrypt reused";
type Fixture = {
  chainId: number; roots: string[]; children: string[]; joined: string; tail: string;
  rootBlock: number; rootBlockHash: string; consumers?: string[]; queuedJoin?: string;
  recovered?: string; independent?: string; reused?: string;
};
type Material = { digest: string; bytes: string; version: string };
type Archive = { id: number; object_key: string; body: string; bucket: string };
export type Finding = {
  handle: string; detection_kind: string; reason: string; is_contained: boolean;
  can_be_healed: boolean; healed_at: string | null; target: string | null;
};
type Descriptor = { handle: string; status: string; ct64_digest?: string; ct128_digest?: string; keyset_id?: string; error_message?: string };
const HEX32 = /^0x[0-9a-f]{64}$/i;
const TARGET = 2;
const NODES = [0, 1, 2];
const FIXTURE_PATH = "/tmp/manifest-healing-fixture.json";
const detector = (index: number) => `coprocessor${index || ""}-consensus-detector`;
const h = (handle: string) => { assert(HEX32.test(handle)); return `decode('${handle.slice(2)}','hex')`; };
const inferred = (f: Fixture) => [f.children[0]!, f.children[1]!, f.joined, f.tail];

export function validateHealingFixture(f: Fixture): Fixture {
  assert(Number.isSafeInteger(f.chainId) && f.chainId >= 0);
  assert(Number.isSafeInteger(f.rootBlock) && f.rootBlock > 0);
  assert(Array.isArray(f.roots) && f.roots.length === DRIFT_CASES.length, "missing drift roots");
  assert(Array.isArray(f.children) && f.children.length === DRIFT_CASES.length, "missing descendants");
  const handles = [...f.roots, ...f.children, f.joined, f.tail];
  assert.equal(new Set(handles).size, handles.length, "fixture handles must be distinct");
  for (const handle of [...handles, f.rootBlockHash, ...(f.consumers ?? []), f.queuedJoin, f.recovered, f.independent, f.reused]) {
    if (handle !== undefined) assert(typeof handle === "string" && HEX32.test(handle), "invalid fixture handle");
  }
  if (f.consumers) assert.equal(f.consumers.length, DRIFT_CASES.length);
  return f;
}

/** Check the exact direct/inferred inventory, including absence on unaffected descendants. */
export function assertDriftMatrix(f: Fixture, rows: Finding[], digests: string[], healed: boolean) {
  const expected = [...f.roots, ...inferred(f)];
  assert.deepEqual(rows.map(r => r.handle).sort(), expected.map(v => v.slice(2)).sort(), "unexpected or missing drift findings");
  for (const [index, root] of f.roots.entries()) {
    const row = rows.find(r => r.handle === root.slice(2))!;
    const entry = DRIFT_CASES[index]!;
    assert.equal(row.detection_kind, "verified", entry.reason);
    assert.equal(row.reason, entry.reason);
    assert.equal(row.is_contained, entry.reason === "ct64_mismatch", entry.reason);
    assert.equal(row.healed_at !== null, healed && entry.healable, `${entry.reason}: healing state`);
    assert.equal(row.can_be_healed, entry.healable && !healed, `${entry.reason}: eligibility`);
    if (entry.healable) assert.equal(row.target, digests[index]);
    if (entry.peer) assert.equal(row.target, null, "peer-side status has no computed quorum target");
  }
  for (const handle of inferred(f)) {
    const row = rows.find(r => r.handle === handle.slice(2))!;
    assert.equal(row.detection_kind, "inferred");
    assert.equal(row.reason, "ct64_mismatch");
    assert.equal(row.is_contained, true);
    assert.equal(row.healed_at !== null, healed);
  }
}

/** Compare actual signed descriptors, not just the resulting findings. */
export function assertManifestFault(healthy: Descriptor | undefined, faulty: Descriptor | undefined, fault: string) {
  assert(healthy && healthy.status === "computed", "healthy root descriptor missing");
  if (fault === "missing_here") { assert.equal(faulty, undefined); return; }
  assert(faulty, "injected descriptor missing");
  if (fault === "error_here" || fault === "uncomputed_here") {
    assert.equal(faulty.status, fault === "error_here" ? "error" : "uncomputed");
    assert.equal(faulty.ct64_digest, undefined);
    assert.equal(faulty.ct128_digest, undefined);
    return;
  }
  const expected = { ...healthy };
  if (fault === "ct64_digest_bit_flip") expected.ct64_digest = flippedDigest(healthy.ct64_digest!);
  else if (fault === "ct128_digest_bit_flip") expected.ct128_digest = flippedDigest(healthy.ct128_digest!);
  else {
    assert.equal(fault, "keyset_id_bit_flip");
    assert.equal(BigInt(faulty.keyset_id!), BigInt(healthy.keyset_id!) ^ 1n);
    expected.keyset_id = faulty.keyset_id;
  }
  assert.deepEqual(faulty, expected);
}

export async function runManifestHealingProfile(
  state: State, query: (database: string, sql: string) => Promise<string>,
  runFixture: (phase: HealingPhase) => Promise<void>,
) {
  const topology = topologyForState(state);
  if (topology.count !== 3 || topology.threshold !== 2 || state.scenario.kind !== "coprocessor-consensus") {
    throw new PreflightError("manifest-healing requires --scenario manifest-lifecycle (three nodes, quorum two)");
  }
  const configPath = (index: number) => path.join(manifestInjectionDir(index), "injection.json");
  // Finish every preflight before stopping containers or creating any faults.
  for (const index of NODES) {
    const [container] = JSON.parse((await run(["docker", "inspect", detector(index)])).stdout);
    assert(container.State.Running && container.Config.Cmd.includes(`--dangerous-drift-injection=${MANIFEST_INJECTION_PATH}`)
      && container.Mounts.some((m: { Source: string; Destination: string }) => m.Source === manifestInjectionDir(index) && m.Destination === "/manifest-drift"),
    `node ${index} requires a rebuilt manifest-lifecycle scenario with its own injection mount`);
    await fs.access(configPath(index)).then(() => { throw new Error(`existing injection configuration: ${configPath(index)}`); },
      (error: NodeJS.ErrnoException) => { if (error.code !== "ENOENT") throw error; });
  }
  const env = await readEnvFile(envPath("coprocessor"));
  const chainId = Number(env.CHAIN_ID);
  assert(Number.isSafeInteger(chainId) && chainId >= 0);
  const q = (index: number, sql: string) => query(coprocessorDatabaseName(index), sql);
  const json = async <T>(index: number, sql: string): Promise<T> => JSON.parse(await q(index, sql));
  const readFixture = async () => validateHealingFixture(JSON.parse((await run(["docker", "exec", TEST_SUITE_CONTAINER, "cat", FIXTURE_PATH])).stdout));
  const report: Record<string, unknown> = { chainId, target: TARGET, cases: DRIFT_CASES, checkpoints: {} };
  const reportPath = path.join(manifestInjectionDir(TARGET), "report.json");
  const checkpoint = async (name: string, value: unknown) => {
    (report.checkpoints as Record<string, unknown>)[name] = value;
    await fs.writeFile(reportPath, JSON.stringify(report, null, 2));
  };
  const configs: { enabled: boolean; chain_id: number; pause_healing: boolean; faults: { handle: string; fault: string }[] }[] = [];
  const writeConfig = async (index: number) => {
    await fs.writeFile(`${configPath(index)}.tmp`, JSON.stringify(configs[index]));
    await fs.rename(`${configPath(index)}.tmp`, configPath(index));
  };
  try {
    for (const index of NODES) {
      await waitForManifestCondition(`node ${index} publication readiness`, () => json<PublicationReadiness>(index,
        `SELECT json_build_object('published',EXISTS(SELECT 1 FROM block_manifest_state WHERE host_chain_id=${chainId} AND consensus_epoch='legacy' AND manifest_published),
          'registryCount',(SELECT count(*) FROM public.gateway_config_coprocessors),
          'minThreshold',(SELECT min(coprocessor_threshold) FROM public.gateway_config_coprocessors),
          'maxThreshold',(SELECT max(coprocessor_threshold) FROM public.gateway_config_coprocessors),
          'epoch',(SELECT consensus_epoch FROM blue_green_consensus_epoch WHERE singleton),
          'manifestRows',(SELECT count(*) FROM block_manifest_state WHERE host_chain_id=${chainId}),
          'lastPublicationError',(SELECT publication_last_error FROM block_manifest_state WHERE host_chain_id=${chainId} AND publication_last_error IS NOT NULL ORDER BY updated_at DESC LIMIT 1))::text`), publicationReady);
    }
    await run(["docker", "stop", ...NODES.map(detector)]);
    await run(["docker", "exec", TEST_SUITE_CONTAINER, "rm", "-f", FIXTURE_PATH]);
    await runFixture("seed matrix");
    let f = await readFixture();
    assert.equal(f.chainId, chainId);
    report.fixture = f;
    const material = (index: number, handle: string) => json<Material | null>(index,
      `SELECT COALESCE((SELECT row_to_json(r)::text FROM (SELECT '0x'||encode(d.ciphertext,'hex') digest,
        encode(c.ciphertext,'hex') bytes,c.xmin::text version FROM ciphertext_digest d JOIN ciphertexts c ON c.handle=d.handle
        WHERE d.host_chain_id=${chainId} AND d.handle=${h(handle)} AND c.ciphertext_version=0
          AND d.s3_publication_verified_at IS NOT NULL AND d.s3_publication_verified_digest=d.ciphertext LIMIT 1) r),'null')`);
    const original: Material[] = [];
    const descendantOriginals = new Map<string, Material>();
    for (const [i, root] of f.roots.entries()) {
      for (const index of NODES) {
        const value = (await waitForManifestCondition(`node ${index} uploaded root ${i}`, () => material(index, root), v => v !== null))!;
        if (index === 0) original.push(value);
        else assertRestoredMaterial(original[i]!, value);
      }
    }
    const complete = (index: number, handles: string[]) => q(index, `SELECT count(*) FROM computations WHERE host_chain_id=${chainId}
      AND output_handle IN (${handles.map(h).join(",")}) AND is_completed AND NOT is_error`);
    for (const index of NODES) {
      await waitForManifestCondition(`node ${index} precomputed descendants`, () => complete(index, [...f.children, f.joined, f.tail]), n => Number(n) === 12);
      for (const handle of inferred(f)) {
        const value = (await waitForManifestCondition(`node ${index} uploaded descendant ${handle}`, () => material(index, handle), v => v !== null))!;
        if (index === 0) descendantOriginals.set(handle, value);
        else assertRestoredMaterial(descendantOriginals.get(handle)!, value);
      }
      configs.push({ enabled: true, chain_id: chainId, pause_healing: true,
        faults: DRIFT_CASES.flatMap((entry, i) => entry.peer === (index !== TARGET) ? [{ handle: f.roots[i]!, fault: entry.fault }] : []) });
      await writeConfig(index);
    }
    // All consumers are already computed and no new work reads this root until containment.
    const before = (await material(TARGET, f.roots[0]!))!;
    const damaged = Buffer.from(before.bytes, "hex");
    assert(damaged.length > 0);
    const offset = Math.floor(damaged.length / 2);
    damaged[offset] = damaged[offset]! ^ 1;
    assert.equal(await q(TARGET, `WITH changed AS (UPDATE ciphertexts
      SET ciphertext=set_byte(ciphertext,${offset},get_byte(ciphertext,${offset}) # 1)
      WHERE handle=${h(f.roots[0]!)} AND ciphertext_version=0 AND xmin::text='${before.version}' RETURNING handle)
      SELECT encode(handle,'hex') FROM changed`), f.roots[0]!.slice(2));
    const corrupted = (await material(TARGET, f.roots[0]!))!;
    assert.equal(corrupted.bytes, damaged.toString("hex"));
    assert.notEqual(corrupted.bytes, before.bytes);
    assert.equal(corrupted.digest, before.digest);
    await checkpoint("corrupted", { handle: f.roots[0], offset, byteLength: damaged.length, originalVersion: before.version, damagedVersion: corrupted.version });
    await run(["docker", "start", ...NODES.map(detector)]);
    const archives: Archive[] = [];
    const descriptors: Descriptor[][] = [];
    for (const index of NODES) {
      const archive = (await waitForManifestCondition(`node ${index} signed matrix manifest`, () => json<Archive | null>(index,
        `SELECT COALESCE((SELECT row_to_json(r)::text FROM (SELECT m.id,m.object_key,encode(m.signed_manifest,'hex') body,g.s3_bucket_url bucket
          FROM block_manifest m JOIN public.gateway_config_coprocessors g ON g.signer_address=m.publisher
          WHERE host_chain_id=${chainId} AND publication_block_number=${f.rootBlock} AND publication_block_hash=${h(f.rootBlockHash)}
          AND consensus_epoch='legacy' AND manifest_source='local' ORDER BY revision DESC LIMIT 1) r),'null')`), v => v !== null))!;
      const url = `${archive.bucket.replace(/\/$/, "")}/${archive.object_key}`;
      const downloaded = await run(["docker", "exec", TEST_SUITE_CONTAINER, "node", "-e",
        "fetch(process.argv[1],{signal:AbortSignal.timeout(15000)}).then(async r=>{if(!r.ok)throw Error('HTTP '+r.status);process.stdout.write(Buffer.from(await r.arrayBuffer()).toString('hex'))}).catch(e=>{console.error(e);process.exit(1)})", url]);
      assert.equal(downloaded.stdout.trim(), archive.body, "S3/archive bytes differ");
      descriptors.push(JSON.parse(Buffer.from(archive.body, "hex").toString()).detailed_range.blocks.flatMap((b: { ciphertexts: Descriptor[] }) => b.ciphertexts));
      archives.push(archive);
    }
    for (const [i, entry] of DRIFT_CASES.entries()) {
      const perNode = descriptors.map(ds => ds.find(d => d.handle.toLowerCase() === f.roots[i]!.toLowerCase()));
      assert.deepEqual(perNode[0], perNode[1], "peer fault descriptors differ");
      const healthy = perNode[entry.peer ? TARGET : 0];
      assert.equal(healthy?.ct64_digest?.toLowerCase(), original[i]!.digest);
      assertManifestFault(healthy, perNode[entry.peer ? 0 : TARGET], entry.fault);
    }
    await checkpoint("manifests", archives.map(({ body: _, ...identity }) => identity));
    for (const [index, archive] of archives.entries()) {
      await waitForManifestCondition(`node ${index} authenticated matrix verification`, () => q(index,
        `SELECT EXISTS(SELECT 1 FROM block_manifest_verification_task t JOIN block_manifest_verification_attempt a
          ON a.task_id=t.id AND a.consensus_epoch=t.consensus_epoch WHERE t.local_manifest_id=${archive.id}
          AND t.required_quorum=2 AND a.local_quorum_status='${index === TARGET ? "differs_from_quorum" : "matches_quorum"}' AND a.localization_complete
          ${index === TARGET ? "" : `AND a.quorum_from_block <= ${f.rootBlock} AND a.quorum_through_block >= ${f.rootBlock}`})`), v => v === "t");
    }
    const allHandles = [...f.roots, ...f.children, f.joined, f.tail];
    const findings = () => json<Finding[]>(TARGET, `SELECT COALESCE(json_agg(row_to_json(r)), '[]'::json)::text FROM
      (SELECT encode(handle,'hex') handle,detection_kind,reason,is_contained,can_be_healed,healed_at,
        '0x'||encode(target_ct64_digest,'hex') target FROM drifted_handle
        WHERE host_chain_id=${chainId} AND consensus_epoch='legacy' AND handle IN (${allHandles.map(h).join(",")})) r`);
    const snapshot = async (healed: boolean) => waitForManifestCondition(healed ? "matrix healed" : "matrix contained", findings,
      rows => { try { assertDriftMatrix(f, rows, original.map(m => m.digest), healed); return true; } catch { return false; } });
    await checkpoint("contained", await snapshot(false));
    await runFixture("submit consumers");
    f = await readFixture();
    assert(f.consumers && f.queuedJoin && f.recovered && f.independent, "missing consumer handles");
    report.fixture = f;
    const blocked = [f.consumers[0]!, f.consumers[1]!, f.queuedJoin, f.recovered];
    const progressing = [...f.consumers.slice(2), f.independent];
    await waitForManifestCondition("independent and non-ct64 branches progress", () => complete(TARGET, progressing), n => Number(n) === progressing.length);
    for (let observation = 0; observation < 5; observation++) {
      assert.equal(Number(await q(TARGET, `SELECT count(*) FROM computations WHERE host_chain_id=${chainId}
        AND output_handle IN (${blocked.map(h).join(",")}) AND NOT is_completed AND NOT is_error`)), blocked.length, "contained chain was not blocked");
      assert.equal(await q(TARGET, `SELECT count(*) FROM ciphertexts WHERE handle IN (${blocked.map(h).join(",")})`), "0");
      assertDriftMatrix(f, await findings(), original.map(m => m.digest), false);
      await Bun.sleep(1_000);
    }
    await checkpoint("blocked", { handles: blocked, progressing });
    // Retain the exact publication faults through restart so persisted seals remain valid.
    // Peers are released too: conservative observations may have contained their copies.
    await run(["docker", "stop", ...NODES.map(detector)]);
    for (const index of NODES) { configs[index]!.pause_healing = false; await writeConfig(index); }
    await run(["docker", "start", ...NODES.map(detector)]);
    const healed = await snapshot(true);
    await checkpoint("healed", healed);
    for (const [i, entry] of DRIFT_CASES.entries()) {
      if (entry.healable) assertRestoredMaterial(original[i]!, await material(TARGET, f.roots[i]!));
    }
    for (const [handle, expected] of descendantOriginals) {
      assert.equal(healed.find(row => row.handle === handle.slice(2))!.target, expected.digest);
      assertRestoredMaterial(expected, await material(TARGET, handle));
    }
    await checkpoint("restoration", { exactBytesRestored: true });
    for (const index of NODES) {
      await waitForManifestCondition(`node ${index} recovered chain`, () => complete(index, [...f.consumers!, f.queuedJoin!, f.recovered!]), n => Number(n) === 12);
    }
    await runFixture("decrypt recovered");
    await checkpoint("decrypted", { recovered: f.recovered, consumers: f.consumers });
    await runFixture("reuse healed chain");
    f = await readFixture();
    assert(f.reused, "missing reused handle");
    for (const index of NODES) await waitForManifestCondition(`node ${index} reuses healed chain`, () => complete(index, [f.reused!]), n => n === "1");
    await runFixture("decrypt reused");
    // Observe non-healable cases again after real healing and a full decryption roundtrip.
    assertDriftMatrix(f, await findings(), original.map(m => m.digest), true);
    await checkpoint("reused", { handle: f.reused, decrypted: true });
    for (const index of NODES) {
      await waitForManifestCondition(`node ${index} has no pending healable findings`, () => q(index,
        `SELECT count(*) FROM drifted_handle WHERE can_be_healed AND healed_at IS NULL`), n => n === "0");
    }
    report.result = "passed";
  } catch (error) {
    report.result = "failed";
    report.error = String(error);
    throw error;
  } finally {
    report.finalDriftTables = await captureDriftTables(q);
    const cleanupErrors: string[] = [];
    for (const index of NODES) {
      try {
        const logs = await run(["docker", "logs", detector(index)], { allowFailure: true });
        await fs.writeFile(path.join(manifestInjectionDir(TARGET), `detector-${index}.log`), logs.stdout + logs.stderr);
      } catch (error) { cleanupErrors.push(`logs node ${index}: ${error}`); }
      try {
        await run(["docker", "stop", detector(index)]);
        await fs.rm(configPath(index), { force: true });
        await fs.rm(`${configPath(index)}.tmp`, { force: true });
      } catch (error) { cleanupErrors.push(`node ${index}: ${error}`); }
      finally {
        try { await run(["docker", "start", detector(index)]); }
        catch (error) { cleanupErrors.push(`restart node ${index}: ${error}`); }
      }
    }
    if (cleanupErrors.length) { report.cleanupErrors = cleanupErrors; report.result = "failed"; }
    await fs.writeFile(reportPath, JSON.stringify(report, null, 2));
    console.log(`[manifest-healing] report: ${reportPath}`);
    if (cleanupErrors.length) throw new Error(`manifest-healing cleanup failed: ${cleanupErrors.join("; ")}`);
  }
}
