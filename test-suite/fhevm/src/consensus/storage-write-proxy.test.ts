import { expect, test } from "bun:test";
import { storageRoute } from "./storage-write-route";

test("write endpoint override observes AWS precedence and preserves the image identity", () => {
  const snapshot = [{ Image: `sha256:${'a'.repeat(64)}`, Config: {
    Labels: { 'com.docker.compose.service': 'coprocessor1-sns-worker' },
    Env: ['AWS_ENDPOINT_URL=http://general:9000', 'AWS_ENDPOINT_URL_S3=http://specific:9000', 'PRIVATE_KEY=retain'],
  } }];
  const route = storageRoute(snapshot, 'http://172.18.0.2:1234');
  expect(route.upstream).toBe('http://specific:9000');
  expect((route.override as any).services['coprocessor1-sns-worker']).toEqual({ image: snapshot[0]!.Image,
    environment: { AWS_ENDPOINT_URL: 'http://172.18.0.2:1234', AWS_ENDPOINT_URL_S3: 'http://172.18.0.2:1234' } });
  for (const url of ['http://proxy:1234', 'http://localhost:1234', 'http://user:password@host', 'https://host', 'http://host/path', 'http://host?override']) expect(() => storageRoute(snapshot, url)).toThrow();
  expect(() => storageRoute([{ ...snapshot[0], Image: 'mutable-tag' }])).toThrow();
  const wrong = structuredClone(snapshot); wrong[0]!.Config.Labels['com.docker.compose.service'] = 'another-worker';
  expect(() => storageRoute(wrong)).toThrow();
});

test("real Node proxy rejects only the named PUT then forwards intact signed headers and bytes", async () => {
  const child = Bun.spawn(['node', '-e', `
    const assert = require('node:assert/strict'), http = require('node:http');
    const { storageProxy } = require(process.argv[1]);
    (async () => {
      const events = [], writes = [], target = '/coproc-1/ct128/' + 'ab'.repeat(32) + '/1';
      const state = { mode: 'reject', path: target, until: Date.now() + 60000 };
      const upstream = http.createServer((req,res) => {
        const chunks=[]; req.on('data', c => chunks.push(c)); req.on('end', () => {
          writes.push({ method:req.method, path:req.url, headers:req.headers, body:Buffer.concat(chunks).toString() });
          res.writeHead(200).end('stored');
        });
      });
      await new Promise(resolve => upstream.listen(0, '127.0.0.1', resolve));
      const proxy = storageProxy({ upstream:'http://127.0.0.1:' + upstream.address().port, control:() => state, observe:e => events.push(e) });
      await new Promise(resolve => proxy.listen(0, '127.0.0.1', resolve));
      const url='http://127.0.0.1:' + proxy.address().port;
      const send = (path=target, method='PUT') => fetch(url+path, { method, body:method==='PUT'?'opaque-ciphertext':undefined,
        headers:{ authorization:'test-signature', 'x-amz-checksum-sha256':'test-checksum' }, signal:AbortSignal.timeout(2000) });
      try {
        for(let i=0;i<2;i++) { const r=await send(); assert.equal(r.status,507); assert.match(await r.text(),/InsufficientStorage/); }
        assert.equal(writes.length,0);
        assert.equal((await send('/coproc-2/ct128/'+'ab'.repeat(32)+'/1')).status,200);
        assert.equal((await send(target,'GET')).status,200);
        state.mode='healthy';
        assert.equal(await (await send()).text(),'stored');
        const recovered = writes.at(-1);
        assert.equal(recovered.path,target); assert.equal(recovered.body,'opaque-ciphertext');
        assert.equal(recovered.headers.authorization,'test-signature');
        assert.equal(recovered.headers['x-amz-checksum-sha256'],'test-checksum');
        assert.equal(recovered.headers.host, '127.0.0.1:' + proxy.address().port);
        state.mode='reject'; state.until=Date.now()-1;
        assert.equal((await send()).status,200);
        assert.equal(events.filter(e=>e.mode==='rejected').length,2);
        assert.ok(events.some(e=>e.mode==='forwarded' && e.status===200));
      } finally { proxy.closeAllConnections(); proxy.close(); upstream.closeAllConnections(); upstream.close(); }
    })().catch(e=>{ console.error(e); process.exitCode=1; });
  `, new URL('../../scripts/lib/storage-write-proxy.cjs', import.meta.url).pathname], {
    stdout: 'pipe', stderr: 'pipe', timeout: 5_000,
  });
  const [status, errors] = await Promise.all([child.exited, new Response(child.stderr).text()]);
  expect(errors).toBe(''); expect(status).toBe(0);
});
