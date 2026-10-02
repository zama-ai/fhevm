import assert from "node:assert/strict";

type Query = (database: string, sql: string) => Promise<string>;

/**
 * Corrupts the ct64 of one handle Green computes during the dry run, on one
 * operator, so the drift is born in the upgrade epoch before cutover. Only an
 * allowed output, which therefore enters a manifest, is chosen, past the
 * dry-run probe's block: the probe keeps anchoring the upgrade, and its
 * manifest still reaches consensus everywhere. The state table
 * lives in `public` so it survives cutover renaming the Green schema.
 */
export const dryRunDriftInstallSql = (gcsSchema: string) => `
BEGIN;
CREATE TABLE public.e2e_dry_run_drift (
  id BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (id),
  handle BYTEA NOT NULL, host_chain_id BIGINT NOT NULL, block_number BIGINT NOT NULL,
  byte_offset INTEGER NOT NULL, original_byte INTEGER NOT NULL,
  injected_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE FUNCTION public.e2e_dry_run_drift_inject() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE
  producer_chain BIGINT;
  producer_block BIGINT;
  produced INTEGER;
  mid INTEGER;
BEGIN
  IF NEW.is_input OR NEW.ciphertext_version <> 0 OR octet_length(NEW.ciphertext) <= 64 THEN RETURN NULL; END IF;
  EXECUTE format(
    'SELECT p.host_chain_id, p.producer_block_number FROM %I.handle_producer_block p
      WHERE p.handle = $1
        -- Past the probe block, whose manifest range starts at start_block.
        AND p.producer_block_number > (SELECT u.start_block + 1 FROM public.upgrade_state u
                                        WHERE u.stack_role = ''GCS'' AND u.host_chain_id = p.host_chain_id)
        AND NOT EXISTS (
        SELECT 1 FROM %I.computations c JOIN public.upgrade_state u ON u.stack_role = ''GCS''
         WHERE c.output_handle = p.handle AND octet_length(c.transaction_id) = 32
           AND position(c.transaction_id IN u.synthetic_txn_hashes) > 0)
      LIMIT 1', TG_TABLE_SCHEMA, TG_TABLE_SCHEMA)
    INTO producer_chain, producer_block USING NEW.handle;
  GET DIAGNOSTICS produced = ROW_COUNT;
  IF produced = 0 THEN RETURN NULL; END IF;
  mid := octet_length(NEW.ciphertext) / 2;
  INSERT INTO public.e2e_dry_run_drift(handle, host_chain_id, block_number, byte_offset, original_byte)
    VALUES (NEW.handle, producer_chain, producer_block, mid, get_byte(NEW.ciphertext, mid))
    ON CONFLICT (id) DO NOTHING;
  IF NOT FOUND THEN RETURN NULL; END IF;
  EXECUTE format(
    'UPDATE %I.ciphertexts SET ciphertext = set_byte(ciphertext, $2, get_byte(ciphertext, $2) # 128)
      WHERE handle = $1 AND ciphertext_version = 0', TG_TABLE_SCHEMA)
    USING NEW.handle, mid;
  RETURN NULL;
END;
$$;
CREATE TRIGGER e2e_dry_run_drift AFTER INSERT ON ${quoteIdent(gcsSchema)}.ciphertexts
  FOR EACH ROW EXECUTE FUNCTION public.e2e_dry_run_drift_inject();
COMMIT;`;

/** Drops the trigger wherever cutover moved its table, then the state. */
export const DRY_RUN_DRIFT_CLEANUP_SQL =
  "DROP FUNCTION IF EXISTS public.e2e_dry_run_drift_inject() CASCADE; DROP TABLE IF EXISTS public.e2e_dry_run_drift;";

export const GCS_SCHEMA_SQL = "SELECT nspname FROM pg_namespace WHERE nspname LIKE 'gcs-%' ORDER BY nspname";

/** Finding reasons healing repairs; drifted_handle.can_be_healed also requires a pinned target. */
const RECOVERABLE_REASONS = "('ct64_mismatch', 'missing_here', 'error_here', 'uncomputed_here')";
const quoteIdent = (name: string) => `"${name.replaceAll('"', '""')}"`;
const sqlText = (value: string) => `'${value.replaceAll("'", "''")}'`;

type Injected = { handle: string; host_chain_id: number; block_number: number; byte_offset: number; original_byte: number };
type Finding = { handle: string; reason: string; detection_kind: string; is_contained: boolean; healed_at: string | null };

/**
 * After cutover: the corrupted dry-run handle is a drift of the upgrade epoch
 * on the faulty operator only, it is healed there, and its bytes then match
 * every peer. Every healable finding the corruption caused, its inferred
 * descendants included, is healed too.
 */
