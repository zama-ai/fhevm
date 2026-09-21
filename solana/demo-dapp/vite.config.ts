import { defineConfig } from 'vite';

import { demoServerPlugin } from './demoServerPlugin';

export default defineConfig(({ mode }) => ({
  server: {
    host: '127.0.0.1',
    port: 5173,
    strictPort: true,
    proxy: {
      '/api/relayer': {
        target: 'http://127.0.0.1:3000',
        rewrite: (requestPath) => requestPath.replace(/^\/api\/relayer/, ''),
      },
    },
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin',
    },
  },
  ssr: {
    // Operator routes load the local SDK through Vite. Bundle it so it goes through the same
    // resolution and transforms as the rest of the app instead of being required as an external.
    noExternal: ['@fhevm/sdk', '@fhevm/confidential-token'],
  },
  // Vitest needs only transforms; omitting the development server plugin keeps tests independent
  // of runtime credentials without creating a credential bypass mode.
  plugins: mode === 'test' ? [] : [demoServerPlugin()],
  build: {
    rollupOptions: {
      input: ['index.html', 'architecture.html'],
    },
  },
}));
