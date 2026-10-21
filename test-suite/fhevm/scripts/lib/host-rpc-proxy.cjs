'use strict';
const http = require('node:http');
const fs = require('node:fs');
const MODES = new Set(['healthy', 'omit-acl']);
function rpcProxy({ upstream, token, control, observe }) {
  const destination = new URL(upstream);
  if (destination.protocol !== 'http:' || !/^[a-f0-9]{32}$/.test(token)) throw new Error('isolated HTTP endpoint/token required');
  const handler = (request, response) => {
    if (request.url !== `/${token}` || request.method !== 'POST') { response.writeHead(404).end(); return; }
    const chunks = []; let size = 0;
    request.on('data', chunk => { size += chunk.length; if (size > 2 * 1024 * 1024) request.destroy(); else chunks.push(chunk); });
    request.on('end', () => {
      let messages, state;
      try {
        const message = JSON.parse(Buffer.concat(chunks).toString());
        messages = Array.isArray(message) ? message : [message];
        state = control();
        if (!MODES.has(state.mode) || !Number.isFinite(state.until)) throw new Error('invalid fault control');
        if (state.mode === 'omit-acl' && (!/^0x[0-9a-f]{64}$/.test(state.transactionHash) || !/^0x[0-9a-f]{40}$/.test(state.address))) throw new Error('exact ACL transaction and address required');
      } catch { response.writeHead(400).end('invalid controlled request'); return; }
      const selected = messages.find(message => message.method === 'eth_getLogs');
      const mode = state.until > Date.now() ? state.mode : 'healthy';
      const forwarded = http.request(destination, { method: 'POST', headers: { 'content-type': 'application/json' }, timeout: 30_000 }, incoming => {
        if (selected && mode === 'omit-acl') {
          // Model a partial delivery, retaining the real block/transaction/log
          // identities. Only the named producer transaction's ACL logs vanish;
          // executor events and every other transaction remain untouched.
          const parts = []; let bytes = 0;
          incoming.on('data', part => { bytes += part.length; if (bytes > 256 * 1024 * 1024) forwarded.destroy(new Error('RPC response too large')); else parts.push(part); });
          incoming.on('end', () => {
            try {
              const body = JSON.parse(Buffer.concat(parts).toString());
              const rows = Array.isArray(body) ? body : [body];
              const ids = new Set(messages.filter(m => m.method === 'eth_getLogs').map(m => m.id));
              let removed = 0;
              for (const row of rows) if (ids.has(row.id) && Array.isArray(row.result)) {
                row.result = row.result.filter(log => {
                  const omit = log.address?.toLowerCase() === state.address && log.transactionHash?.toLowerCase() === state.transactionHash;
                  if (omit) removed++;
                  return !omit;
                });
              }
              observe({ mode, at: new Date().toISOString(), method: selected.method, params: selected.params, status: incoming.statusCode, removed, transactionHash: state.transactionHash });
              response.writeHead(incoming.statusCode || 502, { 'content-type': 'application/json' });
              response.end(JSON.stringify(body));
            } catch { response.writeHead(502).end('invalid upstream RPC response'); }
          });
          return;
        }
        response.writeHead(incoming.statusCode || 502, { 'content-type': incoming.headers['content-type'] || 'application/json' });
        if (selected) observe({ mode: 'healthy', at: new Date().toISOString(), method: selected.method, params: selected.params, status: incoming.statusCode });
        incoming.pipe(response);
      });
      forwarded.on('timeout', () => forwarded.destroy(new Error('upstream timeout')));
      forwarded.on('error', () => { if (!response.headersSent) response.writeHead(502); response.end(); });
      response.on('close', () => forwarded.destroy());
      forwarded.end(Buffer.concat(chunks));
    });
  };
  return http.createServer(handler);
}
module.exports = { rpcProxy };
if (require.main === module) {
  const [upstream, token, controlFile, readyFile, evidenceFile] = process.argv.slice(2);
  const server = rpcProxy({ upstream, token, control: () => JSON.parse(fs.readFileSync(controlFile, 'utf8')),
    observe: event => fs.appendFileSync(evidenceFile, JSON.stringify(event) + '\n') });
  server.listen(0, '0.0.0.0', () => fs.writeFileSync(readyFile, String(server.address().port)));
}
