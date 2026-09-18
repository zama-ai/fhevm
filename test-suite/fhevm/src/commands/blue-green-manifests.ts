/**
 * Manifest publication and verification checks for the blue-green profile
 * (`BLUE_GREEN_MANIFESTS=1`). The pinned v0.14 Blue has no consensus-detector,
 * so only Green publishes: parked before DryRunStarted, in the proposal's
 * consensus epoch during the dry run, and in that same epoch after cutover.
 * Every check is no-drift: any drift outcome or drifted_handle row fails.
 */
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import { PreflightError } from "../errors";
import { TEST_SUITE_CONTAINER } from "../layout";
import { run } from "../utils/process";
import { waitForManifestCondition } from "./manifest-lifecycle";

type Query = (database: string, sql: string) => Promise<string>;

/** One local manifest's detailed range, from its signed JSON body. */
export type PublishedRange = { publicationBlock: number; first: number; last: number };

const sqlText = (value: string) => `'${value.replaceAll("'", "''")}'`;
const detector = (index: number) => `coprocessor${index || ""}-gcs-consensus-detector`;
const block = (value: string) => {
  const parsed = Number(BigInt(value));
  assert(Number.isSafeInteger(parsed), `block number ${value} is out of range`);
  return parsed;
};

/**
 * Consecutive local manifests of one chain must cover consecutive blocks:
 * each detailed range starts right after the previous one ends.
 */
export function rangeGaps(ranges: PublishedRange[]): string[] {
  const sorted = [...ranges].sort((a, b) => a.publicationBlock - b.publicationBlock);
  const gaps: string[] = [];
  for (let index = 1; index < sorted.length; index += 1) {
    const previous = sorted[index - 1]!;
    const current = sorted[index]!;
    if (current.first !== previous.last + 1) {
      gaps.push(`manifest at ${current.publicationBlock} starts at ${current.first}, previous ended at ${previous.last}`);
    }
  }
  return gaps;
}

/** A local manifest whose verification reached quorum with the local operator in it. */
const verifiedManifestSql = (epoch: string, hostChainId: number, fromBlock: number, withComputed: boolean) => `
  SELECT COALESCE((SELECT row_to_json(r)::text FROM (
    SELECT m.id, m.publication_block_number, m.object_key,
           encode(m.signed_manifest, 'hex') AS body, g.s3_bucket_url AS bucket
      FROM block_manifest m
      JOIN public.gateway_config_coprocessors g ON g.signer_address = m.publisher
      JOIN block_manifest_verification_task t
        ON t.local_manifest_id = m.id AND t.consensus_epoch = m.consensus_epoch
      JOIN block_manifest_verification_attempt a
        ON a.task_id = t.id AND a.consensus_epoch = t.consensus_epoch
     WHERE m.consensus_epoch = ${sqlText(epoch)} AND m.manifest_source = 'local'
       AND m.host_chain_id = ${hostChainId} AND m.publication_block_number > ${fromBlock}
       AND a.outcome = 'consensus' AND a.local_quorum_status = 'matches_quorum'
       AND a.drifted_block_count = 0 AND a.drifted_handle_count = 0
       ${withComputed ? `AND EXISTS (
         SELECT 1
           FROM jsonb_array_elements(convert_from(m.signed_manifest, 'UTF8')::jsonb -> 'detailed_range' -> 'blocks') b,
                jsonb_array_elements(b -> 'ciphertexts') c
          WHERE c ->> 'status' = 'computed')` : ""}
     ORDER BY m.publication_block_number LIMIT 1) r), 'null')`;

type VerifiedManifest = { id: number; publication_block_number: number; object_key: string; body: string; bucket: string };

