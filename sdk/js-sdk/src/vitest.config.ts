import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    include: ['src/**/*.test.ts'],
    exclude: ['src/index.hello.test.ts'],
    reporters: ['verbose'],
    passWithNoTests: false,
    typecheck: { enabled: true },
    coverage: {
      provider: 'v8',
      reporter: ['text'],
      //include: ['src/**/*.ts'],
      //exclude: ['src/**/*.test.ts', 'src/**/*.test-d.ts', 'src/vitest.config.ts'],
    },
  },
});
