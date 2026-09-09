import { createSolanaRpc } from '@solana/kit';
import { createHash } from 'node:crypto';
import pg from 'pg';

/** Every deployer for these on-chain identities must use the same coordination database. */
export const withDeploymentLock = async <T>(
  databaseUrl: string,
  rpcUrl: string,
  hostProgramId: string,
  deploy: (signal: AbortSignal) => Promise<T>,
): Promise<T> => {
  const client = new pg.Client({ connectionString: databaseUrl, connectionTimeoutMillis: 10_000, query_timeout: 10_000 });
  const controller = new AbortController();
  client.on('error', () => controller.abort(new Error('deployment lock connection lost')));
  try {
    let genesis: string;
    try {
      await client.connect();
      genesis = await createSolanaRpc(rpcUrl).getGenesisHash().send({ abortSignal: AbortSignal.timeout(10_000) });
    } catch {
      throw new Error('cannot connect to deployment lock database or Solana RPC');
    }
    const key = createHash('sha256')
      .update(`solana-deploy:${genesis}:${hostProgramId}`)
      .digest()
      .readBigInt64BE()
      .toString();
    const { rows } = await client.query<{ acquired: boolean }>('SELECT pg_try_advisory_lock($1::bigint) AS acquired', [
      key,
    ]);
    if (!rows[0]?.acquired) throw new Error('another deployment is running for this Solana host');
    controller.signal.throwIfAborted();
    const result = await deploy(controller.signal);
    controller.signal.throwIfAborted();
    return result;
  } finally {
    // Closing the session releases the lock, including when deployment fails.
    await client.end();
  }
};
