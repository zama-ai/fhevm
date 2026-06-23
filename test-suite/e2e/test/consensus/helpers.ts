/**
 * Shared helpers for multi-coprocessor consensus tests.
 *
 * These helpers provide:
 * - Gateway event polling (AddCiphertextMaterial / AddCiphertextMaterialConsensus)
 * - Cross-database digest comparison (querying each coprocessor's ciphertext_digest table)
 * - Docker container management (stop/start/restart/pause/unpause)
 */
import { ethers } from 'ethers';
import { exec as oldExec } from 'child_process';
import { promisify } from 'util';
import { Pool } from 'pg';

const exec = promisify(oldExec);

// ---------------------------------------------------------------------------
// Gateway event helpers
// ---------------------------------------------------------------------------

const CIPHERTEXT_COMMITS_ABI = [
  'event AddCiphertextMaterial(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address coprocessorTxSender)',
  'event AddCiphertextMaterialConsensus(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address[] coprocessorTxSenders)',
];

export interface ConsensusEvent {
  ctHandle: string;
  keyId: bigint;
  ciphertextDigest: string;
  snsCiphertextDigest: string;
  senders: string[];
  blockNumber: number;
}

export interface SubmissionEvent {
  ctHandle: string;
  keyId: bigint;
  ciphertextDigest: string;
  snsCiphertextDigest: string;
  sender: string;
}

/**
 * Wait for an AddCiphertextMaterialConsensus event for a specific handle.
 * Returns the event data or null if timeout expires.
 */
export async function waitForConsensus(
  gatewayRpcUrl: string,
  ciphertextCommitsAddress: string,
  ctHandle: string,
  timeoutMs: number = 600_000,
): Promise<ConsensusEvent | null> {
  const provider = new ethers.JsonRpcProvider(gatewayRpcUrl);
  const contract = new ethers.Contract(ciphertextCommitsAddress, CIPHERTEXT_COMMITS_ABI, provider);
  const deadline = Date.now() + timeoutMs;

  while (Date.now() < deadline) {
    const events = await contract.queryFilter(
      contract.filters.AddCiphertextMaterialConsensus(ctHandle),
    );
    if (events.length > 0) {
      const e = events[0] as ethers.EventLog;
      return {
        ctHandle: e.args[0],
        keyId: e.args[1],
        ciphertextDigest: e.args[2],
        snsCiphertextDigest: e.args[3],
        senders: e.args[4],
        blockNumber: e.blockNumber,
      };
    }
    await sleep(2000);
  }
  return null;
}

/**
 * Get all AddCiphertextMaterial submissions for a handle.
 */
export async function getSubmissions(
  gatewayRpcUrl: string,
  ciphertextCommitsAddress: string,
  ctHandle: string,
): Promise<SubmissionEvent[]> {
  const provider = new ethers.JsonRpcProvider(gatewayRpcUrl);
  const contract = new ethers.Contract(ciphertextCommitsAddress, CIPHERTEXT_COMMITS_ABI, provider);
  const events = await contract.queryFilter(
    contract.filters.AddCiphertextMaterial(ctHandle),
  );
  return events.map((e) => {
    const ev = e as ethers.EventLog;
    return {
      ctHandle: ev.args[0],
      keyId: ev.args[1],
      ciphertextDigest: ev.args[2],
      snsCiphertextDigest: ev.args[3],
      sender: ev.args[4],
    };
  });
}

// ---------------------------------------------------------------------------
// Cross-database digest helpers
// ---------------------------------------------------------------------------
//
// These helpers target `ciphertext_digest_branch` because the runtime
// (tfhe-worker, sns-worker, transaction-sender, gw-listener) reads from it
// unconditionally. The branch table's PK is
// `(handle, producer_block_hash, block_hash)`, so injecting multi-row fork
// residue works by picking distinct branch context values.

export interface DigestRow {
  handle: Buffer;
  producer_block_hash?: Buffer;
  block_hash?: Buffer;
  ciphertext: Buffer | null;
  ciphertext128: Buffer | null;
  txn_is_sent?: boolean;
}

export interface ArtificialDigestCleanupResult {
  repaired: number;
  unrepaired: number;
}

export interface DriftRevertSignal {
  id: number;
  host_chain_id: number;
  offending_host_block_number: number;
  status: string;
}

function bytes32ToBuffer(value: string): Buffer {
  return Buffer.from(value.replace('0x', ''), 'hex');
}

