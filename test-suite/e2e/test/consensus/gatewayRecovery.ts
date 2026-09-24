import { existsSync } from 'node:fs';
import path from 'node:path';
import { Pool } from 'pg';
import { restoreDigest } from './canary';
import { getCoprocessorDbUrls } from './helpers';
import { HANDSHAKE_DIR, publishHandshake, readHandshake } from './handshake';

export interface PendingGatewayEvent {
  handle: string;
  eventBlock: number;
  eventBlockHash: string;
  eventTxHash: string;
  eventLogIndex: number;
  originals: { operator: number; digest: string }[];
  restored?: boolean;
}

/** Save all originals before any poisoning, so the host EXIT trap can recover
 * even when the arm process dies between its commit and its completion marker. */
export async function recordPendingGatewayEvent(
  urls: string[], event: Omit<PendingGatewayEvent, 'originals'>,
): Promise<PendingGatewayEvent> {
  const originals: PendingGatewayEvent['originals'] = [];
  for (const [operator, connectionString] of urls.entries()) {
    const pool = new Pool({connectionString, max:1, connectionTimeoutMillis:10_000, query_timeout:10_000, statement_timeout:10_000});
    try {
      const result = await pool.query<{digest:string}>("SELECT encode(ciphertext, 'hex') AS digest FROM ciphertext_digest WHERE handle = decode($1, 'hex')", [event.handle.replace(/^0x/, '')]);
      if (result.rowCount !== 1 || !/^[a-f0-9]{64}$/i.test(result.rows[0]?.digest ?? '')) throw new Error(`operator ${operator} has no restorable bytes32 digest`);
      originals.push({operator, digest:result.rows[0].digest});
    } finally {await pool.end();}
  }
  const record = {...event, originals};
  publishHandshake('degraded-gw-event', record);
  return record;
}

/** Independent cleanup entrypoint: no Hardhat hooks or new workload required. */
export async function restorePendingGatewayDigests(): Promise<void> {
  if (!existsSync(path.join(HANDSHAKE_DIR, 'degraded-gw-event.json'))) return;
  const record = readHandshake<PendingGatewayEvent>('degraded-gw-event').payload;
  const count = Number(process.env.COPROCESSOR_COUNT);
  if (!/^0x[a-f0-9]{64}$/i.test(record.handle) || !Number.isInteger(count) || count < 1 || record.originals?.length !== count ||
    record.originals.some((row,index) => row.operator !== index || !/^[a-f0-9]{64}$/i.test(row.digest))) {
    throw new Error('invalid gateway digest recovery record; refusing unscoped restoration');
  }
  const urls = getCoprocessorDbUrls(count);
  const restored = await Promise.allSettled(record.originals.map(({operator,digest}) => restoreDigest(urls[operator], record.handle, Buffer.from(digest, 'hex'))));
  const failures = restored.flatMap((result,index) => result.status === 'rejected' ? [index] : []);
  if (failures.length) throw new Error(`gateway digest cleanup failed for operator(s) ${failures.join(', ')}; originals remain in the handshake`);
  publishHandshake('degraded-gw-event', {...record, restored:true});
}
