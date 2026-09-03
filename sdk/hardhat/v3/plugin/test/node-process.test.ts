// The real `hardhat node`, as a child process with the plugin in its config: the stack must be there
// when the server announces itself, and a second hardhat environment connecting over `localhost` must
// find it without deploying again — the flow `hardhat test --network localhost` relies on.

import assert from 'node:assert/strict';
import { type ChildProcess, spawn } from 'node:child_process';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import { createHardhatRuntimeEnvironment } from 'hardhat/hre';

import plugin from '../pkg/_esm/index.js';
import { LOCALHOST_DEPLOYER } from '../pkg/_esm/internal/constants.js';
import { precomputeLocalhostAddresses } from '../pkg/_esm/internal/deploy.js';

const require = createRequire(import.meta.url);
// The CLI file is not in hardhat's exports map; walk from the package root, which is.
const HARDHAT_CLI = join(dirname(require.resolve('hardhat/package.json')), 'dist', 'src', 'cli.js');
const PLUGIN_DIR = dirname(dirname(fileURLToPath(import.meta.url)));
const CONFIG = join(PLUGIN_DIR, 'test', 'fixtures', 'node.config.ts');

type Node = { readonly child: ChildProcess; readonly url: string; readonly output: string };

// `--port 0` lets the OS pick, so parallel test runs never fight over 8545; the URL is read back
// from the line the node task prints once it listens.
function startNode(extraArgs: readonly string[] = []): Promise<Node> {
  const child = spawn(process.execPath, [HARDHAT_CLI, '--config', CONFIG, ...extraArgs, 'node', '--port', '0'], {
    cwd: PLUGIN_DIR,
    stdio: ['ignore', 'pipe', 'pipe'],
    env: { ...process.env, HARDHAT_DISABLE_TELEMETRY: 'true' },
  });
  return new Promise((resolve, reject) => {
    let output = '';
    const onData = (chunk: Buffer): void => {
      output += chunk.toString();
      const match = /JSON-RPC server at (http:\/\/[^/\s]+)\//.exec(output);
      if (match?.[1] !== undefined) resolve({ child, url: match[1], output });
    };
    child.stdout.on('data', onData);
    child.stderr.on('data', onData);
    child.once('exit', (code) => {
      reject(new Error(`hardhat node exited with ${String(code)} before listening:\n${output}`));
    });
  });
}

function stopNode(node: Node): Promise<void> {
  return new Promise((resolve) => {
    node.child.once('exit', () => {
      resolve();
    });
    node.child.kill('SIGTERM');
  });
}

async function rpc(url: string, method: string, params: unknown[] = []): Promise<unknown> {
  const response = await fetch(url, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ jsonrpc: '2.0', id: 1, method, params }),
  });
  const body = (await response.json()) as { result?: unknown; error?: unknown };
  assert.equal(body.error, undefined, `${method} failed: ${JSON.stringify(body.error)}`);
  return body.result;
}

void test('a hardhat node child process serves the stack, and a localhost connection reuses it', async () => {
  const node = await startNode();
  try {
    const { aclAddress } = precomputeLocalhostAddresses().fhevmAddresses;
    // The fhevm banner is the operator's evidence, printed before hardhat announces the server: one
    // line on a plain run, the addresses only when verbose (checked in the test below).
    const banner = node.output.indexOf('fhevm: cleartext FHEVM stack on');
    assert.notEqual(banner, -1, node.output);
    assert.ok(!node.output.includes(aclAddress), 'a plain run lists no address');
    assert.ok(banner < node.output.indexOf('JSON-RPC server at'), 'the banner precedes the server announcement');
    assert.notEqual(
      await rpc(node.url, 'eth_getCode', [aclAddress, 'latest']),
      '0x',
      'ACL code when the node announces',
    );
    const nonce = await rpc(node.url, 'eth_getTransactionCount', [LOCALHOST_DEPLOYER.address, 'latest']);
    const block = await rpc(node.url, 'eth_blockNumber');

    const hre = await createHardhatRuntimeEnvironment({
      plugins: [plugin],
      networks: { localhost: { type: 'http', url: node.url } },
    });
    const connection = await hre.network.create('localhost');
    try {
      assert.equal(connection.fhevm.network.kind, 'localhost');
      assert.equal(connection.fhevm.isCleartext, true);
      assert.equal(await rpc(node.url, 'eth_getTransactionCount', [LOCALHOST_DEPLOYER.address, 'latest']), nonce);
      assert.equal(await rpc(node.url, 'eth_blockNumber'), block, 'the second process deployed nothing');
    } finally {
      await connection.close();
    }
  } finally {
    await stopNode(node);
  }
});

void test('a verbose hardhat node lists the stack addresses in its banner', async () => {
  // hardhat's verbosity is the count of v's (default 2): -vvv is the first verbose level.
  const node = await startNode(['-vvv']);
  try {
    const { aclAddress, fhevmExecutorAddress } = precomputeLocalhostAddresses().fhevmAddresses;
    assert.ok(node.output.includes('fhevm: cleartext FHEVM stack on'), node.output);
    assert.ok(node.output.includes(`ACL`) && node.output.includes(aclAddress), node.output);
    assert.ok(node.output.includes(fhevmExecutorAddress), node.output);
    assert.ok(node.output.includes(`deployer (ACL owner)`), node.output);
  } finally {
    await stopNode(node);
  }
});
