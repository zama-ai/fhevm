import { expect, test } from 'bun:test';
import { createServer } from 'node:http';
import { createRequire } from 'node:module';

const { rpcProxy } = createRequire(import.meta.url)('../scripts/lib/host-rpc-proxy.cjs');

test('ACL fault removes only the named transaction at the ACL address and heals the identical request', async () => {
  const tx = `0x${'a'.repeat(64)}`,
    acl = `0x${'b'.repeat(40)}`;
  const otherTx = `0x${'c'.repeat(64)}`,
    executor = `0x${'d'.repeat(40)}`;
  const token = 'a'.repeat(32);
  const log = {
    address: acl,
    transactionHash: tx,
    blockHash: `0x${'e'.repeat(64)}`,
    blockNumber: '0x31',
    topics: ['topic'],
    data: '0x1234',
    logIndex: '0x2',
  };
  const logs = [log, { ...log, address: executor }, { ...log, transactionHash: otherTx }];
  let mode = 'omit-acl';
  let address = acl;
  const events: any[] = [];
  const upstream = createServer((request, response) => {
    let body = '';
    request.on('data', (chunk) => {
      body += chunk;
    });
    request.on('end', () => {
      const messages = JSON.parse(body);
      const reply = (message: any) => ({ jsonrpc: '2.0', id: message.id, result: logs });
      response.writeHead(200, { 'content-type': 'application/json' });
      // Deliberately reverse batch responses: identify by id, not order.
      response.end(JSON.stringify(Array.isArray(messages) ? messages.map(reply).reverse() : reply(messages)));
    });
  });
  await new Promise<void>((resolve) => upstream.listen(0, '127.0.0.1', resolve));
  const proxy = rpcProxy({
    upstream: `http://127.0.0.1:${(upstream.address() as any).port}`,
    token,
    control: () => ({ mode, transactionHash: tx, address, until: Date.now() + 60_000 }),
    observe: (event: unknown) => events.push(event),
  });
  await new Promise<void>((resolve) => proxy.listen(0, '127.0.0.1', resolve));
  const request = { jsonrpc: '2.0', id: 7, method: 'eth_getLogs', params: [{ blockHash: log.blockHash }] };
  const send = (body: unknown) =>
    fetch(`http://127.0.0.1:${proxy.address().port}/${token}`, {
      method: 'POST',
      body: JSON.stringify(body),
      signal: AbortSignal.timeout(2000),
    });
  try {
    expect(((await (await send(request)).json()) as any).result).toEqual(logs.slice(1));
    expect(events[0]).toMatchObject({ mode: 'omit-acl', removed: 1, transactionHash: tx, status: 200 });
    const batch = (await (
      await send([request, { ...request, id: 8, method: 'eth_getTransactionReceipt' }])
    ).json()) as any[];
    expect(batch.find((row) => row.id === 7).result).toEqual(logs.slice(1));
    expect(batch.find((row) => row.id === 8).result).toEqual(logs);
    address = '';
    expect((await send(request)).status).toBe(400);
    address = acl;
    mode = 'healthy';
    expect(((await (await send(request)).json()) as any).result).toEqual(logs);
  } finally {
    proxy.closeAllConnections();
    proxy.close();
    upstream.closeAllConnections();
    upstream.close();
  }
});

const { configuration, routedCommand } = createRequire(import.meta.url)('../scripts/lib/artifact-poller-config.cjs');

