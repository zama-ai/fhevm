import { createServer as createHttpsServer } from 'node:https';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { getCertificate } from '@vitejs/plugin-basic-ssl';
import { createServer as createViteServer } from 'vite';

/**
 * Dev server for the browser smoke tests: Vite in middleware mode behind a
 * plain `node:https` server with a throwaway self-signed cert.
 *
 * HTTPS is required so the page is a secure context: `crypto.subtle` (used to
 * verify WASM digests) and COOP/COEP cross-origin isolation only exist there,
 * and plain-HTTP bs-local.com (BrowserStack real devices) does not qualify.
 *
 * Vite's own `server.https` always serves HTTP/2, which breaks through the
 * BrowserStack Local tunnel on Android Chrome (every module request fails
 * with net::ERR_HTTP2_PROTOCOL_ERROR). `node:https` speaks HTTP/1.1 only.
 */

const __dirname = dirname(fileURLToPath(import.meta.url));
const port = 3333;

const pem = await getCertificate(resolve(__dirname, '../../node_modules/.vite/browser-smoke-ssl'), 'browser-smoke', [
  'localhost',
  'bs-local.com',
]);
const httpsServer = createHttpsServer({ key: pem, cert: pem });

const vite = await createViteServer({
  configFile: resolve(__dirname, 'vite.config.ts'),
  appType: 'mpa',
  server: {
    middlewareMode: true,
    hmr: { server: httpsServer },
  },
});
httpsServer.on('request', vite.middlewares);

// Bind on all interfaces, not just loopback: BrowserStack Local's real
// devices navigate to bs-local.com, not `localhost`.
httpsServer.listen(port, '0.0.0.0', () => {
  console.log(`browser-smoke server ready at https://localhost:${port}/`);
});