function bytes32ToHex(value: Buffer): string {
  return '0x' + value.toString('hex');
}

function isArtificialDigest(row: Pick<DigestRow, 'ciphertext' | 'ciphertext128'>): boolean {
  const ciphertext = row.ciphertext?.toString('hex');
  const ciphertext128 = row.ciphertext128?.toString('hex');
  return (
    (ciphertext === 'ff'.repeat(32) && ciphertext128 === 'ee'.repeat(32)) ||
    (ciphertext === 'aa'.repeat(32) && ciphertext128 === 'bb'.repeat(32))
  );
}

function sameBranchContext(left: DigestRow, right: DigestRow): boolean {
  return (
    (left.producer_block_hash ?? Buffer.alloc(0)).equals(right.producer_block_hash ?? Buffer.alloc(0)) &&
    (left.block_hash ?? Buffer.alloc(0)).equals(right.block_hash ?? Buffer.alloc(0))
  );
}

/**
 * Query ciphertext_digest_branch rows for a handle from a specific coprocessor database.
 * Multiple rows per handle are possible for branch-aware/event-scoped residue.
 */
export async function queryDigests(databaseUrl: string, handle: string): Promise<DigestRow[]> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const columns = await digestBranchColumns(pool);
    const producerBlockHashSelect = columns.has('producer_block_hash') ? 'producer_block_hash' : "''::bytea AS producer_block_hash";
    const blockHashSelect = columns.has('block_hash') ? 'block_hash' : "''::bytea AS block_hash";
    const txnIsSentSelect = columns.has('txn_is_sent') ? 'txn_is_sent' : 'false AS txn_is_sent';
    const orderBy = [
      columns.has('txn_is_sent') ? 'txn_is_sent DESC' : undefined,
      columns.has('created_at') ? 'created_at ASC' : undefined,
      columns.has('producer_block_hash') ? 'producer_block_hash ASC' : undefined,
      columns.has('block_hash') ? 'block_hash ASC' : undefined,
    ].filter(Boolean).join(', ');
    const result = await pool.query(
      `SELECT handle, ${producerBlockHashSelect}, ${blockHashSelect}, ciphertext, ciphertext128, ${txnIsSentSelect}
         FROM ciphertext_digest_branch
        WHERE handle = $1
        ${orderBy ? `ORDER BY ${orderBy}` : ''}`,
      [bytes32ToBuffer(handle)],
    );
    return result.rows;
  } finally {
    await pool.end();
  }
}

async function queryArtificialDigestRows(databaseUrl: string): Promise<DigestRow[]> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const columns = await digestBranchColumns(pool);
    const producerBlockHashSelect = columns.has('producer_block_hash') ? 'producer_block_hash' : "''::bytea AS producer_block_hash";
    const blockHashSelect = columns.has('block_hash') ? 'block_hash' : "''::bytea AS block_hash";
    const result = await pool.query(
      `SELECT handle, ${producerBlockHashSelect}, ${blockHashSelect}, ciphertext, ciphertext128
         FROM ciphertext_digest_branch
        WHERE (ciphertext = $1 AND ciphertext128 = $2)
           OR (ciphertext = $3 AND ciphertext128 = $4)`,
      [
        Buffer.alloc(32, 0xFF),
        Buffer.alloc(32, 0xEE),
        Buffer.alloc(32, 0xAA),
        Buffer.alloc(32, 0xBB),
      ],
    );
    return result.rows;
  } finally {
    await pool.end();
  }
}

async function digestBranchColumns(pool: Pool): Promise<Set<string>> {
  const result = await pool.query(
    "SELECT column_name FROM information_schema.columns WHERE table_name = 'ciphertext_digest_branch'",
  );
  return new Set(result.rows.map((row: { column_name: string }) => row.column_name));
}

/**
 * Return the local digest row matching a consensus event's emitted digests.
 * This avoids relying on unordered row position when branch residue exists.
 */
export function findConsensusDigestRow(
  rows: DigestRow[],
  ciphertextDigest: string,
  snsCiphertextDigest: string,
): DigestRow | undefined {
  const ciphertext = bytes32ToBuffer(ciphertextDigest);
  const ciphertext128 = bytes32ToBuffer(snsCiphertextDigest);
  return rows.find((row) => row.ciphertext?.equals(ciphertext) && row.ciphertext128?.equals(ciphertext128));
}

