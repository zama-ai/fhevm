import tsconfigPaths from 'vite-tsconfig-paths'
import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

export default defineConfig({
  test: {
    include: ['**/*.test.{ts,tsx}'],
    environment: 'jsdom',
    globals: true,
    root: './',
    env: {
      VITE_BACK_HTTP_URL: 'http://localhost:3005/graphql',
      VITE_BACK_WS_URL: 'ws://localhost:3005/graphql',
    },
    setupFiles: './test/setup.ts',
  },
  plugins: [tsconfigPaths(), react()],
})
