import { existsSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';

export type LocalstackEnv = {
  readonly rpcUrl: string;
};

const localstackEnvPath = resolve(import.meta.dirname, '../../.env.localstack');
const localstackRestartScript = resolve(import.meta.dirname, '../../scripts/localstack-restart.sh');

export function loadLocalstackEnv(): LocalstackEnv {
  const env = parseEnvFile(localstackEnvPath);
  const rpcUrl = env.RPC_URL ?? process.env.RPC_URL;

  if (rpcUrl === undefined || rpcUrl === '') {
    throw new Error(`RPC_URL is missing. Set it in ${localstackEnvPath} or as an environment variable.`);
  }

  return Object.freeze({ rpcUrl });
}

export async function ensureLocalstackReady(parameters: { readonly restart: boolean }): Promise<LocalstackEnv> {
  const env = loadLocalstackEnv();

  if (parameters.restart) {
    runLocalstackRestart();
  }

  if (await isJsonRpcReady(env.rpcUrl)) {
    return env;
  }

  if (!parameters.restart) {
    throw new Error(
      `Localstack JSON-RPC is not responding at ${env.rpcUrl}. ` +
        'Start it first, or rerun with --restart-localstack.',
    );
  }

  throw new Error(`Localstack JSON-RPC is still not responding at ${env.rpcUrl} after restart.`);
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

function runLocalstackRestart(): void {
  const result = spawnSync(localstackRestartScript, ['--force'], {
    cwd: resolve(import.meta.dirname, '../../..'),
    stdio: 'inherit',
  });

  if (result.status !== 0) {
    throw new Error(`localstack restart failed: ${localstackRestartScript}`);
  }
}

function parseEnvFile(filePath: string): Record<string, string> {
  if (!existsSync(filePath)) {
    return {};
  }

  const result: Record<string, string> = {};
  for (const line of readFileSync(filePath, 'utf-8').split('\n')) {
    const trimmed = line.trim();
    if (trimmed === '' || trimmed.startsWith('#')) {
      continue;
    }

    const eqIndex = trimmed.indexOf('=');
    if (eqIndex === -1) {
      continue;
    }

    const key = trimmed.slice(0, eqIndex).trim();
    let value = trimmed.slice(eqIndex + 1).trim();
    if ((value.startsWith('"') && value.endsWith('"')) || (value.startsWith("'") && value.endsWith("'"))) {
      value = value.slice(1, -1);
    }
    result[key] = value;
  }

  return result;
}
