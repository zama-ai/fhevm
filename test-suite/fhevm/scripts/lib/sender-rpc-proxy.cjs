'use strict';
const http = require('node:http');
const fs = require('node:fs');

// Test-only JSON-RPC transport. Signed transactions still execute on upstream.
function senderProxy({ upstream, token, control, observe }) {
  const destination = new URL(upstream);
  if (destination.protocol !== 'http:' || !/^[a-f0-9]{32}$/.test(token))
    throw new Error('isolated HTTP endpoint/token required');
  const attempts = new Map(),
    dropped = new Set(),
    outgoing = new Set();
  const server = http.createServer((request, response) => {
    if (request.method !== 'POST' || request.url !== `/${token}`) {
      response.writeHead(404).end();
      return;
    }
    const chunks = [];
    let size = 0;
    request.on('data', (chunk) => {
      size += chunk.length;
      if (size > 2 * 1024 * 1024) request.destroy();
      else chunks.push(chunk);
    });
    request.on('end', () => {
      const started = Date.now();
      const body = Buffer.concat(chunks);
      let message, state;
      try {
        message = JSON.parse(body.toString());
        state = control();
        if (
          !['healthy', 'nonce-low', 'accepted-timeout'].includes(state.mode) ||
          !Number.isFinite(state.until) ||
          !Array.isArray(state.handles) ||
          state.handles.some((handle) => !/^0x[a-f0-9]{64}$/.test(handle))
        )
          throw new Error('invalid control');
      } catch {
        response.writeHead(400).end('invalid controlled request');
        return;
      }
      const mode = state.until > Date.now() ? state.mode : 'healthy';
      const raw = message.method === 'eth_sendRawTransactionSync' && message.params?.[0];
      const handles =
        typeof raw === 'string' ? state.handles.filter((handle) => raw.toLowerCase().includes(handle.slice(2))) : [];
      // Sender submits one material per call; reject an ambiguous fault target.
      if (handles.length > 1) {
        response.writeHead(400).end('ambiguous material submission');
        return;
      }
      const handle = handles[0];
      const event = (value) => observe({ at: new Date().toISOString(), method: message.method, handle, ...value });
      if (handle && mode === 'nonce-low') {
        const attempt = (attempts.get(handle) || 0) + 1;
        attempts.set(handle, attempt);
        if (attempt <= 3) {
          event({ mode: 'nonce-low', attempt });
          response.writeHead(200, { 'content-type': 'application/json' });
          response.end(
            JSON.stringify({ jsonrpc: '2.0', id: message.id, error: { code: -32000, message: 'nonce too low' } }),
          );
          return;
        }
      }
      const loseResponse = handle && mode === 'accepted-timeout' && !dropped.has(handle);
      const forwarded = http.request(
        destination,
        { method: 'POST', headers: { 'content-type': 'application/json' }, timeout: 60_000 },
        (incoming) => {
          const reply = [];
          let bytes = 0;
          incoming.on('data', (chunk) => {
            bytes += chunk.length;
            if (bytes > 2 * 1024 * 1024) forwarded.destroy(new Error('oversized upstream response'));
            else reply.push(chunk);
          });
          incoming.on('end', () => {
            let receipt;
            try {
              receipt = JSON.parse(Buffer.concat(reply).toString()).result;
            } catch {
              /* forward malformed replies unchanged */
            }
            const hash = receipt?.transactionHash;
            if (loseResponse && /^0x[a-f0-9]{64}$/i.test(hash || '')) {
              dropped.add(handle);
              event({
                mode: 'accepted-response-dropped',
                hash: hash.toLowerCase(),
                receiptStatus: receipt.status,
                acceptedBeforeCancel: !response.destroyed,
              });
              // No headers: the sender's own deadline must expire. In particular,
              // do not cancel upstream when the caller disconnects: it may accept.
              const cancelled = () =>
                event({ mode: 'client-timeout', hash: hash.toLowerCase(), elapsedMs: Date.now() - started });
              response.on('close', cancelled);
              if (response.destroyed) cancelled();
              return;
            }
            if (handle) event({ mode: 'forwarded', hash, httpStatus: incoming.statusCode });
            if (!response.destroyed) {
              response.writeHead(incoming.statusCode || 502, { 'content-type': 'application/json' });
              response.end(Buffer.concat(reply));
            }
          });
        },
      );
      outgoing.add(forwarded);
      forwarded.on('close', () => outgoing.delete(forwarded));
      forwarded.on('timeout', () => forwarded.destroy(new Error('upstream timeout')));
      forwarded.on('error', () => {
        if (!response.destroyed) response.writeHead(502).end();
      });
      if (!loseResponse) response.on('close', () => forwarded.destroy());
      forwarded.end(body);
    });
  });
  server.stop = () => {
    for (const request of outgoing) request.destroy();
    server.closeAllConnections();
    server.close();
  };
  return server;
}
module.exports = { senderProxy };
if (require.main === module) {
  const [upstream, token, controlFile, readyFile, evidenceFile] = process.argv.slice(2);
  const proxy = senderProxy({
    upstream,
    token,
    control: () => JSON.parse(fs.readFileSync(controlFile, 'utf8')),
    observe: (event) => fs.appendFileSync(evidenceFile, JSON.stringify(event) + '\n'),
  });
  proxy.listen(0, '0.0.0.0', () => fs.writeFileSync(readyFile, JSON.stringify({ port: proxy.address().port })));
  const stop = () => {
    clearInterval(watch);
    proxy.stop();
  };
  const watch = setInterval(() => {
    try {
      if (JSON.parse(fs.readFileSync(controlFile, 'utf8')).mode === 'shutdown') stop();
    } catch {
      /* fail closed on the next request */
    }
  }, 250);
  process.on('SIGTERM', stop);
  process.on('SIGINT', stop);
}
