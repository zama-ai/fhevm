import { expect, test } from 'bun:test';

import { type SenderRpcEvent, assertSenderFault } from '../../../e2e/test/consensus/senderRpcEvidence';
import { senderRoute } from './sender-rpc-route';

const handles = Array.from({ length: 12 }, (_, index) => `0x${(index + 1).toString(16).padStart(64, '0')}`);
test('sender route preserves signer and database arguments and applies the same finite policy to both binaries', () => {
  const source = [
    'transaction_sender',
    '--gateway-url',
    'http://gateway:8546',
    '--database-url=private-db',
    '--private-key=private-key',
    '--add-ciphertexts-max-retries=15',
  ];
  const route = senderRoute(source, 'http://proxy:1234/token');
  expect(route.original).toBe('http://gateway:8546');
  expect(route.command).toContain('--database-url=private-db');
  expect(route.command).toContain('--private-key=private-key');
  expect(route.command).toContain('--gateway-url=http://proxy:1234/token');
  expect(route.command).toContain('--add-ciphertexts-max-retries=3');
  expect(route.command).not.toContain('--add-ciphertexts-max-retries=15');
  for (const command of [
    [],
    ['--gateway-url'],
    ['--gateway-url=ws://gateway'],
    ['--gateway-url=http://one', '--gateway-url=http://two'],
  ])
    expect(() => senderRoute(command)).toThrow();
});

test('fault evidence requires all twelve original handles, not a healthy proxy or a partial fault', () => {
  const events: SenderRpcEvent[] = handles.flatMap((handle) =>
    [1, 2, 3].map((attempt) => ({ handle, attempt, mode: 'nonce-low' })),
  );
  assertSenderFault(events, handles, 'nonce-low');
  for (const incomplete of [
    [],
    events.slice(1),
    events.filter((event) => event.handle !== handles[11]),
    events.map((event) => ({ ...event, mode: 'forwarded' })),
  ])
    expect(() => assertSenderFault(incomplete, handles, 'nonce-low')).toThrow();
  expect(() => assertSenderFault(events, handles.slice(1), 'nonce-low')).toThrow();
  const accepted: SenderRpcEvent[] = handles.flatMap((handle) => [
    { handle, mode: 'accepted-response-dropped', hash: handle, receiptStatus: '0x1', acceptedBeforeCancel: true },
    { handle, mode: 'client-timeout', hash: handle, elapsedMs: 4000 },
  ]);
  assertSenderFault(accepted, handles, 'accepted-timeout');
  for (const incomplete of [
    accepted.filter((event) => event.mode !== 'client-timeout'),
    accepted.map((event) => ({ ...event, acceptedBeforeCancel: false })),
    accepted.map((event) => ({ ...event, elapsedMs: 0 })),
    accepted.map((event) => ({ ...event, receiptStatus: '0x0' })),
    accepted.slice(1),
  ])
    expect(() => assertSenderFault(incomplete, handles, 'accepted-timeout')).toThrow();
});

test('Node proxy bounds nonce failures and retains upstream execution after a client timeout', async () => {
  const child = Bun.spawn(
    [
      'node',
      '-e',
      `
    const assert = require('node:assert/strict'), http = require('node:http');
    const { senderProxy } = require(process.argv[1]);
    (async () => {
      const handle='0x'+'ab'.repeat(32), hash='0x'+'cd'.repeat(32), token='a'.repeat(32);
      let mode='nonce-low', delay=0; const events=[], received=[];
      const upstream=http.createServer((req,res)=>{
        let body=''; req.on('data',chunk=>body+=chunk); req.on('end',()=>{
          received.push(JSON.parse(body));
          setTimeout(()=>res.end(JSON.stringify({jsonrpc:'2.0',id:7,result:{transactionHash:hash,status:'0x1'}})),delay);
        });
      });
      await new Promise(resolve=>upstream.listen(0,'127.0.0.1',resolve));
      const proxy=senderProxy({upstream:'http://127.0.0.1:'+upstream.address().port,token,
        control:()=>({mode,handles:[handle],until:Date.now()+60000}),observe:event=>events.push(event)});
      await new Promise(resolve=>proxy.listen(0,'127.0.0.1',resolve));
      const url='http://127.0.0.1:'+proxy.address().port+'/'+token;
      const request={jsonrpc:'2.0',id:7,method:'eth_sendRawTransactionSync',params:['0x02'+handle.slice(2)]};
      const send=(value=request,ms=2000)=>fetch(url,{method:'POST',body:JSON.stringify(value),signal:AbortSignal.timeout(ms)});
      try {
        for(let i=1;i<=3;i++) assert.equal((await (await send()).json()).error.message,'nonce too low');
        assert.equal(received.length,0);
        assert.equal((await (await send()).json()).result.transactionHash,hash);
        assert.deepEqual(received,[request]);
        const read={...request,method:'eth_getTransactionReceipt',params:[hash]};
        await (await send(read)).text(); assert.deepEqual(received[1],read);
        mode='accepted-timeout'; delay=200;
        await assert.rejects(send(request,50));
        for(let i=0;i<100&&!events.some(e=>e.mode==='client-timeout');i++) await new Promise(r=>setTimeout(r,10));
        assert.ok(events.some(e=>e.mode==='accepted-response-dropped'&&e.hash===hash));
        assert.ok(events.some(e=>e.mode==='client-timeout'&&e.hash===hash));
        assert.deepEqual(received[2],request);
        delay=0;
        assert.equal((await (await send()).json()).result.transactionHash,hash);
        assert.equal(events.filter(e=>e.mode==='accepted-response-dropped').length,1);
        assert.equal((await fetch(url+'wrong',{method:'POST'})).status,404);
      } finally {proxy.stop();upstream.closeAllConnections();upstream.close();}
    })().catch(error=>{console.error(error);process.exitCode=1});
  `,
      new URL('../../scripts/lib/sender-rpc-proxy.cjs', import.meta.url).pathname,
    ],
    { stdout: 'pipe', stderr: 'pipe', timeout: 10_000 },
  );
  const [status, errors] = await Promise.all([child.exited, new Response(child.stderr).text()]);
  expect(errors).toBe('');
  expect(status).toBe(0);
}, 15_000);
