/** Local recovery journal for mutations whose JS finally cannot survive SIGKILL.
 * This directory contains connection URLs: never copy it into result artifacts.
 * The host stops all suite processes before invoking recoverAbortedSuite().
 */
import { createHash, randomUUID } from 'node:crypto';
import { chmodSync, closeSync, existsSync, fsyncSync, mkdirSync, openSync, readFileSync, readdirSync, renameSync, unlinkSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { FetchRequest, JsonRpcProvider } from 'ethers';
import { Pool } from 'pg';

export interface MiningRpc { send(method: string, params: unknown[]): Promise<unknown> }
interface DigestRecovery { kind: 'digest'; databaseUrl: string; handle: string; original: string; publicationMayHaveBegun?: boolean }
interface MiningRecovery { kind: 'mining'; rpcUrl: string; automine: boolean; interval: number }
interface CiphertextRecovery { kind: 'ciphertext'; databaseUrl: string; handle: string; version: number; ciphertext: string; digest: string }
type Recovery = DigestRecovery | MiningRecovery | CiphertextRecovery;
const directory = () => process.env.CONSENSUS_RECOVERY_DIR ?? '/tmp/fhevm-consensus-abort-recovery';
const key = (kind: string, identity: string) => createHash('sha256').update(`${kind}:${identity}`).digest('hex');
const digestKey = (url: string, handle: string) => key('digest', `${url}:${handle.toLowerCase()}`);
const miningKey = (url: string) => key('mining', url);
const file = (id: string) => path.join(directory(), `${id}.json`);

function syncDirectory(): void {
  const fd = openSync(directory(), 'r');
  try { fsyncSync(fd); } finally { closeSync(fd); }
}

function save(id: string, record: Recovery): void {
  mkdirSync(directory(), { recursive: true, mode: 0o700 });
  chmodSync(directory(), 0o700);
  const temporary = `${file(id)}.${randomUUID()}.partial`;
  const fd = openSync(temporary, 'wx', 0o600);
  try { writeFileSync(fd, JSON.stringify(record)); fsyncSync(fd); } finally { closeSync(fd); }
  renameSync(temporary, file(id));
  syncDirectory();
}

function remove(id: string): void {
  if (!existsSync(file(id))) return;
  unlinkSync(file(id));
  syncDirectory();
}

function read(id: string): Recovery | undefined {
  if (!existsSync(file(id))) return undefined;
  return JSON.parse(readFileSync(file(id), 'utf8')) as Recovery;
}

/** Called under the same row lock as publication check, BEFORE the poison write. */
export function saveDigestRecovery(databaseUrl: string, handle: string, original: Buffer): void {
  if (!/^0x[0-9a-f]{64}$/i.test(handle) || original.length !== 32) throw new Error('invalid canary recovery identity');
  const id = digestKey(databaseUrl, handle);
  if (read(id) || read(key('ciphertext', `${databaseUrl}:${handle.toLowerCase()}`))) {
    throw new Error(`unfinished canary recovery for ${handle}; restore it before another mutation`);
  }
  save(id, { kind: 'digest', databaseUrl, handle, original: original.toString('hex') });
}

/** Remove the journal only after a fresh read verifies the restored digest. */
async function restoreDigestRecord(databaseUrl: string, handle: string, original: Buffer, detectorRecoveryVerified: boolean): Promise<void> {
  const id = digestKey(databaseUrl, handle);
  const record = read(id);
  if (record && (record.kind !== 'digest' || record.original !== original.toString('hex'))) {
    throw new Error(`canary original disagrees with durable recovery record for ${handle}`);
  }
  if (record?.kind === 'digest' && record.publicationMayHaveBegun && !detectorRecoveryVerified) {
    throw new Error('detector publication may have begun; exact detector recovery must be verified before releasing this journal');
  }
  const pool = new Pool({ connectionString: databaseUrl, max: 1, connectionTimeoutMillis: 5_000,
    query_timeout: 10_000, statement_timeout: 10_000 });
  try {
    const client = await pool.connect();
    try {
      const bytes = Buffer.from(handle.slice(2), 'hex');
      const updated = await client.query('UPDATE ciphertext_digest SET ciphertext = $2 WHERE handle = $1', [bytes, original]);
      if (updated.rowCount !== 1) throw new Error(`could not restore the original digest for ${handle}: row is absent`);
      const checked = await client.query<{ ciphertext: Buffer }>('SELECT ciphertext FROM ciphertext_digest WHERE handle = $1', [bytes]);
      if (checked.rowCount !== 1 || !checked.rows[0].ciphertext.equals(original)) {
        throw new Error(`restored digest verification failed for ${handle}`);
      }
      remove(id);
    } finally { client.release(); }
  } finally { await pool.end(); }
}

/** Raw-byte canaries restore the ciphertext and its binding atomically. */
export function saveCiphertextRecovery(databaseUrl: string, handle: string, version: number, ciphertext: Buffer, digest: Buffer): void {
  if (!/^0x[0-9a-f]{64}$/i.test(handle) || !Number.isSafeInteger(version) || version < 0 || !ciphertext.length || digest.length !== 32) {
    throw new Error('invalid ciphertext recovery identity');
  }
  const id = key('ciphertext', `${databaseUrl}:${handle.toLowerCase()}`);
  if (read(id) || read(digestKey(databaseUrl, handle))) throw new Error('unfinished canary recovery');
  save(id, { kind: 'ciphertext', databaseUrl, handle, version, ciphertext: ciphertext.toString('hex'), digest: digest.toString('hex') });
}

export async function restoreRecordedCiphertext(databaseUrl: string, handle: string): Promise<void> {
  const id = key('ciphertext', `${databaseUrl}:${handle.toLowerCase()}`);
  const record = read(id);
  if (!record || record.kind !== 'ciphertext' || !Number.isSafeInteger(record.version) || record.version < 0 ||
      !/^(?:[a-f0-9]{2})+$/.test(record.ciphertext) || !/^[a-f0-9]{64}$/.test(record.digest)) throw new Error('invalid ciphertext recovery journal');
  const pool = new Pool({ connectionString: databaseUrl, max: 1, connectionTimeoutMillis: 5_000, query_timeout: 10_000, statement_timeout: 10_000 });
  try {
    const client = await pool.connect();
    try {
      const bytes = Buffer.from(handle.slice(2), 'hex');
      const ciphertext = Buffer.from(record.ciphertext, 'hex');
      const digest = Buffer.from(record.digest, 'hex');
      await client.query('BEGIN');
      try {
        const restoredDigest = await client.query('UPDATE ciphertext_digest SET ciphertext=$2 WHERE handle=$1', [bytes, digest]);
        const restoredValue = await client.query('UPDATE ciphertexts SET ciphertext=$2 WHERE handle=$1 AND ciphertext_version=$3', [bytes, ciphertext, record.version]);
        if (restoredDigest.rowCount !== 1 || restoredValue.rowCount !== 1) throw new Error('canonical canary rows missing during restoration');
        await client.query('COMMIT');
      } catch (error) { await client.query('ROLLBACK'); throw error; }
      const verified = await client.query<{ ciphertext: Buffer; digest: Buffer }>(
        'SELECT c.ciphertext, d.ciphertext AS digest FROM ciphertexts c JOIN ciphertext_digest d USING (handle) WHERE c.handle=$1 AND c.ciphertext_version=$2', [bytes, record.version]);
      if (verified.rowCount !== 1 || !verified.rows[0].ciphertext.equals(ciphertext) || !verified.rows[0].digest.equals(digest)) {
        throw new Error('ciphertext canary restoration verification failed');
      }
      remove(id);
    } finally { client.release(); }
  } finally { await pool.end(); }
}

/** Persist BEFORE the host resumes the poisoned sender. A local digest restore
 * cannot undo an in-flight/on-chain submission or its delayed automatic revert. */
export function markDetectorPublicationStarted(databaseUrl: string, handle: string): void {
  const id = digestKey(databaseUrl, handle);
  const record = read(id);
  if (!record || record.kind !== 'digest') throw new Error('detector original must be journaled before sender release');
  save(id, { ...record, publicationMayHaveBegun: true });
}

export async function restoreRecordedDigest(databaseUrl: string, handle: string, original: Buffer): Promise<void> {
  await restoreDigestRecord(databaseUrl, handle, original, false);
}

/** Called only after the detector suite proves its exact signal reached done
 * and both original/control handles again agree. Abort cleanup cannot call it. */
export async function completeDetectorRecovery(databaseUrl: string, handle: string, original: Buffer): Promise<void> {
  await restoreDigestRecord(databaseUrl, handle, original, true);
}

async function miningState(provider: MiningRpc): Promise<{ automine: boolean; interval: number }> {
  const automine = await provider.send('anvil_getAutomine', []);
  const reportedInterval = await provider.send('anvil_getIntervalMining', []);
  // Anvil returns null when interval mining is disabled; setter uses zero.
  const interval = reportedInterval === null ? 0 : reportedInterval;
  if (typeof automine !== 'boolean' || typeof interval !== 'number' || !Number.isFinite(interval) || interval < 0) {
    throw new Error('cannot read original Anvil mining configuration');
  }
  return { automine, interval };
}

/** Nested mining operations retain the first pre-case state, including reset. */
export async function rememberMiningState(provider: MiningRpc, rpcUrl: string): Promise<void> {
  const id = miningKey(rpcUrl);
  if (read(id)) return;
  const state = await miningState(provider);
  save(id, { kind: 'mining', rpcUrl, ...state });
}

export async function restoreMiningState(provider: MiningRpc, rpcUrl: string): Promise<void> {
  const id = miningKey(rpcUrl);
  const record = read(id);
  if (!record) return;
  if (record.kind !== 'mining') throw new Error('invalid mining recovery record');
  // Disable automine while reconfiguring, then restore the exact original pair.
  await provider.send('evm_setAutomine', [false]);
  await provider.send('evm_setIntervalMining', [record.interval]);
  await provider.send('evm_setAutomine', [record.automine]);
  const actual = await miningState(provider);
  if (actual.automine !== record.automine || actual.interval !== record.interval) {
    throw new Error('Anvil mining restoration did not retain its original configuration');
  }
  remove(id);
}

/** Resolve Hardhat's actual network endpoint without importing its runtime in unit tests. */
export function consensusHostRpcUrl(): string {
  const configured = (require('hardhat').network.config as { url?: string }).url ?? process.env.RPC_URL;
  if (!configured) throw new Error('cannot identify the Anvil endpoint for durable mining recovery');
  return configured;
}

/** Host entrypoint: suite process groups MUST already be confirmed stopped. */
export async function recoverAbortedSuite(): Promise<void> {
  if (!existsSync(directory())) return;
  const failures: string[] = [];
  for (const name of readdirSync(directory()).filter(name => /^[a-f0-9]{64}\.json$/.test(name))) {
    const id = name.slice(0, -5);
    try {
      const record = read(id)!;
      if (record.kind === 'digest') {
        if (!/^[a-f0-9]{64}$/i.test(record.original)) throw new Error('invalid original');
        await restoreRecordedDigest(record.databaseUrl, record.handle, Buffer.from(record.original, 'hex'));
      } else if (record.kind === 'ciphertext') {
        await restoreRecordedCiphertext(record.databaseUrl, record.handle);
      } else if (record.kind === 'mining') {
        const request = new FetchRequest(record.rpcUrl);
        request.timeout = 5_000;
        const provider = new JsonRpcProvider(request);
        try { await restoreMiningState(provider, record.rpcUrl); } finally { provider.destroy(); }
      } else throw new Error('invalid recovery kind');
    } catch {
      // URLs may carry credentials; preserve the private record, not its error text.
      failures.push(id);
    }
  }
  if (failures.length) throw new Error(`suite mutation recovery failed for ${failures.length} record(s); retain the private recovery journal`);
}
