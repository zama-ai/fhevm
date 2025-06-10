import swc from "unplugin-swc";
import tsconfigPaths from "vite-tsconfig-paths";
import { defineConfig, type Plugin } from "vitest/config";
import { config } from "dotenv";

export default defineConfig({
  test: {
    include: ["**/*.e2e-spec.ts"],
    globals: true,
    root: "./",
    env: {
      ...config({ path: ".env.test" }).parsed,
    },
    isolate: true,
    pool: "forks",
    poolOptions: {
      forks: {
        isolate: true,
        maxForks: 10,
      },
    },
    globalSetup: "./tests/setup.e2e.ts",
  },
  plugins: [
    // This is required to build the test files with SWC
    swc.vite({
      // Explicitly set the module type to avoid inheriting this value from a `.swcrc` config file
      module: { type: "es6" },
    }) as Plugin,
    tsconfigPaths(),
  ],
});
