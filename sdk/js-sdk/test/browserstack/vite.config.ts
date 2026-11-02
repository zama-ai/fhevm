import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';

const __dirname = dirname(fileURLToPath(import.meta.url));
const projectRoot = resolve(__dirname, '..', '..');

export default defineConfig({
  root: projectRoot,
  envDir: resolve(__dirname, '..'),
  envPrefix: ['MNEMONIC'],
  server: {
    port: 3333,
    // Loopback only: the served modules embed MNEMONIC (see envPrefix), so the
    // server must not be reachable from the network. BrowserStack Local
    // tunnels bs-local.com back to this loopback address.
    host: '127.0.0.1',
    // Vite 7's DNS-rebinding protection blocks unrecognized Host headers;
    // bs-local.com is BrowserStack Local's dedicated tunnel-back domain (see
    // playwright.config.ts baseURL).
    allowedHosts: ['bs-local.com'],
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
});