export function createBlueGreenManifestChecks(options: {
  databases: string[];
  hostChainIds: number[];
  query: Query;
  reportPath: string;
}) {
  const { databases, hostChainIds, query, reportPath } = options;
  const report: Record<string, unknown> = { checkpoints: [] };
  const checkpoint = (entry: Record<string, unknown>) => {
    (report.checkpoints as unknown[]).push(entry);
    console.log(`[blue-green-manifests] ${JSON.stringify(entry)}`);
  };
  const count = async (db: string, sql: string) => Number(await query(db, sql));
  let upgradeEpoch: string | undefined;
  const cutoverBlocks = new Map<string, Map<number, number>>();

  const assertNoDrift = async (label: string, epoch?: string) => {
    for (const db of databases) {
      const scope = epoch ? `WHERE consensus_epoch = ${sqlText(epoch)}` : "";
      assert.equal(await count(db, `SELECT count(*) FROM drifted_handle ${scope}`), 0, `${db} ${label}: drifted_handle rows`);
      assert.equal(await count(db, `SELECT count(*) FROM block_manifest_verification_attempt ${scope ? `${scope} AND` : "WHERE"} outcome = 'drift'`),
        0, `${db} ${label}: drift verification outcomes`);
    }
  };

  const assertS3Copy = async (db: string, manifest: VerifiedManifest, epoch: string) => {
    assert(manifest.object_key.includes("/consensus_epoch/"), `${db} object key ${manifest.object_key} has no consensus epoch segment`);
    assert(!manifest.object_key.includes("/consensus_epoch/legacy/"), `${db} object key ${manifest.object_key} uses the legacy epoch`);
    const url = `${manifest.bucket.replace(/\/$/, "")}/${manifest.object_key}`;
    const downloaded = await run(["docker", "exec", TEST_SUITE_CONTAINER, "node", "-e",
      "fetch(process.argv[1],{signal:AbortSignal.timeout(15000)}).then(async r=>{if(!r.ok)throw Error('HTTP '+r.status);process.stdout.write(Buffer.from(await r.arrayBuffer()).toString('hex'))}).catch(e=>{console.error(e);process.exit(1)})", url]);
    assert.equal(downloaded.stdout.trim(), manifest.body, `${db} S3 object ${url} differs from the archived manifest`);
    const signed = JSON.parse(Buffer.from(manifest.body, "hex").toString());
    assert.equal(signed.consensus_epoch, epoch, `${db} signed manifest epoch`);
  };

  const waitVerified = async (label: string, db: string, epoch: string, chain: number, fromBlock: number, withComputed: boolean) => {
    const manifest = (await waitForManifestCondition(`${db} chain ${chain} ${label}`,
      async () => JSON.parse(await query(db, verifiedManifestSql(epoch, chain, fromBlock, withComputed))) as VerifiedManifest | null,
      value => value !== null, 600_000))!;
    await assertS3Copy(db, manifest, epoch);
    return manifest;
  };

  const publishedRanges = async (db: string, epoch: string, chain: number): Promise<PublishedRange[]> => {
    const rows = JSON.parse(await query(db, `
      SELECT COALESCE(json_agg(json_build_object(
               'publicationBlock', m.publication_block_number,
               'first', convert_from(m.signed_manifest, 'UTF8')::jsonb -> 'detailed_range' ->> 'first_block_number',
               'last', convert_from(m.signed_manifest, 'UTF8')::jsonb -> 'detailed_range' ->> 'last_block_number')
             ORDER BY m.publication_block_number), '[]')::text
        FROM block_manifest m
       WHERE m.consensus_epoch = ${sqlText(epoch)} AND m.manifest_source = 'local' AND m.host_chain_id = ${chain}`)) as
      { publicationBlock: number; first: string; last: string }[];
    return rows.map(row => ({ publicationBlock: row.publicationBlock, first: block(row.first), last: block(row.last) }));
  };

  return {
    /** Preflight: every operator runs a Green detector that publishes to a bucket. */
    async preflight() {
      if (databases.length < 2) throw new PreflightError("BLUE_GREEN_MANIFESTS needs at least two operators to reach a quorum");
      for (let index = 0; index < databases.length; index += 1) {
        const [container] = JSON.parse((await run(["docker", "inspect", detector(index)])).stdout);
        assert(container?.State?.Running, `${detector(index)} is not running`);
        const args: string[] = container.Config.Cmd ?? [];
        assert(args.some(arg => arg.startsWith("--my-bucket=") && arg !== "--my-bucket=none"),
          `${detector(index)} does not publish manifests (--my-bucket)`);
      }
      for (const db of databases) {
        assert(await count(db, "SELECT count(*) FROM public.gateway_config_coprocessors") >= 2,
          `${db} has no peer registry snapshot; Green gw-listener must fill gateway_config_coprocessors`);
      }
    },

    /** Blue has no detector and Green is parked: nothing is published yet. */
    async beforeProposals() {
      for (const db of databases) {
        assert.equal(await count(db, "SELECT count(*) FROM block_manifest"), 0,
          `${db} published manifests before any DryRunStarted (Green must stay parked)`);
      }
      checkpoint({ phase: "before proposals", manifests: 0 });
    },

    /** The rolled-back epoch stops publishing and leaves no drift. */
    async afterRollback() {
      for (const db of databases) {
        const epochs = JSON.parse(await query(db,
          "SELECT COALESCE(json_agg(consensus_epoch ORDER BY allocated_at), '[]')::text FROM consensus_epoch_history WHERE outcome = 'failed'")) as string[];
        assert.equal(epochs.length, 1, `${db} expected exactly one failed consensus epoch, got ${JSON.stringify(epochs)}`);
        const epoch = epochs[0]!;
        const manifests = `SELECT count(*) FROM block_manifest WHERE consensus_epoch = ${sqlText(epoch)} AND manifest_source = 'local'`;
        const before = await count(db, manifests);
        await Bun.sleep(15_000);
        const after = await count(db, manifests);
        assert.equal(after, before, `${db} kept publishing in failed epoch ${epoch} after rollback (${before} -> ${after})`);
        await assertNoDrift("after rollback", epoch);
        checkpoint({ phase: "after rollback", db, epoch, localManifests: after });
      }
    },

    /** Dry run: Green publishes the proposal epoch and reaches quorum on computed handles. */
    async duringDryRun() {
      const epochs = new Set<string>();
      for (const db of databases) {
        epochs.add(await query(db,
          "SELECT consensus_epoch FROM consensus_epoch_history WHERE outcome IN ('pending', 'succeeded') AND proposal_id IS NOT NULL ORDER BY allocated_at DESC LIMIT 1"));
      }
      assert.equal(epochs.size, 1, `operators disagree on the upgrade epoch: ${JSON.stringify([...epochs])}`);
      upgradeEpoch = [...epochs][0]!;
      assert(upgradeEpoch && upgradeEpoch !== "legacy", `unexpected upgrade epoch ${upgradeEpoch}`);
      for (const db of databases) {
        for (const chain of hostChainIds) {
          const manifest = await waitVerified("dry-run manifest with computed handles", db, upgradeEpoch, chain, 0, true);
          const version = await query(db, "SELECT stack_version FROM versioning");
          checkpoint({ phase: "dry run", db, chain, epoch: upgradeEpoch, manifestId: manifest.id,
            publicationBlock: manifest.publication_block_number, stackVersionAtCheck: version });
        }
      }
      await assertNoDrift("dry run", upgradeEpoch);
    },

    /** Records each chain's latest ingested block once cutover is observed. */
    async markCutover() {
      for (const db of databases) {
        const blocks = new Map<number, number>();
        for (const chain of hostChainIds) {
          blocks.set(chain, await count(db, `SELECT COALESCE(max(block_number), 0) FROM host_chain_blocks_valid WHERE chain_id = ${chain}`));
        }
        cutoverBlocks.set(db, blocks);
      }
      checkpoint({ phase: "cutover", blocks: Object.fromEntries([...cutoverBlocks].map(([db, blocks]) => [db, Object.fromEntries(blocks)])) });
    },

    /** After cutover: same epoch, no gap across cutover, quorum on post-cutover blocks. */
    async afterCutover() {
      assert(upgradeEpoch, "duringDryRun must run before afterCutover");
      for (const db of databases) {
        const current = await query(db, "SELECT consensus_epoch FROM blue_green_consensus_epoch WHERE singleton");
        assert.equal(current, upgradeEpoch, `${db} active epoch after cutover`);
        assert.equal(await query(db, `SELECT outcome FROM consensus_epoch_history WHERE consensus_epoch = ${sqlText(upgradeEpoch)}`),
          "succeeded", `${db} upgrade epoch outcome`);
        assert.equal(await count(db, "SELECT count(*) FROM block_manifest WHERE consensus_epoch = 'legacy'"), 0,
          `${db} has legacy-epoch manifests, but the v0.14 Blue has no detector`);
        for (const chain of hostChainIds) {
          const cutoverBlock = cutoverBlocks.get(db)?.get(chain) ?? 0;
          const manifest = await waitVerified("post-cutover manifest", db, upgradeEpoch, chain, cutoverBlock, false);
          const ranges = await publishedRanges(db, upgradeEpoch, chain);
          assert.deepEqual(rangeGaps(ranges), [], `${db} chain ${chain} manifests leave gaps`);
          const startBlock = await query(db,
            `SELECT start_block FROM consensus_epoch_block_window WHERE consensus_epoch = ${sqlText(upgradeEpoch)} AND host_chain_id = ${chain}`);
          const exhausted = await count(db, `SELECT count(*) FROM block_manifest_verification_task t
            JOIN block_manifest m ON m.id = t.local_manifest_id AND m.consensus_epoch = t.consensus_epoch
           WHERE t.consensus_epoch = ${sqlText(upgradeEpoch)} AND m.host_chain_id = ${chain} AND t.state = 'retry_exhausted'`);
          assert.equal(exhausted, 0, `${db} chain ${chain} has verification tasks that never reached quorum`);
          checkpoint({ phase: "after cutover", db, chain, epoch: upgradeEpoch, manifestId: manifest.id,
            publicationBlock: manifest.publication_block_number, cutoverBlock, startBlock: Number(startBlock),
            firstPublishedBlock: ranges[0]?.first, lastPublishedBlock: ranges.at(-1)?.last, localManifests: ranges.length });
          if (ranges[0] && ranges[0].first !== Number(startBlock)) {
            console.warn(`[blue-green-manifests] ${db} chain ${chain}: first published block ${ranges[0].first} differs from epoch start_block ${startBlock}`);
          }
        }
      }
      await assertNoDrift("after cutover");
    },

    async writeReport(result: "passed" | "failed", error?: unknown) {
      report.result = result;
      report.upgradeEpoch = upgradeEpoch;
      if (error !== undefined) report.error = String(error);
      await fs.mkdir(path.dirname(reportPath), { recursive: true });
      await fs.writeFile(reportPath, JSON.stringify(report, null, 2));
      console.log(`[blue-green-manifests] report: ${reportPath}`);
    },
  };
}