test('poller clone retains deployed options and rejects ambiguous or shared database routes', () => {
  const original = {
    NetworkSettings: { Networks: { isolated: { IPAddress: '172.18.0.2' } } },
    Mounts: [],
    State: { Running: true },
    Image: 'sha256:original',
    Config: {
      Cmd: [
        'host_listener_poller',
        '--url',
        'http://host:8545',
        '--database-url=postgres://victim',
        '--finality-lag=2',
      ],
      Env: ['GATEWAY_URL=http://gateway:8546'],
      Entrypoint: [],
      User: '1000',
    },
  };
  const reference = structuredClone(original);
  reference.Config.Cmd[3] = '--database-url=postgres://reference';
  const sender = { Config: { Cmd: [`--private-key=0x${'a'.repeat(64)}`] } };
  const config = configuration(original, reference, original, [sender, sender]);
  expect(config.image).toBe(original.Image);
  expect(JSON.parse(config.phaseEnv.ARTIFACT_DB_URLS)).toEqual(['postgres://reference', 'postgres://victim']);
  expect(routedCommand(original.Config.Cmd, 'http://proxy')).toEqual([
    'host_listener_poller',
    '--url',
    'http://proxy',
    '--database-url=postgres://victim',
    '--finality-lag=2',
  ]);
  expect(() => routedCommand(['--url=x', '--url=y'], 'http://proxy')).toThrow();
  expect(() => configuration(original, original, original, [sender, sender])).toThrow();
  expect(() => configuration({ ...original, Mounts: [{}] }, reference, original, [sender, sender])).toThrow();
});

test('cleanup restores only owned services and never resumes them after failed cancellation', async () => {
  const { readFileSync } = await import('node:fs');
  const source = readFileSync(new URL('../scripts/run-artifact-ingestion-recovery.sh', import.meta.url), 'utf8');
  const cleanup = source.match(/cleanup\(\) \{[\s\S]*?\n\}\ntrap cleanup EXIT/)![0].replace(/\ntrap cleanup EXIT$/, '');
  for (const failure of [false, true]) {
    const child = Bun.spawnSync(
      [
        'bash',
        '-c',
        `
      PHASES=(phase); STOPPED=(original-poller original-consumer)
      CLONE_CREATED=1; COMPLETE=1; PROXY_PID=''; PRIVATE=private
      SCRIPT_DIR='${new URL('../scripts', import.meta.url).pathname}'
      TEST_CONTAINER=suite; CLONE=clone
      dk() { echo "docker:$*" >&2; [[ "$1" != exec || "$FAIL" != 1 ]]; }
      ${cleanup}
      cleanup
    `,
      ],
      { env: { ...process.env, FAIL: failure ? '1' : '0' } },
    );
    const log = child.stderr.toString();
    expect(child.exitCode).toBe(failure ? 1 : 0);
    expect(log.includes('docker:start original-poller')).toBe(!failure);
    expect(log.includes('docker:start original-consumer')).toBe(!failure);
    expect(log.includes('docker:rm -f clone')).toBe(true);
  }
});

test('phase supervisor enforces the deadline and rejects a previously cancelled launch', async () => {
  const { readFileSync, rmSync } = await import('node:fs');
  const { randomUUID } = await import('node:crypto');
  const supervisor = readFileSync(new URL('../scripts/lib/container-phase.cjs', import.meta.url), 'utf8');
  const token = `artifact_${randomUUID().replaceAll('-', '')}`;
  try {
    const run = Bun.spawn(
      ['node', '-e', supervisor, 'run', token, String(Date.now() + 500), 'node', '-e', 'setInterval(()=>{},1000)'],
      { stdout: 'pipe', stderr: 'pipe' },
    );
    expect(await run.exited).toBe(124);
    const cancel = Bun.spawnSync(['node', '-e', supervisor, 'cancel', token]);
    expect(cancel.exitCode).toBe(0);
    const rejected = Bun.spawnSync([
      'node',
      '-e',
      supervisor,
      'run',
      token,
      String(Date.now() + 5000),
      'node',
      '-e',
      'process.stdout.write("unexpected launch")',
    ]);
    expect(rejected.exitCode).toBe(143);
    expect(rejected.stdout.toString()).toBe('');
  } finally {
    rmSync(`/tmp/fhevm-consensus-phases/${token}.cancelled`, { force: true });
    rmSync(`/tmp/fhevm-consensus-phases/${token}.json`, { force: true });
  }
}, 15000);
