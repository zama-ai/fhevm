// Stand-in for prf-service behind each coprocessor-proxy of the e2e stack, until prf-service ships.
// Over HTTP/1.1, `GET /healthz` answers 200 and the RFC-038 PRF route, with prf-service's canonical
// parameters, answers a fixed binary body with an `X-Prf-Attestation` header that holds its digest.
// A plain-HTTP port answers `GET /count` with how many PRF requests reached the fake, so the e2e
// suite can check that a request the proxy rejects is never forwarded. Node 22.18+ runs this file
// directly: `node fake-prf-service.ts`.
import http from 'node:http';

const PRF_PORT = 8080;
const COUNT_PORT = 9090;
// Decimal `prf` and `padLength` without leading zeros, and a 64-lowercase-hex label.
const PRF_ROUTE = /^\/v1\/prf\/(0|[1-9][0-9]*)\/labels\/[0-9a-f]{64}\/[1-9][0-9]*$/;
// Every byte value, so a client that decodes the body as text changes it.
const OUTPUT = Buffer.from(Array.from({ length: 256 }, (_, i) => i));
// Keccak-256 of OUTPUT: Node has no Keccak, and the fake runs without dependencies.
const OUTPUT_DIGEST = '0xdc924469b334aed2a19fac7252e9961aea41f8d91996366029dbe0884229bf36';
const ATTESTATION = JSON.stringify({ version: 1, sns_ciphertext_digest: OUTPUT_DIGEST });

let prfRequests = 0;

http
  .createServer((req, res) => {
    if (req.method === 'GET' && req.url === '/healthz') {
      res.writeHead(200).end();
    } else if ((req.method === 'GET' || req.method === 'HEAD') && PRF_ROUTE.test(req.url ?? '')) {
      prfRequests += 1;
      res.writeHead(200, { 'content-type': 'application/octet-stream', 'x-prf-attestation': ATTESTATION }).end(OUTPUT);
    } else {
      res.writeHead(404).end();
    }
  })
  .listen(PRF_PORT);

http
  .createServer((req, res) => {
    if (req.method === 'GET' && req.url === '/count') {
      res.writeHead(200, { 'content-type': 'application/json' }).end(JSON.stringify({ prfRequests }));
    } else {
      res.writeHead(404).end();
    }
  })
  .listen(COUNT_PORT);

// PID 1 in its container: exit on `docker stop` instead of waiting for the kill.
process.on('SIGTERM', () => process.exit(0));
