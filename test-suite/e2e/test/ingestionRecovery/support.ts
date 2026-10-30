import { expect } from 'chai';
import { ethers } from 'ethers';
import { readFileSync, renameSync, writeFileSync } from 'node:fs';
import { Pool } from 'pg';

// Kept local to this v0.14 backport: the newer consensus campaign harness and
// automatic startup replay window are not available on this branch.
export function publishHandshake<T>(name: string, payload: T) {
  const file = `${process.env.ARTIFACT_DIR}/${name}.json`;
  writeFileSync(`${file}.tmp`, JSON.stringify({ payload }));
  renameSync(`${file}.tmp`, file);
}
export function readHandshake<T>(name: string): { payload: T } {
  return JSON.parse(readFileSync(`${process.env.ARTIFACT_DIR}/${name}.json`, 'utf8'));
}
export function getCoprocessorDbUrls(): string[] {
  const urls: string[] = JSON.parse(process.env.ARTIFACT_DB_URLS ?? '[]');
  expect(urls.length).to.eq(2);
  expect(new Set(urls).size, 'two independent operator databases').to.eq(2);
  return urls;
}
export async function withDeadline<T>(work: Promise<T>, ms: number, label: string): Promise<T> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    return await Promise.race([
      work,
      new Promise<never>((_, reject) => {
        timer = setTimeout(() => reject(new Error(`timed out: ${label}`)), ms);
      }),
    ]);
  } finally {
    clearTimeout(timer);
  }
}
export async function assertGatewayTopology(rpc: string, address: string) {
  const provider = new ethers.JsonRpcProvider(rpc);
  try {
    const contract = new ethers.Contract(
      address,
      [
        'function getCoprocessorTxSenders() view returns (address[])',
        'function getCoprocessorMajorityThreshold() view returns (uint256)',
      ],
      provider,
    );
    const txSenders: string[] = (await contract.getCoprocessorTxSenders()).map((s: string) => s.toLowerCase());
    expect(new Set(txSenders).size, 'requires an isolated two-of-two deployment').to.eq(2);
    expect(await contract.getCoprocessorMajorityThreshold()).to.eq(2n);
    const deployed: string[] = JSON.parse(process.env.ARTIFACT_SENDER_KEYS ?? '[]').map((key: string) =>
      new ethers.Wallet(key).address.toLowerCase(),
    );
    expect([...txSenders].sort(), 'the observed databases must belong to the authorized senders').to.deep.eq(
      deployed.sort(),
    );
    return { txSenders };
  } finally {
    provider.destroy();
  }
}

export async function assertOperatorsAgree(
  urls: string[],
  indices: number[],
  handle: string,
  options: { timeoutMs: number },
) {
  const pools = indices.map(
    (i) => new Pool({ connectionString: urls[i], max: 1, connectionTimeoutMillis: 10_000, statement_timeout: 15_000 }),
  );
  const deadline = Date.now() + options.timeoutMs;
  try {
    do {
      const results = await Promise.all(
        pools.map((pool) =>
          pool.query(
            `
        SELECT c.transaction_id,c.host_chain_id,c.block_number,c.fhe_operation,
          ct.ciphertext,ct.ciphertext_version,ct.ciphertext_type,
          d.key_id_gw,d.ciphertext AS digest,d.ciphertext128 AS sns_digest
        FROM computations c JOIN ciphertexts ct ON ct.handle=c.output_handle
        JOIN ciphertext_digest d ON d.handle=c.output_handle AND d.host_chain_id=c.host_chain_id
        WHERE c.output_handle=$1 AND c.is_allowed AND c.is_completed AND NOT c.is_error
          AND d.ciphertext IS NOT NULL AND d.ciphertext128 IS NOT NULL`,
            [Buffer.from(handle.slice(2), 'hex')],
          ),
        ),
      );
      for (const r of results) expect(r.rows.length, 'unique producer and ciphertext version').to.be.at.most(1);
      if (results.every((r) => r.rows.length === 1)) {
        for (const r of results)
          expect(r.rows[0], 'ciphertext bytes, digests, and producer provenance').to.deep.eq(results[0].rows[0]);
        return;
      }
      await new Promise((resolve) => setTimeout(resolve, 1000));
    } while (Date.now() < deadline);
    throw new Error(`operators did not complete ${handle}`);
  } finally {
    await Promise.all(pools.map((pool) => pool.end()));
  }
}

export async function assertQuorumOutcome(options: {
  mode: 'required';
  gatewayRpcUrl: string;
  ciphertextCommitsAddress: string;
  handle: string;
  authorizedSenders: string[];
  threshold: number;
  timeoutMs: number;
}) {
  const provider = new ethers.JsonRpcProvider(options.gatewayRpcUrl);
  const iface = new ethers.Interface([
    'event AddCiphertextMaterialConsensus(bytes32 indexed ctHandle, uint256 keyId, bytes32 ciphertextDigest, bytes32 snsCiphertextDigest, address[] coprocessorTxSenders)',
  ]);
  const deadline = Date.now() + options.timeoutMs;
  // The seed phase records Gateway height before submitting any fixture tx.
  // Search only this run's history, not the whole chain.
  let next = Number(process.env.ARTIFACT_GATEWAY_START ?? 0);
  try {
    do {
      const head = await provider.getBlockNumber();
      if (next <= head) {
        const end = Math.min(head, next + 499);
        const logs = await provider.getLogs({
          address: options.ciphertextCommitsAddress,
          topics: iface.encodeFilterTopics('AddCiphertextMaterialConsensus', [options.handle]),
          fromBlock: next,
          toBlock: end,
        });
        for (const log of logs) {
          const parsed = iface.parseLog(log)!;
          const senders = [...new Set<string>(parsed.args.coprocessorTxSenders.map((s: string) => s.toLowerCase()))];
          expect(senders.length).to.eq(options.threshold);
          expect(senders.sort()).to.deep.eq([...options.authorizedSenders].sort());
          return;
        }
        next = end + 1;
        if (next <= head) continue;
      }
      await new Promise((resolve) => setTimeout(resolve, 1000));
    } while (Date.now() < deadline);
    throw new Error(`no unanimous Gateway consensus for ${options.handle}`);
  } finally {
    provider.destroy();
  }
}
