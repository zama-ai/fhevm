import { spawnSync } from 'node:child_process';
import { resolve } from 'node:path';

const localstackRestartScript = resolve(import.meta.dirname, '../../scripts/localstack-restart.sh');

export async function ensureLocalstackReady(parameters: {
  readonly restart: boolean;
  readonly rpcUrl: string;
  readonly chainName: string;
}): Promise<void> {
  const { restart, rpcUrl, chainName } = parameters;

  if (restart) {
    runLocalstackRestart(chainName);
  }

  if (await isJsonRpcReady(rpcUrl)) {
    return;
  }

  if (!restart) {
    throw new Error(
      `Localstack JSON-RPC is not responding at ${rpcUrl}. ` + 'Start it first, or rerun with --restart-localstack.',
    );
  }

  throw new Error(`Localstack JSON-RPC is still not responding at ${rpcUrl} after restart.`);
}

export async function isJsonRpcReady(rpcUrl: string): Promise<boolean> {
  try {
    const response = await fetch(rpcUrl, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        jsonrpc: '2.0',
        id: 1,
        method: 'eth_chainId',
        params: [],
      }),
    });

    if (!response.ok) {
      return false;
    }

    const body = (await response.json()) as { readonly result?: unknown };
    return typeof body.result === 'string';
  } catch {
    return false;
  }
}

function runLocalstackRestart(chainName: string): void {
  const result = spawnSync(localstackRestartScript, ['--force', '--chain', chainName], {
    cwd: resolve(import.meta.dirname, '../../..'),
    stdio: 'inherit',
  });

  if (result.status !== 0) {
    throw new Error(`localstack restart failed: ${localstackRestartScript}`);
  }
}
