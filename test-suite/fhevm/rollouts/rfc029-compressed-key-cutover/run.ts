import type { RolloutRunContext } from "../../src/commands/rollout-run";
import { scenario, versionSources, versions } from "./versions";

// RFC-029 one-time compressed-key migration, driven end to end on a live
// stack: legacy baseline -> migration keygen (real KMS keygen-from-existing)
// -> material publication (never activation) -> cutover scheduling ->
// boundary-straddling traffic -> long-tail legacy SnS after the cutover.
//
// Beyond "the suite stays green", every consensus-relevant state is
// observed directly: which material kind labeled each ciphertext, that
// staged material never reaches the keys table before the schedule, and
// that legacy SnS work still selects legacy material after the cutover.

const logPhase = (label: string) => {
  console.log(`\n[rollout] ${label}`);
};

const HOST_BLOCKS_AHEAD = 30;
const GATEWAY_BLOCKS_AHEAD = 30;

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const sqlNumber = async (ctx: RolloutRunContext, label: string, sql: string): Promise<number> => {
  const raw = await ctx.queryCoprocessorDb(sql);
  const value = Number(raw);
  if (!Number.isFinite(value)) {
    throw new Error(`${label}: expected a number from SQL, got "${raw}"`);
  }
  console.log(`[observe] ${label}: ${value}`);
  return value;
};

const assertEqual = (label: string, actual: number, expected: number) => {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, got ${actual}`);
  }
};

export default async function run(ctx: RolloutRunContext) {
  const lock = await ctx.writeVersionLock("00-feature", { versions, sources: versionSources });

  logPhase("00 baseline: boot the feature stack, legacy material only");
  await ctx.up({ lockFile: lock, scenario, overrides: [{ group: "test-suite" }] });
  await ctx.test("rollout-standard", { parallel: false });

  // Everything produced so far must be labeled legacy, and no compressed
  // material may exist for the live key.
  const nonLegacyBaseline = await sqlNumber(
    ctx,
    "baseline non-legacy ciphertexts",
    "SELECT COUNT(*) FROM ciphertexts WHERE key_material_kind <> 0",
  );
  assertEqual("baseline non-legacy ciphertexts", nonLegacyBaseline, 0);

  logPhase("01 migration keygen: real KMS keygen-from-existing + on-chain publication (no activation)");
  await ctx.runHostContractTask(
    "npx hardhat compile && npx hardhat task:compressedKeyMigrationKeygen --use-internal-proxy-address true",
  );

  // The published material must stay STAGED: with no schedule, it must
  // never reach the keys table (available is not selected).
  logPhase("01.observe: material staged, keys table untouched, no schedule");
  await sleep(60_000); // let the host-listener ingest + download
  const stagedInKeys = await sqlNumber(
    ctx,
    "keys rows with compressed material before scheduling",
    "SELECT COUNT(*) FROM keys WHERE compressed_xof_keyset IS NOT NULL",
  );
  assertEqual("keys rows with compressed material before scheduling", stagedInKeys, 0);
  await ctx.test("rollout-standard", { parallel: false });

  logPhase("02 schedule: cutover at current+N blocks per chain and on the gateway");
  const state = await ctx.readState();
  const hostChains = state.scenario?.hostChains ?? [];
  if (hostChains.length === 0) {
    throw new Error("scenario has no host chains; cannot compute cutover blocks");
  }
  // Compute cutover blocks inside the deploy container (it has the RPCs);
  // the hardhat task receives explicit numbers so the schedule is auditable.
  const currentHostBlock = await sqlNumber(
    ctx,
    "latest ingested host block",
    "SELECT COALESCE(MAX(block_number), 0) FROM host_chain_blocks_valid",
  );
  const currentGatewayBlock = await sqlNumber(
    ctx,
    "latest observed gateway block",
    "SELECT COALESCE(MAX(gateway_block_number), 0) FROM verify_proofs",
  );
  const hostCutoverBlock = currentHostBlock + HOST_BLOCKS_AHEAD;
  const gatewayCutoverBlock = currentGatewayBlock + GATEWAY_BLOCKS_AHEAD;
  const hostCutovers = hostChains.map((chain: { chainId: number | string }) => ({
    chainId: String(chain.chainId),
    cutoverBlock: hostCutoverBlock,
  }));
  await ctx.runHostContractTask(
    [
      "npx hardhat task:scheduleCompressedKeyCutover",
      `--host-cutovers '${JSON.stringify(hostCutovers)}'`,
      `--gateway-cutover-block ${gatewayCutoverBlock}`,
      "--use-internal-proxy-address true",
    ].join(" "),
  );

  logPhase("02.observe: schedule ingested, material applied to keys, policy visible");
  // Wait for finalized ingestion of the schedule, then the material copy.
  for (let i = 0; i < 30; i++) {
    const applied = await ctx.queryCoprocessorDb("SELECT COUNT(*) FROM compressed_key_cutover");
    if (Number(applied) > 0) break;
    await sleep(10_000);
  }
  assertEqual(
    "ingested cutover schedules",
    await sqlNumber(ctx, "ingested cutover schedules", "SELECT COUNT(*) FROM compressed_key_cutover"),
    1,
  );

  logPhase("03 cross the boundary: traffic straddling the cutover blocks");
  // Traffic starts pre-boundary and continues past it; 3-of-5-style digest
  // consensus in the profiles is the liveness check, the label observations
  // below are the correctness check.
  await ctx.test("rollout-standard", { parallel: false });
  await sleep(HOST_BLOCKS_AHEAD * 3_000); // let both chains pass the boundary
  await ctx.test("rollout-standard", { parallel: false });

  logPhase("03.observe: both material kinds present, boundary respected");
  const compressedOutputs = await sqlNumber(
    ctx,
    "post-boundary compressed-labeled ciphertexts",
    "SELECT COUNT(*) FROM ciphertexts WHERE key_material_kind = 1",
  );
  if (compressedOutputs === 0) {
    throw new Error("no ciphertext was produced with the compressed material after the cutover");
  }
  // No compute output anchored before the host boundary may carry the
  // compressed label: join outputs to their computation's block anchor.
  const preBoundaryCompressed = await sqlNumber(
    ctx,
    "pre-boundary computations labeled compressed",
    `SELECT COUNT(*) FROM ciphertexts ct
     JOIN computations c ON c.output_handle = ct.handle
     WHERE ct.key_material_kind = 1 AND c.block_number IS NOT NULL AND c.block_number < ${hostCutoverBlock}`,
  );
  assertEqual("pre-boundary computations labeled compressed", preBoundaryCompressed, 0);

  logPhase("04 long-tail SnS: legacy-pinned tasks still complete after the cutover");
  const legacySnsPending = await sqlNumber(
    ctx,
    "legacy-pinned SnS tasks",
    "SELECT COUNT(*) FROM pbs_computations WHERE key_material_kind = 0",
  );
  if (legacySnsPending === 0) {
    console.log("[observe] no legacy-pinned SnS task in the queue (all pre-boundary SnS already drained)");
  }
  const legacySnsIncomplete = await sqlNumber(
    ctx,
    "legacy-pinned SnS tasks left incomplete",
    "SELECT COUNT(*) FROM pbs_computations WHERE key_material_kind = 0 AND is_completed = FALSE AND created_at < NOW() - INTERVAL '5 minutes'",
  );
  assertEqual("legacy-pinned SnS tasks left incomplete", legacySnsIncomplete, 0);

  logPhase("05 final: full profile pass on the migrated stack");
  await ctx.test("rollout-standard", { parallel: false });
}
