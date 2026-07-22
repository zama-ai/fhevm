import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));

// COOP/COEP enable cross-origin isolation (SharedArrayBuffer → multi-threaded
// TFHE). Toggled by the runner for the with/without-COOP matrix axis; default on.
const coopEnabled = process.env.FHEVM_TEST_COOP !== '0';

// The standalone gateway server (anvil RPC proxy + mini-relayer). Same-origin
// `/gw/*` is rewritten to it so the browser never makes a cross-origin request.
// Keep the default in sync with GATEWAY_PORT in test/infra/config.ts.
const gatewayPort = process.env.FHEVM_TEST_GATEWAY_PORT ?? '8590';

/** @type {import('next').NextConfig} */
const nextConfig = {
  turbopack: {
    root: __dirname,
  },
  async headers() {
    if (!coopEnabled) {
      return [];
    }
    return [
      {
        source: '/:path*',
        headers: [
          { key: 'Cross-Origin-Opener-Policy', value: 'same-origin' },
          { key: 'Cross-Origin-Embedder-Policy', value: 'require-corp' },
        ],
      },
    ];
  },
  async rewrites() {
    return [
      {
        source: '/gw/:path*',
        destination: `http://127.0.0.1:${gatewayPort}/gw/:path*`,
      },
    ];
  },
};

export default nextConfig;
