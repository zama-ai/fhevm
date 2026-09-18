import { defineConfig, type ProxyOptions } from 'vite';

import { readDemoAuthorizationFromEnv } from '../../test-suite/fhevm/demo/authorization';

// The page is static; everything privileged lives in the demo operator (test-suite/fhevm/demo/
// operator-server.ts). The dev server proxies the page's same-origin `/api` calls to it and adds
// the boot capability on the way, so the browser never holds a token and the operator never trusts
// a page context. `/api/relayer` goes straight to the relayer.
const dappUrl = new URL(process.env.DEMO_DAPP_URL ?? 'http://127.0.0.1:5173');
const operatorUrl = process.env.DEMO_OPERATOR_URL ?? 'http://127.0.0.1:8091';
const relayerUrl = process.env.DEMO_RELAYER_URL ?? 'http://127.0.0.1:3000';

const operatorProxy = async (): Promise<ProxyOptions> => {
  const authorization = await readDemoAuthorizationFromEnv();
  return {
    target: operatorUrl,
    rewrite: (requestPath) => requestPath.replace(/^\/api/, ''),
    configure: (proxy) => {
      proxy.on('proxyReq', (proxyRequest) => {
        proxyRequest.setHeader('authorization', `Bearer ${authorization.token}`);
        proxyRequest.setHeader('x-fhevm-demo-boot-id', authorization.bootId);
      });
    },
  };
};

export default defineConfig(async ({ mode }) => ({
  server: {
    host: dappUrl.hostname,
    port: Number(dappUrl.port),
    strictPort: true,
    proxy: {
      // Listed first: proxy contexts match in order and `/api` would otherwise take it.
      '/api/relayer': {
        target: relayerUrl,
        rewrite: (requestPath: string) => requestPath.replace(/^\/api\/relayer/, ''),
      },
      // Vitest needs only transforms; the proxy (and its capability) exists only for a served page.
      ...(mode === 'test' ? {} : { '/api': await operatorProxy() }),
    },
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin',
    },
  },

  build: {
    rollupOptions: {
      input: ['index.html', 'architecture.html'],
    },
  },
}));
