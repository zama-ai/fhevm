import { expect, test } from 'bun:test';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { dbRevertAnchorProgram, dbRevertCanonicalAnchor } from '../commands/test';
import { run } from '../utils/process';

test('revert anchor uses the selected network canonical head and rejects another chain', async () => {
  const requests: string[] = [];
  const server = Bun.serve({ port: 0, async fetch(request) {
    const body = await request.json() as { method: string };
    requests.push(body.method);
    return Response.json({ result: body.method === 'eth_chainId' ? '0x3039' : { number: '0x64' } });
  } });
  const dir = mkdtempSync(path.join(tmpdir(), 'db-revert-rpc-'));
  try {
    mkdirSync(path.join(dir, 'node_modules/hardhat'), { recursive: true });
    writeFileSync(path.join(dir, 'node_modules/hardhat/index.js'), `exports.network={provider:{send:async(method)=>
      (await(await fetch('http://127.0.0.1:${server.port}',{method:'POST',body:JSON.stringify({method})})).json()).result}};`);
    const execute: typeof run = async (args, options) => {
      expect(args).toContain('HARDHAT_NETWORK=staging');
      expect(options?.timeoutMs).toBe(75_000);
      return run(['node', '-e', args.at(-1)!], { cwd: dir });
    };
    expect(await dbRevertCanonicalAnchor('staging', '12345', execute)).toBe(100);
    expect(requests).toEqual(['eth_chainId', 'eth_getBlockByNumber']);
    await expect(dbRevertCanonicalAnchor('staging', '67890', execute)).rejects.toThrow('does not match CHAIN_ID');
  } finally { server.stop(true); rmSync(dir, { recursive: true, force: true }); }
});

test('canonical RPC probe terminates inside its process when the provider hangs', async () => {
  const dir = mkdtempSync(path.join(tmpdir(), 'db-revert-rpc-timeout-'));
  try {
    mkdirSync(path.join(dir, 'node_modules/hardhat'), { recursive: true });
    writeFileSync(path.join(dir, 'node_modules/hardhat/index.js'), 'exports.network={provider:{send:()=>new Promise(()=>{})}};');
    const result = await run(['node', '-e', dbRevertAnchorProgram(100)], { cwd: dir, timeoutMs: 2000, allowFailure: true });
    expect(result.code).toBe(124);
    expect(result.stderr).not.toContain('timed out after');
  } finally { rmSync(dir, { recursive: true, force: true }); }
});
