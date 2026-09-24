'use strict';
const http = require('node:http'), fs = require('node:fs'), crypto = require('node:crypto');
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
function keyProxy({upstream, key, control, observe, wrongKey}) {
  if (!/^[a-f0-9]{64}$/.test(key) || new URL(upstream).protocol !== 'http:') throw new Error('invalid isolated key route');
  const server = http.createServer((request, response) => {
    if (request.method !== 'GET') { response.writeHead(405).end(); return; }
    const destination = new URL(request.url, upstream);
    if (destination.origin !== new URL(upstream).origin) { response.writeHead(400).end(); return; }
    const selected = destination.pathname.endsWith('/CompressedXofKeySet/' + key);
    const forwarded = http.get(destination, incoming => {
      if (!selected || incoming.statusCode !== 200) { response.writeHead(incoming.statusCode || 502, incoming.headers); incoming.pipe(response); return; }
      const chunks = [];
      incoming.on('data', chunk => chunks.push(chunk));
      incoming.on('end', () => {
        const original = Buffer.concat(chunks);
        // The control is rewritten by the holder while requests are in flight;
        // an unreadable or half-written control must fail this one response,
        // not the proxy that every later retry still routes through.
        let state;
        try { state = control(); } catch (error) {
          observe({mode:'control-unreadable', key, error: String(error && error.message || error), at:new Date().toISOString()});
          response.writeHead(503).end(); return;
        }
        const mode = state.until > Date.now() ? state.mode : 'healthy';
        if (!original.length) { response.writeHead(502).end(); return; }
        if (mode === 'interrupt') {
          response.writeHead(200, {'content-length': original.length});
          const partial = original.subarray(0, Math.max(1, Math.floor(original.length / 2)));
          response.write(partial, () => observe({mode, key, bytes:partial.length, total:original.length, at:new Date().toISOString()}));
          // Keep the real GET incomplete until its owning listener is killed.
          response.on('close', () => observe({mode:'interrupted-close',key,at:new Date().toISOString()}));
          return;
        }
        let body = Buffer.from(original);
        if (mode === 'wrong-digest') body[body.length - 1] ^= 1;
        else if (mode === 'malformed') body = Buffer.from('not-a-serialized-compressed-key');
        else if (mode === 'wrong-key') body = fs.readFileSync(wrongKey);
        else if (mode !== 'healthy') { response.writeHead(503).end(); return; }
        const originalHash = hash(original), servedHash = hash(body);
        if (mode !== 'healthy' && originalHash === servedHash) { response.writeHead(500).end('ineffective fault'); return; }
        response.writeHead(200, {'content-length':body.length});
        response.end(body, () => observe({mode,key,originalHash,servedHash,bytes:body.length,at:new Date().toISOString()}));
      });
    });
    forwarded.on('error', () => { if (!response.headersSent) response.writeHead(502); response.end(); });
    response.on('close', () => forwarded.destroy());
  });
  return server;
}
module.exports = {keyProxy};
if (require.main === module) {
  const [upstream,key,controlFile,readyFile,evidenceFile,wrongKey] = process.argv.slice(2);
  const server = keyProxy({upstream,key,wrongKey,control:()=>JSON.parse(fs.readFileSync(controlFile)),observe:e=>fs.appendFileSync(evidenceFile,JSON.stringify(e)+'\n')});
  server.listen(0,'0.0.0.0',()=>fs.writeFileSync(readyFile,JSON.stringify({port:server.address().port})));
  const shutdown=()=>{clearInterval(watch);server.closeAllConnections();server.close(()=>process.exit(0));};
  const watch=setInterval(()=>{try { const state=JSON.parse(fs.readFileSync(controlFile)); if(state.mode==='shutdown'||state.until<Date.now())shutdown(); }catch{}},250);
  process.on('SIGTERM',shutdown);process.on('SIGINT',shutdown);
}
