import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    passWithNoTests: true,
    clearMocks: true,
    include: ['**/*.spec.ts'],
    reporters: ['verbose'],
    testTimeout: 120_000,
    coverage: {
      enabled: true,
      all: false,
      provider: 'istanbul',
      include: ['src/**'],
      reporter: ['json-summary', 'html'],
    },
  },
})
