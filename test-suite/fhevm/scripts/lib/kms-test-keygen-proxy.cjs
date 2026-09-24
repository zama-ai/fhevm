'use strict';
// Isolated test topology only. Preserve protobuf messages and signed responses;
// select the KMS's explicitly insecure preprocessing/keygen APIs, never decrypt APIs.
const http2 = require('node:http2');
const prefix = '/kms_service.v1.CoreServiceEndpoint/';
const methods = new Map([
  ['KeyGenPreproc', 'InsecureKeyGenPreproc'],
  ['GetKeyGenPreprocResult', 'GetInsecureKeyGenPreprocResult'],
  ['KeyGen', 'InsecureKeyGen'],
  ['GetKeyGenResult', 'GetInsecureKeyGenResult'],
]);
function rewritePath(path) {
  if (!path.startsWith(prefix)) return path;
  const method = path.slice(prefix.length);
  return methods.has(method) ? prefix + methods.get(method) : path;
}
function createProxy(destination, observe = () => {}) {
  if (new URL(destination).protocol !== 'http:') throw new Error('test KMS proxy requires an isolated plaintext endpoint');
  const server = http2.createServer();
  server.on('session', session => session.on('error', () => {}));
  server.on('stream', (client, headers) => {
    const original = headers[':path'];
    const selected = rewritePath(original);
    const connection = http2.connect(destination);
    let upstream, trailers = {}, responded = false;
    const unavailable = () => {
      if (client.destroyed || client.closed) return;
      if (!responded) {
        responded = true;
        client.respond({ ':status': 200, 'content-type': 'application/grpc', 'grpc-status': '14' });
        client.end();
      } else client.close(http2.constants.NGHTTP2_INTERNAL_ERROR);
    };
    connection.on('error', unavailable);
    client.on('error', () => {});
    client.on('close', () => { upstream?.destroy(); connection.destroy(); });
    try {
      upstream = connection.request({ ...headers, ':authority': new URL(destination).host, ':path': selected });
      upstream.on('error', unavailable);
      upstream.on('response', response => {
        if (client.destroyed || client.closed) return;
        responded = true;
        client.respond(response, { waitForTrailers: true });
      });
      upstream.on('trailers', value => { trailers = value; });
      client.on('wantTrailers', () => {
        if (!client.destroyed && !client.closed) client.sendTrailers(trailers);
      });
      upstream.on('end', () => observe({ path: original, forwarded: selected, grpcStatus: trailers['grpc-status'] ?? null }));
      client.pipe(upstream);
      upstream.pipe(client);
    } catch { unavailable(); connection.destroy(); }
  });
  return server;
}
module.exports = { createProxy, rewritePath };
if (require.main === module) {
  const [destination, port = '3000'] = process.argv.slice(2);
  const server = createProxy(destination, event => console.log(JSON.stringify(event)));
  server.listen(Number(port), '0.0.0.0', () => console.log('INSECURE_TEST_KEYGEN_PROXY_READY'));
}
