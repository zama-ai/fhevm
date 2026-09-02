import { execFileSync, spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { PACKAGE_ROOT_ABS_PATH } from './constants.ts';
import { format, resolveConfig } from 'prettier';

////////////////////////////////////////////////////////////////////////////////

export function cast(args: readonly string[]): string {
  try {
    return execFileSync('cast', args, {
      cwd: PACKAGE_ROOT_ABS_PATH,
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'pipe'],
    });
  } catch (error) {
    const failure = error as { stdout?: string; stderr?: string };
    throw new Error(`cast ${args.join(' ')} failed\n${failure.stdout ?? ''}${failure.stderr ?? ''}`, { cause: error });
  }
}

////////////////////////////////////////////////////////////////////////////////

export function forge(args: readonly string[]): void {
  try {
    execFileSync('forge', args, { cwd: PACKAGE_ROOT_ABS_PATH, encoding: 'utf8', stdio: 'pipe' });
  } catch (error) {
    const failure = error as { stdout?: string; stderr?: string };
    throw new Error(`forge ${args.join(' ')} failed\n${failure.stdout ?? ''}${failure.stderr ?? ''}`, { cause: error });
  }
}

////////////////////////////////////////////////////////////////////////////////

export function readJson<T>(path: string): T {
  return JSON.parse(readFileSync(path, 'utf8')) as T;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Writes `value` as pretty JSON with a trailing newline, and returns exactly what it wrote.
 *
 * The return value is what lets a caller hash the file without serializing a second time — two
 * serializations are two things that can drift, and a digest over the wrong bytes is silently wrong.
 */
export function writeJson(path: string, value: unknown): string {
  const json = `${toJsonLiteral(value)}\n`;
  writeFileSync(path, json, 'utf8');
  return json;
}

////////////////////////////////////////////////////////////////////////////////

export function toJsonLiteral(value: unknown): string {
  return JSON.stringify(value, null, 2);
}

////////////////////////////////////////////////////////////////////////////////

export async function writeTypeScript(path: string, source: string): Promise<void> {
  writeFileSync(
    path,
    await format(source, { ...((await resolveConfig(path)) ?? {}), filepath: path, parser: 'typescript' }),
  );
}

////////////////////////////////////////////////////////////////////////////////

export function normalizeHex(value: string, label: string): string {
  if (!/^0x[0-9a-fA-F]*$/.test(value)) {
    throw new Error(`${label} is not a hex string`);
  }

  const hex = value.slice(2).toLowerCase();
  if (hex.length % 2 !== 0) {
    throw new Error(`${label} has an odd hex length`);
  }

  return hex;
}

////////////////////////////////////////////////////////////////////////////////

export function run(command: string, args: readonly string[], cwd: string): void {
  const result = spawnSync(command, args, { cwd, encoding: 'utf8', stdio: 'inherit' });
  if (result.status !== 0) {
    throw new Error(`\`${command} ${args.join(' ')}\` failed in ${cwd} (status ${String(result.status)})`);
  }
}

////////////////////////////////////////////////////////////////////////////////

export async function rpc<T>(url: string, method: string, params: readonly unknown[] = []): Promise<T> {
  const response = await fetch(url, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ jsonrpc: '2.0', id: 1, method, params }),
  });
  const payload = (await response.json()) as { result?: T; error?: { message: string } };
  if (payload.error !== undefined) {
    throw new Error(`${method} failed: ${payload.error.message}`);
  }
  return payload.result as T;
}

////////////////////////////////////////////////////////////////////////////////

export async function waitForNode(url: string, timeoutMs = 20_000): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    try {
      await rpc<string>(url, 'eth_chainId');
      return;
    } catch {
      if (Date.now() > deadline) {
        throw new Error(`timed out waiting for a node at ${url}`);
      }
      await new Promise((r) => setTimeout(r, 200));
    }
  }
}