export async function waitForConsensusDigestRows(
  databaseUrls: string[],
  handle: string,
  ciphertextDigest: string,
  snsCiphertextDigest: string,
  timeoutMs: number = 60_000,
): Promise<DigestRow[][]> {
  const deadline = Date.now() + timeoutMs;
  let allRows: DigestRow[][] = [];
  while (Date.now() < deadline) {
    allRows = await Promise.all(databaseUrls.map((url) => queryDigests(url, handle)));
    if (allRows.every((rows) => findConsensusDigestRow(rows, ciphertextDigest, snsCiphertextDigest))) {
      return allRows;
    }
    await sleep(2_000);
  }
  return allRows;
}

async function hasBranchCiphertext(databaseUrl: string, handle: string): Promise<boolean> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query(
      `SELECT EXISTS(
         SELECT 1
           FROM ciphertexts_branch
          WHERE handle = $1
            AND ciphertext IS NOT NULL
            AND producer_block_hash <> ''::bytea
            AND block_number IS NOT NULL
          LIMIT 1
       ) AS found`,
      [bytes32ToBuffer(handle)],
    );
    return result.rows[0]?.found === true;
  } finally {
    await pool.end();
  }
}

export async function waitForBranchCiphertexts(
  databaseUrls: string[],
  handle: string,
  timeoutMs: number = 60_000,
): Promise<boolean[]> {
  const deadline = Date.now() + timeoutMs;
  let found: boolean[] = [];
  while (Date.now() < deadline) {
    found = await Promise.all(databaseUrls.map((url) => hasBranchCiphertext(url, handle)));
    if (found.every(Boolean)) {
      return found;
    }
    await sleep(2_000);
  }
  return found;
}

/**
 * Count ciphertext_digest_branch rows for a handle.
 */
export async function countDigestRows(databaseUrl: string, handle: string): Promise<number> {
  const rows = await queryDigests(databaseUrl, handle);
  return rows.length;
}

/**
 * Inject an extra ciphertext_digest_branch row (simulating fork residue).
 *
 * The branch-table PK `(handle, producer_block_hash, block_hash)` lets a
 * distinct branch context create a second row for the same handle, which is
 * what the drift detector must tolerate under branch-aware reads.
 */
export async function injectDigestRow(
  databaseUrl: string,
  handle: string,
  producerBlockHash: string,
  ciphertext: Buffer,
  ciphertext128: Buffer,
  blockHash: string = producerBlockHash,
): Promise<void> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const columns = await digestBranchColumns(pool);
    if (columns.has('block_hash')) {
      await pool.query(
        `INSERT INTO ciphertext_digest_branch (tenant_id, handle, producer_block_hash, block_hash, ciphertext, ciphertext128, host_chain_id, key_id_gw)
         VALUES (0, $1, $2, $3, $4, $5, 12345, $6)
         ON CONFLICT DO NOTHING`,
        [
          bytes32ToBuffer(handle),
          Buffer.from(producerBlockHash.replace('0x', ''), 'hex'),
          Buffer.from(blockHash.replace('0x', ''), 'hex'),
          ciphertext,
          ciphertext128,
          Buffer.alloc(32),
        ],
      );
    } else {
      await pool.query(
        `INSERT INTO ciphertext_digest_branch (tenant_id, handle, producer_block_hash, ciphertext, ciphertext128, host_chain_id, key_id_gw)
         VALUES (0, $1, $2, $3, $4, 12345, $5)
         ON CONFLICT DO NOTHING`,
        [
          bytes32ToBuffer(handle),
          Buffer.from(producerBlockHash.replace('0x', ''), 'hex'),
          ciphertext,
          ciphertext128,
          Buffer.alloc(32),
        ],
      );
    }
  } finally {
    await pool.end();
  }
}

export async function deleteDigestRowByBranchContext(
  databaseUrl: string,
  handle: string,
  producerBlockHash: string,
  blockHash: string = producerBlockHash,
): Promise<void> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const columns = await digestBranchColumns(pool);
    if (columns.has('block_hash')) {
      await pool.query(
        `DELETE FROM ciphertext_digest_branch
          WHERE handle = $1
            AND producer_block_hash = $2
            AND block_hash = $3`,
        [
          bytes32ToBuffer(handle),
          Buffer.from(producerBlockHash.replace('0x', ''), 'hex'),
          Buffer.from(blockHash.replace('0x', ''), 'hex'),
        ],
      );
    } else {
      await pool.query(
        `DELETE FROM ciphertext_digest_branch
          WHERE handle = $1
            AND producer_block_hash = $2`,
        [
          bytes32ToBuffer(handle),
          Buffer.from(producerBlockHash.replace('0x', ''), 'hex'),
        ],
      );
    }
  } finally {
    await pool.end();
  }
}

