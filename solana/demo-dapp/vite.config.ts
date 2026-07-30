import { defineConfig } from 'vite';

import { demoServerPlugin } from './demoServerPlugin';

export default defineConfig(({ mode }) => ({
  resolve: {
    // Bun links local package files; keep the consumer path so SDK dependencies resolve from this lockfile.
    preserveSymlinks: true,
  },
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
  // Vitest needs only transforms; omitting the development server plugin keeps tests independent
  // of runtime credentials without creating a credential bypass mode.
  plugins: mode === 'test' ? [] : [demoServerPlugin()],
  build: {
    rollupOptions: {
      input: ['index.html', 'architecture.html'],
    },
  },
}));
