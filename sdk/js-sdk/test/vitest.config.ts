import { join } from "node:path";
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    alias: [{ find: "~test", replacement: join(__dirname, "./src") }],
    benchmark: {
      outputFile: "./bench/report.json",
      reporters: process.env.CI ? ["default"] : ["verbose"],
    },
    coverage: {
      provider: "v8",
      reporter: process.env.CI ? ["lcov"] : ["text", "json", "html"],
      exclude: [
        "**/account-abstraction/**",
        "**/errors/utils.ts",
        "**/zksync/**",
        "**/_cjs/**",
        "**/_esm/**",
        "**/_types/**",
        "**/*.bench.ts",
        "**/*.bench-d.ts",
        "**/*.test.ts",
        "**/*.test-d.ts",
        "**/test/**",
      ],
    },
    exclude: ["**/node_modules/**", "**/_esm/**", "**/_cjs/**", "**/_types/**"],
    retry: 3,
    projects: [
      {
        extends: true,
        test: {
          name: "core",
          include: ["src/**/*.test.ts"],
          setupFiles: [join(__dirname, "./setup.ts")],
          globalSetup: [join(__dirname, "./setup.global.ts")],
          hookTimeout: 60_000,
          testTimeout: 60_000,
          sequence: { groupOrder: 0 },
        },
      },
    ],
  },
});