/**
 * Update digest values across all ciphertext_digest_branch rows for a handle.
 */
export async function updateDigestRow(
  databaseUrl: string,
  handle: string,
  ciphertext: Buffer,
  ciphertext128: Buffer,
): Promise<void> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    await pool.query(
      'UPDATE ciphertext_digest_branch SET ciphertext = $1, ciphertext128 = $2 WHERE handle = $3',
      [ciphertext, ciphertext128, bytes32ToBuffer(handle)],
    );
  } finally {
    await pool.end();
  }
}

async function updateDigestRowByBranchContext(
  pool: Pool,
  row: DigestRow,
  ciphertext: Buffer | null,
  ciphertext128: Buffer | null,
  hasBlockHash: boolean,
): Promise<void> {
  const result = hasBlockHash
    ? await pool.query(
      `UPDATE ciphertext_digest_branch
          SET ciphertext = $1, ciphertext128 = $2
        WHERE handle = $3
          AND producer_block_hash = $4
          AND block_hash = $5`,
      [
        ciphertext,
        ciphertext128,
        row.handle,
        row.producer_block_hash ?? Buffer.alloc(0),
        row.block_hash ?? Buffer.alloc(0),
      ],
    )
    : await pool.query(
      `UPDATE ciphertext_digest_branch
          SET ciphertext = $1, ciphertext128 = $2
        WHERE handle = $3
          AND producer_block_hash = $4`,
      [
        ciphertext,
        ciphertext128,
        row.handle,
        row.producer_block_hash ?? Buffer.alloc(0),
      ],
    );

  if (result.rowCount !== 1) {
    throw new Error(`expected to update exactly one digest row, updated ${result.rowCount}`);
  }
}

export async function tamperDigestRows(
  databaseUrl: string,
  rows: DigestRow[],
  ciphertext: Buffer,
  ciphertext128: Buffer,
): Promise<void> {
  if (rows.length === 0) return;

  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const columns = await digestBranchColumns(pool);
    const hasBlockHash = columns.has('block_hash');
    for (const row of rows) {
      await updateDigestRowByBranchContext(pool, row, ciphertext, ciphertext128, hasBlockHash);
    }
  } finally {
    await pool.end();
  }
}

export async function restoreDigestRows(databaseUrl: string, rows: DigestRow[]): Promise<void> {
  if (rows.length === 0) return;

  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const columns = await digestBranchColumns(pool);
    const hasBlockHash = columns.has('block_hash');
    for (const row of rows) {
      await updateDigestRowByBranchContext(pool, row, row.ciphertext, row.ciphertext128, hasBlockHash);
    }
  } finally {
    await pool.end();
  }
}

export async function restoreArtificialDigestRows(
  databaseUrls: string[],
  repair: boolean = false,
): Promise<ArtificialDigestCleanupResult> {
  const artificialRowsByDb = await Promise.all(databaseUrls.map((url) => queryArtificialDigestRows(url)));
  let repaired = 0;
  let unrepaired = 0;

  for (let dbIndex = 0; dbIndex < artificialRowsByDb.length; dbIndex++) {
    for (const artificialRow of artificialRowsByDb[dbIndex]) {
      const handle = bytes32ToHex(artificialRow.handle);
      let replacement: DigestRow | undefined;

      for (let peerIndex = 0; peerIndex < databaseUrls.length && !replacement; peerIndex++) {
        if (peerIndex === dbIndex) continue;
        const peerRows = await queryDigests(databaseUrls[peerIndex], handle);
        replacement = peerRows.find((row) => sameBranchContext(row, artificialRow) && !isArtificialDigest(row));
      }

      if (replacement?.ciphertext && replacement.ciphertext128) {
        if (repair) {
          await restoreDigestRows(databaseUrls[dbIndex], [
            {
              ...artificialRow,
              ciphertext: replacement.ciphertext,
              ciphertext128: replacement.ciphertext128,
            },
          ]);
        }
        repaired++;
      } else {
        unrepaired++;
      }
    }
  }

  return { repaired, unrepaired };
}

// ---------------------------------------------------------------------------
// Docker helpers
// ---------------------------------------------------------------------------

export async function dockerStop(...containers: string[]): Promise<void> {
  await exec(`docker stop ${containers.join(' ')}`);
}

