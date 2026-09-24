'use strict';
const http = require('node:http');
const fs = require('node:fs');

// A bounded storage-capacity response, never host disk exhaustion. Only the
// named object's PUT is rejected; signed headers and all other traffic pass on.
function storageProxy({ upstream, control, observe }) {
  const destination = new URL(upstream);
  if (destination.protocol !== 'http:' || destination.username || destination.password || destination.pathname !== '/') throw new Error('isolated HTTP storage root required');
  return http.createServer((request, response) => {
    if (!request.url.startsWith('/') || request.url.startsWith('//')) { response.writeHead(400).end(); return; }
    let state;
    try { state = control(); } catch { response.writeHead(503).end(); return; }
    const pathname = new URL(request.url, destination).pathname;
    if (!['healthy', 'reject'].includes(state.mode) || !Number.isFinite(state.until) ||
        (state.mode === 'reject' && !/^\/coproc-1\/ct128\/[0-9a-f]{64}\/1$/.test(state.path))) {
      response.writeHead(503).end(); return;
    }
    const selected = request.method === 'PUT' && pathname === state.path;
    if (selected && state.mode === 'reject' && state.until > Date.now()) {
      const body = '<Error><Code>InsufficientStorage</Code><Message>Isolated test write capacity exhausted</Message></Error>';
      observe({ mode: 'rejected', method: 'PUT', path: pathname, status: 507, at: new Date().toISOString() });
      request.resume();
      response.writeHead(507, { 'content-type': 'application/xml', 'content-length': Buffer.byteLength(body), 'retry-after': '1' }).end(body);
      return;
    }
    // Preserve Host as well as authorization/checksum headers: changing signed
    // fields would turn storage recovery into an unrelated signature failure.
    const forwarded = http.request(new URL(request.url, destination), {
      method: request.method, headers: request.headers, timeout: 30_000,
    }, incoming => {
      response.writeHead(incoming.statusCode || 502, incoming.headers);
      if (selected) observe({ mode: 'forwarded', method: request.method, path: pathname, status: incoming.statusCode, at: new Date().toISOString() });
      incoming.pipe(response);
    });
    forwarded.on('timeout', () => forwarded.destroy(new Error('storage upstream timeout')));
    forwarded.on('error', () => { if (!response.headersSent) response.writeHead(502); response.end(); });
    response.on('close', () => forwarded.destroy());
    request.pipe(forwarded);
  });
}
module.exports = { storageProxy };
if (require.main === module) {
  const [upstream, controlFile, readyFile, evidenceFile] = process.argv.slice(2);
  const server = storageProxy({ upstream, control: () => JSON.parse(fs.readFileSync(controlFile, 'utf8')),
    observe: event => fs.appendFileSync(evidenceFile, JSON.stringify(event) + '\n') });
  server.listen(0, '0.0.0.0', () => fs.writeFileSync(readyFile, JSON.stringify({ port: server.address().port })));
  const shutdown = () => { clearInterval(watch); server.closeAllConnections(); server.close(() => process.exit(0)); };
  const watch = setInterval(() => {
    try { if (JSON.parse(fs.readFileSync(controlFile, 'utf8')).mode === 'shutdown') shutdown(); } catch { /* Requests fail closed. */ }
  }, 250);
  process.on('SIGINT', shutdown); process.on('SIGTERM', shutdown);
}
