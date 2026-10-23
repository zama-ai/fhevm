import { expect, test } from 'bun:test';
import { createServer } from 'node:http';
import { createRequire } from 'node:module';

import { artifactReplayRoute } from './artifact-recovery';

const { rpcProxy } = createRequire(import.meta.url)('../../scripts/lib/host-rpc-proxy.cjs');

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

test('startup overlap and explicit cursor replay retain unrelated deployed poller options', () => {
  const input = ['--batch-size=4', '--database-url=private', '--url=http://proxy', '--replay-window', '64'];
  expect(artifactReplayRoute(input, 'overlap')).toEqual([...input.slice(0, 3), '--replay-window=512']);
  expect(artifactReplayRoute(input, 'cursor')).toEqual([...input.slice(0, 3), '--replay-window=1']);
  for (const command of [['--replay-window'], ['--replay-window=x'], ['--replay-window=1', '--replay-window=2'], [1]]) {
    expect(() => artifactReplayRoute(command, 'cursor')).toThrow();
  }
  expect(() => artifactReplayRoute(input, 'unrecognized')).toThrow();
});

test('runner cleanup never resumes publication over an unhealed graph or an unquiesced test', async () => {
  const { readFileSync, mkdtempSync, rmSync, existsSync } = await import('node:fs');
  const { tmpdir } = await import('node:os');
  const path = await import('node:path');
  const source = readFileSync(new URL('../../scripts/run-artifact-ingestion-recovery.sh', import.meta.url), 'utf8');
  const cleanup = source.match(/cleanup\(\) \{[\s\S]*?\n\}\ntrap cleanup EXIT/)![0].replace(/\ntrap cleanup EXIT$/, '');
  for (const [cancel, unhealed] of [
    [0, 0],
    [1, 0],
    [0, 1],
  ]) {
    const dir = mkdtempSync(path.join(tmpdir(), 'artifact-cleanup-'));
    try {
      const child = Bun.spawnSync(
        [
          'bash',
          '-c',
          `
        hc_cleanup_signals() { :; }
        hc_begin_cleanup() { :; }
        sp_cancel_all() { echo cancelled; return "$CANCEL_FAILURE"; }
        sp_recover_suite_state() { echo recovered-journal; }
        restore_route() { echo restored-route; }
        sc_run_restores() { echo resumed-publisher; }
        rs_finalize_results() { echo "result:$1:$2"; }
        sp_dispose() { :; }
        ${cleanup}
        cleanup
      `,
        ],
        {
          env: {
            ...process.env,
            CANCEL_FAILURE: String(cancel),
            NEEDS_HEAL: String(unhealed),
            PROXY_PID: '',
            SP_FORCED_STOP: '0',
            SP_CONTAMINATION: path.join(dir, 'contaminated'),
            PRIVATE: dir,
            DETECTOR: 'test-detector',
          },
        },
      );
      const out = child.stdout.toString();
      expect(child.exitCode).toBe(cancel || unhealed ? 1 : 0);
      expect(out.includes('resumed-publisher')).toBe(!cancel && !unhealed);
      expect(out.includes('restored-route')).toBe(!cancel);
      expect(existsSync(path.join(dir, 'contaminated'))).toBe(Boolean(unhealed));
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  }
});
