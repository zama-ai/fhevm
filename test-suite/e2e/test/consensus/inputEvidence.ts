import { ethers } from 'ethers';
import { Pool } from 'pg';
import { ComparisonMismatch } from './mismatch';

export interface InputIdentity {
  handles: string[];
  types: number[];
  blobHash: string;
  chainId: bigint;
  acl: string;
}
export interface InputRow {
  handle: string;
  bytes: string;
  type: number;
  version: number;
  index: number;
  blobHash: string;
  isInput: boolean;
}

/** Independent public handle derivation, including the input index and chain. */
export function inputHandle(identity: Omit<InputIdentity, 'handles' | 'types'>, index: number, type: number): string {
  if (!Number.isInteger(index) || index < 0 || index > 254) throw new Error('input index outside public handle range');
  const bytes = ethers.getBytes(ethers.keccak256(ethers.concat([
    ethers.toUtf8Bytes('ZK-w_hdl'), identity.blobHash, Uint8Array.of(index), identity.acl,
    ethers.zeroPadValue(ethers.toBeHex(identity.chainId), 32),
  ])));
  bytes[21] = index;
  bytes.set(ethers.getBytes(ethers.zeroPadValue(ethers.toBeHex(identity.chainId), 8)), 22);
  bytes[30] = type;
  bytes[31] = 0;
  return ethers.hexlify(bytes);
}

/** No computation/SNS output row is assumed for a verified input. */
export function compareInputRows(identity: InputIdentity, operators: InputRow[][]): void {
  if (operators.length < 2 || !identity.handles.length || identity.types.length !== identity.handles.length ||
      new Set(identity.handles).size !== identity.handles.length) throw new Error('incomplete input participant/handle set');
  const baseline = new Map<string, string>();
  operators.forEach((rows, operator) => {
    if (rows.length !== identity.handles.length) throw new ComparisonMismatch('storage-row-uniqueness', identity.handles[0], [operator], 'missing, extra or duplicate input rows');
    identity.handles.forEach((handle, index) => {
      const found = rows.filter(row => row.handle === handle);
      if (found.length !== 1) throw new ComparisonMismatch('storage-row-uniqueness', handle, [operator], 'input must have one canonical row');
      const row = found[0];
      if (row.type !== identity.types[index] || row.version !== 0 || !row.isInput) throw new ComparisonMismatch('type-version', handle, [operator], 'wrong input type/version');
      if (row.index !== index || row.blobHash !== identity.blobHash || handle !== inputHandle(identity, index, identity.types[index])) {
        throw new ComparisonMismatch('provenance', handle, [operator], 'input blob/index/ACL/chain binding differs');
      }
      if (!/^0x(?:[0-9a-f]{2})+$/.test(row.bytes)) throw new ComparisonMismatch('evidence-missing', handle, [operator], 'empty input ciphertext');
      if (operator === 0) baseline.set(handle, row.bytes);
      else if (baseline.get(handle) !== row.bytes) throw new ComparisonMismatch('raw-bytes', handle, [0, operator], 'verified input bytes differ');
    });
  });
}

export async function readInputRows(database: string, blobHash: string): Promise<InputRow[]> {
  const pool = new Pool({ connectionString: database, max: 1, connectionTimeoutMillis: 10_000, statement_timeout: 10_000 });
  try {
    const result = await pool.query(`SELECT '0x' || encode(handle, 'hex') AS handle,
      '0x' || encode(ciphertext, 'hex') AS bytes, ciphertext_type AS type,
      ciphertext_version AS version, input_blob_index AS index,
      '0x' || encode(input_blob_hash, 'hex') AS "blobHash", is_input AS "isInput"
      FROM ciphertexts WHERE input_blob_hash = decode($1, 'hex') ORDER BY input_blob_index`, [blobHash.slice(2)]);
    return result.rows;
  } finally { await pool.end(); }
}

export async function waitForInputAgreement(databases: string[], identity: InputIdentity, timeoutMs = 300_000): Promise<InputRow[][]> {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    const rows = await Promise.all(databases.map(database => readInputRows(database, identity.blobHash)));
    // Missing rows are pending only. All other mismatches fail immediately.
    if (rows.every(operator => operator.length >= identity.handles.length) || Date.now() >= deadline) {
      compareInputRows(identity, rows);
      return rows;
    }
    await new Promise(resolve => setTimeout(resolve, 1_000));
  }
}