export async function dockerStart(...containers: string[]): Promise<void> {
  await exec(`docker start ${containers.join(' ')}`);
}

export async function dockerRestart(...containers: string[]): Promise<void> {
  await exec(`docker restart ${containers.join(' ')}`);
}

export async function dockerPause(...containers: string[]): Promise<void> {
  await exec(`docker pause ${containers.join(' ')}`);
}

export async function dockerUnpause(...containers: string[]): Promise<void> {
  await exec(`docker unpause ${containers.join(' ')}`);
}

// ---------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------

export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

// ---------------------------------------------------------------------------
// Metrics helpers
// ---------------------------------------------------------------------------

/**
 * Scrape a Prometheus metric value from a coprocessor's metrics endpoint.
 * Returns the current counter/gauge value, or null if not found.
 */
export async function scrapeMetric(
  metricsUrl: string,
  metricName: string,
): Promise<number | null> {
  try {
    const resp = await fetch(metricsUrl);
    const text = await resp.text();
    // Prometheus text format: metric_name{labels} value
    const lines = text.split('\n');
    for (const line of lines) {
      if (line.startsWith(metricName) && !line.startsWith('#')) {
        const parts = line.split(/\s+/);
        const val = parseFloat(parts[parts.length - 1]);
        if (!isNaN(val)) return val;
      }
    }
    return null;
  } catch {
    return null;
  }
}

/**
 * Scrape a metric that is part of the test's proof obligation.
 * Failing closed here prevents consensus e2e tests from passing when the
 * endpoint is missing, misconfigured, or the metric name drifted.
 */
export async function scrapeRequiredMetric(
  metricsUrl: string,
  metricName: string,
): Promise<number> {
  const value = await scrapeMetric(metricsUrl, metricName);
  if (value === null) {
    throw new Error(`Required metric ${metricName} was not found at ${metricsUrl}`);
  }
  return value;
}

export async function scrapeMetricValues(
  metricsUrls: string[],
  metricName: string,
): Promise<Array<number | null>> {
  return Promise.all(metricsUrls.map((url) => scrapeMetric(url, metricName)));
}

export function metricIncreased(
  before: Array<number | null>,
  after: Array<number | null>,
): boolean {
  return after.some((afterValue, index) => {
    const beforeValue = before[index];
    if (beforeValue === null || beforeValue === undefined || afterValue === null) {
      return false;
    }
    return afterValue > beforeValue;
  });
}

export async function waitForMetricIncrease(
  metricsUrls: string[],
  metricName: string,
  before: Array<number | null>,
  timeoutMs: number = 60_000,
): Promise<Array<number | null>> {
  const deadline = Date.now() + timeoutMs;
  let after: Array<number | null> = [];
  while (Date.now() < deadline) {
    after = await scrapeMetricValues(metricsUrls, metricName);
    if (metricIncreased(before, after)) {
      return after;
    }
    await sleep(2_000);
  }
  return after;
}

export async function latestDriftRevertSignal(databaseUrl: string): Promise<DriftRevertSignal | null> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query(
      `SELECT id, host_chain_id, offending_host_block_number, status
         FROM drift_revert_signal
        ORDER BY id DESC
        LIMIT 1`,
    );
    return result.rows[0] ? driftRevertSignalFromRow(result.rows[0]) : null;
  } finally {
    await pool.end();
  }
}

function driftRevertSignalFromRow(row: {
  id: string | number;
  host_chain_id: string | number;
  offending_host_block_number: string | number;
  status: string;
}): DriftRevertSignal {
  return {
    id: toSafeNumber(row.id, 'drift_revert_signal.id'),
    host_chain_id: toSafeNumber(row.host_chain_id, 'drift_revert_signal.host_chain_id'),
    offending_host_block_number: toSafeNumber(
      row.offending_host_block_number,
      'drift_revert_signal.offending_host_block_number',
    ),
    status: row.status,
  };
}

export function toSafeNumber(value: string | number, field: string): number {
  const parsed = Number(value);
  if (!Number.isSafeInteger(parsed)) {
    throw new Error(`${field} value ${value} is outside JavaScript's safe integer range`);
  }
  return parsed;
}

async function driftRevertSignalsAtOrAfter(databaseUrl: string, minimumId: number): Promise<DriftRevertSignal[]> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query(
      `SELECT id, host_chain_id, offending_host_block_number, status
         FROM drift_revert_signal
        WHERE id >= $1
        ORDER BY id ASC`,
      [minimumId],
    );
    return result.rows.map(driftRevertSignalFromRow);
  } finally {
    await pool.end();
  }
}

