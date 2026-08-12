import { fileURLToPath } from 'node:url';

import { defineConfig } from 'vite';

import { demoServerPlugin } from './demoServerPlugin';

export default defineConfig(({ mode }) => ({
  resolve: {
    // `node_modules/@fhevm/sdk` is a symlink into the SDK source tree (this package's postinstall
    // swaps bun's `file:` snapshot for it). Default symlink resolution follows it to the real
    // path, so the SDK's own runtime dependencies resolve from the SDK's location (the
    // repository-root workspace graph) rather than this lockfile.
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
    // Operator routes load the local SDK through Vite. Bundle it so it goes through the same
    // resolution and transforms as the rest of the app instead of being required as an external.
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
