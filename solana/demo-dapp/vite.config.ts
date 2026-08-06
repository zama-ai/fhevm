import { fileURLToPath } from 'node:url';

import { defineConfig } from 'vite';

import { demoServerPlugin } from './demoServerPlugin';

export default defineConfig(({ mode }) => ({
  resolve: {
    // Bun links local package files; keep the consumer path so SDK dependencies resolve from this lockfile.
    preserveSymlinks: true,
    alias: {
      // The vault module (src/vault) reaches into the built SDK for internals the published
      // package does not export (fhevm-internal#1859 §6d). Resolving inside node_modules keeps
      // one module instance with the package's own exports. Mirrored in tsconfig.*.json paths.
      '@sdk-src': fileURLToPath(new URL('./node_modules/@fhevm/sdk/_esm', import.meta.url)),
    },
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
  ssr: {
    // Operator routes load the local SDK through Vite. Bundle it so its runtime dependencies
    // resolve from this app's frozen graph instead of the file-linked SDK source directory.
    noExternal: ['@fhevm/sdk'],
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