export async function waitForDriftRevertSignalAtOrAfter(
  databaseUrl: string,
  minimumId: number,
  timeoutMs: number = 60_000,
  predicate: (signal: DriftRevertSignal) => boolean = () => true,
): Promise<DriftRevertSignal | null> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const signals = await driftRevertSignalsAtOrAfter(databaseUrl, minimumId);
    const signal = signals.find(predicate);
    if (signal) {
      return signal;
    }
    await sleep(2_000);
  }
  return null;
}

export async function waitForGwListenerBlock(
  databaseUrl: string,
  minimumBlockNumber: number,
  timeoutMs: number = 60_000,
): Promise<number | null> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const deadline = Date.now() + timeoutMs;
    while (Date.now() < deadline) {
      const result = await pool.query(
        `SELECT last_block_num
           FROM gw_listener_last_block
          WHERE dummy_id = TRUE`,
      );
      const lastBlock = result.rows[0]?.last_block_num;
      if (lastBlock !== null && lastBlock !== undefined) {
        const blockNumber = toSafeNumber(lastBlock, 'gw_listener_last_block.last_block_num');
        if (blockNumber >= minimumBlockNumber) {
          return blockNumber;
        }
      }
      await sleep(2_000);
    }
    return null;
  } finally {
    await pool.end();
  }
}

export async function queryHandleBlockNumber(
  databaseUrl: string,
  handle: string,
  hostChainId?: number,
): Promise<number | null> {
  const pool = new Pool({ connectionString: databaseUrl, max: 1 });
  try {
    const result = await pool.query(
      `SELECT MIN(block_number)::bigint AS block_number
         FROM computations_branch
        WHERE output_handle = $1
          AND ($2::bigint IS NULL OR host_chain_id = $2)
          AND block_number IS NOT NULL`,
      [bytes32ToBuffer(handle), hostChainId ?? null],
    );
    const blockNumber = result.rows[0]?.block_number;
    return blockNumber === null || blockNumber === undefined
      ? null
      : toSafeNumber(blockNumber, 'computations_branch.block_number');
  } finally {
    await pool.end();
  }
}

/**
 * Get metrics URLs for each coprocessor instance.
 * Convention: METRICS_URL_0, METRICS_URL_1, METRICS_URL_2
 */
export function getCoprocessorMetricsUrls(count: number): string[] {
  const urls: string[] = [];
  for (let i = 0; i < count; i++) {
    const envKey = `METRICS_URL_${i}`;
    const explicit = process.env[envKey];
    if (explicit) {
      urls.push(explicit);
    } else {
      urls.push(`http://${containerName(i, 'gw-listener')}:9100/metrics`);
    }
  }
  return urls;
}

/**
 * Convert a handle value to a hex string suitable for event filtering
 * and database queries. Handles ethers BigInt returns from view functions.
 */
export function handleToHex(handle: string | bigint): string {
  if (typeof handle === 'bigint') {
    return '0x' + handle.toString(16).padStart(64, '0');
  }
  if (handle.startsWith('0x')) return handle;
  return '0x' + handle;
}

// ---------------------------------------------------------------------------
// Environment helpers
// ---------------------------------------------------------------------------

/**
 * Get coprocessor database URLs from environment.
 * Convention: DATABASE_URL_0, DATABASE_URL_1, DATABASE_URL_2
 * Falls back to computed names using POSTGRES_HOST.
 */
export function getCoprocessorDbUrls(count: number): string[] {
  const urls: string[] = [];
  for (let i = 0; i < count; i++) {
    const envKey = `DATABASE_URL_${i}`;
    const explicit = process.env[envKey];
    if (explicit) {
      urls.push(explicit);
    } else {
      const host = process.env.POSTGRES_HOST || 'coprocessor-and-kms-db:5432';
      const dbName = i === 0 ? 'coprocessor' : `coprocessor_${i}`;
      urls.push(`postgresql://postgres:postgres@${host}/${dbName}`);
    }
  }
  return urls;
}

/**
 * Get container name for a coprocessor service instance.
 * Instance 0: coprocessor-{service}
 * Instance N: coprocessorN-{service}
 */
export function containerName(instanceIndex: number, service: string): string {
  const prefix = instanceIndex === 0 ? 'coprocessor' : `coprocessor${instanceIndex}`;
  return `${prefix}-${service}`;
}
