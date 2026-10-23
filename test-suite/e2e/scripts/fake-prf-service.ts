// Stand-in for prf-service behind each coprocessor-proxy of the e2e stack, until prf-service ships.
// Over HTTP/1.1, `GET /healthz` answers 200 and the RFC-038 PRF route answers a fixed body with
// an `X-Prf-Attestation` header.
// A plain-HTTP port answers `GET /count` with how many PRF requests reached the fake, so the e2e
// suite can check that a request the proxy rejects is never forwarded. Node 22.18+ runs this file
// directly: `node fake-prf-service.ts`.
import http from 'node:http';

const PRF_PORT = 8080;
const COUNT_PORT = 9090;
const PRF_ROUTE = /^\/v1\/prf\/[^/]+\/labels\/[^/]+\/[^/]+$/;

let prfRequests = 0;

http
  .createServer((req, res) => {
    if (req.method === 'GET' && req.url === '/healthz') {
      res.writeHead(200).end();
    } else if ((req.method === 'GET' || req.method === 'HEAD') && PRF_ROUTE.test(req.url ?? '')) {
      prfRequests += 1;
      res
        .writeHead(200, { 'content-type': 'application/octet-stream', 'x-prf-attestation': 'fake-prf-attestation' })
        .end('fake-prf-output');
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
