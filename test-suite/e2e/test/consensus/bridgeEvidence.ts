import { ethers, type TransactionReceipt } from 'ethers';
import { Pool } from 'pg';
import { ComparisonMismatch } from './mismatch';

export interface BridgeIdentity {
  source: string; destination: string; sourceChain: number; destinationChain: number;
  sender: string; receiver: string; guid: string;
  send: Pick<TransactionReceipt, 'hash' | 'blockHash' | 'blockNumber'>;
  receive: Pick<TransactionReceipt, 'hash' | 'blockHash' | 'blockNumber'>;
}

/** Public bridge derivation; a destination handle is an association, not a computation. */
export function bridgeHandle(source: string, acl: string, chain: bigint, parentHash: string, timestamp: bigint): string {
  const result = ethers.getBytes(ethers.keccak256(ethers.concat([
    ethers.toUtf8Bytes('FHE_brdg'), source, acl, ethers.zeroPadValue(ethers.toBeHex(chain), 32),
    parentHash, ethers.zeroPadValue(ethers.toBeHex(timestamp), 32),
  ])));
  result[21] = 255;
  result.set(ethers.getBytes(ethers.zeroPadValue(ethers.toBeHex(chain), 8)), 22);
  result[30] = ethers.getBytes(source)[30];
  result[31] = 0;
  return ethers.hexlify(result);
}

export interface BridgeRows { source: string; destination: string; sourceType: number; destinationType: number; sourceVersion: number; destinationVersion: number }
export function compareBridgeRows(handle: string, operators: BridgeRows[][]): void {
  if (operators.length < 2) throw new Error('bridge evidence requires multiple operators');
  operators.forEach((rows, index) => {
    if (rows.length !== 1) throw new ComparisonMismatch('provenance', handle, [index], 'one canonical receipt-bound association and approval required');
    const row = rows[0];
    if (row.sourceType !== 5 || row.destinationType !== 5 || row.sourceVersion !== 0 || row.destinationVersion !== 0) {
      throw new ComparisonMismatch('type-version', handle, [index], 'bridge changed u64 type/version');
    }
    if (!/^0x(?:[0-9a-f]{2})+$/.test(row.source) || row.destination !== row.source || row.destination !== operators[0][0].destination) {
      throw new ComparisonMismatch('raw-bytes', handle, [index], 'bridge must preserve the agreed source representation');
    }
  });
}

export async function readBridgeRows(database: string, identity: BridgeIdentity): Promise<BridgeRows[]> {
  const pool = new Pool({ connectionString: database, max: 1, connectionTimeoutMillis: 10_000, statement_timeout: 10_000 });
  const bytes = (value: string) => Buffer.from(value.slice(2), 'hex');
  try {
    return (await pool.query(`SELECT '0x'||encode(s.ciphertext,'hex') AS source, '0x'||encode(d.ciphertext,'hex') AS destination,
      s.ciphertext_type AS "sourceType", d.ciphertext_type AS "destinationType",
      s.ciphertext_version AS "sourceVersion", d.ciphertext_version AS "destinationVersion"
      FROM handle_bridged_events e JOIN bridge_handle_events a ON a.src_handle=e.src_handle AND a.dst_chain_id=e.dst_chain_id
      JOIN ciphertexts s ON s.handle=e.src_handle JOIN ciphertexts d ON d.handle=e.dst_handle
      JOIN host_chain_blocks_valid sb ON sb.chain_id=a.src_chain_id AND sb.block_hash=a.block_hash AND sb.block_status='finalized'
      JOIN host_chain_blocks_valid db ON db.chain_id=e.dst_chain_id AND db.block_hash=e.block_hash AND db.block_status='finalized'
      WHERE e.src_handle=$1 AND e.dst_handle=$2 AND a.src_chain_id=$3 AND e.dst_chain_id=$4
        AND a.sender_dapp=$5 AND e.receiver_dapp=$6 AND a.guid=$7 AND e.guid=$7 AND e.is_associated
        AND a.transaction_id=$8 AND a.block_hash=$9 AND a.block_number=$10
        AND e.transaction_id=$11 AND e.block_hash=$12 AND e.block_number=$13`,
      [bytes(identity.source), bytes(identity.destination), identity.sourceChain, identity.destinationChain,
        bytes(identity.sender), bytes(identity.receiver), bytes(identity.guid), bytes(identity.send.hash), bytes(identity.send.blockHash), identity.send.blockNumber,
        bytes(identity.receive.hash), bytes(identity.receive.blockHash), identity.receive.blockNumber])).rows;
  } finally { await pool.end(); }
}

export async function waitForBridgeAgreement(databases: string[], identity: BridgeIdentity): Promise<void> {
  const deadline = Date.now() + 300_000;
  for (;;) {
    const rows = await Promise.all(databases.map(database => readBridgeRows(database, identity)));
    if (rows.every(operator => operator.length > 0) || Date.now() >= deadline) {
      compareBridgeRows(identity.destination, rows);
      return;
    }
    await new Promise(resolve => setTimeout(resolve, 1_000));
  }
}
