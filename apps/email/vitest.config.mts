import swc from "unplugin-swc";
import tsconfigPaths from "vite-tsconfig-paths";
import { defineConfig, type Plugin } from "vitest/config";
import { config } from "dotenv";

export default defineConfig({
  test: {
    include: ["**/*.spec.ts"],
    globals: true,
    root: "./",
    env: {
      ...config({ path: ".env.test" }).parsed,
    },
    setupFiles: "./tests/setup.ts",
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
