/** Signed manifest faults exercise containment or restoration of damaged local ct64 bytes. */
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

const TARGET = 2;
const DETECTOR = "coprocessor2-consensus-detector";
const FIXTURE_PATH = "/tmp/manifest-lifecycle-fixture.json";
const HEX32 = /^0x[0-9a-f]{64}$/i;

type Fixture = { chainId: number; root: string; child?: string; rootBlock: number; rootBlockHash: string; blocked?: string; independent?: string };
type Archive = { id: number; publisher: string; object_key: string; body: string; bucket: string };
type Material = { digest: string; bytes: string; version: string };
type Finding = { handle: string; detection_kind: string; reason: string; is_contained: boolean; healed_at: string | null; target: string | null };
type Query = (database: string, sql: string) => Promise<string>;

export function validateFixture(value: Fixture): Fixture {
  assert(Number.isSafeInteger(value.chainId) && value.chainId >= 0, "invalid fixture chain");
  assert(Number.isSafeInteger(value.rootBlock) && value.rootBlock > 0, "invalid fixture block");
  for (const handle of [value.root, value.rootBlockHash]) assert(typeof handle === "string" && HEX32.test(handle), "missing/invalid fixture identity");
  for (const handle of [value.child, value.blocked, value.independent]) {
    if (handle !== undefined) assert(HEX32.test(handle), "invalid fixture handle/block hash");
  }
  if (value.child !== undefined) assert.notEqual(value.root, value.child, "fixture must have a distinct descendant");
  return value;
}

export type PublicationReadiness = {
  published: boolean;
  registryCount: number;
  minThreshold: number | null;
  maxThreshold: number | null;
  epoch: string | null;
  manifestRows: number;
  lastPublicationError: string | null;
};

export const publicationReady = (value: PublicationReadiness) =>
  value.published && value.epoch === "legacy" && value.registryCount === 3
  && value.minThreshold === 2 && value.maxThreshold === 2;

export function flippedDigest(digest: string): string {
  assert(HEX32.test(digest), "invalid ct64 digest");
  const bytes = Buffer.from(digest.slice(2), "hex");
  bytes[0] = bytes[0]! ^ 1;
  return `0x${bytes.toString("hex")}`;
}

export function assertRestoredMaterial(original: Material, restored: Material | null): asserts restored is Material {
  assert(restored, "healed ciphertext is missing");
  assert.equal(restored.bytes, original.bytes, "healing did not restore the exact original ct64 bytes");
  assert.equal(restored.digest, original.digest, "healing changed the expected ct64 digest");
}

export async function waitForManifestCondition<T>(
  label: string, read: () => Promise<T>, ready: (value: T) => boolean,
  timeoutMs = 300_000,
): Promise<T> {
  const deadline = Date.now() + timeoutMs;
  let last: T;
  for (;;) {
    last = await read(); // Infrastructure/query errors must fail, not masquerade as pending work.
    if (ready(last)) return last;
    if (Date.now() >= deadline) throw new Error(`${label} timed out; last observation: ${JSON.stringify(last)}`);
    await Bun.sleep(1_000);
  }
}

