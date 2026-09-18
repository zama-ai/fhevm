/** Healthy publication and verification with every detector continuously running. */
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
import { validateFixture, waitForManifestCondition } from "./manifest-lifecycle";

type Descriptor = { handle: string; status: string; ct64_digest?: string };
type Archive = { id: number; publication_block_number: number; object_key: string; body: string; bucket: string };
const NODES = [0, 1, 2];
const FIXTURE_PATH = "/tmp/manifest-lifecycle-fixture.json";
const detector = (index: number) => `coprocessor${index || ""}-consensus-detector`;

export function assertHealthyDescriptor(descriptors: Descriptor[], handle: string, digest: string) {
  const matches = descriptors.filter(d => d.handle.toLowerCase() === handle.toLowerCase());
  assert.equal(matches.length, 1, "manifest must contain the fixture handle exactly once");
  assert.equal(matches[0]!.status, "computed", "fixture handle must be computed, not a timeout placeholder");
  assert.equal(matches[0]!.ct64_digest?.toLowerCase(), digest.toLowerCase(), "manifest ct64 digest differs from uploaded material");
}

export async function runManifestLifecycleNoDriftProfile(
  state: State,
  query: (database: string, sql: string) => Promise<string>,
  runFixture: (phase: "seed graph" | "submit consumers") => Promise<void>,
) {
  const topology = topologyForState(state);
  if (topology.count !== 3 || topology.threshold !== 2 || state.scenario.kind !== "coprocessor-consensus") {
    throw new PreflightError("manifest-lifecycle-no-drift requires --scenario manifest-lifecycle (three nodes, quorum two)");
  }
  for (const node of NODES) {
    const [container] = JSON.parse((await run(["docker", "inspect", detector(node)])).stdout);
    assert(container.State.Running && container.Config.Cmd.includes(`--dangerous-drift-injection=${MANIFEST_INJECTION_PATH}`)
      && container.Mounts.some((m: { Source: string; Destination: string }) => m.Source === manifestInjectionDir(node) && m.Destination === "/manifest-drift"),
    `node ${node} requires the running manifest-lifecycle scenario`);
    await fs.access(path.join(manifestInjectionDir(node), "injection.json")).then(
      () => { throw new PreflightError(`node ${node} has an injection configuration; use a fresh stack`); },
      (error: NodeJS.ErrnoException) => { if (error.code !== "ENOENT") throw error; });
  }
  const env = await readEnvFile(envPath("coprocessor"));
  const chainId = Number(env.CHAIN_ID);
  assert(Number.isSafeInteger(chainId) && chainId >= 0);
  const q = (node: number, sql: string) => query(coprocessorDatabaseName(node), sql);
  const json = async <T>(node: number, sql: string): Promise<T> => JSON.parse(await q(node, sql));
  const readFixture = async () => validateFixture(JSON.parse((await run(["docker", "exec", TEST_SUITE_CONTAINER, "cat", FIXTURE_PATH])).stdout));
  const report: Record<string, unknown> = { chainId, checkpoints: [] };
  const reportPath = path.join(manifestInjectionDir(2), "no-drift-report.json");
  try {
    for (const node of NODES) assert.equal(await q(node, "SELECT count(*) FROM drifted_handle"), "0", `node ${node} already has drift findings; use a fresh stack`);
    await run(["docker", "exec", TEST_SUITE_CONTAINER, "rm", "-f", FIXTURE_PATH]);
    for (const phase of ["seed graph", "submit consumers"] as const) {
      await runFixture(phase);
      const fixture = await readFixture();
      assert.equal(fixture.chainId, chainId);
      report.fixture = fixture;
      const handles = phase === "seed graph" ? [fixture.root, fixture.child] : [fixture.blocked, fixture.independent];
      assert(handles.every(handle => handle !== undefined), "fixture omitted expected handles");
      for (const handle of handles as string[]) {
        // validateFixture has checked these values before they enter SQL.
        const h = `decode('${handle.slice(2)}','hex')`;
        let expectedDigest: string | undefined;
        for (const node of NODES) {
          const digest = await waitForManifestCondition(`node ${node} computed/uploaded ${handle}`, () => q(node,
            `SELECT COALESCE((SELECT '0x'||encode(d.ciphertext,'hex') FROM ciphertext_digest d
              WHERE d.host_chain_id=${chainId} AND d.handle=${h}
              AND d.s3_publication_verified_at IS NOT NULL AND d.s3_publication_verified_digest=d.ciphertext
              AND EXISTS(SELECT 1 FROM computations c WHERE c.host_chain_id=${chainId}
                AND c.output_handle=${h} AND c.is_completed AND NOT c.is_error) LIMIT 1),'')`), value => /^0x[0-9a-f]{64}$/.test(value));
          expectedDigest ??= digest;
          assert.equal(digest, expectedDigest, `node ${node} ciphertext digest differs`);
          const archive = (await waitForManifestCondition(`node ${node} manifest containing ${handle}`, () => json<Archive | null>(node,
            `SELECT COALESCE((SELECT row_to_json(r)::text FROM (
              SELECT m.id,m.publication_block_number,m.object_key,encode(m.signed_manifest,'hex') body,g.s3_bucket_url bucket
              FROM block_manifest m JOIN public.gateway_config_coprocessors g ON g.signer_address=m.publisher
              JOIN handle_producer_block p ON p.host_chain_id=m.host_chain_id
                AND p.producer_block_number=m.publication_block_number AND p.producer_block_hash=m.publication_block_hash
              WHERE p.host_chain_id=${chainId} AND p.handle=${h} AND m.consensus_epoch='legacy'
                AND m.manifest_source='local' ORDER BY m.revision DESC LIMIT 1) r),'null')`), value => value !== null))!;
          const url = `${archive.bucket.replace(/\/$/, "")}/${archive.object_key}`;
          const downloaded = await run(["docker", "exec", TEST_SUITE_CONTAINER, "node", "-e",
            "fetch(process.argv[1],{signal:AbortSignal.timeout(15000)}).then(async r=>{if(!r.ok)throw Error('HTTP '+r.status);process.stdout.write(Buffer.from(await r.arrayBuffer()).toString('hex'))}).catch(e=>{console.error(e);process.exit(1)})", url]);
          assert.equal(downloaded.stdout.trim(), archive.body, "S3/archive bytes differ");
          const manifest = JSON.parse(Buffer.from(archive.body, "hex").toString());
          assertHealthyDescriptor(manifest.detailed_range.blocks.flatMap((b: { ciphertexts: Descriptor[] }) => b.ciphertexts), handle, digest);
          await waitForManifestCondition(`node ${node} quorum for ${handle}`, () => q(node,
            `SELECT EXISTS(SELECT 1 FROM block_manifest_verification_task t JOIN block_manifest_verification_attempt a
              ON a.task_id=t.id AND a.consensus_epoch=t.consensus_epoch WHERE t.local_manifest_id=${archive.id}
              AND t.required_quorum=2 AND a.outcome='consensus' AND a.local_quorum_status='matches_quorum'
              AND a.quorum_from_block <= ${fixture.rootBlock}
              AND a.quorum_through_block >= ${archive.publication_block_number}
              AND a.localization_complete AND a.drifted_block_count=0 AND a.drifted_handle_count=0)`), value => value === "t");
          const coverage = await json(node, `SELECT row_to_json(r)::text FROM (
            SELECT a.quorum_from_block,a.quorum_through_block,
              a.unverified_prefix_from_block,a.unverified_prefix_through_block
            FROM block_manifest_verification_task t JOIN block_manifest_verification_attempt a
              ON a.task_id=t.id AND a.consensus_epoch=t.consensus_epoch
            WHERE t.local_manifest_id=${archive.id} AND a.local_quorum_status='matches_quorum'
              AND a.quorum_from_block <= ${fixture.rootBlock}
              AND a.quorum_through_block >= ${archive.publication_block_number}
            ORDER BY a.attempt DESC LIMIT 1) r`);
          (report.checkpoints as unknown[]).push({ phase, node, handle, digest, manifestId: archive.id, coverage });
          console.log(`[manifest-lifecycle-no-drift] node ${node} quorum coverage: ${JSON.stringify(coverage)}`);
          console.log(`[manifest-lifecycle-no-drift] node ${node}: ${phase}, ${handle} published and verified`);
        }
      }
      for (const node of NODES) assert.equal(await q(node, "SELECT count(*) FROM drifted_handle"), "0", `node ${node} recorded unexpected drift`);
    }
    report.result = "passed";
  } catch (error) {
    report.result = "failed";
    report.error = String(error);
    throw error;
  } finally {
    report.finalDriftTables = await captureDriftTables(q);
    await fs.writeFile(reportPath, JSON.stringify(report, null, 2));
    console.log(`[manifest-lifecycle-no-drift] report: ${reportPath}`);
  }
}
