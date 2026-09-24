'use strict';
const http = require('node:http');
const https = require('node:https');
const fs = require('node:fs');
const MODES = new Set(['healthy', '429', '503', '408', 'reset', 'stall', 'tls-trust']);
function rpcProxy({ upstream, token, control, observe, tls }) {
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
      } catch { response.writeHead(400).end('invalid controlled request'); return; }
      const selected = messages.find(message => message.method === 'eth_getLogs');
      const mode = state.until > Date.now() ? state.mode : 'healthy';
      if (selected && mode !== 'healthy' && mode !== 'tls-trust') {
        observe({ mode, at: new Date().toISOString(), method: selected.method, params: selected.params });
        if (mode === 'reset') { request.socket.destroy(); return; }
        if (mode === 'stall') {
          // Send no headers or bytes. Only the client's own deadline can
          // satisfy the aborted observation while the control remains armed.
          const started = Date.now();
          let released = false;
          const release = setInterval(() => {
            let current;
            try { current = control(); } catch { return; }
            if (current.mode !== 'stall' || current.until <= Date.now()) {
              released = true;
              request.socket.destroy();
            }
          }, 100);
          release.unref();
          request.socket.once('close', () => {
            clearInterval(release);
            let current;
            try { current = control(); } catch { return; }
            if (!released && current.mode === 'stall' && current.until > Date.now()) {
              observe({ mode: 'stall-aborted', at: new Date().toISOString(), method: selected.method,
                params: selected.params, elapsedMs: Date.now() - started });
            }
          });
          return;
        }
        response.writeHead(Number(mode), { 'content-type': 'application/json', 'retry-after': '1' });
        response.end(JSON.stringify({ jsonrpc: '2.0', id: selected.id, error: { code: -32000, message: `isolated host RPC ${mode}` } }));
        return;
      }
      const forwarded = http.request(destination, { method: 'POST', headers: { 'content-type': 'application/json' }, timeout: 30_000 }, incoming => {
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
  const server = tls ? https.createServer(tls.valid, handler) : http.createServer(handler);
  let trusted = true;
  server.refreshTrust = () => {
    if (!tls) return;
    const state = control();
    const next = state.mode !== 'tls-trust' || state.until <= Date.now();
    if (next !== trusted) {
      trusted = next;
      server.setSecureContext(trusted ? tls.valid : tls.invalid);
      server.setTicketKeys(require('node:crypto').randomBytes(48));
      server.closeAllConnections();
    }
  };
  if (tls) server.on('tlsClientError', error => {
    if (!trusted) observe({ mode: 'tls-trust', at: new Date().toISOString(), error: error.message });
  });
  return server;
}
module.exports = { rpcProxy };
if (require.main === module) {
  const [upstream, token, controlFile, readyFile, evidenceFile, tlsDirectory] = process.argv.slice(2);
  const tls = tlsDirectory ? { valid: { key: fs.readFileSync(tlsDirectory + '/server.key'), cert: fs.readFileSync(tlsDirectory + '/server.crt') }, invalid: { key: fs.readFileSync(tlsDirectory + '/bad.key'), cert: fs.readFileSync(tlsDirectory + '/bad.crt') } } : undefined;
  const server = rpcProxy({ upstream, token, tls, control: () => JSON.parse(fs.readFileSync(controlFile, 'utf8')),
    observe: event => fs.appendFileSync(evidenceFile, JSON.stringify(event) + '\n') });
  server.listen(0, '0.0.0.0', () => fs.writeFileSync(readyFile, JSON.stringify({ port: server.address().port, token })));
  const shutdown = () => { clearInterval(watch); server.closeAllConnections(); server.close(() => process.exit(0)); };
  const watch = setInterval(() => {
    try { if (JSON.parse(fs.readFileSync(controlFile, 'utf8')).mode === 'shutdown') shutdown(); else server.refreshTrust(); } catch { /* Requests fail closed until a valid control returns. */ }
  }, 250);
  process.on('SIGINT', shutdown); process.on('SIGTERM', shutdown);
}