export async function runManifestLifecycleProfile(
  state: State,
  query: Query,
  runFixture: (phase: "seed root" | "seed graph" | "submit consumers") => Promise<void>,
): Promise<void> {
  const topology = topologyForState(state);
  if (topology.count !== 3 || topology.threshold !== 2 || state.scenario.kind !== "coprocessor-consensus") {
    throw new PreflightError("manifest-lifecycle requires --scenario manifest-lifecycle (three coprocessors, quorum two)");
  }
  const directory = manifestInjectionDir(TARGET);
  const configPath = path.join(directory, "injection.json");
  const reportPath = path.join(directory, "report.json");
  const [container] = JSON.parse((await run(["docker", "inspect", DETECTOR])).stdout);
  if (!container.State.Running || !container.Config.Cmd.includes(`--dangerous-drift-injection=${MANIFEST_INJECTION_PATH}`)
      || !container.Mounts.some((mount: { Source: string; Destination: string }) =>
        mount.Destination === "/manifest-drift" && mount.Source === directory)) {
    throw new PreflightError("detector is not running with the manifest-lifecycle scenario's configuration mount and flag; rebuild/up that scenario first");
  }
  if (await fs.stat(configPath).then(() => true, (error: NodeJS.ErrnoException) => {
    if (error.code === "ENOENT") return false;
    throw error;
  })) throw new PreflightError(`Remove the existing injection configuration before starting a fresh exercise: ${configPath}`);
  const env = await readEnvFile(envPath("coprocessor"));
  const chainId = Number(env.CHAIN_ID);
  assert(Number.isSafeInteger(chainId) && chainId >= 0, "coprocessor CHAIN_ID is missing/invalid");
  const q = (index: number, sql: string) => query(coprocessorDatabaseName(index), sql);
  const json = async <T>(index: number, sql: string): Promise<T> => JSON.parse(await q(index, sql));
  const fixture = async () => validateFixture(JSON.parse((await run(["docker", "exec", TEST_SUITE_CONTAINER, "cat", FIXTURE_PATH])).stdout));
  const report: Record<string, unknown> = { target: TARGET, chainId };
  let stopped = false;
  let f: Fixture | undefined;
  try {
    for (const index of [0, 1, 2]) {
      await waitForManifestCondition(`node ${index} publication readiness`, () => json<PublicationReadiness>(index,
        `SELECT json_build_object(
          'published', EXISTS(SELECT 1 FROM block_manifest_state WHERE host_chain_id=${chainId} AND consensus_epoch='legacy' AND manifest_published),
          'registryCount', (SELECT count(*) FROM public.gateway_config_coprocessors),
          'minThreshold', (SELECT min(coprocessor_threshold) FROM public.gateway_config_coprocessors),
          'maxThreshold', (SELECT max(coprocessor_threshold) FROM public.gateway_config_coprocessors),
          'epoch', (SELECT consensus_epoch FROM blue_green_consensus_epoch WHERE singleton),
          'manifestRows', (SELECT count(*) FROM block_manifest_state WHERE host_chain_id=${chainId}),
          'lastPublicationError', (SELECT publication_last_error FROM block_manifest_state
            WHERE host_chain_id=${chainId} AND publication_last_error IS NOT NULL ORDER BY updated_at DESC LIMIT 1)
        )::text`), publicationReady);
    }
    await run(["docker", "stop", DETECTOR]);
    stopped = true;
    await run(["docker", "exec", TEST_SUITE_CONTAINER, "rm", "-f", FIXTURE_PATH]);
    await runFixture("seed graph");
    f = await fixture();
    assert.equal(f.chainId, chainId);
    assert(f.child, "containment fixture omitted its descendant");
    report.fixture = f;
    const h = (handle: string) => `decode('${handle.slice(2)}','hex')`;
    const material = async (index: number, handle: string) => json<Material | null>(index,
      `SELECT COALESCE((SELECT row_to_json(r)::text FROM (SELECT '0x'||encode(d.ciphertext,'hex') digest,
          encode(c.ciphertext,'hex') bytes,c.xmin::text version FROM ciphertext_digest d
          JOIN ciphertexts c ON c.handle=d.handle WHERE d.host_chain_id=${chainId} AND d.handle=${h(handle)}
          AND d.s3_publication_verified_at IS NOT NULL AND d.s3_publication_verified_digest=d.ciphertext LIMIT 1) r),'null')`);
    // Only this root is needed in full; the descendant is checked for completion separately.
    const originals: Material[] = [];
    for (const index of [0, 1, 2]) {
      originals.push((await waitForManifestCondition(`node ${index} root material`, () => material(index, f!.root), value => value !== null))!);
      if (f.child) await waitForManifestCondition(`node ${index} descendant computed`, () => q(index,
        `SELECT EXISTS(SELECT 1 FROM computations WHERE host_chain_id=${chainId} AND output_handle=${h(f!.child!)} AND is_completed AND NOT is_error)`), value => value === "t");
    }
    assert.equal(originals[0]!.digest, originals[1]!.digest);
    assert.equal(originals[0]!.digest, originals[2]!.digest);
    assert.equal(originals[0]!.bytes, originals[1]!.bytes);
    assert.equal(originals[0]!.bytes, originals[2]!.bytes);
    const original = originals[2]!;
    const injected = flippedDigest(original.digest);
    report.originalDigest = original.digest;
    report.injectedDigest = injected;
    // Resume publication as soon as fixture computation/upload checks pass.
    await fs.writeFile(`${configPath}.tmp`, JSON.stringify({ enabled: true, pause_healing: true, chain_id: chainId, handle: f.root, fault: "ct64_digest_bit_flip" }));
    await fs.rename(`${configPath}.tmp`, configPath);
    await run(["docker", "start", DETECTOR]);
    stopped = false;
    const rootScope = `host_chain_id=${chainId} AND publication_block_number=${f.rootBlock} AND publication_block_hash=${h(f.rootBlockHash)} AND consensus_epoch='legacy'`;
    const localArchive = (index: number) => json<Archive | null>(index,
      `SELECT COALESCE((SELECT row_to_json(r)::text FROM (SELECT m.id,encode(m.publisher,'hex') publisher,m.object_key,
          encode(m.signed_manifest,'hex') body,g.s3_bucket_url bucket FROM block_manifest m
          JOIN public.gateway_config_coprocessors g ON g.signer_address=m.publisher
          WHERE ${rootScope} AND manifest_source='local' ORDER BY revision DESC LIMIT 1) r),'null')`);
    const archives: Archive[] = [];
    for (const index of [0, 1]) {
      archives.push((await waitForManifestCondition(`node ${index} healthy manifest`, () => localArchive(index), value => value !== null))!);
    }
    archives.push((await waitForManifestCondition("injected publication", () => localArchive(TARGET), value => value !== null))!);
    report.manifests = archives.map(({ body: _, ...identity }) => identity);
    for (const [index, archive] of archives.entries()) {
      const url = `${archive.bucket.replace(/\/$/, "")}/${archive.object_key}`;
      const downloaded = await run(["docker", "exec", TEST_SUITE_CONTAINER, "node", "-e",
        "fetch(process.argv[1],{signal:AbortSignal.timeout(15000)}).then(async r=>{if(!r.ok)throw Error('HTTP '+r.status);process.stdout.write(Buffer.from(await r.arrayBuffer()).toString('hex'))}).catch(e=>{console.error(e);process.exit(1)})", url]);
      assert.equal(downloaded.stdout.trim(), archive.body, `node ${index} S3 bytes differ from archive`);
      const signed = JSON.parse(Buffer.from(archive.body, "hex").toString());
      const descriptor = signed.detailed_range.blocks.flatMap((block: { ciphertexts: { handle: string; ct64_digest: string }[] }) => block.ciphertexts)
        .find((entry: { handle: string }) => entry.handle.toLowerCase() === f!.root.toLowerCase());
      assert.equal(descriptor?.ct64_digest.toLowerCase(), index === TARGET ? injected : original.digest);
    }
    for (const [index, archive] of archives.entries()) {
      const expected = index === TARGET ? "differs_from_quorum" : "matches_quorum";
      await waitForManifestCondition(`node ${index} authenticated verification`, () => q(index,
        `SELECT EXISTS(SELECT 1 FROM block_manifest_verification_task t JOIN block_manifest_verification_attempt a
          ON a.task_id=t.id AND a.consensus_epoch=t.consensus_epoch WHERE t.local_manifest_id=${archive.id}
          AND t.required_quorum=2 AND a.local_quorum_status='${expected}' AND a.localization_complete
          ${index === TARGET ? "" : `AND a.quorum_from_block <= ${f!.rootBlock} AND a.quorum_through_block >= ${f!.rootBlock}`})`), value => value === "t");
    }
    const findings = () => json<Finding[]>(TARGET, `SELECT COALESCE(json_agg(row_to_json(r)), '[]'::json)::text FROM
      (SELECT encode(handle,'hex') handle,detection_kind,reason,is_contained,healed_at,
       '0x'||encode(target_ct64_digest,'hex') target FROM drifted_handle WHERE host_chain_id=${chainId}
       AND consensus_epoch='legacy' AND handle IN (${h(f!.root)},${h(f!.child!)})) r`);
    const contained = await waitForManifestCondition("root and descendant containment", findings, rows =>
      rows.some(r => r.handle === f!.root.slice(2) && r.detection_kind === "verified" && r.reason === "ct64_mismatch" && r.is_contained && r.target === original.digest)
      && rows.some(r => r.handle === f!.child!.slice(2) && r.detection_kind === "inferred" && r.is_contained));
    assert(contained.every(row => row.healed_at === null), "healing completed before containment assertions; this profile needs a healing gate when installation is enabled");
    report.findings = contained;
    assert.deepEqual(await material(TARGET, f.root), original, "manifest injection changed stored ciphertext/digest/version");
    await runFixture("submit consumers");
    f = await fixture();
    assert(f.blocked && f.independent, "consumer fixture omitted its handles");
    report.fixture = f;
    for (const index of [0, 1, 2]) {
      await waitForManifestCondition(`node ${index} independent progress`, () => q(index,
        `SELECT EXISTS(SELECT 1 FROM computations WHERE host_chain_id=${chainId} AND output_handle=${h(f!.independent!)} AND is_completed AND NOT is_error)`), value => value === "t");

    }
    // Independent work in this same transaction has completed: the worker had an opportunity to run it.
    for (let observation = 0; observation < 5; observation++) {
      assert.equal(await q(TARGET, `SELECT EXISTS(SELECT 1 FROM computations WHERE host_chain_id=${chainId} AND output_handle=${h(f.blocked)} AND NOT is_completed AND NOT is_error)
        AND NOT EXISTS(SELECT 1 FROM ciphertexts WHERE handle=${h(f.blocked)})`), "t", "contained consumer progressed unexpectedly");
      await Bun.sleep(1_000);
    }
    report.result = "passed";
    console.log("[pass] manifest-lifecycle: signed publication, quorum verification, containment, and independent progress");
  } catch (error) {
    report.result = "failed";
    report.error = String(error);
    for (const index of [0, 1, 2]) {
      const name = index === 0 ? "coprocessor-consensus-detector" : `coprocessor${index}-consensus-detector`;
      const logs = await run(["docker", "logs", name], { allowFailure: true });
      await fs.writeFile(path.join(directory, `detector-${index}.log`), logs.stdout + logs.stderr);
    }
    throw error;
  } finally {
    report.finalDriftTables = await captureDriftTables(q);
    // Restore normal startup even when submission/assertions failed. Preserve the immutable evidence.
    try {
      if (!stopped) await run(["docker", "stop", DETECTOR]);
      await fs.rm(configPath, { force: true });
      await fs.rm(`${configPath}.tmp`, { force: true });
    } finally {
      await run(["docker", "start", DETECTOR]);
      await fs.writeFile(reportPath, JSON.stringify(report, null, 2));
      console.log(`[manifest-lifecycle] report: ${reportPath}`);
    }
  }
}