export async function assertDryRunDriftHealed(options: {
  faultyDatabase: string;
  databases: string[];
  epoch: string;
  query: Query;
  waitFor: (label: string, read: () => Promise<string>, done: (value: string) => boolean, timeoutMs: number) => Promise<string>;
  checkpoint: (entry: Record<string, unknown>) => void;
}) {
  const { faultyDatabase: db, databases, epoch, query, waitFor, checkpoint } = options;
  const injected = JSON.parse(await query(db, `SELECT COALESCE((SELECT row_to_json(r)::text FROM (
      SELECT encode(handle, 'hex') AS handle, host_chain_id, block_number, byte_offset, original_byte
        FROM public.e2e_dry_run_drift) r), 'null')`)) as Injected | null;
  assert(injected, `${db}: the dry-run drift trigger never fired (no allowed handle computed by Green before cutover)`);
  const window = JSON.parse(await query(db, `SELECT COALESCE((SELECT row_to_json(w)::text FROM (
      SELECT start_block, upload_start_block FROM consensus_epoch_block_window
       WHERE consensus_epoch = ${sqlText(epoch)} AND host_chain_id = ${injected.host_chain_id}) w), 'null')`)) as
    { start_block: number; upload_start_block: number | null } | null;
  assert(window, `${db}: epoch ${epoch} has no block window on chain ${injected.host_chain_id}`);
  assert(window.upload_start_block !== null, `${db}: epoch ${epoch} recorded no upload start at cutover`);
  assert(injected.block_number >= window.start_block && injected.block_number < window.upload_start_block,
    `${db}: drifted block ${injected.block_number} is outside the dry run [${window.start_block}, ${window.upload_start_block})`);
  checkpoint({ phase: "dry-run drift injected", db, epoch, ...injected, dryRun: window });

  const handle = `decode('${injected.handle}', 'hex')`;
  const finding = async () => JSON.parse(await query(db, `SELECT COALESCE((SELECT row_to_json(r)::text FROM (
      SELECT encode(handle, 'hex') AS handle, reason, detection_kind, is_contained, healed_at
        FROM drifted_handle WHERE consensus_epoch = ${sqlText(epoch)} AND handle = ${handle}
       ORDER BY id LIMIT 1) r), 'null')`)) as Finding | null;
  await waitFor(`${db} detects the dry-run drift in epoch ${epoch}`, async () => JSON.stringify(await finding()),
    value => value !== "null", 600_000);
  const detected = (await finding())!;
  assert.equal(detected.reason, "ct64_mismatch", `${db}: dry-run drift reason`);
  await waitFor(`${db} heals the dry-run drift`, async () => JSON.stringify((await finding())?.healed_at ?? null),
    value => value !== "null", 900_000);
  // Pinned or not: an inferred descendant has no quorum digest until healing pins one, yet it
  // keeps its TFHE consumers frozen, so `can_be_healed` alone would let it slip through.
  await waitFor(`${db} has no unresolved recoverable finding in epoch ${epoch}`, () => query(db,
    `SELECT count(*) FROM drifted_handle WHERE consensus_epoch = ${sqlText(epoch)}
        AND reason IN ${RECOVERABLE_REASONS} AND healed_at IS NULL AND superseded_at IS NULL`),
    value => value === "0", 900_000);

  const restored = await query(db, `SELECT get_byte(ciphertext, ${injected.byte_offset}) FROM public.ciphertexts
     WHERE handle = ${handle} AND ciphertext_version = 0`);
  assert.equal(Number(restored), injected.original_byte, `${db}: healed ct64 keeps the corrupted byte`);
  // Every finding, the root and each inferred descendant, must end with the peers' ct64:
  // healed or recomputed, the frozen work ran again.
  const findings = JSON.parse(await query(db, `SELECT COALESCE(json_agg(json_build_object(
        'handle', encode(handle, 'hex'), 'detection', detection_kind) ORDER BY id), '[]')::text
      FROM drifted_handle WHERE consensus_epoch = ${sqlText(epoch)} AND superseded_at IS NULL`)) as
    { handle: string; detection: string }[];
  assert(findings.some(f => f.handle === injected.handle), `${db}: the injected handle has no finding`);
  for (const { handle: finding, detection } of findings) {
    const digests = new Map<string, string>();
    for (const peer of databases) {
      digests.set(peer, await query(peer, `SELECT COALESCE((SELECT encode(sha256(ciphertext), 'hex')
          FROM public.ciphertexts WHERE handle = decode('${finding}', 'hex') AND ciphertext_version = 0), '')`));
    }
    const values = [...digests.values()];
    assert(values.every(value => value !== "") && new Set(values).size === 1,
      `${detection} finding ${finding} did not recover to the peers' ct64: ${JSON.stringify(Object.fromEntries(digests))}`);
  }
  const byDetection = findings.reduce<Record<string, number>>((counts, f) => ({ ...counts, [f.detection]: (counts[f.detection] ?? 0) + 1 }), {});
  checkpoint({ phase: "dry-run drift healed", db, epoch, handle: injected.handle, detection: detected.detection_kind,
    contained: detected.is_contained, findings: byDetection });
}
